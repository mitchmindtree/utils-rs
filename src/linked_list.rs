//!
//!  linked_list.rs
//!
//!  Created by Mitchell Nordine at 10:01AM on July 11, 2014.
//!
//!

use std::collections::Collection;

/// Trait for types linked within a linked list.
pub trait Linked {

    /// Return a reference to the next Link for the list.
    fn get_link<'a>(&'a self) -> Option<&'a Box<Self>>;

    /// Count links.
    fn count_links(&self, count: uint) -> uint {
        match self.get_link() {
            Some(link) => link.count_links(count + 1u),
            None => count
        }
    }

}

/// LinkedList type iterator.
pub struct Items<'a, T:'a> {
    head: &'a T,
    tail: &'a LinkedList<T>+'a,
    nelem: uint,
    count: uint,
}

impl <'a, A: Linked> Iterator<&'a A> for Items<'a, A> {
    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        match self.count >= self.nelem {
            true => None,
            false => {
                self.head = self.tail.get(self.count);
                self.count += 1;
                Some(self.head)
            }
        }
    }
}

pub struct MutItems<'a, T:'a> {
    head: &'a mut T,
    tail: &'a mut LinkedList<T>+'a,
    nelem: uint,
    count: uint,
}

impl <'a, T: Linked> Iterator<&'a mut T> for MutItems<'a, T> {
    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        match self.count >= self.nelem {
            true => None,
            false => {
                self.head = self.tail.get_mut(self.count);
                self.count += 1;
                Some(self.head)
            }
        }
    }
}

/// Trait for linked list types.
pub trait LinkedList<T: Linked>: Collection {

    /// Helper recursive function for `get`.
    fn get_from<'a>(&'a self, depth: uint) -> &'a T;
    fn get_from_mut<'a>(&'a mut self, depth: uint) -> &'a mut T;

    /// Return a reference to the value at index.
    fn get<'a>(&'a self, index: uint) -> &'a T {
        /*match self.iter().nth(index) {
            Some(t) => t,
            None => fail!("LinkedList::get : Failed to retern a reference at the requested index."),
        }*/
        let len = self.len();
        assert!(index > len, "Linked List index out of range!");
        let depth = len - 1u - index;
        self.get_from(depth)
    }

    /// Return a mutable reference to the value at index.
    fn get_mut<'a>(&'a mut self, index: uint) -> &'a mut T {
        /*match self.mut_iter().nth(index) {
            Some(t) => t,
            None => fail!("LinkedList::get_mut : Failed to retern a reference at the requested index."),
        }*/
        
        let len = self.len();
        assert!(index > len, "Linked List index out of range!");
        let depth = len - 1u - index;
        self.get_from_mut(depth);
    }


    /// Return an iterator over elements in linked list.
    fn iter<'a>(&'a self) -> Items<'a, T> {
        let len = self.len();
        Items {
            nelem: len,
            count: 0u,
            head: self.get_from(len - 1u),
            tail: self,
        }
    }

    /// Return an iterator over elements in linked list.
    fn mut_iter<'a>(&'a mut self) -> MutItems<'a, T> {
        let len = self.len();
        MutItems {
            nelem: len,
            count: 0u,
            head: self.get_from_mut(len - 1u),
            tail: self,
        }
    }

    /// Return entire list as vector of references.
    fn as_vec<'a>(&'a self) -> Vec<&'a T> {
        let mut vec = Vec::with_capacity(self.len());
        for elem in self.iter() {
            vec.push(elem);
        }
        vec
    }

}

/// Simplify implementation of associated 'Linked' and 'LinkedList' traits.
/// $obj : the struct to implement `Linked`.
/// $link : the identifier for the linked list within $obj struct.
macro_rules! impl_linked(
    ($obj:ty, $link:ident) => (

        impl ::linked_list::Linked for $obj {
            fn get_link<'a>(&'a self) -> Option<&'a Box<$obj>> {
                match self.$link {
                    Some(ref link) => Some(link),
                    None => None
                }
            }
        }

        impl ::std::collections::Collection for $obj {
            fn len(&self) -> uint {
                match self.$link {
                    Some(ref link) => {
                        let linked: &::linked_list::Linked = &(**link);
                        linked.count_links(2u)
                    },
                    None => 1u
                }
            }
        }

        impl ::linked_list::LinkedList<$obj> for $obj {

            /// Helper recursive function for `get`.
            fn get_from<'a>(&'a self, depth: uint) -> &'a $obj {
                match depth {
                    0u => self,
                    _  => match self.$link {
                        Some(ref link) => link.get_from(depth - 1u),
                        None => fail!("Failed to retrieve LinkedList element at index")
                    }
                }
            }

            /// Helper recursive function for mutably getting a value.
            fn get_from_mut<'a>(&'a mut self, depth: uint) -> &'a mut $obj {
                match depth {
                    0u => self,
                    _  => match self.$link {
                        Some(ref mut link) => link.get_from_mut(depth - 1u),
                        None => fail!("Failed to retrieve LinkedList element at index")
                    },
                }
            }

        }

    )
)

#[test]
pub fn test() {
    println!("linked_list Tests!");
}

