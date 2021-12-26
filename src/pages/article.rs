use yew::prelude::*;
use reqwasm;
use crate::components::article_content::ArticleContent;

async fn fetch_blog(blog_id: String) -> String {
    let url = format!("/content/{}.md", blog_id);

    reqwasm::http::Request::get(&url)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap()
}

#[derive(Properties, PartialEq)]
pub struct BlogProps {
    pub id: String,
}

#[function_component(Blog)]
pub fn blog(props: &BlogProps) -> Html {
    let content = use_state(|| String::new());

    {
        let article_id = props.id.clone();
        let content = content.clone();
        use_effect_with_deps(move |_| {
            let content = content.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let blog_content = fetch_blog(article_id).await;
                content.set(blog_content);
            });

            || ()
        }, {});
    }

    html! {
        <ArticleContent content={(*content).clone()}/>
    }
}
