use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use cynic::QueryFragment;

use std::collections::BTreeMap;
use std::convert::identity;

use crate::graphql;

const PRIMARY_COLOR: &str = "#00d1b2";

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
    use graphql::queries::clients_with_projects as query_mod;

    let project_mapper = |project: query_mod::Project| (
        project.id.parse().expect("parse project Ulid"), 
        Project { name: project.name }
    );

    let client_mapper = |client: query_mod::Client| (
        client.id.parse().expect("parse client Ulid"),
        Client {
            name: client.name,
            projects: client.projects.into_iter().map(project_mapper).collect()
        }
    );

    Ok(
        graphql::send_query(query_mod::Query::fragment(()))
            .await?
            .query_client
            .expect("get clients")
            .into_iter()
            .filter_map(identity)
            .map(client_mapper)
            .collect()
    )
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

        Msg::ChangesSaved(None) => {
            log!("Msg::ChangesSaved");
        },
        Msg::ChangesSaved(Some(fetch_error)) => {
            log!("Msg::ChangesSaved", fetch_error);
        },

        Msg::ClearErrors => {
            log!("Msg::ClearErrors");
        },

        // ------ Client ------

        Msg::AddClient => {
            log!("Msg::AddClient");
        },
        Msg::DeleteClient(client_id) => {
            log!("Msg::DeleteClient", client_id);
        },

        Msg::ClientNameChanged(client_id, name) => {
            if let RemoteData::Loaded(clients) = &mut model.clients {
                if let Some(client) = clients.get_mut(&client_id) {
                    log!("Msg::ClientNameChanged", client_id, name);
                    client.name = name;
                }
            }
        },
        Msg::SaveClientName(client_id) => {
            log!("Msg::SaveClientName", client_id);
        },

        // ------ Project ------

        Msg::AddProject(client_id) => {
            log!("Msg::AddProject", client_id);
        },
        Msg::DeleteProject(client_id, project_id) => {
            log!("Msg::DeleteProject", client_id, project_id);
        },

        Msg::ProjectNameChanged(client_id, project_id, name) => {
            if let RemoteData::Loaded(clients) = &mut model.clients {
                if let Some(client) = clients.get_mut(&client_id) {
                    if let Some(project) = client.projects.get_mut(&project_id) {
                        log!("Msg::ProjectNameChanged", client_id, name);
                        project.name = name;
                    }
                }
            }
        },
        Msg::SaveProjectName(client_id, project_id) => {
            log!("Msg::SaveProjectName", client_id, project_id);
        },
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    section![
        h1![C!["title", "ml-6", "my-6"],
            "Clients & Projects",
        ],
        div![C!["columns", "is-centered"],
            div![C!["column", "is-half"],
                view_add_client_button(),
                match &model.clients {
                    RemoteData::NotAsked | RemoteData::Loading => {
                        progress![C!["progress", "is-link", "mt-6"]].into_nodes()
                    },
                    RemoteData::Loaded(clients) => {
                        clients.iter().rev().map(|(client_id, client)| view_client(*client_id, client)).collect()
                    }
                }
            ]
        ]
    ]
}

fn view_add_client_button() -> Node<Msg> {
    div![C!["level", "is-mobile"],
        button![C!["button", "is-primary", "is-rounded"],
            style!{
                St::MarginLeft => "auto",
                St::MarginRight => "auto",
            },
            ev(Ev::Click, |_| Msg::AddClient),
            span![C!["icon"],
                i![C!["fas", "fa-plus"]]
            ],
            span!["Add Client"],
        ],
    ]
}

fn view_client(client_id: ClientId, client: &Client) -> Node<Msg> {
    div![C!["box", "has-background-link", "mt-6"],
        div![C!["level", "is-mobile"],
            input![C!["input", "is-size-3", "has-text-link-light"], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(3.5),
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                    St::MaxWidth => percent(85),
                },
                attrs!{At::Value => client.name},
                input_ev(Ev::Input, move |name| Msg::ClientNameChanged(client_id, name)),
                ev(Ev::Change, move |_| Msg::SaveClientName(client_id)),
            ],
            view_delete_button(move || Msg::DeleteClient(client_id)),
        ],
        view_add_project_button(client_id),
        client.projects.iter().rev().map(|(project_id, project)| view_project(client_id, *project_id, project)),
    ]
}

fn view_add_project_button(client_id: ClientId) -> Node<Msg> {
    div![C!["level", "is-mobile"],
        button![C!["button", "is-primary", "is-rounded"],
            style!{
                St::MarginLeft => "auto",
                St::MarginRight => "auto",
            },
            ev(Ev::Click, move |_| Msg::AddProject(client_id)),
            span![C!["icon"],
                i![C!["fas", "fa-plus"]]
            ],
            span!["Add Project"],
        ],
    ]
}

fn view_project(client_id: ClientId, project_id: ProjectId, project: &Project) -> Node<Msg> {
    div![C!["box"],
        div![C!["level", "is-mobile"],
            input![C!["input", "is-size-4"], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(3),
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                    St::MaxWidth => percent(85),
                },
                attrs!{At::Value => project.name},
                input_ev(Ev::Input, move |name| Msg::ProjectNameChanged(client_id, project_id, name)),
                ev(Ev::Change, move |_| Msg::SaveProjectName(client_id, project_id)),
            ],
            view_delete_button(move || Msg::DeleteProject(client_id, project_id)),
        ],
    ]
}

fn view_delete_button(on_click: impl Fn() -> Msg + Clone + 'static) -> Node<Msg> {
    button![C!["button", "is-primary", "is-rounded"],
        style!{
            St::Width => 0,
        },
        ev(Ev::Click, move |_| on_click()),
        span![C!["icon"],
            i![C!["fas", "fa-trash-alt"]]
        ],
    ]
}
