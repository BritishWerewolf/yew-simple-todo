use yew::prelude::*;
use crate::todos::list::TodoList;
use crate::todos::add::TodoAdd;
use crate::store::{Store, StoreProvider};

#[function_component]
pub fn App() -> Html {
    let store = use_reducer(|| Store::default());

    html! {
        <StoreProvider context={store}>
            <TodoAdd />
            <TodoList />
        </StoreProvider>
    }
}
