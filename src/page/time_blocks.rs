use seed::{prelude::*, *};

use chrono::{prelude::*, Duration};
use ulid::Ulid;

use std::collections::BTreeMap;

type ClientId = Ulid;
type TimeBlockId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
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
    time_blocks: BTreeMap<Ulid, TimeBlock>,
    tracked: Duration,
}

struct TimeBlock {
    name: String,
    status: TimeBlockStatus,
    duration: Duration,
    invoice: Option<Invoice>,
}

pub enum TimeBlockStatus {
    NonBillable,
    Unpaid,
    Paid,
}

struct Invoice {
    custom_id: Option<String>,
    url: Option<String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ClientsFetched(fetch::Result<BTreeMap<ClientId, Client>>),
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
        Msg::ClientsFetched(Ok(clients)) => {},
        Msg::ClientsFetched(Err(fetch_error)) => {},

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
