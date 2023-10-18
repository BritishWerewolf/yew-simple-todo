use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};
use crate::todos::item::Item;

pub type StoreContext = UseReducerHandle<Store>;

pub enum StoreAction {
    AddItem(Item),
}

#[derive(Clone, PartialEq)]
pub struct Store {
    pub items: Vec::<Item>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            items: Vec::<Item>::new(),
        }
    }
}

impl Reducible for Store {
    type Action = StoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StoreAction::AddItem(item) => {
                let mut new_items = self.items.clone();
                new_items.push(Item {
                    completed: item.completed,
                    name: item.name,
                });

                Store {
                    items: new_items
                }.into()
            }
        }
    }
}
