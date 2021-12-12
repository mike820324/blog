use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BNavBarProps {
    pub children: Children
}

#[function_component(BNavBar)]
pub fn b_navbar(props: &BNavBarProps) -> Html {
    html!{
        <div class={classes!("navbar", "navbar-expand-lg", "navbar-light", "bg-light")}>
            <div class="container-fluid">
            {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BNavBrandProps {
    pub children: Children,
}

#[function_component(BNavBrand)]
pub fn b_navbrand(props: &BNavBrandProps) -> Html {
    html!{
        <div class="navbar-brand">
            { props.children.clone() }
        </div>
    }
}