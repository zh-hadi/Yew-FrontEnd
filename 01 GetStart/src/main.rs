use yew::prelude::*;


#[function_component(App)]
fn app()-> Html {
    html! {
        <h2>{"To day is day for yew learning ....."}</h2>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}