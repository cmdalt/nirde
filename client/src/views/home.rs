use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::components::form::Form;

#[function_component(Home)]
pub fn home() -> Html {
    
    html! {
        <>
        <div class="container">
        <div class="row">
        
        <Link<Route> to={Route::Contact}>
        <h1>{"Contact"}</h1>
        </Link<Route>>

        <Form />

        </div>
        </div>
        </>
    }
}