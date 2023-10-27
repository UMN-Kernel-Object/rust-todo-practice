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

    pub fn add_full_item(&mut self, item: &Item) {
        self.items.push(item.clone());
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
        self.items.iter().enumerate().filter(|(_, item)| {
            (urgency_at_least.is_some() && // Some > None, add this check
             Some(&item.urgency) >= urgency_at_least.as_ref()) ||
            // no need to worry about is_some check as Some > None
            Some(&item.urgency) <= urgency_at_most.as_ref() ||
            Some(&item.text) == text_substr.as_ref() ||
            Some(&item.checked) == checked.as_ref() ||
            item.due_at.map_or(false, 
                |date| Some(date - Utc::now()) <= due_in_less_than
            ) 
        }).map(|(ind, item)| (ind, (*item).clone())).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    /* urgency tests */
    #[test]
    fn urgency_ord_does_the_right_thing() {
        assert!(Urgency::High > Urgency::Normal);
        assert!(Urgency::Normal > Urgency::Low);
    }

    /* search tests */
    fn setup_search_tests(add: bool) -> (Vec<Item>, TodoList) {
        let mut list = TodoList::new();
        let mut item_vec: Vec<Item> = vec![];
        if add {
            let item1 = Item { 
                text: String::from("I am item1!"),
                checked: false,
                due_at: None,
                urgency: Urgency::Low   
            };
    
            let item2 = Item {
                text: String::from("I am item2!"),
                checked: true,
                due_at: Some(Utc::now() + Duration::seconds(100000)), // 27 hours roughly
                urgency: Urgency::High
            };
    
            let item3 = Item {
                text: String::from("I am item3!"),
                checked: false,
                due_at: None,
                urgency: Urgency::Normal
            };

            item_vec.extend([item1, item2, item3]);
            for item in item_vec.iter() {
                list.add_full_item(&item);
            }
        };

        (item_vec, list)
    }

    fn compare_items(item1: &Item, item2: &Item) {
        assert!(item1.text == item2.text);
        assert!(item1.urgency == item2.urgency);
        assert!(item1.checked == item2.checked);
        assert!(item1.due_at == item2.due_at);
    }

    fn compare_item_lists(list1: &Vec<(usize, Item)>, list2: &Vec<Item>) {
        assert!(list1.len() == list2.len());
        for (ind, item) in list1.iter() {
            assert!(ind < &list2.len());
            let act_item = &list2[*ind];
            compare_items(item, act_item);
        }
    }

    #[test]
    fn none_search_on_empty_list_returns_nothing() {
        let (_, list) = setup_search_tests(false);

        let empty_list = list.search(None, None, None, None, None);

        assert!(empty_list.len() == 0);
    }

    #[test]
    fn non_none_search_on_empty_list_returns_nothing() {
        let (_, list) = setup_search_tests(false);

        let empty_list = list.search(
            Some(String::from("nothing here!")), 
            None, None, None, None
        );

        assert!(empty_list.len() == 0);
    }

    #[test]
    fn none_search_on_full_list_returns_nothing() {
        let (_, list) = setup_search_tests(true);

        let empty_list = list.search(None, None, None, None, None);

        assert!(empty_list.len() == 0);
    }

    #[test]
    fn search_on_full_list_with_no_matchable_items_returns_nothing() {
        let (_, list) = setup_search_tests(true);

        let empty_list = list.search(
            Some(String::from("nothing here!")),
            None, None, None, None
        );

        assert!(empty_list.len() == 0);
    }

    #[test]
    fn search_on_list_with_one_matching_item() {
        let (items, list) = setup_search_tests(true);

        let one_list = list.search(
            Some(String::from("I am item1!")), 
            None, None, None, None
        );

        assert!(one_list.len() == 1);
        
        let (ind, item) = &one_list[0];
        let (act_ind, act_item) = (0, &items[0]);
        assert!(*ind == act_ind);
        compare_items(&item, act_item);
    }

    #[test]
    fn search_on_list_matching_all() {
        let (items, list) = setup_search_tests(true);

        let all_list = list.search(
            None, None, None, Some(Urgency::Low), Some(Urgency::High)
        );

        compare_item_lists(&all_list, &items);
    }

    #[test]
    fn search_returned_values_arent_modified() {
        let (_, list) = setup_search_tests(true);

        let all_list = list.search(
            None, None, None, Some(Urgency::Low), Some(Urgency::High)
        );

        compare_item_lists(&all_list, &list.items);
    }

    #[test]
    fn search_on_string() {
        let (items, list) = setup_search_tests(true);

        let string_list = list.search(
            Some(String::from("I am item2!")), None, None, None, None
        );

        assert!(string_list.len() == 1);
        let (ind, item) = &string_list[0];
        assert!(*ind < items.len());
        assert!(*ind == 1);
        compare_items(&item, &items[*ind]);
    }

    #[test]
    fn search_on_checked() {
        let (items, list) = setup_search_tests(true);

        let checked_list = list.search(
            None, Some(true), None, None, None
        );

        assert!(checked_list.len() == 1);
        let (ind, item) = &checked_list[0];
        assert!(*ind < items.len());
        assert!(*ind == 1);
        compare_items(&item, &items[*ind]);
    }

    #[test]
    fn search_on_due_before() {
        let (items, list) = setup_search_tests(true);

        let due_list = list.search(
            None, None, Some(Duration::seconds(200000)), None, None
        );
        assert!(due_list.len() == 1);
        let (ind, item) = &due_list[0];
        assert!(*ind < items.len());
        assert!(*ind == 1);
        compare_items(&item, &items[*ind]);
    }

    #[test]
    fn search_on_least_urgency() {
        let (items, list) = setup_search_tests(true);

        let least_list = list.search(
            None, None, None, Some(Urgency::Normal), None
        );

        assert!(least_list.len() == 2);
        let (ind1, item1) = &least_list[0];
        let (ind2, item2) = &least_list[1];
        assert!(*ind1 < items.len() && *ind2 < items.len());
        assert!(*ind1 == 1 && *ind2 == 2);
        compare_items(&item1, &items[*ind1]);
        compare_items(&item2, &items[*ind2]);
    }

    #[test]
    fn search_on_most_urgency() {
        let (items, list) = setup_search_tests(true);

        let most_list = list.search(
            None, None, None, None, Some(Urgency::Normal)
        );

        assert!(most_list.len() == 2);
        let (ind1, item1) = &most_list[0];
        let (ind2, item2) = &most_list[1];
        assert!(*ind1 < items.len() && *ind2 < items.len());
        assert!(*ind1 == 0 && *ind2 == 2);
        compare_items(&item1, &items[*ind1]);
        compare_items(&item2, &items[*ind2]);
    }

    // doesn't actually test anything, just times the search function
    // should be run with the --nocapture argument to actually show stdout
    #[test]
    fn timing_test() {
        let (_, mut list) = setup_search_tests(false);
        for i in 1..1_000_000 {
            list.add_full_item(&Item {
                text: format!("I am item {i}!"),
                checked: true,
                due_at: None,
                urgency: Urgency::Normal
            })
        }

        let before = Utc::now();
        list.search(None, Some(true), None, None, None);
        let after = Utc::now();

        let diff = (after - before).num_milliseconds();
        println!("Duration: {diff}");
    }
}
