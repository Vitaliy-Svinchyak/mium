extern crate num_cpus;

use std::sync::mpsc::Sender;

use structopt::StructOpt;

mod gui;
mod job;
mod proceed;
pub mod sync;

#[derive(StructOpt, Debug, Clone)]
pub struct CliArgs {
    #[structopt(short, long, default_value = "anime")]
    query: String,
    #[structopt(short, long, default_value = "2")]
    pages: usize,
    #[structopt(short, long, default_value = "result")]
    file: String,
}

fn main() {
    let args: CliArgs = CliArgs::from_args();
    let max_cpus = num_cpus::get();
    let thread_number = if args.pages < max_cpus {
        args.pages
    } else {
        max_cpus
    };

    let (query_sender, thread_connections, image_rx) =
        proceed::create_threads(args.clone(), thread_number);

    send_jobs(query_sender, args.pages, args.query);

    gui::main(thread_connections, args.pages, image_rx).unwrap();
}

fn send_jobs(query_sender: Sender<Option<String>>, pages_to_parse: usize, query: String) {
    for page in 1..pages_to_parse + 1 {
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", query, page);
        query_sender
            .send(Some(query))
            .expect("Can't send query to channel");
    }

    query_sender.send(None).expect("Can't send end of channel");
}
