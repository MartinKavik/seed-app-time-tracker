use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use cynic::QueryFragment;

use std::collections::BTreeMap;
use std::convert::identity;

use crate::graphql;

const PRIMARY_COLOR: &str = "#00d1b2";
const LINK_COLOR: &str = "#3273dc";

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

        clients: RemoteData::Loading,
        timer_handle: orders.stream_with_handle(streams::interval(1000, || Msg::OnSecondTick)),
    }
}

async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
    use graphql::queries::clients_with_projects_with_time_entries as query_mod;

    let time_entry_mapper = |time_entry: query_mod::TimeEntry| (
        time_entry.id.parse().expect("parse time_entry Ulid"),
        TimeEntry {
            name: time_entry.name,
            started: time_entry.started.0.parse().expect("parse time_entry started time"),
            stopped: time_entry.stopped.map(|time| time.0.parse().expect("parse time_entry started time")),
            change: None,
        }
    );

    let project_mapper = |project: query_mod::Project| (
        project.id.parse().expect("parse project Ulid"), 
        Project { 
            name: project.name, 
            time_entries: project.time_entries.into_iter().map(time_entry_mapper).collect()
        },
    );

    let client_mapper = |client: query_mod::Client| (
        client.id.parse().expect("parse client Ulid"),
        Client {
            name: client.name,
            projects: client.projects.into_iter().map(project_mapper).collect()
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
    timer_handle: StreamHandle, 
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
    change: Option<TimeEntryChange>,
}

#[derive(Debug)]
enum TimeEntryChange {
    StartedDate(String),
    StartedTime(String),
    StoppedDate(String),
    StoppedTime(String),
    Duration(String),
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
    
    TimeEntryStartedDateChanged(ClientId, ProjectId, TimeEntryId, String),
    TimeEntryStartedTimeChanged(ClientId, ProjectId, TimeEntryId, String),

    TimeEntryDurationChanged(ClientId, ProjectId, TimeEntryId, String),
    
    TimeEntryStoppedDateChanged(ClientId, ProjectId, TimeEntryId, String),
    TimeEntryStoppedTimeChanged(ClientId, ProjectId, TimeEntryId, String),

    SaveTimeEntryChange(ClientId, ProjectId, TimeEntryId),

    OnSecondTick,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
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

        Msg::Start(client_id, project_id) => {
            let mut start_time_entry = move |client_id, project_id| -> Option<()> {
                let time_entries = &mut model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries;

                let previous_name = time_entries
                    .iter()
                    .next_back()
                    .map(|(_, time_entry)| time_entry.name.to_owned());

                let time_entry_id = TimeEntryId::new();
                let time_entry = TimeEntry {
                    name: previous_name.unwrap_or_default(),
                    started: chrono::Local::now(),
                    stopped: None,
                    change: None,
                };
                // @TODO: Send request.
                time_entries.insert(time_entry_id, time_entry);

                Some(())
            };
            start_time_entry(client_id, project_id);
        },
        Msg::Stop(client_id, project_id) => {
            let mut stop_time_entry = move |client_id, project_id| -> Option<()> {
                let (time_entry_id, time_entry) = model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .iter_mut()
                    .find(|(_, time_entry)| time_entry.stopped.is_none())?;
                
                time_entry.stopped = Some(chrono::Local::now());
                // @TODO: Send request.
                Some(())
            };
            stop_time_entry(client_id, project_id);
        },

        Msg::DeleteTimeEntry(client_id, project_id, time_entry_id) => {
            let mut delete_time_entry = move |client_id, project_id, time_entry_id| -> Option<()> {
                let time_entries = &mut model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries;

                let time_entry_name = &time_entries.get_mut(&time_entry_id)?.name;

                if let Ok(true) = window().confirm_with_message(&format!("Time Entry \"{}\" will be deleted.", time_entry_name)) {
                    time_entries.remove(&time_entry_id);
                    // @TODO: Send request.
                }
                Some(())
            };
            delete_time_entry(client_id, project_id, time_entry_id);
        },

        Msg::TimeEntryNameChanged(client_id, project_id, time_entry_id, name) => {
            let mut set_time_entry_name = move |name| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?
                    .name = name)
            };
            set_time_entry_name(name);
        },
        Msg::SaveTimeEntryName(client_id, project_id, time_entry_id) => {
            // @TODO: Send request.
        },

        Msg::TimeEntryStartedDateChanged(client_id, project_id, time_entry_id, date) => {
            let mut set_time_entry_change = move |change| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?
                    .change = Some(change))
            };
            set_time_entry_change(TimeEntryChange::StartedDate(date));
        },
        Msg::TimeEntryStartedTimeChanged(client_id, project_id, time_entry_id, time) => {
            let mut set_time_entry_change = move |change| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?
                    .change = Some(change))
            };
            set_time_entry_change(TimeEntryChange::StartedTime(time));
        },

        Msg::TimeEntryDurationChanged(client_id, project_id, time_entry_id, duration) => {
            let mut set_time_entry_change = move |change| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?
                    .change = Some(change))
            };
            set_time_entry_change(TimeEntryChange::Duration(duration));
        },

        Msg::TimeEntryStoppedDateChanged(client_id, project_id, time_entry_id, date) => {
            let mut set_time_entry_change = move |change| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?
                    .change = Some(change))
            };
            set_time_entry_change(TimeEntryChange::StoppedDate(date));
        },
        Msg::TimeEntryStoppedTimeChanged(client_id, project_id, time_entry_id, time) => {
            let mut set_time_entry_change = move |change| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?
                    .change = Some(change))
            };
            set_time_entry_change(TimeEntryChange::StoppedTime(time));
        },

        Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id) => {
            let mut save_time_entry_change = move || -> Option<()> {
                let time_entry = model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .projects
                    .get_mut(&project_id)?
                    .time_entries
                    .get_mut(&time_entry_id)?;

                match time_entry.change.take()? {
                    TimeEntryChange::StartedDate(date) => {
                        let date = chrono::NaiveDate::parse_from_str(&date, "%F").ok()?;
                        let time = time_entry.started.time();
                        time_entry.started = Local.from_local_date(&date).and_time(time).single()?;
                    }
                    TimeEntryChange::StartedTime(time) => {
                        let time = chrono::NaiveTime::parse_from_str(&time, "%X").ok()?;
                        let date = time_entry.started.naive_local().date();
                        time_entry.started = Local.from_local_date(&date).and_time(time).single()?;
                    }
                    TimeEntryChange::Duration(mut duration) => {
                        let negative = duration.chars().next()? == '-';
                        if negative {
                            duration.remove(0);
                        }
                        let mut duration_parts = duration.split(':');
                        let hours: i64 = duration_parts.next()?.parse().ok()?;
                        let minutes: i64 = duration_parts.next()?.parse().ok()?;
                        let seconds: i64 = duration_parts.next()?.parse().ok()?;

                        let mut total_seconds = hours * 3600 + minutes * 60 + seconds;
                        if negative {
                            total_seconds = -total_seconds;
                        }
                        let duration = chrono::Duration::seconds(total_seconds);
                        time_entry.stopped = Some(time_entry.started + duration);
                    }
                    TimeEntryChange::StoppedDate(date) => {
                        let date = chrono::NaiveDate::parse_from_str(&date, "%F").ok()?;
                        let time = time_entry.stopped?.time();
                        time_entry.stopped = Some(Local.from_local_date(&date).and_time(time).single()?);
                    }
                    TimeEntryChange::StoppedTime(time) => {
                        let time = chrono::NaiveTime::parse_from_str(&time, "%X").ok()?;
                        let date = time_entry.stopped?.naive_local().date();
                        time_entry.stopped = Some(Local.from_local_date(&date).and_time(time).single()?);
                    }
                }
                // @TODO: Send request.
                Some(())
            };
            save_time_entry_change();
        },

        Msg::OnSecondTick => (),
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    section![
        h1![C!["title", "ml-6", "mt-6", "mb-5"],
            "Time Tracker",
        ],
        div![C!["columns", "is-centered"],
            div![C!["column", "is-two-thirds"],
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

fn view_client(client_id: ClientId, client: &Client) -> Node<Msg> {
    div![C!["box", "has-background-link", "mt-6",],
        div![C!["level", "is-mobile"],
            div![C!["is-size-3", "has-text-link-light"], 
                &client.name,
            ],
        ],
        client.projects.iter().rev().map(|(project_id, project)| view_project(client_id, *project_id, project)),
    ]
}

fn view_project(client_id: ClientId, project_id: ProjectId, project: &Project) -> Node<Msg> {
    let active_time_entry = project
        .time_entries
        .iter()
        .find(|(_, time_entry)| time_entry.stopped.is_none());

    div![C!["box", "mt-6"],
        div![C!["level", "is-mobile"],
            div![C!["is-size-4"], 
                &project.name,
            ],
            view_start_stop_button(client_id, project_id, active_time_entry.is_some()),
        ],
        project.time_entries.iter().rev().map(|(time_entry_id, time_entry)| {
            view_time_entry(client_id, project_id, *time_entry_id, time_entry)
        }),
    ]
}

fn view_start_stop_button(client_id: ClientId, project_id: ProjectId, started: bool) -> Node<Msg> {
    div![C!["level", "is-mobile"],
        button![C!["button", if started { "is-warning" } else { "is-primary" }, "is-rounded"],
            ev(Ev::Click, move |_| if started { 
                Msg::Stop(client_id, project_id) 
            } else { 
                Msg::Start(client_id, project_id) 
            }),
            span![if started { "Stop" } else { "Start" }],
        ],
    ]
}

fn view_time_entry(
    client_id: ClientId, 
    project_id: ProjectId, 
    time_entry_id: TimeEntryId, 
    time_entry: &TimeEntry
) -> Node<Msg> {
    let active = time_entry.stopped.is_none();
    let stopped = time_entry.stopped.as_ref().cloned().unwrap_or_else(chrono::Local::now);
    let duration = stopped - time_entry.started;

    div![C!["box", if active { "has-background-warning" } else { "has-background-link"}, IF!(not(active) => "has-text-link-light")],
        div![C!["level", "is-mobile"], style!{St::MarginBottom => px(5)},
            input![C!["input", "is-size-4", IF!(not(active) => "has-text-link-light")], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(3),
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", if active { LINK_COLOR } else { PRIMARY_COLOR }, px(2)),
                    St::MaxWidth => percent(85),
                },
                attrs!{At::Value => time_entry.name},
                input_ev(Ev::Input, move |name| Msg::TimeEntryNameChanged(client_id, project_id, time_entry_id, name)),
                ev(Ev::Change, move |_| Msg::SaveTimeEntryName(client_id, project_id, time_entry_id)),
            ],
            view_delete_button(move || Msg::DeleteTimeEntry(client_id, project_id, time_entry_id), active),
        ],
        div![C!["level", "is-mobile", "is-hidden-tablet"], style!{St::MarginBottom => 0},
            view_duration(client_id, project_id, time_entry_id, &duration, time_entry.change.as_ref(), active)
        ],
        div![C!["level", "is-mobile"],
            view_started(client_id, project_id, time_entry_id, time_entry.change.as_ref(), active, &time_entry.started),
            div![C!["is-hidden-mobile"],
                view_duration(client_id, project_id, time_entry_id, &duration, time_entry.change.as_ref(), active),
            ],
            view_stopped(client_id, project_id, time_entry_id,  time_entry.change.as_ref(), active, &stopped),
        ],
    ]
}

fn view_started(
    client_id: ClientId, 
    project_id: ProjectId, 
    time_entry_id: TimeEntryId, 
    time_entry_change: Option<&TimeEntryChange>,
    for_active_time_entry: bool,
    started: &chrono::DateTime<chrono::Local>,
) -> Node<Msg> {
    div![C!["is-flex"], style!{St::FlexDirection => "column"},
        input![C!["input", "has-text-centered", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }],
            style!{
                St::BoxShadow => "none",
                St::BackgroundColor => "transparent",
                St::Height => rem(2),
                St::Border => "none",
                St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(1)),
                St::MaxWidth => rem(10),
            },
            attrs!{
                At::Value => if let Some(TimeEntryChange::StartedDate(date)) = time_entry_change {
                    date.to_owned()
                } else {
                    started.format("%F").to_string()
                }
            },
            input_ev(Ev::Input, move |date| Msg::TimeEntryStartedDateChanged(client_id, project_id, time_entry_id, date)),
            ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
        ],
        input![C!["input", "is-size-5", "has-text-centered", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }], 
            style!{
                St::BoxShadow => "none",
                St::BackgroundColor => "transparent",
                St::Height => rem(3),
                St::Border => "none",
                St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                St::MaxWidth => rem(10),
            },
            attrs!{
                At::Value => if let Some(TimeEntryChange::StartedTime(time)) = time_entry_change {
                    time.to_owned()
                } else {
                    started.format("%X").to_string()
                }
            },
            input_ev(Ev::Input, move |time| Msg::TimeEntryStartedTimeChanged(client_id, project_id, time_entry_id, time)),
            ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
        ],
    ]
}

