use yew::prelude::*;
use crate::components::article_list_item::ArticleListItem;
use crate::components::article_list_pagination::ArticleListPagination;


#[function_component(ArticleList)]
pub fn article_list() -> Html
{
        html!{
                <div
                        class={classes!("pl-5")}
                >
                        <ArticleListItem
                                publish_time="OCTOBER 2021"
                                labels="BACKEND"
                                title="Article - Very First Post"
                                description="dfadsfas fdsafeafe dfadsfadsf"
                                link="/blog/blog1"
                        />
                        <ArticleListItem 
                                publish_time="OCTOBER 2021"
                                labels="BACKEND"
                                title="Article - Very Second Post"
                                description="dfadsfas fdsafeafe dfadsfadsf"
                                link="/blog/blog2"
                        />

                        <ArticleListPagination />
                </div>
        }
}