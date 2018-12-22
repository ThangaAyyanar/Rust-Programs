
// Linked list that works
use std::mem;

struct Node{

    elem: i32,
    next: Link,
}

enum Link {

    Empty,
    More(Box<Node>),
}

pub struct List{

    head: Link
}

impl List{

    pub fn new() -> Self {
        
        List{ head: Link::Empty }
    }

    pub fn push(&mut self,value:i32) {

        let newNode = Box::new(Node{ elem: value,
                                     next: mem::replace(&mut self.head,Link::Empty) });
        self.head = Link::More(newNode);
    }

    // Pop operation remove the element from the List in rust it is easy , Match statement in rust borrow the value hence the value is removed
    // but the problem is when we access the 
    pub fn pop(&mut self) -> Option<i32> { 

       match mem::replace(&mut self.head,Link::Empty) {
            
           Link::Empty => {
                
               None
           }
           Link::More(boxed_node) => {
                
               let nodeValue = *boxed_node;
               self.head = nodeValue.next;
               Some(nodeValue.elem)
           }
       }
       //unimplemented!() // this is to tell compiler we haven't implemented the function so it will panic when called
    }
}

impl Drop for List{

    fn drop(&mut self) {

        let mut cur_link = mem::replace(&mut self.head,Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            
            cur_link = mem::replace(&mut self.head,Link::Empty);
        }
    }
}

#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basic() {

        let mut list = List::new();

        assert_eq!(list.pop(),None);

        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.pop(),Some(3));
        assert_eq!(list.pop(),Some(2));
        
        list.push(4);
        list.push(5);

        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(),Some(4));

        assert_eq!(list.pop(),Some(1));
        assert_eq!(list.pop(),None);
    }
}
