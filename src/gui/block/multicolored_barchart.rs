use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols,
    widgets::{Block, Widget},
};
use std::cmp::min;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct MulticoloredBarChart<'a> {
    /// Block to wrap the widget in
    block: Option<Block<'a>>,
    /// The width of each bar
    bar_width: u16,
    /// The gap between each bar
    bar_gap: u16,
    /// Set of symbols used to display the data
    bar_set: symbols::bar::Set,
    /// Style for each bar
    multi_bar_style: Vec<Style>,
    /// Style of the values printed at the bottom of each bar
    value_style: Style,
    /// Style of the labels printed under each bar
    label_style: Style,
    /// Style for the widget
    style: Style,
    /// Slice of (label, value) pair to plot on the chart
    data: &'a [(&'a str, u64)],
    /// Value necessary for a bar to reach the maximum height (if no value is specified,
    /// the maximum value in the data is taken as reference)
    max: Option<u64>,
    /// Values to display on the bar (computed when the data is passed to the widget)
    values: Vec<String>,
}

impl<'a> Default for MulticoloredBarChart<'a> {
    fn default() -> MulticoloredBarChart<'a> {
        MulticoloredBarChart {
            block: None,
            max: None,
            data: &[],
            values: Vec::new(),
            multi_bar_style: vec![],
            bar_width: 1,
            bar_gap: 1,
            bar_set: symbols::bar::NINE_LEVELS,
            value_style: Default::default(),
            label_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a> MulticoloredBarChart<'a> {
    pub fn data(mut self, data: &'a [(&'a str, u64)]) -> MulticoloredBarChart<'a> {
        self.data = data;
        self.values = Vec::with_capacity(self.data.len());
        for &(_, v) in self.data {
            self.values.push(format!("{}", v));
        }
        self
    }

    pub fn block(mut self, block: Block<'a>) -> MulticoloredBarChart<'a> {
        self.block = Some(block);
        self
    }

    #[allow(dead_code)]
    pub fn max(mut self, max: u64) -> MulticoloredBarChart<'a> {
        self.max = Some(max);
        self
    }

    pub fn multi_bar_style(mut self, style: Vec<Style>) -> MulticoloredBarChart<'a> {
        self.multi_bar_style = style;
        self
    }

    pub fn bar_width(mut self, width: u16) -> MulticoloredBarChart<'a> {
        self.bar_width = width;
        self
    }

    #[allow(dead_code)]
    pub fn bar_gap(mut self, gap: u16) -> MulticoloredBarChart<'a> {
        self.bar_gap = gap;
        self
    }

    #[allow(dead_code)]
    pub fn bar_set(mut self, bar_set: symbols::bar::Set) -> MulticoloredBarChart<'a> {
        self.bar_set = bar_set;
        self
    }

    pub fn value_style(mut self, style: Style) -> MulticoloredBarChart<'a> {
        self.value_style = style;
        self
    }

    #[allow(dead_code)]
    pub fn label_style(mut self, style: Style) -> MulticoloredBarChart<'a> {
        self.label_style = style;
        self
    }

    #[allow(dead_code)]
    pub fn style(mut self, style: Style) -> MulticoloredBarChart<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for MulticoloredBarChart<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        let chart_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if chart_area.height < 2 {
            return;
        }

        let max = self
            .max
            .unwrap_or_else(|| self.data.iter().map(|t| t.1).max().unwrap_or_default());
        let max_index = min(
            (chart_area.width / (self.bar_width + self.bar_gap)) as usize,
            self.data.len(),
        );
        let mut data = self
            .data
            .iter()
            .take(max_index)
            .map(|&(l, v)| {
                (
                    l,
                    v * u64::from(chart_area.height - 1) * 8 / std::cmp::max(max, 1),
                )
            })
            .collect::<Vec<(&str, u64)>>();

        for j in (0..chart_area.height - 1).rev() {
            for (i, d) in data.iter_mut().enumerate() {
                let bar_style = self.multi_bar_style[i];

                let symbol = match d.1 {
                    0 => self.bar_set.empty,
                    1 => self.bar_set.one_eighth,
                    2 => self.bar_set.one_quarter,
                    3 => self.bar_set.three_eighths,
                    4 => self.bar_set.half,
                    5 => self.bar_set.five_eighths,
                    6 => self.bar_set.three_quarters,
                    7 => self.bar_set.seven_eighths,
                    _ => self.bar_set.full,
                };

                for x in 0..self.bar_width {
                    buf.get_mut(
                        chart_area.left() + i as u16 * (self.bar_width + self.bar_gap) + x,
                        chart_area.top() + j,
                    )
                        .set_symbol(symbol)
                        .set_style(bar_style);
                }

                if d.1 > 8 {
                    d.1 -= 8;
                } else {
                    d.1 = 0;
                }
            }
        }

        for (i, &(label, value)) in self.data.iter().take(max_index).enumerate() {
            if value != 0 {
                let value_label = &self.values[i];
                let width = value_label.width() as u16;
                if width < self.bar_width {
                    buf.set_string(
                        chart_area.left()
                            + i as u16 * (self.bar_width + self.bar_gap)
                            + (self.bar_width - width) / 2,
                        chart_area.bottom() - 2,
                        value_label,
                        self.value_style,
                    );
                }
            }
            buf.set_stringn(
                chart_area.left() + i as u16 * (self.bar_width + self.bar_gap),
                chart_area.bottom() - 1,
                label,
                self.bar_width as usize,
                self.label_style,
            );
        }
    }
}
