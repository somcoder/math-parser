#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::{services::ConsoleService, prelude::*};

use parser_util::*;
use std::collections::HashMap;

enum Msg {
    Parse(String),
    Error,
}

struct MainComponent {
    link: ComponentLink<Self>,
    values: Vec<Value>,
    converter: Converter<'static>
}

impl MainComponent {
    fn parse(&mut self, input: String) {
        ConsoleService::log("Parse value!");
        let parser = ExpressionParser::new();
        let value: Result<Value, _> = parser.parse(&self.converter, input.as_str());
        match value {
            Ok(v) => {
                self.values.push(v);
            },
            Err(err) => {
                ConsoleService::log("Not supported input!");
                ConsoleService::error(err.to_string().as_str());
            }
        }
    }

    fn view_value(&self, value: String) -> Html {
        html! {
            <div>{ value }</div>
        }
    }
}

impl Component for MainComponent {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::log("Create is called!");

        let mut conversions = HashMap::new();
        conversions.insert((Unit::new("day"), Unit::new("hour")), 24.);
        conversions.insert((Unit::new("hour"), Unit::new("minute")), 60.);
        conversions.insert((Unit::new("minute"), Unit::new("second")), 60.);
        conversions.insert((Unit::new("second"), Unit::new("millisecond")), 60.);
        conversions.insert((Unit::new("km"), Unit::new("m")), 1000.);
        conversions.insert((Unit::new("m"), Unit::new("cm")), 100.);
        conversions.insert((Unit::new("cm"), Unit::new("mm")), 10.);

        let converter = Converter::new(conversions);

        Self { link, values: vec![], converter }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Msg::Parse(val) = msg {
            // self.values.push(val);
            self.parse(val);

            return true;
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {

        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Math Parser" }</h1>

                <h4>{ "Help" }</h4>
                <ul>
                    <li>{ "10 plus 40" }</li>
                    <li>{ "10 * 4" }</li>
                    <li>{ "10 mod 4" }</li>
                    <li>{ "45 - 20%" }</li>
                    <li>{ "1hour + 60minute" }</li>
                    <li>{ "1km + 6000m" }</li>
                </ul>

                <div>{ for self.values.iter().map(|v| self.view_value(v.to_string())) }</div>

                <input onchange=self.link.callback(|cd: ChangeData| {
                    match cd {
                        ChangeData::Value(val) => Msg::Parse(val),
                        _ => Msg::Error
                    }
                }) />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainComponent>::new().mount_to_body();

    ConsoleService::log("App is running!");
}