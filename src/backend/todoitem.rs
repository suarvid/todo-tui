use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    title: String,
    completed: bool,
    sub_items: Vec<TodoItem>,
}

impl TodoItem {

    pub fn new_item(title: &str) -> TodoItem {
        TodoItem { title: String::from(title), completed: false, sub_items: Vec::new() }
    }

    pub fn get_title(&self) -> &str {
        &self.title.as_str()
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn set_completed(&mut self) {
        self.completed = true;
        for item in &mut self.sub_items {
            item.completed = true;
        }
    }

    pub fn set_not_completed(&mut self) {
        self.completed = false;
        for item in &mut self.sub_items {
            item.completed = false;
        }
    }

    pub fn add_sub_item(&mut self, sub_item: TodoItem) {
        self.sub_items.push(sub_item);
    }

    pub fn get_nb_sub_items(&self) -> usize {
        self.sub_items.len()
    }

    pub fn get_sub_items(&self) -> &Vec<TodoItem> {
        &self.sub_items
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_title_returns_title() {
        let item1 = TodoItem::new_item("Test Item 1");
        let item2 = TodoItem::new_item("Test Item 2");
        assert_eq!(item1.get_title(), "Test Item 1");
        assert_eq!(item2.get_title(), "Test Item 2");
    }

    #[test]
    fn test_created_item_is_not_completed() {
        let item = TodoItem::new_item("Test Item");
        assert!(!item.is_completed())
    }

    #[test]
    fn test_sub_item_added() {
        let mut main_item = TodoItem::new_item("Main Item");
        assert_eq!(main_item.get_nb_sub_items(), 0);
        let sub_item = TodoItem::new_item("Sub Item");
        main_item.add_sub_item(sub_item);
        assert_eq!(main_item.get_nb_sub_items(), 1);
    }

    #[test]
    fn test_sub_items_are_completed_when_completed() {
        let sub_item_1 = TodoItem::new_item("Sub Item 1");
        let sub_item_2 = TodoItem::new_item("Sub Item 2");
        let mut main_item = TodoItem::new_item("Main Item");
        main_item.add_sub_item(sub_item_1);
        main_item.add_sub_item(sub_item_2);
        main_item.set_completed();
        assert!(main_item.is_completed());
        for item in main_item.get_sub_items() {
            assert!(item.is_completed())
        }
    }

    #[test]
    fn test_sub_items_are_uncompleted_when_uncompleted() {
        let sub_item_1 = TodoItem::new_item("Sub Item 1");
        let sub_item_2 = TodoItem::new_item("Sub Item 2");
        let mut main_item = TodoItem::new_item("Main Item");
        main_item.add_sub_item(sub_item_1);
        main_item.add_sub_item(sub_item_2);
        main_item.set_not_completed();
        assert!(!main_item.is_completed());
        for item in main_item.get_sub_items() {
            assert!(!item.is_completed())
        }
    }

}
