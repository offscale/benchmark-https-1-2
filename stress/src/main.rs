use anyhow::Context;
use argh::FromArgs;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::time::{Duration, Instant};
use url::Url;

use hyper::body::HttpBody;
use hyper::http::{Request, StatusCode};
use hyper::{client::conn, Body};
use std::time::SystemTime;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
struct Stat {
    time_to_connect: Duration,
    time_to_first_byte: Duration,
    time_to_last_byte: Duration,
    time_to_completion: Duration,
    body_size: usize,
}

#[derive(Debug, Clone)]
struct Ctx {
    pb: ProgressBar,
}

async fn fetch_video(url: &Url, ctx: &Ctx) -> anyhow::Result<Stat> {
    let now = Instant::now();

    let host = url.host_str().context("host")?;
    let port = url.port_or_known_default().context("port")?;
    let connstr = format!("{host}:{port}");

    dbg!(&connstr);

    let target_stream = TcpStream::connect(&connstr).await?;

    let (mut request_sender, connection) =
        conn::handshake(target_stream).await?;

    // spawn a task to poll the connection and drive the HTTP state
    // XXX: get rid of task
    let conn_handler = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let time_to_connect = now.elapsed();

    let date: time::OffsetDateTime = SystemTime::now().into();
    let date = date.format(&time::format_description::well_known::Rfc2822)?;

    dbg!(&date);

    let request = Request::builder()
        .method("GET")
        .header("User-Agent", "blah/1.0")
        .header("Host", host)
        .header(hyper::header::DATE, date)
        .uri(url.path())
        .body(Body::from(""))?;

    dbg!(&request);

    let response = request_sender.send_request(request).await?;

    if response.status() != StatusCode::OK {
        anyhow::bail!("Invalid StatusCode: {:?}", response.status());
    }

    let mut body: Body = response.into_body();

    let mut body_size: usize = 0;

    if let Some(buf) = body.data().await {
        let buf = buf?;
        body_size += buf.len();
        ctx.pb.inc(buf.len() as u64);
    }

    let time_to_first_byte = now.elapsed();

    while let Some(buf) = body.data().await {
        let buf = buf?;
        body_size += buf.len();
        ctx.pb.inc(buf.len() as u64);
    }

    let time_to_last_byte = now.elapsed();

    drop(request_sender);
    conn_handler.await?;

    let time_to_completion = now.elapsed();

    let stat = Stat {
        time_to_connect,
        time_to_first_byte,
        time_to_last_byte,
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
    for _req_no in 0..num_reqs {
        match fetch_video(&url, &ctx).await {
            Ok(_stat) => {}
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
