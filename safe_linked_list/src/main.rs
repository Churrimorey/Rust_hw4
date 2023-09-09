use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use linked_list::LinkedList;

mod linked_list;

fn main() {
    let a = Rc::new(LinkedList {
        element: 1,
        next: RefCell::new(vec![]),
        prev: RefCell::new(Weak::new()),
    });
    a.next.borrow_mut().push(Rc::clone(&a));
    *a.prev.borrow_mut() = Rc::downgrade(&a);
    assert_eq!(1, a.element);

    let b = Rc::new(LinkedList {
        element: 2,
        next: RefCell::new(vec![Rc::clone(&a)]),
        prev: RefCell::new(Weak::new()),
    });
    a.next.borrow_mut()[0] = Rc::clone(&b);
    *b.prev.borrow_mut() = Rc::downgrade(&a);
    *a.prev.borrow_mut() = Rc::downgrade(&b);

    assert_eq!(1, b.next.borrow()[0].element);
    assert_eq!(2, a.prev.borrow().upgrade().unwrap().element);
}
