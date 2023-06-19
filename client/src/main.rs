use yew::prelude::*;
use yew_router::prelude::*;
mod views;
mod components;
use crate::views::{home::Home,about::About,contact::Contact};

#[derive(Clone,Routable,PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/about")]
    About,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Contact => html! { <Contact /> },
        Route::About => html! { <About /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route>render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