fn view_stopped(
    client_id: ClientId, 
    project_id: ProjectId, 
    time_entry_id: TimeEntryId, 
    time_entry_change: Option<&TimeEntryChange>,
    for_active_time_entry: bool,
    stopped: &chrono::DateTime<chrono::Local>,
) -> Node<Msg> {
    div![C!["is-flex"], style!{St::FlexDirection => "column"},
        input![C!["input", "has-text-centered", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }],
            style!{
                St::BoxShadow => "none",
                St::BackgroundColor => "transparent",
                St::Height => rem(2),
                St::Border => "none",
                St::BorderBottom => IF!(not(for_active_time_entry) => {
                    format!("{} {} {}", "solid", PRIMARY_COLOR, px(1))
                }),
                St::MaxWidth => rem(10),
            },
            attrs!{
                At::Disabled => for_active_time_entry.as_at_value(),
                At::Value => if let Some(TimeEntryChange::StoppedDate(date)) = time_entry_change {
                    date.to_owned()
                } else {
                    stopped.format("%F").to_string()
                }
            },
            input_ev(Ev::Input, move |date| Msg::TimeEntryStoppedDateChanged(client_id, project_id, time_entry_id, date)),
            ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
        ],
        input![C!["input", "has-text-centered", "is-size-5", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }], 
            style!{
                St::BoxShadow => "none",
                St::BackgroundColor => "transparent",
                St::Height => rem(3),
                St::Border => "none",
                St::BorderBottom => IF!(not(for_active_time_entry) => {
                    format!("{} {} {}", "solid", PRIMARY_COLOR, px(2))
                }),
                St::MaxWidth => rem(10),
            },
            attrs!{
                At::Disabled => for_active_time_entry.as_at_value(),
                At::Value => if let Some(TimeEntryChange::StoppedTime(time)) = time_entry_change {
                    time.to_owned()
                } else {
                    stopped.format("%X").to_string()
                }
            },
            input_ev(Ev::Input, move |time| Msg::TimeEntryStoppedTimeChanged(client_id, project_id, time_entry_id, time)),
            ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
        ],
    ]
}

