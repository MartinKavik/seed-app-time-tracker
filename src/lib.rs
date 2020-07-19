#![allow(clippy::wildcard_imports)]
// @TODO: Remove.
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

mod page;

const CLIENTS_AND_PROJECTS: &str = "clients_and_projects";
const TIME_TRACKER: &str = "time_tracker";
const TIME_BLOCKS: &str = "time_blocks";
const SETTINGS: &str = "settings";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::Home,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

struct User {
    username: String,
    email: String,
}

enum Page {
    Home,
    ClientsAndProjects(page::clients_and_projects::Model),
    TimeTracker(page::time_tracker::Model),
    TimeBlocks(page::time_blocks::Model),
    Settings(page::settings::Model),
    NotFound,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn clients_and_projects(self) -> Url {
        self.base_url().add_path_part(CLIENTS_AND_PROJECTS)
    }
    fn time_tracker(self) -> Url {
        self.base_url().add_path_part(CLIENTS_AND_PROJECTS)
    }
    fn time_blocks(self) -> Url {
        self.base_url().add_path_part(TIME_BLOCKS)
    }
    fn settings(self) -> Url {
        self.base_url().add_path_part(SETTINGS)
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(url) => {},
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div!["Root view"]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
