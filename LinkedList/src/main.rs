use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // 入队
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    list.push_back(3);

    println!("{:?}", list);

    // 出队
    while let Some(value) = list.pop_front() {
        println!("{}", value);
    }


}
