use chrono::prelude::*;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
	users: vec!["zqsl".to_string(), "dzfn".to_string()],
        dataset: "c4c".to_string(),
	dataset_users: vec![],
	user: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    users: Vec<String>,
    dataset: String,
    dataset_users: Vec<String>,
    user: Option<String>
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    MoveToDs,
    MoveFromDs,
    UserChosen(String),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::MoveToDs => {
	    let u = model.user.clone().unwrap_or("".to_string());
	    model.users.retain(|x| x.ne(&u));
	    model.dataset_users.push(u);
	}
        Msg::MoveFromDs => {
	    let u = model.user.clone().unwrap_or("".to_string());
	    model.dataset_users.retain(|x| x.ne(&u));
	    model.users.push(u);
	}
	Msg::UserChosen(user) => {
	    log!("USER {:#?}", user);
	    model.user = Some(user);
	}
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    section![C!["col-start-2 col-span-11 grid grid-cols-11 auto-rows-min"],
	 div![C!["col-span-11 h-16 p-6 text-2xl"], h2![format!("Users for dataset {}", model.dataset)]],
	 view_users(&model.users),
	 view_actions(),
	 view_users(&model.dataset_users),
    ]
}

fn view_users(users: &Vec<String>) -> Node<Msg> {
    div![C!["col-span-5 p-10 self-start"],
	 select![C!["w-full border-2 rounded-md"], attrs!{At::Size => 30}, construct_select_list(users), input_ev(Ev::Input, Msg::UserChosen)]
    ]
}

fn view_actions() -> Node<Msg> {
    div![C!["col-span-1 self-center justify-self-center"],
	 div![button![C!["border-2 m-2 p-1"], ev(Ev::Click, |_| Msg::MoveToDs), "->"]],
	 div![button![C!["border-2 m-2 p-1"], ev(Ev::Click, |_| Msg::MoveFromDs), "<-"]],
    ]
}

fn construct_select_list(list: &Vec<String>) -> Vec<Node<Msg>> {
    let l: Vec<Node<Msg>> = list
        .iter()
        .map(|x| option![x])
	.collect();
    l
}
