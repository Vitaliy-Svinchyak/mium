extern crate num_cpus;

use std::sync::mpsc::Sender;

use structopt::StructOpt;

mod gui;
mod job;
mod proceed;

#[derive(StructOpt, Debug, Clone)]
pub struct CliArgs {
    #[structopt(short, long, default_value = "anime")]
    query: String,
    #[structopt(short, long, default_value = "2")]
    pages: usize,
    #[structopt(short, long, default_value = "result")]
    file: String,
}

#[tokio::main]
async fn main() {
    let args: CliArgs = CliArgs::from_args();
    let max_cpus = num_cpus::get();
    let thread_number = if args.pages < max_cpus {
        args.pages
    } else {
        max_cpus
    };

    let (query_senders, thread_connections) = proceed::create_threads(args.clone(), thread_number);

    send_jobs(query_senders, args.pages, args.query);

    gui::main(thread_connections).unwrap();
}

fn send_jobs(query_senders: Vec<Sender<Option<String>>>, pages_to_parse: usize, query: String) {
    let mut i = 0;

    for page in 1..pages_to_parse + 1 {
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", query, page);
        let query_tx = &query_senders[i];
        query_tx
            .send(Some(query))
            .expect("Can't send query to channel");

        i += 1;
        if i == query_senders.len() {
            i = 0;
        }
    }

    for query_tx in query_senders {
        query_tx.send(None).expect("Can't send end of channel");
    }
}
