use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <header style = "    
        position: fixed;
        right: 0;
        top: 0;
        z-index: 1000;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 33px;">

        <a style="    
        font-size: 30px;
        font-weight: 700;
        color: white;" href="#">
        {"COC-STATS"}
        </a>
        
        <ul>

        <a href="#">{"Home"}</a>
        <a href="#">{"About"}</a>
        <a href="#">{"Clans"}</a>
        <a href="#">{"Troops"}</a>
        <a href="#">{"Contact"}</a>

        </ul>

        <div class="bx bx-menu" id="menu-icon">{"#"}</div>

        </header>
    }
}