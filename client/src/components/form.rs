use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Form)]
pub fn form() -> Html {
    web_sys::console::log_1(&"merhaga".into());

    let name = use_state(|| String::new());
    
    let oninput = Callback::from({
        let name = name.clone();
        move |input: InputEvent| {
            let target: HtmlInputElement = input.target().unwrap_throw().dyn_into().unwrap_throw();
            web_sys::console::log_1(&target.value().into());
            name.set(target.value());
        }
    });
    
    html! {
        <>
        <div class="container">
        <div class="row">
        <input oninput={oninput}  />
        <h1>{&*name}</h1>

        </div>
        </div>
        </>
    }
}