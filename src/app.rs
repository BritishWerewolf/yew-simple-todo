use yew::prelude::*;
use crate::dark_mode::DarkModeToggle;
use crate::todos::list::TodoList;
use crate::todos::add::TodoAdd;
use crate::store::{Store, StoreProvider};

#[function_component]
pub fn App() -> Html {
    let store = use_reducer(|| Store::default());

    html! {
        <div class="w-full mx-auto text-gray-900 md:max-w-5xl dark:text-gray-300">
            <StoreProvider context={store}>
                <TodoAdd />
                <TodoList />

                <DarkModeToggle />
            </StoreProvider>
        </div>
    }
}
