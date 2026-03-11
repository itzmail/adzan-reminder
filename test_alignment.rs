use tui_big_text::{BigTextBuilder, PixelSize};
use ratatui::layout::Alignment;

fn main() {
    let _ = BigTextBuilder::default()
        .pixel_size(PixelSize::HalfHeight)
        .alignment(Alignment::Center)
        .build();
}
