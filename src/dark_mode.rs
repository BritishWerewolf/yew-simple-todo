use yew::prelude::*;
use wasm_bindgen::JsValue;
use crate::store::{StoreContext, StoreAction};

pub fn dark_class_toggle(add_dark: bool) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let html = body.parent_element().expect("body parent is element");

    let class_str = html.class_name();
    let mut classes: Vec<&str> = class_str.split(" ").collect();
    let dark_class = "dark";

    if add_dark && !classes.contains(&dark_class) {
        classes.push(dark_class);
    } else if !add_dark && classes.contains(&dark_class) {
        // We can safely unwrap because we know from above that it contains it.
        classes.remove(classes.iter().position(|&x| x == dark_class).unwrap());
    }

    let class_str = classes.join(" ").trim().to_owned();
    html.set_class_name(&class_str);

    Ok(())
}

#[derive(PartialEq, Properties)]
pub struct DarkModeToggleProps {}

#[function_component]
pub fn DarkModeToggle(props: &DarkModeToggleProps) -> Html {
    let DarkModeToggleProps {} = props;
    let store = use_context::<StoreContext>().unwrap();

    let onclick = {
        let store = store.clone();

        Callback::from(move |_event| {
            store.dispatch(StoreAction::DarkModeToggle)
        })
    };

    let text = if store.dark_mode { "Light mode" } else { "Dark mode" };
    html! {
        <button {onclick} class="absolute top-4 left-4 inline-flex items-center justify-center p-0.5 mb-2 mr-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 dark:from-pink-500 dark:to-orange-400 dark:group-hover:from-pink-500 dark:group-hover:to-orange-400 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800">
            <span class="relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0">
                {text}
            </span>
        </button>
    }
}
