mod components;
mod pages;
mod binding;

use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use crate::components::sidebar::*;
use crate::pages::blog::Blog;
use crate::pages::blog_list::BlogList;

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
			<div class={classes!("flex", "flex-row", "px-10", "py-10")}>
				<div 
					class={classes!("basis-2/6")}
				>
					<SideBar
						author={"Mike Jiang"}
						description={"A senior developer and a rust lover"}
						avatar={"https://blog.micromike.dev/photo.jpg"}
						items={
							vec![
								("Articles", "/blog"),
								("About me", "/profile"),
								("Contact me", "/contact"),
							]
						}
					/>
				</div>

				<div
					class={classes!("basis-4/6")}
				>
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
		}
	}
}

fn main() {
    yew::start_app::<App>();
}
