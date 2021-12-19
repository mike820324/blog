use yew::prelude::*;
use crate::components::article::Article;

pub struct BlogList;

impl Component for BlogList {
	type Message = ();
	type Properties = ();

	fn create(_: &Context<Self>) -> Self {
        Self
	}

	fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
                html!{
                <div
                        class={classes!("pl-5")}
                >
                        <Article 
                                publish_time="OCTOBER 2021"
                                labels="BACKEND"
                                title="Article - Very First Post"
                                description="dfadsfas fdsafeafe dfadsfadsf"
                                link="/blog/blog1"
                        />
                        <Article 
                                publish_time="OCTOBER 2021"
                                labels="BACKEND"
                                title="Article - Very Second Post"
                                description="dfadsfas fdsafeafe dfadsfadsf"
                                link="/blog/blog2"
                        />
                </div>
                }
        }
}