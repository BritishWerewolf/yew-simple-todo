use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::macros::props;
use yew_hooks::prelude::*;
use crate::store::{StoreContext, StoreAction};

pub type ItemId = usize;

#[derive(Clone, PartialEq)]
pub struct ItemAutoIncrementId(ItemId);
impl ItemAutoIncrementId {
    pub fn new() -> ItemAutoIncrementId {
        ItemAutoIncrementId(0)
    }

    #[allow(dead_code)]
    pub fn current(&self) -> ItemId {
        self.0
    }

    pub fn next(&mut self) -> ItemId {
        self.0 += 1;
        self.0
    }
}

// Create a new type for handling individual items.
pub type Item = TodoItemProps;
pub type ItemState = UseStateHandle<Item>;
impl Item {
    pub fn default() -> Item {
        // This macro will create a new struct using the defaults.
        props!(Item {})
    }

    // Reset the state of item, and force a re-render.
    pub fn reset(item: &UseStateHandle<Item>) {
        let reset_item = {
            let item = item.clone();
            Callback::from(move |_| {
                item.set(Item::default());
            })
        };

        // Force a re-render.
        reset_item.emit(());
    }
}


#[derive(Clone, PartialEq, Properties)]
pub struct TodoItemProps {
    // This is so that we can call "store.dispatch(AddItem)" and not have to
    // worry about what the ID _should_ be.
    #[prop_or(0)]
    pub id: usize,
    #[prop_or(false)]
    pub completed: bool,
    #[prop_or_default]
    pub name: AttrValue,
}

#[function_component]
pub fn TodoItem(props: &TodoItemProps) -> Html {
    let editing = use_toggle(false, true);
    let item = use_state(|| Item::from(props.clone()));
    let store = use_context::<StoreContext>().unwrap();

    // Click handlers
    let onclick_edit = {
        let editing = editing.clone();
        Callback::from(move |_| editing.toggle())
    };
    let onclick_delete = {
        let id = item.id.clone();
        let store = store.clone();

        Callback::from(move |_| {
            let should_delete = web_sys::window().unwrap().confirm_with_message("Are you sure you want to delete?").unwrap_or_else(|_| false);
            if should_delete {
                store.dispatch(StoreAction::DeleteItem(id))
            }
        })
    };
    let onclick_save = {
        let item = props.clone();
        let editing = editing.clone();
        let store = store.clone();

        Callback::from(move |_| {
            store.dispatch(StoreAction::SaveItem(item.clone()));
            editing.toggle();
        })
    };

    // Change handlers
    let onchange_completed = {
        let item = item.clone();

        Callback::from(move |e: Event| {
            item.set(Item {
                completed: e.target_unchecked_into::<HtmlInputElement>().checked(),
                ..(*item).clone()
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

    let onkeyup_submit = {
        let item = item.clone();
        let editing = editing.clone();

        Callback::from(move |e: KeyboardEvent| {
            // Early return if the user doesn't press the carriage return.
            if e.key() != "Enter" && e.key_code() != 13 {
                return;
            }

            // We don't want to submit if the user typed nothing in.
            if item.name == "" {
                return;
            }

            store.dispatch(StoreAction::SaveItem((*item).clone()));
            editing.toggle();
        })
    };

    html! {
        <div class="w-full p-2 text-gray-900 border border-gray-900 rounded-md md:w-56 dark:text-gray-300 dark:border-gray-300">
            if *editing {
                <p>{"ID: "}{ &item.id } </p>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input name="completed" type="checkbox" value="" class="sr-only peer" onchange={onchange_completed} checked={item.completed} />
                    <div class="peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 peer-checked:bg-blue-600 w-11 h-6 dark:bg-gray-700 bg-gray-400 peer-focus:outline-none peer-focus:ring-4 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600"></div>
                    <span class="ml-3 text-sm font-medium">{"Is completed?"}</span>
                </label>
                <input type="text" name="name" onchange={onchange_name} onkeyup={onkeyup_submit} value={item.name.clone()} class="bg-gray-50 border border-gray-400 text-sm rounded-t-lg focus:ring-blue-500 inline-block focus:border-blue-500 w-full p-2.5 dark:bg-gray-700 dark:border-gray-700 dark:placeholder-gray-400 dark:focus:ring-blue-500 dark:focus:border-blue-500" />
                <button onclick={onclick_save} class="rounded-lg rounded-t-none focus:outline-none text-white dark:text-gray-900 bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 block w-full font-medium text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">{"Save"}</button>

            } else {
                <p>{"ID: "}{ (*item).id } </p>
                <p>{"Completed: "}{ (*item).completed }</p>
                <p>{"Name: "}{ (*item).name.clone() }</p>

                <div class="flex flex-row justify-stretch mt-4">
                    <button class="rounded-lg rounded-r-none focus:outline-none text-white dark:text-gray-900 bg-yellow-500 hover:bg-yellow-800 focus:ring-4 focus:ring-yellow-300 block w-full font-medium text-sm px-5 py-2.5 dark:bg-yellow-600 dark:hover:bg-yellow-700 dark:focus:ring-yellow-700" onclick={onclick_edit}>{"Edit"}</button>
                    <button class="rounded-lg rounded-l-none focus:outline-none text-white dark:text-gray-900 bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 block w-full font-medium text-sm px-5 py-2.5 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800" onclick={onclick_delete}>{"Delete"}</button>
                </div>
            }
        </div>
    }
}
