use yew::prelude::*;
use yew::events::MouseEvent;

#[derive(Properties, PartialEq)]
pub struct BButtonProps
{
    pub children: Children,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(BDefaultButton)]
pub fn b_default_button(props: &BButtonProps) -> Html
{
    html!{
        <button class={classes!("btn", "btn-default")}> { for props.children.clone() } </button>
    }
}


#[function_component(BPrimaryButton)]
pub fn b_primary_button(props: &BButtonProps) -> Html
{
    html!{
        <button class={classes!("btn", "btn-primary")} onclick={props.onclick.clone()}> { for props.children.clone() } </button>
    }
}