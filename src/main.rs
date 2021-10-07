extern crate num_cpus;

use std::sync::mpsc::{channel, Sender};
use std::time::Instant;

use structopt::StructOpt;

mod job;
mod proceed;
mod collect;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long, default_value = "anime")]
    query: String,
    #[structopt(short, long, default_value = "1")]
    pages: usize,
    #[structopt(short, long, default_value = "result")]
    file: String,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::from_args();

    let (result_image_tx, result_image_rx) = channel();
    let start = Instant::now();
    let query_senders = proceed::create_threads(result_image_tx);
    send_jobs(query_senders, args.pages, args.query);

    let result_picture = collect::collect_result(result_image_rx, args.pages);
    println!("done in: {:?}", start.elapsed());

    result_picture
        .save(format!("./{}.jpeg", args.file))
        .expect("Can't save image");
}

fn send_jobs(query_senders: Vec<Sender<Option<String>>>, pages_to_parse: usize, query: String) {
    let mut i = 0;

    for page in 1..pages_to_parse + 1 {
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", query, page);
        let query_tx = &query_senders[i];
        query_tx.send(Some(query)).expect("Can't send query to channel");

        i += 1;
        if i == query_senders.len() {
            i = 0;
        }
    }

    for query_tx in query_senders {
        query_tx.send(None).expect("Can't send end of channel");
    }
}
