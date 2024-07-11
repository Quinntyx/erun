pub mod commands;
pub use commands::*;

use strum::EnumString;

#[derive(EnumString)]
pub enum FileFormat {
    #[strum(serialize = "infer")]
    Infer,
    #[strum(serialize = "ron")]
    Ron,
    #[strum(serialize = "kdl")]
    Kdl,
    #[strum(serialize = "json")]
    Json,
    // hjson,
    #[strum(serialize = "yaml")]
    Yaml,
    #[strum(serialize = "toml")]
    Toml,
    #[strum(serialize = "sexpr")]
    SExpression,
    #[strum(serialize = "url")]
    Url,
    // TODO: implement csv support (?)
    // #[strum(serialize = "csv")]
    // Csv,
    #[strum(serialize = "xml")]
    Xml,
}
