use seed::{prelude::*, *};

use chrono::{prelude::*, Duration};
use ulid::Ulid;

use cynic::QueryFragment;

use std::collections::BTreeMap;
use std::convert::identity;
use std::ops::Add;

use crate::graphql;

const PRIMARY_COLOR: &str = "#00d1b2";

type ClientId = Ulid;
type TimeBlockId = Ulid;

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
    use graphql::queries::clients_with_time_blocks_and_time_entries as query_mod;

    let invoice_mapper = |invoice: query_mod::Invoice| {
        Invoice {
            custom_id: invoice.custom_id,
            url: invoice.url,
        }
    };

    let status_mapper = |status: query_mod::TimeBlockStatus| {
        match status {
            query_mod::TimeBlockStatus::NON_BILLABLE => TimeBlockStatus::NonBillable,
            query_mod::TimeBlockStatus::UNPAID => TimeBlockStatus::Unpaid,
            query_mod::TimeBlockStatus::PAID => TimeBlockStatus::Paid,
        }
    };

    let time_block_mapper = |time_block: query_mod::TimeBlock| (
        time_block.id.parse().expect("parse time_block Ulid"), 
        TimeBlock { 
            name: time_block.name,
            status: status_mapper(time_block.status),
            duration: Duration::seconds(i64::from(time_block.duration)),
            duration_change: None,
            invoice: time_block.invoice.map(invoice_mapper),
            name_input: ElRef::new(),
        }
    );

    let compute_tracked_time = |projects: Vec<query_mod::Project>| {
        projects
            .into_iter()
            .flat_map(|project| project.time_entries)
            .map(|time_entry| {
                let started: DateTime<Local> = 
                    time_entry.started.0.parse().expect("parse time_entry started");
                
                let stopped: DateTime<Local> = if let Some(stopped) = time_entry.stopped {
                    stopped.0.parse().expect("parse time_entry stopped")
                } else {
                    chrono::Local::now()
                };
                
                stopped - started
            })
            .fold(Duration::seconds(0), Duration::add)
    };

    let client_mapper = |client: query_mod::Client| (
        client.id.parse().expect("parse client Ulid"),
        Client {
            name: client.name,
            time_blocks: client.time_blocks.into_iter().map(time_block_mapper).collect(),
            tracked: compute_tracked_time(client.projects),
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
    time_blocks: BTreeMap<Ulid, TimeBlock>,
    tracked: Duration,
}

#[derive(Debug)]
struct TimeBlock {
    name: String,
    status: TimeBlockStatus,
    duration: Duration,
    duration_change: Option<String>,
    invoice: Option<Invoice>,
    name_input: ElRef<web_sys::HtmlInputElement>,
}

#[derive(Debug, Copy, Clone)]
pub enum TimeBlockStatus {
    NonBillable,
    Unpaid,
    Paid,
}

#[derive(Debug)]
struct Invoice {
    custom_id: Option<String>,
    url: Option<String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,

    // ------ TimeBlock ------
    
    AddTimeBlock(ClientId),
    DeleteTimeBlock(ClientId, TimeBlockId),
    SetTimeBlockStatus(ClientId, TimeBlockId, TimeBlockStatus),
    FocusTimeBlockName(ClientId, TimeBlockId),

    TimeBlockNameChanged(ClientId, TimeBlockId, String),
    SaveTimeBlockName(ClientId, TimeBlockId),

    TimeBlockDurationChanged(ClientId, TimeBlockId, String),
    SaveTimeBlockDuration(ClientId, TimeBlockId),

    // ------ Invoice ------

    AttachInvoice(ClientId, TimeBlockId),
    DeleteInvoice(ClientId, TimeBlockId),

    InvoiceCustomIdChanged(ClientId, TimeBlockId, String),
    SaveInvoiceCustomId(ClientId, TimeBlockId),

    InvoiceUrlChanged(ClientId, TimeBlockId, String),
    SaveInvoiceUrl(ClientId, TimeBlockId),
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

        // ------ TimeBlock ------
        
        Msg::AddTimeBlock(client_id) => {
            let mut add_time_block = move |client_id| -> Option<()> {
                let time_blocks = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.time_blocks;

                let previous_duration = time_blocks
                    .iter()
                    .next_back()
                    .map(|(_, time_block)| time_block.duration);

                let time_block_id = TimeBlockId::new();
                let time_block = TimeBlock {
                    name: "".to_owned(),
                    status: TimeBlockStatus::Unpaid,
                    duration: previous_duration.unwrap_or_else(|| chrono::Duration::hours(20)),
                    duration_change: None,
                    invoice: None,
                    name_input: ElRef::new(),
                };
                // @TODO: Send request.
                time_blocks.insert(time_block_id, time_block);
                orders.after_next_render(move |_| Msg::FocusTimeBlockName(client_id, time_block_id));

                Some(())
            };
            add_time_block(client_id);
        },
        Msg::DeleteTimeBlock(client_id, time_block_id) => {
            let mut delete_time_block = move |client_id, time_block_id| -> Option<()> {
                let time_blocks = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.time_blocks;
                let time_block_name = time_blocks.get(&time_block_id).map(|time_block| &time_block.name)?;

                if let Ok(true) = window().confirm_with_message(&format!("Time Block \"{}\" will be deleted.", time_block_name)) {
                    time_blocks.remove(&time_block_id);
                    // @TODO: Send request.
                }
                Some(())
            };
            delete_time_block(client_id, time_block_id);
        },
        Msg::SetTimeBlockStatus(client_id, time_block_id, time_block_status) => {
            let mut set_time_block_status = move |status| -> Option<()> {
                model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?
                    .status = status;
                // @TODO: Send request.
                Some(())
            };
            set_time_block_status(time_block_status);
        },
        Msg::FocusTimeBlockName(client_id, time_block_id) => {
            let mut focus_time_block_name = move |client_id, time_block_id| -> Option<()> {
                model
                    .clients
                    .loaded_mut()?
                    .get(&client_id)?
                    .time_blocks
                    .get(&time_block_id)?
                    .name_input
                    .get()?
                    .focus()
                    .ok()
            };
            focus_time_block_name(client_id, time_block_id);
        }

        Msg::TimeBlockNameChanged(client_id, time_block_id, name) => {
            let mut set_time_block_name = move |name| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?
                    .name = name)
            };
            set_time_block_name(name);
        },
        Msg::SaveTimeBlockName(client_id, time_block_id) => {
            // @TODO: Send request.
        },

        Msg::TimeBlockDurationChanged(client_id, time_block_id, duration) => {
            let mut set_time_block_duration_change = move |duration| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?
                    .duration_change = Some(duration))
            };
            set_time_block_duration_change(duration);
        },
        Msg::SaveTimeBlockDuration(client_id, time_block_id) => {
            let mut set_time_block_duration = move || -> Option<()> {
                let time_block = model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?;

                let hours = time_block.duration_change.take()?.parse::<f64>().ok()?;
                time_block.duration = chrono::Duration::seconds((hours * 3600.0) as i64);
                // @TODO: Send request.
                Some(())
            };
            set_time_block_duration();
        },

        // ------ Invoice ------

        Msg::AttachInvoice(client_id, time_block_id) => {
            let mut attach_invoice = move |client_id, time_block_id| -> Option<()> {
                let time_block = model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?;

                let invoice = Invoice {
                    custom_id: Some("".to_owned()),
                    url: Some("".to_owned()),
                };
                // @TODO: Send request.
                time_block.invoice = Some(invoice);
                Some(())
            };
            attach_invoice(client_id, time_block_id);
        },
        Msg::DeleteInvoice(client_id, time_block_id) => {
            let mut delete_invoice = move |client_id, time_block_id| -> Option<()> {
                let time_block = model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?;

                if let Ok(true) = window().confirm_with_message(&format!("Invoice attached to Time Block \"{}\" will be deleted.", time_block.name)) {
                    time_block.invoice = None;
                    // @TODO: Send request.
                }
                Some(())
            };
            delete_invoice(client_id, time_block_id);
        },

        Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id) => {
            let mut set_invoice_custom_id = move |client_id, time_block_id, custom_id| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?
                    .invoice.as_mut()?
                    .custom_id = Some(custom_id))
            };
            set_invoice_custom_id(client_id, time_block_id, custom_id);
        },
        Msg::SaveInvoiceCustomId(client_id, time_block_id) => {
            // @TODO: Send request.
        },

        Msg::InvoiceUrlChanged(client_id, time_block_id, url) => {
            let mut set_invoice_url = move |client_id, time_block_id, url| -> Option<()> {
                Some(model
                    .clients
                    .loaded_mut()?
                    .get_mut(&client_id)?
                    .time_blocks
                    .get_mut(&time_block_id)?
                    .invoice.as_mut()?
                    .url = Some(url))
            };
            set_invoice_url(client_id, time_block_id, url);
        },
        Msg::SaveInvoiceUrl(client_id, time_block_id) => {
            // @TODO: Send request.
        },
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    section![
        h1![C!["title", "ml-6", "mt-6", "mb-5"],
            "Time Blocks",
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
        div![C!["level", "is-mobile"], style!{St::FlexWrap => "wrap", St::MarginBottom => 0},
            div![C!["is-size-3", "has-text-link-light", "mb-2"], 
                &client.name,
            ],
            view_statistics(client.time_blocks.values(), &client.tracked),
        ],
        view_add_time_block_button(client_id),
        client.time_blocks.iter().rev().map(|(time_block_id, time_block)| view_time_block(client_id, *time_block_id, time_block)),
    ]
}

