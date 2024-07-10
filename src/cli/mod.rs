pub mod commands;
pub use commands::*;

use strum::EnumString;

#[derive(EnumString)]
pub enum FileFormat {
    #[strum(serialize = "infer")]
    Infer,
    #[strum(serialize = "ron")]
    Ron,
    // kdl,
    // json,
    // hjson,
    // yaml,
    // toml
}
