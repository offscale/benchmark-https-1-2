use argh::FromArgs;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use reqwest::Client;
use std::fmt::Write;
use std::time::{Duration, Instant};
use url::Url;

#[derive(Debug, Clone)]
struct Stat {
    time_to_first_byte: Duration,
    time_to_completion: Duration,
    body_size: usize,
}

#[derive(Debug, Clone)]
struct Ctx {
    pb: ProgressBar,
}

impl Ctx {
    fn report_download_progress(&self, len: usize) {
        self.pb.inc(len as u64);
    }
}

async fn fetch_video(
    url: &Url,
    client: Client,
    ctx: &Ctx,
) -> anyhow::Result<Stat> {
    let now = Instant::now();

    let resp = client.get(url.clone()).send().await?;

    let mut resp = resp.error_for_status()?;

    let content_length = resp.content_length();

    let time_to_first_byte = now.elapsed();

    let mut body_size: usize = 0;

    while let Some(chunk) = resp.chunk().await? {
        ctx.report_download_progress(chunk.len());
        body_size += chunk.len();
    }

    match (content_length, body_size) {
        (Some(len), total) if len as usize == total => {}
        _ => {
            eprintln!("Length-mismatch or unknown content-length");
        }
    }

    drop(resp);

    let time_to_completion = now.elapsed();

    let stat = Stat {
        time_to_first_byte,
        time_to_completion,
        body_size,
    };

    Ok(stat)
}

async fn client(
    _client_id: usize,
    num_reqs: usize,
    ctx: Ctx,
    url: Url,
) -> anyhow::Result<()> {
    let client = Client::builder().build()?;

    for _req_no in 0..num_reqs {
        match fetch_video(&url, client.clone(), &ctx).await {
            Ok(stat) => {
                dbg!(stat);
            }
            Err(err) => {
                eprintln!("ERR: {err:?}");
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, FromArgs)]
/// HTTP benchmark
struct Options {
    /// number of clients
    #[argh(option, short = 'C')]
    clients: usize,

    /// number of total requests
    #[argh(option, short = 'N')]
    requests: usize,

    /// URL to fetch
    #[argh(positional)]
    url: Url,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Options = argh::from_env();
    println!("{opts:?}");

    let reqs_per_client = opts.requests / opts.clients;

    let pb = ProgressBar::new(opts.requests as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
          .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let ctx = Ctx { pb };

    let clients: Vec<_> = (0..opts.clients)
        .into_iter()
        .map(|client_id: usize| {
            let ctx = ctx.clone();
            let url = opts.url.clone();
            tokio::task::spawn(async move {
                client(client_id, reqs_per_client, ctx, url).await
            })
        })
        .collect();

    let joiner = tokio::task::spawn(async move {
        for client in clients {
            let _result = client.await.unwrap();
        }
    });

    let () = joiner.await?;

    Ok(())
}
