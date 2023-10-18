use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::todos::item::Item;
use crate::store::{StoreContext, StoreAction};

#[derive(PartialEq, Properties)]
pub struct TodoAddProps {}

#[function_component]
pub fn TodoAdd(props: &TodoAddProps) -> Html {
    let TodoAddProps {} = props;

    let item = use_state(|| Item::default());
    let store = use_context::<StoreContext>().unwrap();

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

    let onclick_add = {
        let item = item.clone();
        Callback::from(move |_e: MouseEvent| {
            store.dispatch(StoreAction::AddItem(Item {
                completed: item.completed,
                name: item.name.clone()
            }));

            Item::reset(&item);
        })
    };

    html! {
        <div>
            <p>{"Add todo"}</p>
            <input type="checkbox" name="completed" onchange={onchange_completed} checked={item.completed} />
            <input type="text" name="name" class="border border-black" onchange={onchange_name} value={item.name.clone()} />
            <button onclick={onclick_add}>{"Add item"}</button>
        </div>
    }
}
