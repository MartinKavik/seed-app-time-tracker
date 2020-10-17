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
        Project { 
            name: project.name, 
            name_input: ElRef::new(), 
        }
    );

    let client_mapper = |client: query_mod::Client| (
        client.id.parse().expect("parse client Ulid"),
        Client {
            name: client.name,
            projects: client.projects.into_iter().map(project_mapper).collect(),
            name_input: ElRef::new(),
        }
    );

    Ok(
        graphql::send_query(query_mod::Query::fragment(&()))
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

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

// ---- Remote Data ----

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

impl<T> RemoteData<T> {
    fn loaded(&self) -> Option<&T> {
        if let Self::Loaded(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn loaded_mut(&mut self) -> Option<&mut T> {
        if let Self::Loaded(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

// --- Entities ----

#[derive(Debug)]
pub struct Client {
    name: String,
    projects: BTreeMap<ProjectId, Project>,
    name_input: ElRef<web_sys::HtmlInputElement>,
}

#[derive(Debug)]
struct Project {
    name: String,
    name_input: ElRef<web_sys::HtmlInputElement>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<graphql::GraphQLError>),
    ClearErrors,
    
    // ------ Client ------

    AddClient,
    DeleteClient(ClientId),
    FocusClientName(ClientId),

    ClientNameChanged(ClientId, String),
    SaveClientName(ClientId),
    
    // ------ Project ------

    AddProject(ClientId),
    DeleteProject(ClientId, ProjectId),
    FocusProjectName(ClientId, ProjectId),
    
    ProjectNameChanged(ClientId, ProjectId, String),
    SaveProjectName(ClientId, ProjectId),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClientsFetched(Ok(clients)) => {
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
            model.errors.clear();
        },

        // ------ Client ------

        Msg::AddClient => {
            if let Some(clients) = model.clients.loaded_mut() {
                let client_id = ClientId::new();
                let client = Client {
                    name: "".to_owned(),
                    projects: BTreeMap::new(),
                    name_input: ElRef::new(),
                };

                let args = graphql::mutations::client::add::AddClientArguments {
                    id: client_id.to_string(),
                    user: "DUMMY_USER_ID".to_owned(),
                };
                orders.perform_cmd(async move { Msg::ChangesSaved(
                    graphql::send_mutation(
                        graphql::mutations::client::add::Mutation::fragment(&args)
                    ).await.err()
                )});

                clients.insert(client_id, client);
                orders.after_next_render(move |_| Msg::FocusClientName(client_id));
            }
        },
        Msg::DeleteClient(client_id) => {
            let mut delete_client = move |client_id| -> Option<()> {
                let clients = model.clients.loaded_mut()?;
                let client_name = clients.get(&client_id).map(|client| &client.name)?;

                if let Ok(true) = window().confirm_with_message(&format!("Client \"{}\" will be deleted.", client_name)) {
                    clients.remove(&client_id);
                    
                    let args = graphql::mutations::client::delete::DeleteClientArguments {
                        id: client_id.to_string(),
                    };
                    orders.perform_cmd(async move { Msg::ChangesSaved(
                        graphql::send_mutation(
                            graphql::mutations::client::delete::Mutation::fragment(&args)
                        ).await.err()
                    )});
                }
                Some(())
            };
            delete_client(client_id);
        },
        Msg::FocusClientName(client_id) => {
            let mut focus_client_name = move |client_id| -> Option<()> {
                model
                    .clients
                    .loaded_mut()?
                    .get(&client_id)?
                    .name_input
                    .get()?
                    .focus()
                    .ok()
            };
            focus_client_name(client_id);
        }

        Msg::ClientNameChanged(client_id, name) => {
            let mut set_client_name = move |name| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .name = name)
            };
            set_client_name(name);
        },
        Msg::SaveClientName(client_id) => {
            let mut save_client_name = move |client_id| -> Option<()> {
                let name = &model
                    .clients
                    .loaded()?
                    .get(&client_id)?
                    .name;

                let args = graphql::mutations::client::rename::RenameClientArguments {
                    id: client_id.to_string(),
                    name: name.clone(),
                };
                orders.perform_cmd(async move { Msg::ChangesSaved(
                    graphql::send_mutation(
                        graphql::mutations::client::rename::Mutation::fragment(&args)
                    ).await.err()
                )});
                Some(())
            };
            save_client_name(client_id);
        }

        // ------ Project ------

        Msg::AddProject(client_id) => {
            let mut add_project = move |client_id| -> Option<()> {
                let projects = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.projects;

                let project_id = ProjectId::new();
                let project = Project {
                    name: "".to_owned(),
                    name_input: ElRef::new(),
                };
                
                let args = graphql::mutations::project::add::AddProjectArguments {
                    id: project_id.to_string(),
                    client: client_id.to_string(),
                };
                orders.perform_cmd(async move { Msg::ChangesSaved(
                    graphql::send_mutation(
                        graphql::mutations::project::add::Mutation::fragment(&args)
                    ).await.err()
                )});

                projects.insert(project_id, project);
                orders.after_next_render(move |_| Msg::FocusProjectName(client_id, project_id));

                Some(())
            };
            add_project(client_id);
        },
        Msg::DeleteProject(client_id, project_id) => {
            let mut delete_project = move |client_id, project_id| -> Option<()> {
                let projects = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.projects;
                let project_name = projects.get(&project_id).map(|project| &project.name)?;

                if let Ok(true) = window().confirm_with_message(&format!("Project \"{}\" will be deleted.", project_name)) {
                    projects.remove(&project_id);
                    
                    let args = graphql::mutations::project::delete::DeleteProjectArguments {
                        id: project_id.to_string(),
                    };
                    orders.perform_cmd(async move { Msg::ChangesSaved(
                        graphql::send_mutation(
                            graphql::mutations::project::delete::Mutation::fragment(&args)
                        ).await.err()
                    )});

                }
                Some(())
            };
            delete_project(client_id, project_id);
        },
        Msg::FocusProjectName(client_id, project_id) => {
            let mut focus_project_name = move |client_id, project_id| -> Option<()> {
                model
                    .clients
                    .loaded_mut()?
                    .get(&client_id)?
                    .projects
                    .get(&project_id)?
                    .name_input
                    .get()?
                    .focus()
                    .ok()
            };
            focus_project_name(client_id, project_id);
        }

        Msg::ProjectNameChanged(client_id, project_id, name) => {
            let mut set_project_name = move |name| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .name = name)
            };
            set_project_name(name);
        },
        Msg::SaveProjectName(client_id, project_id) => {
            let mut save_project_name = move |project_id| -> Option<()> {
                let name = &model
                    .clients
                    .loaded()?
                    .get(&client_id)?
                    .projects
                    .get(&project_id)?
                    .name;

                let args = graphql::mutations::project::rename::RenameProjectArguments {
                    id: project_id.to_string(),
                    name: name.clone(),
                };
                orders.perform_cmd(async move { Msg::ChangesSaved(
                    graphql::send_mutation(
                        graphql::mutations::project::rename::Mutation::fragment(&args)
                    ).await.err()
                )});
                Some(())
            };
            save_project_name(project_id);
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
                el_ref(&client.name_input),
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
                el_ref(&project.name_input),
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
