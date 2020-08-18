use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use cynic::QueryFragment;

use std::collections::BTreeMap;

use crate::graphql;

type ClientId = Ulid;
type ProjectId = Ulid;
type TimeEntryId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::ClientsFetched(request_clients().await) });

    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::NotAsked,
        timer_handle: orders.stream_with_handle(streams::interval(1000, || Msg::OnSecondTick)),
    }
}

async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
    let response_data = graphql::send_query(
        graphql::queries::clients_with_projects_with_time_entries::Query::fragment(())
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
                                    Project { 
                                        name: project.name, 
                                        time_entries: project.time_entries.into_iter().map(|time_entry| {
                                            (
                                                time_entry.id.parse().expect("parse time_entry Ulid"),
                                                TimeEntry {
                                                    name: time_entry.name,
                                                    started: time_entry.started.0.parse().expect("parse time_entry started time"),
                                                    stopped: time_entry.stopped.map(|time| time.0.parse().expect("parse time_entry started time")),
                                                }
                                            )
                                        }).collect()
                                    },
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
    timer_handle: StreamHandle, 
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
    projects: BTreeMap<Ulid, Project>,
}

#[derive(Debug)]
struct Project {
    name: String,
    time_entries: BTreeMap<Ulid, TimeEntry>,
}

#[derive(Debug)]
struct TimeEntry {
    name: String,
    started: DateTime<Local>,
    stopped: Option<DateTime<Local>>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,
    
    Start(ClientId, ProjectId),
    Stop(ClientId, ProjectId),

    DeleteTimeEntry(ClientId, ProjectId, TimeEntryId),
    
    TimeEntryNameChanged(ClientId, ProjectId, TimeEntryId, String),
    SaveTimeEntryName(ClientId, ProjectId, TimeEntryId),
    
    TimeEntryStartedChanged(ClientId, ProjectId, TimeEntryId, String),
    SaveTimeEntryStarted(ClientId, ProjectId, TimeEntryId),

    TimeEntryDurationChanged(ClientId, ProjectId, TimeEntryId, String),
    
    TimeEntryStoppedChanged(ClientId, ProjectId, TimeEntryId, String),
    SaveTimeEntryStopped(ClientId, ProjectId, TimeEntryId),

    OnSecondTick,
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

        Msg::Start(client_id, project_id) => {},
        Msg::Stop(client_id, project_id) => {},

        Msg::DeleteTimeEntry(client_id, project_id, time_entry_id) => {},

        Msg::TimeEntryNameChanged(client_id, project_id, time_entry_id, name) => {},
        Msg::SaveTimeEntryName(client_id, project_id, time_entry_id) => {},

        Msg::TimeEntryStartedChanged(client_id, project_id, time_entry_id, name) => {},
        Msg::SaveTimeEntryStarted(client_id, project_id, time_entry_id) => {},

        Msg::TimeEntryDurationChanged(client_id, project_id, time_entry_id, name) => {},

        Msg::TimeEntryStoppedChanged(client_id, project_id, time_entry_id, name) => {},
        Msg::SaveTimeEntryStopped(client_id, project_id, time_entry_id) => {},

        Msg::OnSecondTick => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["TimeTracker view"]
}
