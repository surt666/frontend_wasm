use crate::common::common::{accordion, modal, construct_select_list};
use seed::{prelude::*, *};
use std::collections::HashMap;
use crate::data_struct::*;
use strum::IntoEnumIterator;
use std::str::FromStr;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        accordion: HashMap::new(),
        datasets: vec![
            Dataset{
                name: "c4c".to_owned(),
                owner: "sla".to_owned(),
                ..Default::default()
            },
            Dataset{
                name: "aact".to_owned(),
                owner: "sla".to_owned(),
                ..Default::default()
            },
        ],
        add_dataset_modal: false,
        new_dataset: Dataset{..Default::default()},
    }
}

// ------ ------
//     Model
// ------ ------
#[derive(Default, Debug)]
pub struct Model {
    pub accordion: HashMap<String, String>,
    pub datasets: Vec<Dataset>,
    pub add_dataset_modal: bool,
    pub new_dataset: Dataset,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ShowAccordion(String),
    AddDataset,
    CancelDataset,
    CreateDataset,
    UpdateField(String, String),
}

fn show(id: String) -> Msg {
    Msg::ShowAccordion(id)
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
        },
        Msg::AddDataset => model.add_dataset_modal = true,
        Msg::CancelDataset =>  model.add_dataset_modal = false,
        Msg::CreateDataset => {
            log!("DATA {:#?}", model.new_dataset);
            let new_ds = model.new_dataset.clone();
            model.datasets.push(new_ds);
            model.new_dataset = Dataset {..Default::default()};
            model.add_dataset_modal = false;
        },
        Msg::UpdateField(k, v) => {
            //log!("Update {} {}", k.clone(), v.clone());
            let key = k.as_ref();
            match key {
                "name" => model.new_dataset.name = v,
                "owner" => model.new_dataset.owner = v,
                "company" => model.new_dataset.company = Some(v),
                "domain" => model.new_dataset.domain = Some(v),
                "cost_center" => model.new_dataset.cost_center = v,
                "client_account" => model.new_dataset.client_account = v,
                "sns_topic" => model.new_dataset.sns_topic = v,
                "managed" => model.new_dataset.managed = not(model.new_dataset.managed),
                "esh" => model.new_dataset.esh = not(model.new_dataset.esh),
                "financial" => model.new_dataset.financial = not(model.new_dataset.financial),
                "gxp" => model.new_dataset.gxp = not(model.new_dataset.gxp),
                "pii" => model.new_dataset.pii = not(model.new_dataset.pii),
                "region" => model.new_dataset.region = Region::from_str(&v).unwrap(),
                "confidentiality" => model.new_dataset.confidentiality = Confidentiality::from_str(&v).unwrap(),
                "criticality" => model.new_dataset.criticality = Criticality::from_str(&v).unwrap(),
                _ => ()
            };
        },
    }
}