fn view_statistics<'a>(time_blocks: impl Iterator<Item = &'a TimeBlock>, tracked: &Duration, ) -> Node<Msg> {
    let mut blocked = 0.;
    let mut unpaid_total = 0.;
    let mut paid_total = 0.;

    for time_block in time_blocks {
        let hours = time_block.duration.num_minutes() as f64 / 60.;
        blocked += hours;

        match time_block.status {
            TimeBlockStatus::NonBillable => (),
            TimeBlockStatus::Unpaid => unpaid_total += hours,
            TimeBlockStatus::Paid => paid_total += hours,
        };
    }

    let tracked = tracked.num_minutes() as f64 / 60.;
    let to_block = tracked - blocked;

    let pair = |key: &str, value: f64| {
        div![C!["is-flex"], style!{St::JustifyContent => "space-between"},
            span![
                key
            ],
            span![style!{St::MarginLeft => rem(1)},
                format!("{:.1}", value)
            ],
        ]
    };

    div![C!["level", "is-mobile"], style!{St::AlignItems => "baseline"},
        div![C!["box", "has-background-link", "has-text-link-light"],
            pair("Blocked", blocked),
            div![style!{St::Height => rem(1)}],
            pair("Unpaid", unpaid_total),
            pair("Paid", paid_total),
        ],
        div![
            div![C!["box", "has-background-link", "has-text-link-light"],
                style!{St::MarginBottom => 0},
                pair("Tracked", tracked),
            ],
            div![C!["box", "has-background-link", "has-text-link-light"],
                pair("To Block", to_block),
            ],
        ]
    ]
}

