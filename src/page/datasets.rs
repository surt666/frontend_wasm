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
    Show(String),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
	Msg::Show(id) => {
	    match model.accordion.get(&"one".to_string()) {
		Some(x) =>  {
		    if x.to_string() == "hidden".to_string() {
			model.accordion.insert(id, "".to_owned());
		    } else {
			model.accordion.insert(id, "hidden".to_owned());
		    }
		},
		None => {
		    let x = model.accordion.insert(id, "".to_owned());
		}
	    }
	}
    }
}

// ------ ------
//     View
// ------ ------


fn accordion(id: &str, title: &str, content: Node<Msg>, model: &Model) -> Node<Msg> {
    let i = id.to_string();
    div![C!["rounded-sm"],
         div![C!["border border-b-0 bg-gray-100 px-10 py-6"],
	      button![C!["underline text-blue-500 hover:text-blue-700 focus:outline-none"], ev(Ev::Click, |_| Msg::Show(i)), title]],
	 
         div![C!["border border-b-0 px-10 py-6"], attrs!{At::Class => model.accordion.get(&id.to_string()).unwrap_or(&"hidden".to_string())},
	      content]]
}

pub fn view(model: &Model) -> Node<Msg> {
    let a1 = "one".to_string();
    let content = p!["Anim pariatur cliche reprehenderit, enim eiusmod high life accusamus terry richardson ad squid. 3 wolf moon officia aute, non cupidatat skateboard dolor brunch. Food truck quinoa nesciunt laborum eiusmod. Brunch 3 wolf moon tempor, sunt aliqua put a bird on it squid single-origin coffee nulla assumenda shoreditch et. Nihil anim keffiyeh helvetica, craft beer labore wes anderson cred nesciunt sapiente ea proident. Ad vegan excepteur butcher vice lomo. Leggings occaecat craft beer farm-to-table, raw denim aesthetic synth nesciunt you probably haven't heard of them accusamus labore sustainable VHS."];
    div![C!["w-full my-4"],
	 accordion(&a1, "Collapsible Group Item #1", content, model)
    ]
}
