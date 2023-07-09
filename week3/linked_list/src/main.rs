use linked_list::LinkedList;
use linked_list::ComputeNorm;
pub mod linked_list;

fn main() {

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}

    let mut list : LinkedList<String> = LinkedList::new();
    list.push_front("hello".to_string());
    list.push_front("world".to_string());
    let list_clone = list.clone();
    println!("{}", list == list_clone);
    println!("{}", list_clone);
    println!("{}", list);
    list.pop_front();
    println!("{}", list);
    println!("{}", list == list_clone);

    let mut list: LinkedList<f64> = LinkedList::new();
    list.push_front(3.0);
    list.push_front(4.0);
    println!("{}", list.compute_norm());
}

#[test]
fn eq_test() {
    let mut list : LinkedList<String> = LinkedList::new();
    list.push_front("hello".to_string());
    list.push_front("world".to_string());
    let list_clone = list.clone();

    assert_eq!(list_clone == list, true);
    list.push_front("Rust".to_string());
    assert_eq!(list_clone != list, true);
    list.pop_front();
    assert_eq!(list_clone == list, true);
}


#[test]
fn u32_test() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
}
