use seed::{prelude::*, *};
use std::collections::HashMap;
use crate::{Page};

pub fn accordion<T>(
    id: String, title: String, content: Node<T>, accordion: &HashMap<String, String>,
    show: &'static dyn Fn(String) -> T,
) -> Node<T> {
    let id2 = id.clone();
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
            attrs! {At::Class => accordion.get(&id2).unwrap_or(&"hidden".to_string())},
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

pub fn modal<T>(content: Node<T>) -> Node<T> {
    div![C!["fixed z-10 inset-0 overflow-y-auto"], 
	     div![C!["flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0"],
		  div![C!["fixed inset-0 transition-opacity"],
		       div![C!["absolute inset-0 bg-gray-500 opacity-75"]],
		       span![C!["hidden sm:inline-block sm:align-middle sm:h-screen", "&#8203;"]],
		       div![C!["inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full"],
			    content]]]]
}

pub fn search_box<T>(list: Vec<String>, choose: &'static dyn Fn(String) -> T) -> Node<T> {
    div![C!["grid cols-5"],
        input![C!["border p-1 m-3"], attrs!{"list" => "search_list", "placeholder" => "Search"}, input_ev(Ev::Change, move |term| choose(term))],
        datalist![attrs!{At::Id => "search_list"},
            construct_select_list(&list),
        ]
    ]
} 
