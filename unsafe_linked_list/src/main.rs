use std::ptr;

use linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut a = LinkedList {
        element: 1,
        next: ptr::null(),
        prev: ptr::null(),
    };
    a.next = &a as *const LinkedList<i32>;
    a.prev = &a as *const LinkedList<i32>;

    let mut b = LinkedList {
        element: 2,
        next: &a as *const LinkedList<i32>,
        prev: &a as *const LinkedList<i32>,
    };

    a.next = &b as *const LinkedList<i32>;
    a.prev = &b as *const LinkedList<i32>;

    let mut c = LinkedList {
        element: 3,
        next: &b as *const LinkedList<i32>,
        prev: &a as *const LinkedList<i32>,
    };

    b.prev = &c as *const LinkedList<i32>;
    a.next = &c as *const LinkedList<i32>;

    assert_eq!(3, unsafe { a.next.as_ref().unwrap().element });
    assert_eq!(2, unsafe { a.prev.as_ref().unwrap().element });
    assert_eq!(3, unsafe { b.prev.as_ref().unwrap().element });
    assert_eq!(1, unsafe { b.next.as_ref().unwrap().element });
    assert_eq!(1, unsafe { c.prev.as_ref().unwrap().element });
    assert_eq!(2, unsafe { c.next.as_ref().unwrap().element });
}
