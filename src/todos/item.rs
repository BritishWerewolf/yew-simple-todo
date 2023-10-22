use yew::prelude::*;
use yew::macros::props;

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
    #[prop_or(false)]
    pub completed: bool,
    #[prop_or_default]
    pub name: AttrValue,
}

#[function_component]
pub fn TodoItem(props: &TodoItemProps) -> Html {
    let TodoItemProps {
        completed,
        name,
    } = props;

    html! {
        <div class="w-full p-2 text-gray-900 border border-gray-900 rounded-md md:w-56 dark:text-gray-300 dark:border-gray-300">
            <p>{ "Completed: " }{ &completed }</p>
            <p>{ "Name: " }{ &name }</p>
        </div>
    }
}