fn view_add_time_block_button(client_id: ClientId) -> Node<Msg> {
    div![C!["level", "is-mobile"],
        button![C!["button", "is-primary", "is-rounded"],
            style!{
                St::MarginLeft => "auto",
                St::MarginRight => "auto",
            },
            ev(Ev::Click, move |_| Msg::AddTimeBlock(client_id)),
            span![C!["icon"],
                i![C!["fas", "fa-plus"]]
            ],
            span!["Add Time Block"],
        ],
    ]
}

fn view_time_block(client_id: ClientId, time_block_id: TimeBlockId, time_block: &TimeBlock) -> Node<Msg> {
    div![C!["box"],
        div![C!["level", "is-mobile"],
            input![C!["input", "is-size-4"],
                el_ref(&time_block.name_input),
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(3),
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                    St::MaxWidth => percent(47),
                },
                attrs!{At::Value => time_block.name},
                input_ev(Ev::Input, move |name| Msg::TimeBlockNameChanged(client_id, time_block_id, name)),
                ev(Ev::Change, move |_| Msg::SaveTimeBlockName(client_id, time_block_id)),
            ],
            div![C!["is-flex"], style!{St::AlignItems => "center"},
                input![C!["input", "is-size-4", "has-text-right"], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Height => rem(3),
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                        St::MaxWidth => rem(6),
                    },
                    attrs!{
                        At::Value => if let Some(duration) = &time_block.duration_change {
                            duration.to_owned()
                        } else {
                            format!("{:.1}", time_block.duration.num_minutes() as f64 / 60.)
                        }
                    },
                    input_ev(Ev::Input, move |duration| Msg::TimeBlockDurationChanged(client_id, time_block_id, duration)),
                    ev(Ev::Change, move |_| Msg::SaveTimeBlockDuration(client_id, time_block_id)),
                ],
                div![
                    "h"
                ],
            ],
            view_delete_button(move || Msg::DeleteTimeBlock(client_id, time_block_id)),
        ],
        div![C!["level", "is-mobile"],
            view_status_buttons(client_id, time_block_id, time_block.status),
            IF!(time_block.invoice.is_none() => view_attach_invoice_button(client_id, time_block_id)),
        ],
        time_block.invoice.as_ref().map(move |invoice| view_invoice(client_id, time_block_id, invoice)),
    ]
}

