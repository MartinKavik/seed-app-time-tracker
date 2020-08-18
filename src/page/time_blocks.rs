use seed::{prelude::*, *};

use chrono::{prelude::*, Duration};
use ulid::Ulid;

use cynic::QueryFragment;

use std::collections::BTreeMap;
use std::convert::identity;
use std::ops::Add;

use crate::graphql;

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
            invoice: time_block.invoice.map(invoice_mapper),
        }
    );

    let compute_tracked_time = |projects: Vec<query_mod::Project>| {
        projects
            .into_iter()
            .flat_map(|project| project.time_entries)
            .filter_map(|time_entry| {
                if let Some(stopped) = time_entry.stopped {
                    
                    let started: DateTime<Local> = 
                        time_entry.started.0.parse().expect("parse time_entry started");
                    
                    let stopped: DateTime<Local> = 
                        stopped.0.parse().expect("parse time_entry stopped");
                    
                    Some(stopped - started)
                } else {
                    None
                }
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
    time_blocks: BTreeMap<Ulid, TimeBlock>,
    tracked: Duration,
}

#[derive(Debug)]
struct TimeBlock {
    name: String,
    status: TimeBlockStatus,
    duration: Duration,
    invoice: Option<Invoice>,
}

#[derive(Debug)]
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

        // ------ TimeBlock ------
        
        Msg::AddTimeBlock(client_id) => {},
        Msg::DeleteTimeBlock(client_id, time_block_id) => {},
        Msg::SetTimeBlockStatus(client_id, time_block_id, time_block_status) => {},

        Msg::TimeBlockDurationChanged(client_id, time_block_id, duration) => {},
        Msg::SaveTimeBlockDuration(client_id, time_block_id) => {},

        // ------ Invoice ------

        Msg::AttachInvoice(client_id, time_block_id) => {},
        Msg::DeleteInvoice(client_id, time_block_id) => {},

        Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id) => {},
        Msg::SaveInvoiceCustomId(client_id, time_block_id) => {},

        Msg::InvoiceUrlChanged(client_id, time_block_id, url) => {},
        Msg::SaveInvoiceUrl(client_id, time_block_id) => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["TimeBlocks view"]
}
