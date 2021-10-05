extern crate jpeg_decoder as jpeg;
extern crate num_cpus;

use std::sync::mpsc::channel;
use std::thread;

use futures::future::join_all;

use crate::job::{accumulate, decode, download};
use crate::job::parse;
use std::fs::File;
use std::io::Write;

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
                break;
            }

            println!("{} downloaded", page.clone());
            ()
        }).join().expect("Failed to create download thread");

        let decode_job = thread::spawn(move || async move {
            for bytes in bytes_rx {
                let (pixels, info) = decode::job(bytes).await;
                pixels_tx.send((pixels, info)).expect("Can't send bytes to channel");
            }

            println!("{} decoded", page.clone());
            ()
        }).join().expect("Failed to create decode thread");

        let accumulate_job = thread::spawn(move || async move {
            let mut medium = pixels_rx.recv().unwrap().0;
            for (i, (pixels, info)) in pixels_rx.iter().enumerate() {
                medium = accumulate::job(&pixels, i, &medium);
            }

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
        medium_picture = accumulate::job(picture, i, &medium_picture);
    }
    let mut file = File::create("result.jpeg").unwrap();
    // file.write_all(medium_picture.as_slice()).unwrap();
    // join(
    //     // join_all(parse_jobs),
    //     join_all(download_jobs),
    //     join_all(decode_jobs),
    // ).await;
}