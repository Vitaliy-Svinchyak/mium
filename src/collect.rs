use std::sync::mpsc::Receiver;

use image::DynamicImage;

use crate::job::accumulate;

pub fn collect_result(result_image_rx: Receiver<Option<DynamicImage>>, pages: usize, thread_number: usize) -> DynamicImage {
    let mut results_received = 0;
    let mut valid_results_received = 1;
    let mut medium_picture = loop {
        if let Ok(medium_picture) = result_image_rx.recv() {
            results_received += 1;
            println!("none {} / {}", results_received, thread_number);

            if let Some(medium_picture) = medium_picture {
                break medium_picture;
            }
        }
    };

    if pages == 1 {
        return medium_picture;
    }

    loop {
        if let Ok(picture) = result_image_rx.recv() {
            if let Some(picture) = picture {
                accumulate::accumulate(&picture, valid_results_received, &mut medium_picture);
                valid_results_received += 1;
            }

            results_received += 1;
            println!("some {} / {}", results_received, thread_number);

            if results_received == thread_number {
                medium_picture
                    .save("./result.jpeg")
                    .expect("Can't save image");

                break medium_picture;
            }
        }
    }
}