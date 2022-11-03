use argh::FromArgs;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;

async fn fetch_video(url: &str, pb: &ProgressBar) -> anyhow::Result<usize> {
    let res = reqwest::get(url).await?;

    // println!("Response: {:?}", res);

    let mut res = res.error_for_status()?;

    let content_length = res.content_length();

    let mut total_size: usize = 0;

    while let Some(chunk) = res.chunk().await? {
        // println!("Chunk: {}", chunk.len());
        pb.inc(chunk.len() as u64);
        total_size += chunk.len();
    }

    match (content_length, total_size) {
        (Some(len), total) if len as usize == total => {}
        _ => {
            println!("Length-mismatch or unknown content-length");
        }
    }

    Ok(total_size)
}

async fn client(
    _client_id: usize,
    num_reqs: usize,
    pb: ProgressBar,
    url: String,
) -> anyhow::Result<()> {
    for _req_no in 0..num_reqs {
        let _ = fetch_video(&url, &pb).await?;
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
    url: String,
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

    let clients: Vec<_> = (0..opts.clients)
        .into_iter()
        .map(|client_id: usize| {
            let pb = pb.clone();
            let url = opts.url.clone();
            tokio::task::spawn(async move {
                client(client_id, reqs_per_client, pb, url).await
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
