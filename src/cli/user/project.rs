use crate::common::{print_object_list, print_single_object, Execute, Format};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct ProjectListFilter {
    #[clap(short, long, help = "Display all projects", action)]
    all: bool,

    #[clap(short, long, help = "Display projects of given user class")]
    // TODO: use enum for this
    user_class: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectCommand {
    #[clap(about = "List projects")]
    List {
        #[clap(flatten)]
        filter: ProjectListFilter,
    },

    #[clap(about = "Show project with given ID")]
    Get { id: u32 },

    #[clap(about = "Create a new project")]
    Create {
        #[clap(help = "Name of the project")]
        name: String,

        #[clap(help = "Openstack UUIDv4 of the project")]
        openstack_id: String,

        // TODO we need some enum here
        #[clap(
            long,
            short,
            help = "User class of the project (0,1,2,3,4,5,6)"
        )]
        user_class: Option<u32>,
    },

    #[clap(about = "Delete project with given ID")]
    Delete { id: u32 },
}

impl Execute for ProjectCommand {
    fn execute(
        &self,
        api: lrzcc::Api,
        format: Format,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            ProjectCommand::List { filter } => list(api, format, filter),
            ProjectCommand::Get { id } => get(api, format, id),
            ProjectCommand::Create {
                name,
                openstack_id,
                user_class,
            } => create(
                api,
                format,
                name.to_owned(),
                openstack_id.to_owned(),
                *user_class,
            ),
            ProjectCommand::Delete { id } => delete(api, id),
        }
    }
}

fn list(
    api: lrzcc::Api,
    format: Format,
    filter: &ProjectListFilter,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project.list();
    if filter.all {
        request.all();
    } else if let Some(user_class) = filter.user_class {
        request.user_class(user_class);
    }
    print_object_list(request.send()?, format)
}

fn get(
    api: lrzcc::Api,
    format: Format,
    id: &u32,
) -> Result<(), Box<dyn Error>> {
    print_single_object(api.project.get(*id)?, format)
}

// TODO something here doesn't work ... no idea why so far
fn create(
    api: lrzcc::Api,
    format: Format,
    name: String,
    openstack_id: String,
    user_class: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    let mut request = api.project.create(name, openstack_id);
    if let Some(user_class) = user_class {
        request.user_class(user_class);
    }
    print_single_object(request.send()?, format)
}

fn delete(api: lrzcc::Api, id: &u32) -> Result<(), Box<dyn Error>> {
    // TODO dangerous operations like this one should be protected by a
    // confirmation prompt
    Ok(api.project.delete(*id)?)
}
