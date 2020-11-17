use seed::{prelude::*, *};
use std::collections::HashMap;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
       accordion: HashMap::new()
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    accordion: HashMap<String, String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ShowAccordion(String),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
	Msg::ShowAccordion(id) => {
	    if let Some(x) = model.accordion.get(&id) {
		if x == "hidden" {
		    model.accordion.insert(id, "".to_owned());
		} else {
		    model.accordion.insert(id, "hidden".to_owned());
		}
	    } else {
		model.accordion.insert(id, "".to_owned());
	    }
	}
    }
}

// ------ ------
//     View
// ------ ------


fn accordion(id: &'static str, title: &str, content: Node<Msg>, model: &Model) -> Node<Msg> {
    div![C!["rounded-sm"],
         div![C!["border border-b-0 bg-gray-100 px-10 py-6"],
	      button![C!["underline text-blue-500 hover:text-blue-700 focus:outline-none"], ev(Ev::Click, move |_| Msg::ShowAccordion(id.to_owned())), title]],
	 
         div![C!["border border-b-0 px-10 py-6"],
	      attrs!{At::Class => model.accordion.get(id).unwrap_or(&"hidden".to_string())},
	      content]]
}

pub fn view(model: &Model) -> Node<Msg> {
    let content = p!["Anim pariatur cliche reprehenderit, enim eiusmod high life accusamus terry richardson ad squid. 3 wolf moon officia aute, non cupidatat skateboard dolor brunch. Food truck quinoa nesciunt laborum eiusmod. Brunch 3 wolf moon tempor, sunt aliqua put a bird on it squid single-origin coffee nulla assumenda shoreditch et. Nihil anim keffiyeh helvetica, craft beer labore wes anderson cred nesciunt sapiente ea proident. Ad vegan excepteur butcher vice lomo. Leggings occaecat craft beer farm-to-table, raw denim aesthetic synth nesciunt you probably haven't heard of them accusamus labore sustainable VHS."];
    div![C!["w-full my-4"],
	 accordion("one", "Collapsible Group Item #1", content, model)
    ]
}
