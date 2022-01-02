use yew::prelude::*;
use pulldown_cmark::{html, Parser};
use gloo_utils::document;

#[derive(Properties, PartialEq)]
pub struct ArticleContentProps
{
    pub content: String
}

#[function_component(ArticleContent)]
pub fn article_content(props: &ArticleContentProps) -> Html
{
    use_effect(move || {
        crate::binding::highlightjs::highlight_all();

        || ()
    });

    let parser = Parser::new(&props.content);
    let mut html_content: String = String::new();
    html::push_html(&mut html_content, parser);


    let div = document().create_element("div").unwrap();
    div.set_class_name("prose prose-stone");
    div.set_inner_html(&html_content);

    Html::VRef(div.into())
}