#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod common;
mod page;
mod data_struct;

const USERS: &str = "users";
const DATASETS: &str = "datasets";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));
    // .perform_cmd(async {
    //     Msg::AuthConfigFetched(
    //         async { fetch("/auth_config.json").await?.check_status()?.json().await }.await
    //     )
    // });

    Model {
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
        menu_visible: false,
        auth_config: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
    menu_visible: bool,
    auth_config: Option<AuthConfig>,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

#[derive(Deserialize)]
struct User {
    email: String,
    name: Option<String>,
}

// ------ AuthConfig ------

#[derive(Deserialize)]
struct AuthConfig {
    domain: String,
    client_id: String,
}

pub enum Page {
    Home,
    Users(page::users::Model),
    Datasets(page::datasets::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [USERS] => Self::Users(page::users::init(url, &mut orders.proxy(Msg::UsersPage))),
            [DATASETS] => Self::Datasets(page::datasets::init(
                url,
                &mut orders.proxy(Msg::DatasetsPage),
            )),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn users(self) -> Url {
        self.base_url().add_path_part(USERS)
    }
    fn datasets(self) -> Url {
        self.base_url().add_path_part(DATASETS)
    }
}

enum Msg {
    UsersPage(page::users::Msg),
    DatasetsPage(page::datasets::Msg),
    UrlChanged(subs::UrlChanged),
    // AuthConfigFetched(fetch::Result<AuthConfig>),
    // AuthInitialized(Result<String, String>),
    ToggleMenu,
    HideMenu,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::UsersPage(msg) => {
            if let Page::Users(model) = &mut model.page {
                page::users::update(msg, model, &mut orders.proxy(Msg::UsersPage))
            }
        }
        Msg::DatasetsPage(msg) => {
            if let Page::Datasets(model) = &mut model.page {
                page::datasets::update(msg, model, &mut orders.proxy(Msg::DatasetsPage))
            }
        }
        Msg::ToggleMenu => model.menu_visible = not(model.menu_visible),
        Msg::HideMenu => {
            if model.menu_visible {
                model.menu_visible = false;
            } else {
                orders.skip();
            }
        }
        // Msg::AuthConfigFetched(Ok(auth_config)) => {
        //     let domain = auth_config.domain.clone();
        //     let client_id = auth_config.client_id.clone();

        //     orders.perform_cmd({ Msg::AuthInitialized(
        //         init_auth(client_id)
        //     )});
        //     model.auth_config = Some(auth_config);
        // },
        // Msg::AuthConfigFetched(Err(fetch_error)) => error!("AuthConfig fetch failed!", fetch_error),
        // Msg::AuthInitialized(Ok(user)) => {
        //     if not(user.is_undefined()) {
        //         match serde_wasm_bindgen::from_value(user) {
        //             Ok(user) => model.ctx.user = Some(user),
        //             Err(error) => error!("User deserialization failed!", error),
        //         }
        //     }

        //     let search = model.base_url.search_mut();
        //     if search.remove("code").is_some() && search.remove("state").is_some() {
        //         model.base_url.go_and_replace();
        //     }
        // }
        // Msg::AuthInitialized(Err(error)) => {
        //     error!("Auth initialization failed!", error);
        // }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let u = Page::Users(page::users::Model {..Default::default()});
    let d = Page::Datasets(page::datasets::Model {..Default::default()});
    let h = Page::Home;
    let h_url = Urls::new(&model.base_url).home();
    let u_url = Urls::new(&model.base_url).users();
    let d_url = Urls::new(&model.base_url).datasets();
    let pages = vec![(&h, &h_url, "Home"), (&u, &u_url, "Users"), (&d, &d_url, "Datasets")];
    div![
        C!["grid grid-cols-12 xl:w-full"],
        div![
            C!["col-start-1 xl:col-end-13 bg-blue-500 text-white"],
            "Administration"
        ],
	common::common::navbar(model.menu_visible, &model.base_url, &model.page, pages),
        view_content(&model.page, &model.base_url),
    ]
}

fn view_content(page: &Page, base_url: &Url) -> Node<Msg> {
    match page {
        Page::Home => page::home::view(base_url),
        Page::Users(model) => page::users::view(model).map_msg(Msg::UsersPage),
        Page::Datasets(model) => page::datasets::view(model).map_msg(Msg::DatasetsPage),
        Page::NotFound => page::not_found::view(),
    }
}

fn init_auth(user: String) -> Result<String, String> {
    Ok("zqsl".to_string())
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
    // App::builder(update, view)
    //	.after_mount(after_mount)
    //     .build_and_start();
}
