use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleListPaginationProps 
{}

#[function_component(ArticleListPagination)]
pub fn article_list_pagination(props: &ArticleListPaginationProps) -> Html
{
    html!(
        <nav class={classes!("flex", "justify-between")}>
                <a class={classes!(
                        "text-2xl",
                        "font-bold",
                        "text-orange-400",
                        "hover:text-blue-400",
                        "hover:border-b-4"
                )}> {"PREV"} </a>
                <a class={classes!(
                        "text-2xl",
                        "font-bold",
                        "text-orange-400",
                        "hover:text-blue-400",
                        "hover:border-b-4"
                )}> {"NEXT"} </a>
        </nav>
    )
}