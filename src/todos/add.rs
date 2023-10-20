use yew::prelude::*;
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
                name: item.name.clone()
            }));

            Item::reset(&item);
        })
    }

    let onkeyup_submit = event_submit::<KeyboardEvent>(&item, &store);
    let onclick_submit = event_submit::<MouseEvent>(&item, &store);


    html! {
            <input type="checkbox" name="completed" onchange={onchange_completed} checked={item.completed} />
            <input type="text" name="name" class="border border-black" onchange={onchange_name} onkeyup={onkeyup_submit} value={item.name.clone()} />
            <button onclick={onclick_submit}>{"Add item"}</button>
        </div>
    }
}
