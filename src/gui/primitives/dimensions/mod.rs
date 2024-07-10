pub mod signed;
pub mod unsigned;

pub use signed::*;
pub use unsigned::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Default, Debug)]
pub enum ComboSize {
    #[default]
    ZERO,
    Even(UPos),
    Symmetric(UPos, UPos),
    PerEdge(UPos, UPos, UPos, UPos),
}

impl Into<egui::Margin> for ComboSize {
    fn into(self) -> egui::Margin {
        use egui::Margin as Em;
        use ComboSize::*;
        match self {
            ZERO => Em::same(0f32),
            Even(x) => Em::same(x.get() as f32),
            Symmetric(x, y) => Em::symmetric(x.get() as f32, y.get() as f32),
            PerEdge(left, right, top, bottom) => Em {
                left: left.get() as f32,
                right: right.get() as f32,
                top: top.get() as f32,
                bottom: bottom.get() as f32,
            },
        }
    }
}
