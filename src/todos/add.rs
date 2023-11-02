use yew::{prelude::*, props};
use web_sys::HtmlInputElement;
use crate::todos::item::{Item, ItemState};
use crate::store::{StoreContext, StoreAction};

#[derive(PartialEq, Properties)]
pub struct TodoAddProps {}

#[function_component]
pub fn TodoAdd(props: &TodoAddProps) -> Html {
    let TodoAddProps {} = props;

    let item = use_state(|| Item::default());
    let store = use_context::<StoreContext>().unwrap();

    //--------------------------------------------------------------------------
    // Event handlers
    let onchange_completed = {
        let item = item.clone();
        Callback::from(move |e: Event| {
            item.set(Item {
                // We can unchecked_into because we are calling this only on the
                // checkbox, which has no children.
                completed: e.target_unchecked_into::<HtmlInputElement>().checked(),
                ..(*item).clone() // Clone because of AttrValue.
            });
        })
    };

    let onchange_name = {
        let item = item.clone();
        Callback::from(move |e: Event| {
            item.set(Item {
                name: e.target_unchecked_into::<HtmlInputElement>().value().into(),
                ..*item
            });
        })
    };

    // Create an event for different elements to add an item.
    fn event_submit<T: wasm_bindgen::JsCast>(item: &ItemState, store: &StoreContext) -> yew::Callback<T>  {
        let item = item.clone();
        let store = store.clone();

        Callback::from(move |e: T| {
            // Early return if the user presses the carriage return.
            if let Some(keyboard_event) = e.dyn_ref::<KeyboardEvent>() {
                if keyboard_event.key() != "Enter" && keyboard_event.key_code() != 13 {
                    return;
                }
            }

            // We don't want to submit if the user typed nothing in.
            if item.name == "" {
                return;
            }

            store.dispatch(StoreAction::AddItem(Item {
                completed: item.completed,
                name: item.name.clone(),
                ..props!(Item {}) // Don't worry about the ID right now!
            }));

            Item::reset(&item);
        })
    }

    let onkeyup_submit = event_submit::<KeyboardEvent>(&item, &store);
    let onclick_submit = event_submit::<MouseEvent>(&item, &store);


    html! {
        <div class="w-full max-w-xs p-2 mx-auto border border-gray-900 rounded-md dark:border-gray-300">
            <p class="mb-2 text-lg font-bold text-center">{"Add todo"}</p>
            <div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input name="completed" type="checkbox" value="" class="sr-only peer" onchange={onchange_completed} checked={item.completed} />
                    <div class="peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 peer-checked:bg-blue-600 w-11 h-6 dark:bg-gray-700 bg-gray-400 peer-focus:outline-none peer-focus:ring-4 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600"></div>
                    <span class="ml-3 text-sm font-medium">{"Is completed?"}</span>
                </label>
                <input type="text" name="name" onchange={onchange_name} onkeyup={onkeyup_submit} value={item.name.clone()} class="bg-gray-50 border border-gray-400 text-sm rounded-t-lg focus:ring-blue-500 inline-block focus:border-blue-500 w-full p-2.5 dark:bg-gray-700 dark:border-gray-700 dark:placeholder-gray-400 dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>

            <button onclick={onclick_submit} class="focus:outline-none text-white dark:text-gray-900 bg-green-700 hover:bg-green-800 focus:ring-4 rounded-t-none focus:ring-green-300 block w-full font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">{"Add item"}</button>
        </div>
    }
}
