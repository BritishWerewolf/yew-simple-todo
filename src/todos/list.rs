use yew::prelude::*;
use crate::{todos::item::TodoItem, store::StoreContext};

#[derive(PartialEq, Properties)]
pub struct TodoListProps {}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let TodoListProps {} = props;
    let store = use_context::<StoreContext>().unwrap();

    html! {
        <div>
            <p>{"Todo list"}</p>
            { store.items.iter().map(|item| {
                html! {
                    <TodoItem
                        completed={item.completed}
                        name={item.name.clone()}
                    />
                }
            }).collect::<Html>() }
        </div>
    }
}
