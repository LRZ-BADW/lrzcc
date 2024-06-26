use clap::{builder::PossibleValue, ValueEnum};
use serde::Serialize;
use std::borrow::Cow;
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

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
pub(crate) use TableFormat::*;

pub(crate) fn apply_table_style(table: &mut Table, format: TableFormat) {
    match format {
        Empty => table.with(Style::empty()),
        Blank => table.with(Style::blank()),
        Ascii => table.with(Style::ascii()),
        Psql => table.with(Style::psql()),
        Markdown => table.with(Style::markdown()),
        Modern => table.with(Style::modern()),
        Sharp => table.with(Style::sharp()),
        Rounded => table.with(Style::rounded()),
        ModernRounded => table.with(Style::modern_rounded()),
        Extended => table.with(Style::extended()),
        Dots => table.with(Style::dots()),
        ReStructuredText => table.with(Style::re_structured_text()),
        AsciiRounded => table.with(Style::ascii_rounded()),
    };
}

#[allow(dead_code)]
pub(crate) fn print_single_object<T>(
    object: T,
    format: Format,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Tabled,
{
    match format {
        Format::Json => println!("{}", serde_json::to_string(&object)?),
        Format::Table(format) => {
            let mut keys = vec![Cow::Owned("key".to_owned())];
            keys.extend(T::headers());
            let mut values = vec![Cow::Owned("value".to_owned())];
            values.extend(object.fields());
            let data = vec![keys, values];
            let mut table = Builder::from_iter(data)
                .index()
                .column(0)
                .transpose()
                .build();
            apply_table_style(&mut table, format);
            let output = table.to_string();
            println!("{output}");
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn print_object_list<T>(
    objects: Vec<T>,
    format: Format,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Tabled,
{
    match format {
        Format::Json => println!("{}", serde_json::to_string(&objects)?),
        Format::Table(format) => {
            let mut table = Table::new(objects);
            apply_table_style(&mut table, format);
            let output = table.to_string();
            println!("{output}");
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) enum Format {
    Json,
    Table(TableFormat),
}
pub(crate) use Format::*;

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Json,
            Table(Empty),
            Table(Blank),
            Table(Ascii),
            Table(Psql),
            Table(Markdown),
            Table(Modern),
            Table(Sharp),
            Table(Rounded),
            Table(ModernRounded),
            Table(Extended),
            Table(Dots),
            Table(ReStructuredText),
            Table(AsciiRounded),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Format::Json => Some(PossibleValue::new("json")),
            Format::Table(format) => match format {
                Empty => Some(PossibleValue::new("empty")),
                Blank => Some(PossibleValue::new("blank")),
                Ascii => Some(PossibleValue::new("ascii")),
                Psql => Some(PossibleValue::new("psql")),
                Markdown => Some(PossibleValue::new("markdown")),
                Modern => Some(PossibleValue::new("modern")),
                Sharp => Some(PossibleValue::new("sharp")),
                Rounded => Some(PossibleValue::new("rounded")),
                ModernRounded => Some(PossibleValue::new("modern-rounded")),
                Extended => Some(PossibleValue::new("extended")),
                Dots => Some(PossibleValue::new("dots")),
                ReStructuredText => {
                    Some(PossibleValue::new("re-structured-text"))
                }
                AsciiRounded => Some(PossibleValue::new("ascii-rounded")),
            },
        }
    }

    fn from_str(value: &str, _ignore_case: bool) -> Result<Self, String> {
        match value {
            "json" => Ok(Json),
            "empty" => Ok(Table(Empty)),
            "blank" => Ok(Table(Blank)),
            "ascii" => Ok(Table(Ascii)),
            "psql" => Ok(Table(Psql)),
            "markdown" => Ok(Table(Markdown)),
            "modern" => Ok(Table(Modern)),
            "sharp" => Ok(Table(Sharp)),
            "rounded" => Ok(Table(Rounded)),
            "modern-rounded" => Ok(Table(ModernRounded)),
            "extended" => Ok(Table(Extended)),
            "dots" => Ok(Table(Dots)),
            "re-structured-text" => Ok(Table(ReStructuredText)),
            "ascii-rounded" => Ok(Table(AsciiRounded)),
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