fn view_status_buttons(client_id: ClientId, time_block_id: TimeBlockId, status: TimeBlockStatus) -> Node<Msg> {
    div![C!["buttons", "has-addons"], style!{St::MarginBottom => 0},
        button![
            C!["button", "is-rounded", IF!(matches!(status, TimeBlockStatus::NonBillable) => 
                ["is-selected", "is-primary"].as_ref()
            )], 
            style!{St::MarginBottom => 0},
            "Non-billable",
            ev(Ev::Click, move |_| Msg::SetTimeBlockStatus(client_id, time_block_id, TimeBlockStatus::NonBillable)),
        ],
        button![
            C!["button", IF!(matches!(status, TimeBlockStatus::Unpaid) => 
                ["is-selected", "is-primary"].as_ref()
            )], 
            style!{St::MarginBottom => 0},
            "Unpaid",
            ev(Ev::Click, move |_| Msg::SetTimeBlockStatus(client_id, time_block_id, TimeBlockStatus::Unpaid)),
        ],
        button![
            C!["button", "is-rounded", IF!(matches!(status, TimeBlockStatus::Paid) => 
                ["is-selected", "is-primary"].as_ref()
            )],
            style!{St::MarginBottom => 0},
            "Paid",
            ev(Ev::Click, move |_| Msg::SetTimeBlockStatus(client_id, time_block_id, TimeBlockStatus::Paid)),
        ],
    ]
}

fn view_attach_invoice_button(client_id: ClientId, time_block_id: TimeBlockId) -> Node<Msg> {
    button![C!["button", "is-primary", "is-rounded"],
        ev(Ev::Click, move |_| Msg::AttachInvoice(client_id, time_block_id)),
        span![C!["icon"],
            i![C!["fas", "fa-plus"]]
        ],
        span!["Attach Invoice"],
    ]
}

fn view_invoice(client_id: ClientId, time_block_id: TimeBlockId, invoice: &Invoice) -> Node<Msg> {
    div![C!["box", "has-text-link-light", "has-background-link"],
        div![C!["level", "is-mobile"],
            div!["Invoice ID"],
            input![C!["input", "has-text-link-light"], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                    St::MaxWidth => percent(55),
                },
                attrs!{At::Value => invoice.custom_id.as_ref().map(String::as_str).unwrap_or_default()},
                input_ev(Ev::Input, move |custom_id| Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id)),
                ev(Ev::Change, move |_| Msg::SaveInvoiceCustomId(client_id, time_block_id)),
            ],
            view_delete_button(move || Msg::DeleteInvoice(client_id, time_block_id)),
        ],
        div![C!["level", "is-mobile"],
            div!["URL"],
            input![C!["input", "has-text-link-light"], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                    St::MaxWidth => percent(67),
                },
                attrs!{At::Value => invoice.url.as_ref().map(String::as_str).unwrap_or_default()},
                input_ev(Ev::Input, move |url| Msg::InvoiceUrlChanged(client_id, time_block_id, url)),
                ev(Ev::Change, move |_| Msg::SaveInvoiceUrl(client_id, time_block_id)),
            ],
            invoice.url.as_ref().map(move |url| view_go_button(url)),
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

fn view_go_button(url: &str) -> Node<Msg> {
    a![C!["button", "is-primary", "is-rounded"],
        style!{
            St::Width => 0,
        },
        attrs!{
            At::Href => url,
            At::Target => "_blank",
        },
        span![C!["icon"],
            i![C!["fas", "fa-external-link-alt"]]
        ],
    ]
}

