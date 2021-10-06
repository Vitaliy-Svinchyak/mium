extern crate num_cpus;

use std::sync::mpsc::channel;
use std::thread;

use futures::future::join_all;

use crate::job::{accumulate, decode, download};
use crate::job::parse;

mod job;


#[tokio::main]
async fn main() {
    let max_cpus = num_cpus::get();
    let max_cpus = 2;
    let mut parse_jobs: Vec<_> = vec![];
    let mut download_jobs: Vec<_> = vec![];
    let mut decode_jobs: Vec<_> = vec![];
    let mut accumulate_jobs: Vec<_> = vec![];

    for page in 1..max_cpus {
        let (url_tx, url_rx) = channel();
        let (bytes_tx, bytes_rx) = channel();
        let (pixels_tx, pixels_rx) = channel();
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", "anime", page);

        let parse_job = thread::spawn(move || async move {
            parse::job(query, url_tx).await;
            println!("{} parsed", page.clone());
        }).join().expect("Failed to create parse thread");

        let download_job = thread::spawn(move || async move {
            download::job(url_rx, bytes_tx).await;
            println!("{} downloaded", page.clone());
        }).join().expect("Failed to create download thread");

        let decode_job = thread::spawn(move || async move {
            decode::job(bytes_rx, pixels_tx);
            println!("{} decoded", page.clone());
        }).join().expect("Failed to create decode thread");

        let accumulate_job = thread::spawn(move || async move {
            let medium = accumulate::job(pixels_rx);
            println!("{} accumulated", page.clone());
            medium
        }).join().expect("Failed to create decode thread");

        parse_jobs.push(parse_job);
        download_jobs.push(download_job);
        decode_jobs.push(decode_job);
        accumulate_jobs.push(accumulate_job);

        println!("{} scheduled", page);
    }

    join_all(parse_jobs).await;
    join_all(download_jobs).await;
    join_all(decode_jobs).await;
    let pictures = join_all(accumulate_jobs).await;

    let mut medium_picture = pictures[0].clone();
    for (i, picture) in pictures.iter().skip(1).enumerate() {
        medium_picture.save(format!("./result{}.jpeg", i))
            .expect("Can't save middle image");
        accumulate::accumulate(picture, i, &mut medium_picture);
    }
    medium_picture.save("./result.jpeg")
        .expect("Can't save image");
}