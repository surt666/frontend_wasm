use seed::{prelude::*, *};
use std::collections::HashMap;
use crate::{Page};

pub fn accordion<T>(
    id: &'static str, title: &str, content: Node<T>, accordion: &HashMap<String, String>,
    show: &'static dyn Fn(String) -> T,
) -> Node<T> {
    div![
        C!["rounded-sm"],
        div![
            C!["border border-b-0 bg-gray-100 px-10 py-6"],
            button![
                C!["underline text-blue-500 hover:text-blue-700 focus:outline-none"],
                ev(Ev::Click, move |_| show(id.to_owned())),
                title
            ]
        ],
        div![
            C!["border border-b-0 px-10 py-6"],
            attrs! {At::Class => accordion.get(id).unwrap_or(&"hidden".to_string())},
            content]
    ]
}

pub fn construct_select_list<T>(list: &Vec<String>) -> Vec<Node<T>> {
    let l: Vec<Node<T>> = list.iter().map(|x| option![x]).collect();
    l
}

pub fn navbar<T>(menu_visible: bool, base_url: &Url, page: &Page, pages: Vec<(&Page, &Url, &str)>) -> Node<T> {
    nav![
        C!["bg-gray-200 col-start-1 h-screen border-2 border-gray-600"],
	pages.
	    iter().
	    map(|x| {
		let p = x.0;
		div![
		    C!["px-8 py-2"],
		    a![
			C![
			    "navbar-item",
			    IF!(matches!(page, p) => "is-active"),
			],
			attrs! {At::Href => x.1},
			x.2,
		    ]
		]}).
		collect::<Vec<Node<T>>>()
    ]
}
