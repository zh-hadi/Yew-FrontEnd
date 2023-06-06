use yew::prelude::*;
use gloo::console::log;

#[function_component(App)]
pub fn app()-> Html {
    log!("hello world");
    html! {
        <h2>{"To day is day for yew learning .....hi component is working now."}</h2>
    }
}

