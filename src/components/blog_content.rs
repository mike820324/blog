use yew::prelude::*;
use pulldown_cmark::{html, Parser};
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

    let parser = Parser::new(&props.content);
    let mut html_content: String = String::new();
    html::push_html(&mut html_content, parser);


    let div = document().create_element("div").unwrap();
    div.set_class_name("blog-content");
    div.set_inner_html(&html_content);

    Html::VRef(div.into())
}