use std::cmp::{max, min};

use image::imageops::FilterType;
use image::{imageops::resize, RgbaImage};
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Widget};

const BLOCK_FULL: char = '\u{2588}';

/// A tui-rs Widget which displays an image.
pub struct Image<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// Image to display
    img: Option<&'a RgbaImage>,
    /// Alignment of the image
    alignment: Alignment,
}

impl<'a> Image<'a> {
    /// Construct an Image widget with a single image.
    pub fn with_img(img: &'a RgbaImage) -> Image<'a> {
        Image {
            block: None,
            style: Default::default(),
            img: Some(img),
            alignment: Alignment::Center,
        }
    }

    #[allow(dead_code)]
    pub fn block(mut self, block: Block<'a>) -> Image<'a> {
        self.block = Some(block);
        self
    }

    #[allow(dead_code)]
    pub fn style(mut self, style: Style) -> Image<'a> {
        self.style = style;
        self
    }

    #[allow(dead_code)]
    pub fn alignment(mut self, alignment: Alignment) -> Image<'a> {
        self.alignment = alignment;
        self
    }

    fn draw_img(&self, area: Rect, buf: &mut Buffer, img: &RgbaImage) {
        let bg_rgb = match self.style.bg {
            Some(Color::Black) => vec![0f32, 0f32, 0f32],
            Some(Color::White) => vec![1f32, 1f32, 1f32],
            Some(Color::Rgb(r, g, b)) => {
                vec![r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32]
            }
            _ => vec![0f32, 0f32, 0f32],
        };

        // calc offset

        let ox = max(
            0,
            min(
                area.width as i32 - 1,
                match self.alignment {
                    Alignment::Center => (area.width as i32 - img.width() as i32) / 2i32,
                    Alignment::Left => 0i32,
                    Alignment::Right => area.width as i32 - img.width() as i32,
                },
            ),
        ) as u16;
        let oy = max(
            0,
            min(
                area.height - 1,
                (area.height - (img.height() / 2) as u16) / 2,
            ),
        ) as u16;

        // draw

        for y in oy..(oy + min((img.height() / 2) as u16, area.height - 1)) {
            for x in ox..min(ox + img.width() as u16, area.width - 1) {
                let p = img.get_pixel((x - ox) as u32, 2 * (y - oy) as u32).0;

                // composite onto background
                let a = p[3] as f32 / 255.0;
                let r = p[0] as f32 * a / 255.0 + bg_rgb[0] * (1f32 - a);
                let g = p[1] as f32 * a / 255.0 + bg_rgb[1] * (1f32 - a);
                let b = p[2] as f32 * a / 255.0 + bg_rgb[2] * (1f32 - a);

                let cell = buf.get_mut(area.left() + x, area.top() + y);

                cell.set_char(BLOCK_FULL).set_fg(Color::Rgb(
                    (255.0 * r) as u8,
                    (255.0 * g) as u8,
                    (255.0 * b) as u8,
                ));
            }
        }
    }
}

impl<'a> Widget for Image<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(ref mut b) => b.inner(area),
            None => area,
        };

        if area.width < 1 || area.height < 1 {
            return;
        }

        buf.set_style(area, self.style);

        if let Some(ref img) = self.img {
            if img.width() > area.width as u32 || img.height() / 2 > area.height as u32 {
                let scaled = resize(
                    img.clone(),
                    area.width as u32,
                    2 * area.height as u32,
                    FilterType::Nearest,
                );
                self.draw_img(area, buf, &scaled)
            } else {
                self.draw_img(area, buf, img)
            }
        }
    }
}
