extern crate jpeg_decoder as jpeg;
extern crate num_cpus;

use std::sync::mpsc::channel;
use std::thread;

use futures::future::join_all;

use crate::job::{decode, download};
use crate::job::parse;

mod job;


#[tokio::main]
async fn main() {
    let max_cpus = num_cpus::get() / 2;
    let mut parse_jobs: Vec<_> = vec![];
    let mut download_jobs: Vec<_> = vec![];
    let mut decode_jobs: Vec<_> = vec![];

    for page in 1..max_cpus {
        let (url_tx, url_rx) = channel();
        let (bytes_tx, bytes_rx) = channel();
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", "car", page);

        let parse_job = thread::spawn(move || async move {
            let urls = parse::job(query).await;

            for url in urls {
                url_tx.send(url).expect("Can't send url to channel");
            }
            println!("{} parsed", page.clone());
            ()
        }).join().expect("Failed to create parse thread");

        let download_job = thread::spawn(move || async move {
            for url in url_rx {
                let bytes = download::job(url).await.expect("Failed to download picture");
                bytes_tx.send(bytes).expect("Can't send bytes to channel");
            }
            println!("{} downloaded", page.clone());
            ()
        }).join().expect("Failed to create download thread");

        let decode_job = thread::spawn(move || async move {
            for bytes in bytes_rx {
                let (pixels, info) = decode::job(bytes).await;
                println!("{:?}", info);
            }
            println!("{} decoded", page.clone());
            ()
        }).join().expect("Failed to create decode thread");


        parse_jobs.push(parse_job);
        download_jobs.push(download_job);
        decode_jobs.push(decode_job);

        println!("{} scheduled", page);
    }

    join_all(parse_jobs).await;
    join_all(download_jobs).await;
    join_all(decode_jobs).await;
    // join(
    //     // join_all(parse_jobs),
    //     join_all(download_jobs),
    //     join_all(decode_jobs),
    // ).await;
}