fn generate_dataset_node(model: &Model) -> Vec<Node<Msg>> {
    let mut view_datasets: Vec<Node<Msg>> = vec![];
    let mut count: i32 = 0;
    for d in &model.datasets {
        count = count + 1;
        let id = count.to_string(); 
        let content: Node<Msg> = div![
            span![C!["block m-2"],label!["Name: "], label![&d.name]],
            span![C!["block m-2"],label!["Owner: "], label![&d.owner]],
            span![C!["block m-2"],
                label!["Managed: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => (&d.managed == &true).as_at_value()}],
                label!["Financial: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => (&d.financial == &true).as_at_value()}],
                label!["ESH: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => (&d.esh == &true).as_at_value()}],
                label!["GxP: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => (&d.gxp == &true).as_at_value()}],
                label!["PII: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => (&d.pii == &true).as_at_value()}],
            ],
        ];
        view_datasets.push(accordion(id, d.name.clone(), content, &model.accordion, &show));
    };
    view_datasets
}

fn generate_add_dataset_form(model: &Model) -> Node<Msg> {
    let region_options: Vec<String> = Region::iter().map(|x| x.to_string()).collect();
    let confidentiality_options: Vec<String> = Confidentiality::iter().map(|x| x.to_string()).collect();
    let criticality_options: Vec<String> = Criticality::iter().map(|x| x.to_string()).collect();
    div![
        h1![C!["text-xl"], "New dataset"],
        span![C!["block m-2"],
                label!["Managed: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => model.new_dataset.managed.as_at_value()}, input_ev(Ev::Change, move |v| Msg::UpdateField("managed".to_owned(), v))],
                label!["Financial: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => model.new_dataset.financial.as_at_value()}, input_ev(Ev::Change, move |v| Msg::UpdateField("financial".to_owned(), v))],
                label!["ESH: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => model.new_dataset.esh.as_at_value()}, input_ev(Ev::Change, move |v| Msg::UpdateField("esh".to_owned(), v))],
                label!["GxP: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => model.new_dataset.gxp.as_at_value()}, input_ev(Ev::Change, move |v| Msg::UpdateField("gxp".to_owned(), v))],
                label!["PII: "], input![C!["m-1"],attrs!{At::Type => "checkbox", At::Checked => model.new_dataset.pii.as_at_value()}], input_ev(Ev::Change, move |v| Msg::UpdateField("pii".to_owned(), v))],
        span![C!["block m-2"],label!["Dataset name *:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.name}, input_ev(Ev::Input, move |v| Msg::UpdateField("name".to_owned(), v))]],
        span![C!["block m-2"],label!["Confidentiality level *:"], select![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.confidentiality}, construct_select_list(&confidentiality_options), input_ev(Ev::Input, move |v| Msg::UpdateField("confidentiality".to_owned(), v))]],
        span![C!["block m-2"],label!["Criticality *:"], select![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.criticality}, construct_select_list(&criticality_options), input_ev(Ev::Input, move |v| Msg::UpdateField("criticality".to_owned(), v))]],
        span![C!["block m-2"],label!["Owner *:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.owner}, input_ev(Ev::Input, move |v| Msg::UpdateField("owner".to_owned(), v))]],
        span![C!["block m-2"],label!["Company:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.company.clone().unwrap_or("".to_owned())}, input_ev(Ev::Input, move |v| Msg::UpdateField("company".to_owned(), v))]],
        span![C!["block m-2"],label!["Domain:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.domain.clone().unwrap_or("".to_owned())}, input_ev(Ev::Input, move |v| Msg::UpdateField("domain".to_owned(), v))]],
        span![C!["block m-2"],label!["Cost Center *:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.cost_center}, input_ev(Ev::Input, move |v| Msg::UpdateField("cost_center".to_owned(), v))]],
        span![C!["block m-2"],label!["Region *:"], select![C!["m-2 border-2 rounded-md"], attrs!{At::Value => model.new_dataset.region}, construct_select_list(&region_options), input_ev(Ev::Input, move |v| Msg::UpdateField("region".to_owned(), v))]],
        span![C!["block m-2"],label!["Client account *:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.client_account}, input_ev(Ev::Input, move |v| Msg::UpdateField("client_account".to_owned(), v))]],
        span![C!["block m-2"],label!["SNS Topic ARN *:"], input![C!["m-2 border-2"], attrs!{At::Value => model.new_dataset.sns_topic}, input_ev(Ev::Input, move |v| Msg::UpdateField("sns_topic".to_owned(), v))]],
        span![C!["block m-2"],
            button![C!["bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full m-4"], ev(Ev::Click, |_| Msg::CancelDataset), "Cancel"], 
            button![C!["bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"], ev(Ev::Click, |_| Msg::CreateDataset), "Submit"]],
    ]
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let ds = generate_dataset_node(model);
    let ds_modal_content = generate_add_dataset_form(model);
    div![C!["col-span-11"],
        IF!(model.add_dataset_modal => modal(div![ds_modal_content])),
        div![C!["m-2 grid row-span-1 center"], div![C!["text-xl m-3 col-start-1"], "My Datasets"], button![C!["bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full col-start-11"], ev(Ev::Click, |_| Msg::AddDataset), "+"]],
        div![
            ds
        ] 
    ]
}
