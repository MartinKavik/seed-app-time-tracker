use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use std::collections::BTreeMap;

type ClientId = Ulid;
type ProjectId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::NotAsked,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    changes_status: ChangesStatus,
    errors: Vec<FetchError>,

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

pub struct Client {
    name: String,
    projects: BTreeMap<ProjectId, Project>,
}

struct Project {
    name: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ClientsFetched(fetch::Result<BTreeMap<ClientId, Client>>),
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
        Msg::ClientsFetched(Ok(clients)) => {},
        Msg::ClientsFetched(Err(fetch_error)) => {},

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
