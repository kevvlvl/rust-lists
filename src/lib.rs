use crate::second::List;

pub mod first;
pub mod second;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {

        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);


    }
}

 #[test]
fn peek() {
     let mut list = List::new();
     assert_eq!(list.peek(), None);
     assert_eq!(list.peek_mut(), None);
     list.push(1); list.push(2); list.push(3);

     assert_eq!(list.peek(), Some(&3));
     assert_eq!(list.peek_mut(), Some(&mut 3));
     list.peek_mut().map(|value| {
         *value = 42
     });

     assert_eq!(list.peek(), Some(&42));
     assert_eq!(list.pop(), Some(42));
 }