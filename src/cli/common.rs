use clap::{builder::PossibleValue, ValueEnum};
use tabled::settings::Style;
use tabled::Table;

#[derive(Debug, Clone)]
pub(crate) enum TableFormat {
    Empty,
    Blank,
    Ascii,
    Psql,
    Markdown,
    Modern,
    Sharp,
    Rounded,
    ModernRounded,
    Extended,
    Dots,
    ReStructuredText,
    AsciiRounded,
}

pub(crate) fn apply_table_style(table: &mut Table, format: TableFormat) {
    match format {
        TableFormat::Empty => table.with(Style::empty()),
        TableFormat::Blank => table.with(Style::blank()),
        TableFormat::Ascii => table.with(Style::ascii()),
        TableFormat::Psql => table.with(Style::psql()),
        TableFormat::Markdown => table.with(Style::markdown()),
        TableFormat::Modern => table.with(Style::modern()),
        TableFormat::Sharp => table.with(Style::sharp()),
        TableFormat::Rounded => table.with(Style::rounded()),
        TableFormat::ModernRounded => table.with(Style::modern_rounded()),
        TableFormat::Extended => table.with(Style::extended()),
        TableFormat::Dots => table.with(Style::dots()),
        TableFormat::ReStructuredText => {
            table.with(Style::re_structured_text())
        }
        TableFormat::AsciiRounded => table.with(Style::ascii_rounded()),
    };
}

#[derive(Debug, Clone)]
pub(crate) enum Format {
    Json,
    Table(TableFormat),
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Format::Json,
            Format::Table(TableFormat::Empty),
            Format::Table(TableFormat::Blank),
            Format::Table(TableFormat::Ascii),
            Format::Table(TableFormat::Psql),
            Format::Table(TableFormat::Markdown),
            Format::Table(TableFormat::Modern),
            Format::Table(TableFormat::Sharp),
            Format::Table(TableFormat::Rounded),
            Format::Table(TableFormat::ModernRounded),
            Format::Table(TableFormat::Extended),
            Format::Table(TableFormat::Dots),
            Format::Table(TableFormat::ReStructuredText),
            Format::Table(TableFormat::AsciiRounded),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Format::Json => Some(PossibleValue::new("json")),
            Format::Table(format) => match format {
                TableFormat::Empty => Some(PossibleValue::new("empty")),
                TableFormat::Blank => Some(PossibleValue::new("blank")),
                TableFormat::Ascii => Some(PossibleValue::new("ascii")),
                TableFormat::Psql => Some(PossibleValue::new("psql")),
                TableFormat::Markdown => Some(PossibleValue::new("markdown")),
                TableFormat::Modern => Some(PossibleValue::new("modern")),
                TableFormat::Sharp => Some(PossibleValue::new("sharp")),
                TableFormat::Rounded => Some(PossibleValue::new("rounded")),
                TableFormat::ModernRounded => {
                    Some(PossibleValue::new("modern-rounded"))
                }
                TableFormat::Extended => Some(PossibleValue::new("extended")),
                TableFormat::Dots => Some(PossibleValue::new("dots")),
                TableFormat::ReStructuredText => {
                    Some(PossibleValue::new("re-structured-text"))
                }
                TableFormat::AsciiRounded => {
                    Some(PossibleValue::new("ascii-rounded"))
                }
            },
        }
    }

    fn from_str(value: &str, _ignore_case: bool) -> Result<Self, String> {
        match value {
            "json" => Ok(Format::Json),
            "empty" => Ok(Format::Table(TableFormat::Empty)),
            "blank" => Ok(Format::Table(TableFormat::Blank)),
            "ascii" => Ok(Format::Table(TableFormat::Ascii)),
            "psql" => Ok(Format::Table(TableFormat::Psql)),
            "markdown" => Ok(Format::Table(TableFormat::Markdown)),
            "modern" => Ok(Format::Table(TableFormat::Modern)),
            "sharp" => Ok(Format::Table(TableFormat::Sharp)),
            "rounded" => Ok(Format::Table(TableFormat::Rounded)),
            "modern-rounded" => Ok(Format::Table(TableFormat::ModernRounded)),
            "extended" => Ok(Format::Table(TableFormat::Extended)),
            "dots" => Ok(Format::Table(TableFormat::Dots)),
            "re-structured-text" => {
                Ok(Format::Table(TableFormat::ReStructuredText))
            }
            "ascii-rounded" => Ok(Format::Table(TableFormat::AsciiRounded)),
            _ => Err(format!("Invalid format: {}", value)),
        }
    }
}

pub(crate) trait Execute {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
