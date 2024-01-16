use std::collections::LinkedList;

pub fn linkedlist_dsa_examples(){

    //declaring a new empty Linkedlist.
    let mut _list:LinkedList<u32>  = LinkedList::new();

    //A LinkedList with a known list of items can be initialized from an array:
    let mut list2 = LinkedList::from([1,2,30]);

    //fn append(&mut self, other: &mut LinkedList<T>). append function appends all the elements from other list to current list.
    _list.append(&mut list2);

    println!("the value of _list now becomes {:?}",_list);

    //pub fn iter(&self) -> Iter<'_, T> this function provides a forward iterator.
    let mut iter = _list.iter();
    println!("iter next value is: {:?}{:?}",iter.next(), Some(&0));
    println!("iter next value is: {:?}{:?}",iter.next(), Some(&1));


    //pub fn iter_mut(&mut self) -> IterMut<'_, T> provides  a forward iterator with mutable references.
    for element in _list.iter_mut() {
        *element += 10;
    }

    println!("after iter_mut the list value is: {:?}",_list);


    /*
    pub fn is_empty(&self) -> bool
    Returns true if the LinkedList is empty.
    This operation should compute in O(1) time
     */

    /*
    pub fn len(&self) -> usize
    Returns the length of the LinkedList.
     */

    /*  pub fn contains(&self, x: &T) -> bool
    where
    T: PartialEq,
    Returns true if the LinkedList contains an element equal to the given value.
    This operation should compute linearly in O(n) time.
     */

    println!("does _list contains 50 ? {:?}",_list.contains(&50));

    /*
    pub fn push_front(&mut self, elt: T)
    Adds an element first in the list.
    This operation should compute in O(1) time.

    pub fn pop_front(&mut self) -> Option<T>
    Removes the first element and returns it, or None if the list is empty.
    This operation should compute in O(1) time.

    pub fn push_back(&mut self, elt: T)
    Appends an element to the back of a list.
    This operation should compute in O(1) time.


    pub fn pop_back(&mut self) -> Option<T>
    Removes the last element from a list and returns it, or None if it is empty.
    This operation should compute in O(1) time.
     */
    _list.push_back(5);
    _list.push_front(10);
    _list.push_back(14);
    _list.pop_front();


    println!("after push and pop functions on either side the list now: {:?}",_list);


    /*
    pub fn split_off(&mut self, at: usize) -> LinkedList<T, A>
    where
    A: Clone,
    Splits the list into two at the given index. Returns everything after the given index, including the index.
    This operation should compute in O(n) time.
     */

     let split = _list.split_off(2);
     println!("after split off at 2: {:?}",split);


}