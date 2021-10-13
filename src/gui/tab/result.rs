use std::sync::mpsc::Receiver;

use image::DynamicImage;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::Frame;

use crate::gui::app::App;
use crate::gui::block::image::Image;
use crate::gui::theme::theme_block;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect, image_rx: &Receiver<DynamicImage>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .split(area);

    let img = image_rx.try_recv();
    match img {
        Ok(i) => {
            let img = i.to_rgba8();
            app.result_image = Some(img);
        }
        Err(_) => {}
    }

    match &app.result_image {
        Some(img) => {
            let b = Image::with_img(img);
            f.render_widget(b, chunks[0]);
        }
        None => {
            let block = theme_block("Image will be here");
            f.render_widget(block, chunks[0]);
        }
    };
}
