use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct LinkedList<T> {
    pub element: T,
    pub next: RefCell<Vec<Rc<LinkedList<T>>>>,
    pub prev: RefCell<Weak<LinkedList<T>>>,
}
