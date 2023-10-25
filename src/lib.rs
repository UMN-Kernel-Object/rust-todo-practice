use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

/// An item in the to-do list.
#[derive(Clone, Debug, Serialize)]
pub struct Item {
    /// The label of the item.
    pub text: String,

    /// Whether the item has been checked off or not.
    pub checked: bool,

    /// The "due date" of the item.
    pub due_at: Option<DateTime<Utc>>,

    /// How urgent the to-do item is.
    pub urgency: Urgency,
}

/// The urgency of a to-do item.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Urgency {
    Low,
    Normal,
    High,
}

#[test]
fn urgency_ord_does_the_right_thing() {
    assert!(Urgency::High > Urgency::Normal);
    assert!(Urgency::Normal > Urgency::Low);
}

/// The entire to-do list.
#[derive(Debug, Default)]
pub struct TodoList {
    /// The items in the to-do list.
    pub items: Vec<Item>,
}

impl TodoList {
    /// Returns a new, empty to-do list.
    pub fn new() -> TodoList {
        TodoList::default()
    }

    /// Adds a new item with default settings to the to-do list.
    pub fn add(&mut self, item_text: String) {
        todo!("Insert code here to add an item to the todo list")
    }

    /// Gets the details of an item in the to-do list by index.
    pub fn get(&self, index: usize) -> Item {
        todo!("Insert code here to get an item by index.")
    }

    /// Returns the indices and items that matched the given search criteria. All of the criteria
    /// are optional.
    ///
    /// The criteria are:
    ///
    /// - `text_substr`: The to-do item's text contains this substring.
    /// - `tag`: The to-do item has this tag.
    /// - `checked`: The to-do item is or isn't checked off.
    /// - `due_in_less_than`: The to-do item is due in less than this amount of time, including
    ///   being due in the past.
    /// - `urgency_at_least`: The to-do item's urgency is at least this value.
    /// - `urgency_at_most`: The to-do item's urgency is at most this value.
    ///
    pub fn search(
        &self,
        text_substr: Option<String>,
        checked: Option<bool>,
        due_in_less_than: Option<Duration>,
        urgency_at_least: Option<Urgency>,
        urgency_at_most: Option<Urgency>,
    ) -> Vec<(usize, Item)> {
        todo!("Insert code here to search for items based on given criteria.")
    }

    /// Sets whether a to-do item is checked off.
    pub fn set_checked(&mut self, index: usize, checked: bool) {
        todo!("Insert code here to check off the item as done.")
    }

    /// Sets the due date of a to-do list item.
    pub fn set_due_at(&mut self, index: usize, due_at: Option<DateTime<Utc>>) {
        todo!("Insert code here to set when an item is due")
    }

    /// Sets the label of a to-do item.
    pub fn set_text(&mut self, index: usize, text: String) {
        todo!("Insert code here to change the text of an item")
    }

    /// Sets how urgent a to-do item is.
    pub fn set_urgent(&mut self, index: usize, urgency: Urgency) {
        todo!("Insert code here to set the urgency of an item.")
    }
}
