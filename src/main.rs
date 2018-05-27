#![recursion_limit="128"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate stdweb;
extern crate web_logger;
#[macro_use]
extern crate yew;

use stdweb::web::{IParentNode, document};

use yew::prelude::*;

use std::collections::HashMap;

struct Context;

fn main() {
    web_logger::init();

    trace!("Initializing yew...");
    yew::initialize();

    trace!("Creating a context...");
    let context = Context;
    let app: App<_, DestampComponent> = App::new(context);

    let elt = document().query_selector("#destamp").unwrap().unwrap();
    let env = app.mount(elt);

    yew::run_loop();
}

pub enum DestampMsg {
    SerTest,
    AddElts,
}

pub struct DestampComponent {
    num_elts: usize,
}

#[derive(Debug, Serialize)]
pub struct BigSubData {
    pub a: u32,
    pub b: String,
    pub c: String,
    pub d: bool,
}
js_serializable!( BigSubData );

type BigData = HashMap<String, BigSubData>;

fn loadit(jobs_map: &BigData) {
    js!{ @(no_return)
        window._logobble_jobs = @{jobs_map};
    }
}
fn loaditjson(jobs_map: &BigData) {
    let jobs_map_json = serde_json::to_string(&jobs_map).unwrap();
    js!{ @(no_return)
        window._logobble_jobs = JSON.parse(@{jobs_map_json});
    }
}

impl<CTX> Component<CTX> for DestampComponent where CTX: 'static {
    type Message = DestampMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        DestampComponent { num_elts: 0 }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            DestampMsg::SerTest => {
                let mut bigdata = HashMap::new();
                for i in 0..5000u32 {
                //for i in 0..15000u32 {
                    bigdata.insert(i.to_string(), BigSubData {
                        a: i,
                        b: "AAA=UUUUUUUUUUUUUUUU BBB=XXXXXXXXXXXXX CCC=YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY DDD=ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ".to_owned(),
                        c: "shorterstring::::".to_owned(),
                        d: i == 25,
                    });
                }

                js!{ console.time("stdweb ser"); }
                loadit(&bigdata);
                js!{ console.timeEnd("stdweb ser"); }
                js!{ console.time("json ser"); }
                loaditjson(&bigdata);
                js!{ console.timeEnd("json ser"); }
                false
            },
            DestampMsg::AddElts => {
                self.num_elts = 14000;
                true
            },
        }
    }
}

impl<CTX> Renderable<CTX, DestampComponent> for DestampComponent where CTX: 'static {
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                { for (0..self.num_elts).map(|_| html! { <div></div> }) }
                <input type="button", value="Test serialization", onclick=|_| DestampMsg::SerTest,>{ "Hello" }</input>
                <input type="button", value="Add elts", onclick=|_| DestampMsg::AddElts,>{ "Add elts" }</input>
            </div>
        }
    }
}