fn view_duration(
    client_id: ClientId, 
    project_id: ProjectId, 
    time_entry_id: TimeEntryId, 
    duration: &chrono::Duration, 
    time_entry_change: Option<&TimeEntryChange>, 
    for_active_time_entry: bool
) -> Node<Msg> {
    let num_seconds = duration.num_seconds();

    let negative = num_seconds < 0;
    let num_seconds = num_seconds.abs();
    let hours = num_seconds / 3600;
    let minutes = num_seconds % 3600 / 60;
    let seconds = num_seconds % 60;

    input![C!["input", "has-text-centered", "is-size-4", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }], 
        style!{
            St::Margin => "auto",
            St::BoxShadow => "none",
            St::BackgroundColor => "transparent",
            St::Height => rem(3),
            St::Border => "none",
            St::BorderBottom => IF!(not(for_active_time_entry) => {
                format!("{} {} {}", "solid", PRIMARY_COLOR, px(2))
            }),
            St::MaxWidth => rem(10),
        },
        attrs!{
            At::Disabled => for_active_time_entry.as_at_value(),
            At::Value => if let Some(TimeEntryChange::Duration(duration)) = time_entry_change {
                duration.to_owned()
            } else {
                format!("{}{}:{:02}:{:02}", if negative { "-" } else { "" }, hours, minutes, seconds)
            }
        },
        input_ev(Ev::Input, move |duration| Msg::TimeEntryDurationChanged(client_id, project_id, time_entry_id, duration)),
        ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
    ]
}

fn view_delete_button(on_click: impl Fn() -> Msg + Clone + 'static, for_active_time_entry: bool) -> Node<Msg> {
    button![C!["button", if for_active_time_entry { "is-link" } else { "is-primary" }, "is-rounded"],
        style!{
            St::Width => 0,
        },
        ev(Ev::Click, move |_| on_click()),
        span![C!["icon"],
            i![C!["fas", "fa-trash-alt"]]
        ],
    ]
}
