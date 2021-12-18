use yew::prelude::*;

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
            <>
                <div>
                        <h6 class="card-subtitle"> {"October 2021"} </h6>
                        <h5 class="card-title"> {"Blog 1"} </h5>
                        <p class="card-text"> {"This is the very first blog post"} </p>
                        <a class="card-link" href="/blog/blog1"> {"Read"} </a>
                </div>
                <div>
                        <h6 class="card-subtitle"> {"October 2021"} </h6>
                        <h5 class="card-title"> {"Blog 2"} </h5>
                        <p class="card-text"> {"This is the very first blog post"} </p>
                        <a class="card-link" href="/blog/blog2"> {"Read"} </a>
                </div>
            </>
        }
    }
}