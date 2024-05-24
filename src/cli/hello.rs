use crate::common::{apply_table_style, Execute, Format};
use clap::Subcommand;
use lrzcc::hello::Hello;
use std::borrow::Cow;
use std::error::Error;
use tabled::builder::Builder;
use tabled::Tabled;

#[derive(Subcommand, Debug)]
pub(crate) enum HelloCommand {
    #[clap(about = "Hello admin command")]
    Admin,

    #[clap(about = "Hello user command")]
    User,
}

impl Execute for HelloCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            HelloCommand::Admin {} => admin(api, format),
            HelloCommand::User {} => user(api, format),
        }
    }
}

fn admin(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    let hello = api.hello.admin()?;
    match format {
        Format::Json => println!("{}", serde_json::to_string(&hello)?),
        Format::Table(format) => {
            let mut keys = vec![Cow::Owned("key".to_owned())];
            keys.extend(Hello::headers());
            let mut values = vec![Cow::Owned("value".to_owned())];
            values.extend(hello.fields());
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

fn user(api: lrzcc::Api, format: Format) -> Result<(), Box<dyn Error>> {
    let hello = api.hello.user()?;
    match format {
        Format::Json => println!("{}", serde_json::to_string(&hello)?),
        Format::Table(format) => {
            let mut keys = vec![Cow::Owned("key".to_owned())];
            keys.extend(Hello::headers());
            let mut values = vec![Cow::Owned("value".to_owned())];
            values.extend(hello.fields());
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
