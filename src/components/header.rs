use yew::prelude::*;
use crate::components::bootstraps::b_btn::BPrimaryButton;
use crate::components::bootstraps::b_navbar::*;

pub struct CaishenHeader;

impl Component for CaishenHeader {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self
	}

	fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
		false
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
		  <BNavBar>
			<BNavBrand> {"Micromike's Blog"} </BNavBrand>
			<div class="d-flex">
				<BPrimaryButton> {"Login"} </BPrimaryButton>
			</div>
		  </BNavBar>
		}
	}
}
