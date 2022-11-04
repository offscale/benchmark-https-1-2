use argh::FromArgs;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::time::{Duration, Instant};
use url::Url;

#[derive(Debug, Clone)]
struct Ctx {
    pb: ProgressBar,
}

impl Ctx {
    fn report_download_progress(&self, len: usize) {
        self.pb.inc(len as u64);
    }
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

    let pb = ProgressBar::new(opts.requests as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}")
        .unwrap()
        .progress_chars("#>-"));

    let ctx = Ctx { pb };

    let bm = Benchmark {
        url: opts.url.clone(),
        num_clients: opts.clients,
        reqs_per_client: opts.requests / opts.clients,
    };

    let bm_res: BenchmarkResult = bm.run(ctx).await?;

    println!("{}", serde_json::to_string(&bm_res.to_json())?);

    Ok(())
}

#[derive(Debug, Clone)]
enum Stat {
    Complete {
        time_to_first_byte: Duration,
        time_to_completion: Duration,
        body_size: usize,
    },
    Failed(String),
}

struct Benchmark {
    url: Url,
    num_clients: usize,
    reqs_per_client: usize,
}

#[derive(Debug)]
struct BenchmarkResult {
    started: std::time::SystemTime,
    total_runtime: Duration,
    client_results: Vec<Vec<Stat>>,
    reqs_per_client: usize,
}

impl BenchmarkResult {
    fn to_json(self) -> serde_json::Value {
        let num_clients = self.client_results.len();
        let total_requests: usize =
            self.client_results.iter().map(|r| r.len()).sum();
        let successful_requests: usize = self
            .client_results
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|s| matches!(s, Stat::Complete { .. }))
                    .count()
            })
            .sum();
        let failed_requests: usize = self
            .client_results
            .iter()
            .map(|r| {
                r.iter().filter(|s| matches!(s, Stat::Failed(_))).count()
            })
            .sum();

        let total_size: usize = self
            .client_results
            .iter()
            .map(|r| {
                r.iter()
                    .filter_map(|s| {
                        if let Stat::Complete { body_size, .. } = s {
                            Some(body_size)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum();

        let all_stats: Vec<Stat> = self
            .client_results
            .iter()
            .flat_map(|r| r.iter().cloned())
            .collect();

        let all_time_to_first_byte: Vec<_> = all_stats
            .iter()
            .filter_map(|s| {
                if let Stat::Complete {
                    time_to_first_byte, ..
                } = s
                {
                    Some(time_to_first_byte.as_secs_f64())
                } else {
                    None
                }
            })
            .collect();

        let all_time_to_completion: Vec<_> = all_stats
            .iter()
            .filter_map(|s| {
                if let Stat::Complete {
                    time_to_completion, ..
                } = s
                {
                    Some(time_to_completion.as_secs_f64())
                } else {
                    None
                }
            })
            .collect();

        // TODO: per_client statistics
        //
        let started: time::OffsetDateTime = self.started.into();
        let started = started
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap();

        serde_json::json!({
            "total_runtime": self.total_runtime.as_secs_f64(),
            "num_clients": num_clients,
            "reqs_per_client": self.reqs_per_client,
            "total_requests": total_requests,
            "successful_requests": successful_requests,
            "failed_requests": failed_requests,
            "total_size": total_size,
            "throughput_in_mib": total_size as f64 / self.total_runtime.as_secs_f64() / (1024.0 * 1024.0),
            "time_to_first_byte": all_time_to_first_byte,
            "time_to_completion": all_time_to_completion,
            "started": started,
        })
    }
}

impl Benchmark {
    async fn run(&self, ctx: Ctx) -> anyhow::Result<BenchmarkResult> {
        let started = std::time::SystemTime::now();
        let now = Instant::now();

        let clients: Vec<_> = (0..self.num_clients)
            .into_iter()
            .map(|_client_id: usize| {
                let ctx = ctx.clone();
                let url = self.url.clone();
                let reqs_per_client = self.reqs_per_client;
                tokio::task::spawn(async move {
                    run_client(url, reqs_per_client, ctx).await
                })
            })
            .collect();

        let mut client_results = Vec::with_capacity(clients.len());

        for client in clients {
            client_results.push(client.await??);
        }

        let total_runtime = now.elapsed();

        Ok(BenchmarkResult {
            started,
            client_results,
            total_runtime,
            reqs_per_client: self.reqs_per_client,
        })
    }
}

async fn run_client(
    url: Url,
    reqs_per_client: usize,
    ctx: Ctx,
) -> anyhow::Result<Vec<Stat>> {
    let client = Client::builder().build()?;

    let mut results: Vec<Stat> = Vec::<Stat>::with_capacity(reqs_per_client);

    for _req_no in 0..reqs_per_client {
        let res = match fetch_video(&url, client.clone(), &ctx).await {
            Ok(stat) => stat,
            Err(err) => {
                eprintln!("ERR: {err:?}");
                Stat::Failed(format!("{err:?}"))
            }
        };
        results.push(res);
    }
    Ok(results)
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

    Ok(Stat::Complete {
        time_to_first_byte,
        time_to_completion,
        body_size,
    })
}
