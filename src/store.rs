use std::rc::Rc;
use yew::{Reducible, UseReducerHandle, ContextProvider};
use crate::{todos::item::{Item, ItemAutoIncrementId}, dark_mode::dark_class_toggle};

pub type StoreContext = UseReducerHandle<Store>;
pub type StoreProvider = ContextProvider<StoreContext>;

pub enum StoreAction {
    DarkModeToggle,
    AddItem(Item),
}

#[derive(Clone, PartialEq)]
pub struct Store {
    pub dark_mode: bool,
    item_id: ItemAutoIncrementId,
    pub items: Vec::<Item>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            dark_mode: false,
            item_id: ItemAutoIncrementId::new(),
            items: Vec::<Item>::new(),
        }
    }
}

impl Reducible for Store {
    type Action = StoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        // Create variables that _might_ be mutated through an action.
        let mut new_dark_mode = self.dark_mode;
        let mut item_id = self.item_id.clone();
        let mut new_items = self.items.clone();

        match action {
            Self::Action::DarkModeToggle => {
                new_dark_mode = !new_dark_mode;
                dark_class_toggle(new_dark_mode).expect("body can add 'dark' class");
            },
            Self::Action::AddItem(item) => {
                new_items.push(Item {
                    id: item_id.next(),
                    completed: item.completed,
                    name: item.name,
                });
            }
        }

        // Recreate a new store with these (potentially changed) variables.
        Store {
            dark_mode: new_dark_mode,
            item_id: item_id,
            items: new_items,
        }.into()
    }
}
