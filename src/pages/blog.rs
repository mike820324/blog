use yew::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};
use reqwasm;
use gloo_console;

#[derive(Clone)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
}

async fn fetch_blog(blog_id: String) -> String {
    let url = format!("/content/{}.md", blog_id);
    gloo_console::log!(format!("{}", url));

    reqwasm::http::Request::get(&url)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap()
}

pub enum Msg {
    SetMarkdownFetchState(FetchState<String>),
    GetMarkdown(String),
    GetError,
}

pub struct Blog {
    pub blog_content: FetchState<String>,
}

#[derive(Properties, PartialEq)]
pub struct BlogProps {
    pub id: String,
}

impl Component for Blog {
	type Message = Msg;
	type Properties = BlogProps;

	fn create(_: &Context<Self>) -> Self {
        Blog {
            blog_content: FetchState::NotFetching,
        }
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetMarkdownFetchState(fetch_state) => {
                self.blog_content = fetch_state;
                true
            },
            Msg::GetMarkdown(blog_id) => {
                ctx.link()
                    .send_future(async {
                        Msg::SetMarkdownFetchState(
                            FetchState::Success(fetch_blog(blog_id).await)
                        )
                });
                false
            },
            Msg::GetError => {
                false
            }
        }
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
        match self.blog_content.clone() {
            FetchState::NotFetching => {
                ctx.link().send_message(Msg::GetMarkdown(ctx.props().id.clone()));
                html! {
                    <h1> { "Loading" } </h1>
                }
            },
            FetchState::Fetching => {
                html! {
                    <h1> { "Loading" } </h1>
                }
            },
            FetchState::Success(content) => {
                let html = markdown_to_html(&content, &ComrakOptions::default());
                let div = gloo_utils::document().create_element("div").unwrap();
                div.set_class_name("blog-content");
                div.set_inner_html(&html);
                Html::VRef(div.into())
            },
        }

    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        crate::binding::highlightjs::highlight_all();
    }
}