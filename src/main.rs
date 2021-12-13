mod components;
mod pages;
mod binding;

use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use crate::pages::blog::Blog;
use crate::pages::blog_list::BlogList;
use crate::components::bootstraps::b_card::*;

#[derive(Routable, Debug, Clone, PartialEq)]
enum AppRoute {
	#[at("/blog/:id")]
	Blog { id: String},
	#[at("/blog")]
	BlogList,
	#[at("/")]
	Default,
}

struct App;

impl Component for App {
	type Message = ();
	type Properties = ();

	fn create(_: &Context<Self>) -> Self {
		Self
	}

	fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
		false
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<div class={classes!("container")} style="padding: 5em;">
				<div class="row">
					<div class="col-sm-4">
						<BCard>
							<img src="https://blog.micromike.dev/photo.jpg" class="card-img-top" style="width: 75px; height: 75px; border-radius:50%;"/>
							<BCardBody>
								<h5 class="card-title"> {"Rueimin Jiang Blog"} </h5>
								<h6 class="card-subtitle"> {"Love Programming and sharing"} </h6>
							</BCardBody>
						</BCard>
					</div>
					<div class="col-sm-8">
						<BrowserRouter>
							<Switch<AppRoute>
								render = {
									Switch::render(|switch: &AppRoute| {
									match switch {
										AppRoute::Blog{id} => html!{<Blog id={id.clone()}/>},
										AppRoute::BlogList => html!{<BlogList />},
										AppRoute::Default => html!{<BlogList />},
									}
								})
							}
							/>
						</BrowserRouter>
					</div>
				</div>
			</div>
		}
	}
}

fn main() {
    yew::start_app::<App>();
}
