use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BAccordionProps {
    pub children: Children
}

#[function_component(BAccordion)]
pub fn b_accordion(props: &BAccordionProps) -> Html {
    html!(
        <div class="accordion">
            { props.children.clone() }
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct BAccordionItemProps {
    pub children: Children
}

#[function_component(BAccordionItem)]
pub fn b_accordion_item(props: &BAccordionItemProps) -> Html {
    html!(
        <div class="accordion-item">
            { props.children.clone() }
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct BAccordionHeaderProps {
    pub children: Children
}

#[function_component(BAccordionHeader)]
pub fn b_accordion_header(props: &BAccordionHeaderProps) -> Html {
    html!(
        <div class="accordion-header">
            { for props.children.clone() }
        </div>
    )
}


#[derive(Properties, PartialEq)]
pub struct BAccordionBodyProps {
    pub children: Children
}

#[function_component(BAccordionBody)]
pub fn b_accordion_body(props: &BAccordionBodyProps) -> Html {
    html!(
        <div class="accordion-body">
            { for props.children.clone() }
        </div>
    )
}
