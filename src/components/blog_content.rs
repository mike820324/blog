use yew::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};
use gloo_utils::document;

#[derive(Properties, PartialEq)]
pub struct BlogContentProps
{
    pub content: String
}

#[function_component(BlogContent)]
pub fn blog_content(props: &BlogContentProps) -> Html
{
    use_effect(move || {
        crate::binding::highlightjs::highlight_all();

        || {}
    });

    let html = markdown_to_html(&props.content, &ComrakOptions::default());

    let div = document().create_element("div").unwrap();
    div.set_class_name("blog-content");
    div.set_inner_html(&html);

    Html::VRef(div.into())
}