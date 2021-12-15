use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BListGroupProps {
    pub children: Children,
}

#[function_component(BListGroup)]
pub fn b_list_group(props: &BListGroupProps) -> Html {
    html! {
        <ul class={classes!("list-group")}>
            {props.children.clone()}
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct BListGroupItemProps {
    pub children: Children,
}

#[function_component(BListGroupItem)]
pub fn b_list_group_item(props: &BListGroupItemProps) -> Html {
    html! {
        <li class={classes!("list-group-item")}>
            {props.children.clone()}
        </li>
    }
}