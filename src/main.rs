mod components;
mod pages;
mod binding;

use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use crate::components::header::CaishenHeader;
use crate::pages::blog::Blog;

#[derive(Routable, Debug, Clone, PartialEq)]
enum AppRoute {
	#[at("/blog/:id")]
	Blog { id: String},
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
			<div class={classes!("container")}>
				<CaishenHeader/>
				<p/>
				<div>
					<BrowserRouter>
						<Switch<AppRoute>
							render = {
								Switch::render(|switch: &AppRoute| {
								match switch {
									AppRoute::Blog{id} => html!{<Blog id={id.clone()}/>},
									AppRoute::Default => html!{<h1> {"Mike Jiang's Blog"} </h1>},
								}
							})
						}
						/>
					</BrowserRouter>
				</div>
			</div>
		}
	}
}

fn main() {
    yew::start_app::<App>();
}
