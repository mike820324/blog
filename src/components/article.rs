use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleProps 
{
    pub publish_time: String,
    pub labels: String,
    pub title: String,
    pub description: String,
    pub link: String,
}

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html
{
    html!(
        <article class={classes!("pb-10")}>
                <div class={classes!("flex", "space-x-2")}>
                        <span class={classes!("font-sm")}>
                            {props.publish_time.clone()}
                        </span>
                        <a class={classes!("font-sm", "text-orange-400", "hover:text-blue-400")}>
                            {props.labels.clone()}
                        </a>
                </div>
                <h2 class={classes!("text-2xl", "py-4")}> {props.title.clone()} </h2>
                <p class={classes!("py-4")}> {props.description.clone()} </p>
                <a class={classes!("text-blue-400", "hover:border-b-4")} href={props.link.clone()}> {"Read"} </a>
        </article>
    )
}