use chrono::DateTime;
use uko_todo::{TodoList, Urgency};

fn main() {
    // Instantiate a todo list
    let mut todo_list = TodoList::new();

    // Create the first item
    todo_list.add("Eat Breakfast".to_string());
    todo_list.set_urgent(0, Urgency::Low);
    todo_list.set_due_at(0, DateTime::from_timestamp(1698330604, 0));
    println!("{:?}", todo_list);

    // Modify the text of the first item
    todo_list.set_text(0, "Eat a HEALTHY breakfast".to_string());
    println!("{:?}", todo_list);

    // Create a second item
    todo_list.add("Take a shower".to_string());
    todo_list.set_urgent(0, Urgency::Normal);
    todo_list.set_due_at(0, DateTime::from_timestamp(1698334204, 0));
    println!("{:?}", todo_list);

    // Create the third item
    todo_list.add("Eat Lunch".to_string());
    todo_list.set_urgent(0, Urgency::High);
    todo_list.set_due_at(0, DateTime::from_timestamp(1698341404, 0));
    println!("{:?}", todo_list);

    // Check off the first item
    todo_list.set_checked(0, true);
    println!("{:?}", todo_list);

    for (index, item) in todo_list.search(Some("Eat".to_string()), None, None, None, None) {
        println!("Item: {:?} {:?}", index, item);
    }
}
