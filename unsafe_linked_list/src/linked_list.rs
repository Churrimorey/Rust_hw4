pub struct LinkedList<T> {
    pub element: T,
    pub next: *const LinkedList<T>,
    pub prev: *const LinkedList<T>,
}
