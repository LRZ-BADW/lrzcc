use std::{
    borrow::Cow,
    fmt::Display,
    io::{stdin, stdout, Write},
};

use anyhow::{anyhow, Context};
use chrono::Datelike;
use clap::{builder::PossibleValue, ValueEnum};
use serde::Serialize;
use tabled::{
    builder::Builder, grid::records::vec_records::Text, settings::Style, Table,
    Tabled,
};

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
pub(crate) fn print_json<T>(object: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    println!("{}", serde_json::to_string(&object)?);
    Ok(())
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

#[allow(dead_code)]
pub(crate) fn print_hashmap<K, V>(
    hashmap: std::collections::HashMap<K, V>,
    key_name: &str,
    value_name: &str,
    format: Format,
) -> Result<(), Box<dyn std::error::Error>>
where
    K: Serialize + Display,
    V: Serialize + Display,
    (K, V): Tabled,
{
    match format {
        Format::Json => println!("{}", serde_json::to_string(&hashmap)?),
        Format::Table(format) => {
            let mut table = Table::new(hashmap);
            table.get_records_mut()[0] = vec![
                Text::new(key_name.to_owned()),
                Text::new(value_name.to_owned()),
            ];
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
            _ => Err(format!("Invalid format: {value}")),
        }
    }
}

pub(crate) trait Execute {
    fn execute(
        &self,
        api: avina::Api,
        format: Format,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub(crate) fn ask_for_confirmation() -> Result<(), anyhow::Error> {
    let confirmation = "Yes, I really really mean it!".to_owned();
    print!("This is dangerous. Are you sure? Type: {confirmation}\n> ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .context("Confirmation input was not a valid string.")?;
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if input != confirmation {
        return Err(anyhow!("No confirmation provided! Aborting!"));
    }
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn find_id(
    _api: &avina::Api,
    name_or_id: &str,
) -> Result<u32, anyhow::Error> {
    if let Ok(id) = name_or_id.parse::<u32>() {
        return Ok(id);
    }
    Err(anyhow!(
        "This is not a valid integer ID: {name_or_id}. Name or OpenStack \
        UUIDv4 lookup is unavailable as the respective module is disabled \
        in this client."
    ))
}

pub(crate) fn current_year() -> i32 {
    chrono::Utc::now().year()
}
