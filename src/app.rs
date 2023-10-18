use yew::prelude::*;
use crate::todos::list::TodoList;
use crate::todos::add::TodoAdd;
use crate::store::{Store, StoreContext};

#[function_component]
pub fn App() -> Html {
    let store = use_reducer(|| Store::default());

    html! {
        <ContextProvider<StoreContext> context={store}>
            <TodoAdd />
            <TodoList />
        </ContextProvider<StoreContext>>
    }
}
