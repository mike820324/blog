use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SideBarProps 
{
    pub author: String,
    pub avatar: String,
    pub description: String,
    pub items: Vec<(&'static str, &'static str)>,
}

#[function_component(SideBar)]
pub fn sidebar(props: &SideBarProps) -> Html
{
    let nav_items: Vec<_> = props.items.clone()
    .into_iter()
    .map(|item| html!{
        <li
            class={classes!("py-1")}
        >
            <a
                class={classes!("hover:border-b-4", "hover:border-color-gray", "hover:text-blue-400", "text-sm")}
                href={item.1}
            > {item.0}</a>
        </li>
    })
    .collect();

    html!{
        <div
            class={classes!("border-r", "mr-2")}
        >
            <img
                class={classes!("w-[75px]", "h-[75px]", "rounded-full")}
                src={props.avatar.clone()}
            />

            <section
                class={classes!("py-5")}
            >
                <h1
                    class={classes!("py-2", "leading-4", "font-semibold")}
                >
                    {props.author.clone()}
                </h1>
                <p
                    class={classes!("py-2", "text-sm", "text-gray-500")}
                > {props.description.clone()} </p>
            </section>

            <nav>
                <ul>
                    {nav_items}
                </ul>
            </nav>

        </div>
    }
}