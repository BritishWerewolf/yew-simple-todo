use yew::prelude::*;
use crate::{todos::item::TodoItem, store::StoreContext};

#[derive(PartialEq, Properties)]
pub struct TodoListProps {}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let TodoListProps {} = props;
    let store = use_context::<StoreContext>().unwrap();

    html! {
        <>
            <p class="mt-6 mb-2 text-lg font-bold text-center">{"Todo list"}</p>
            <div class="flex flex-wrap justify-center gap-2">
                { store.items.iter().map(|item| {
                    html! {
                        <TodoItem
                            id={item.id}
                            completed={item.completed}
                            name={item.name.clone()}
                        />
                    }
                }).collect::<Html>() }
            </div>
        </>
    }
}
