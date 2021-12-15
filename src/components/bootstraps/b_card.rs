use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BCardProps
{
    pub children: Children,
}

#[function_component(BCard)]
pub fn b_card(props: &BCardProps) -> Html
{
    html!{
        <div class={classes!("card")}> 
            { for props.children.clone() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BCardHeaderProps
{
    pub children: Children,
}

#[function_component(BCardHeader)]
pub fn b_card_header(props: &BCardHeaderProps) -> Html
{
    html!{
        <div class={classes!("card-header")}> 
            { for props.children.clone() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BCardBodyProps
{
    pub children: Children,
}

#[function_component(BCardBody)]
pub fn b_card_body(props: &BCardBodyProps) -> Html
{
    html!{
        <div class={classes!("card-body")}> 
            { for props.children.clone() }
        </div>
    }
}

