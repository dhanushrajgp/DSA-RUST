

use std::collections::VecDeque;

pub fn vec_deque_examples(){
//A VecDeque with a known list of items can be initialized from an array:
let _deq = VecDeque::from([-1,0,1]);

//to create an empty deque.
let mut  _empty_deque:VecDeque<u32> = VecDeque::new();

//with_capacity function provides a initial capacity for the VecDeque.
let _capacity_deque:VecDeque<u32> = VecDeque::with_capacity(10);

//get(&self, index: usize) -> Option<&T> provides reference to the element at the given index.
_empty_deque.push_back(10);
_empty_deque.push_back(20);
_empty_deque.push_back(30);
println!("element at index 2 of _empty_deque {:?} ",_empty_deque.get(2));


//get_mut(&mut self, index: usize)  provides a mutable reference to the element at the given index.
if let Some(elem) = _empty_deque.get_mut(2){
    *elem = 40;
}

println!("the element of deque are {:?}",_empty_deque);

/*
swap(&mut self, i: usize, j: usize)
Swaps elements at indices i and j.
i and j may be equal.
Element at index 0 is the front of the queue.
Panics
Panics if either index is out of bounds.
*/
_empty_deque.swap(0, 2);
println!("after swap at index 0 and 2:  {:?}",_empty_deque);


/*
capacity(&self) -> usize
Returns the number of elements the deque can hold without reallocating.
*/
println!("capacity of _empty_deque: {:?}",_empty_deque.capacity());

/*
truncate(&mut self, len: usize)
Shortens the deque, keeping the first len elements and dropping the rest.

If len is greater or equal to the deque’s current length, this has no effect.
*/

_empty_deque.truncate(2);
println!("after truncating the deque upto 2 {:?}",_empty_deque);

_empty_deque.push_front(50);
_empty_deque.push_back(90);
println!("after inserting elements to front and back of deque {:?}",_empty_deque);


/*
len(&self) -> usize
Returns the number of elements in the deque.
*/
println!("length of deque: {:?}",_empty_deque.len());

/*
is_empty(&self) -> bool returns boolean value indicating whether the deque is empty or not.
*/
println!("is deque empty: {:?}",_empty_deque.is_empty());


/*
contains(&self, x: &T) -> bool
where
    T: PartialEq,
Returns true if the deque contains an element equal to the given value.

This operation is O(n).

Note that if you have a sorted VecDeque, binary_search may be faster.
*/
println!("does deque contains 30 ? {:?}",_empty_deque.contains(&30));


/*
append(&mut self, other: &mut VecDeque<T, A>)
Moves all the elements of other into self, leaving other empty.

Panics
Panics if the new number of elements in self overflows a usize.
*/
let mut buf: VecDeque<_> = [1, 2].into();
let mut buf2: VecDeque<_> = [3, 4].into();
buf.append(&mut buf2);
assert_eq!(buf, [1, 2, 3, 4]);
assert_eq!(buf2, []);

}