use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
        <div class="container">
        <Link<Route> to={Route::About}>
        <h1>{"Contact"}</h1>
        </Link<Route>>
        </div>
        </>
    }
}