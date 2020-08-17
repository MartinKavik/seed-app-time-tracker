use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use cynic::QueryFragment;

use std::collections::BTreeMap;

use crate::graphql;

type ClientId = Ulid;
type ProjectId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::ClientsFetched(request_clients().await) });

    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::Loading,
    }
}

async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
    let response_data = graphql::send_query(
        graphql::queries::ClientsWithProjects::fragment(())
    ).await?;

    let clients = 
        response_data
            .query_client
            .expect("get clients")
            .into_iter()
            .filter_map(|client| {
                client.map(|client|
                    (
                        client.id.parse().expect("parse client Ulid"),
                        Client {
                            name: client.name,
                            projects: client.projects.into_iter().map(|project| {
                                (
                                    project.id.parse().expect("parse project Ulid"), 
                                    Project { name: project.name },
                                )
                            }).collect()
                        }
                    )
                )
            })
            .collect();
    Ok(clients)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    changes_status: ChangesStatus,
    errors: Vec<graphql::GraphQLError>,

    clients: RemoteData<BTreeMap<ClientId, Client>>,
}

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

#[derive(Debug)]
pub struct Client {
    name: String,
    projects: BTreeMap<ProjectId, Project>,
}

#[derive(Debug)]
struct Project {
    name: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,
    
    // ------ Client ------

    AddClient,
    DeleteClient(ClientId),

    ClientNameChanged(ClientId, String),
    SaveClientName(ClientId),
    
    // ------ Project ------

    AddProject(ClientId),
    DeleteProject(ClientId, ProjectId),
    
    ProjectNameChanged(ClientId, ProjectId, String),
    SaveProjectName(ClientId, ProjectId),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClientsFetched(Ok(clients)) => {
            log!("Msg::ClientsFetched", clients);
            model.clients = RemoteData::Loaded(clients);
        },
        Msg::ClientsFetched(Err(graphql_error)) => {
            model.errors.push(graphql_error);
        },

        Msg::ChangesSaved(None) => {},
        Msg::ChangesSaved(Some(fetch_error)) => {},

        Msg::ClearErrors => {},

        // ------ Client ------

        Msg::AddClient => {},
        Msg::DeleteClient(client_id) => {},

        Msg::ClientNameChanged(client_id, name) => {},
        Msg::SaveClientName(client_id) => {},

        // ------ Project ------

        Msg::AddProject(client_id) => {},
        Msg::DeleteProject(client_id, project_id) => {},

        Msg::ProjectNameChanged(client_id, project_id, name) => {},
        Msg::SaveProjectName(client_id, project_id) => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["ClientsAndProjects view"]
}
