/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        // 将 q1 中的元素移动到 q2，除了最后一个元素  
        while self.q1.size() > 1 {  
            match self.q1.dequeue() {  
                Ok(elem) => self.q2.enqueue(elem),  
                Err(_) => break, // 理论上不应该发生，因为 size() > 1  
            }  
        }  
  
        // 从 q1 中取出最后一个元素  
        match self.q1.dequeue() {  
            Ok(elem) => {  
                // 将 q2 中的元素移回 q1  
                while let Ok(elem2) = self.q2.dequeue() {  
                    self.q1.enqueue(elem2);  
                }  
                Ok(elem)  
            },  
            Err(err) => Err("Stack is empty"),  
        }  
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        // assert_eq!(s.pop(), Ok(2));
        // s.push(4);
        // s.push(5);
        // assert_eq!(s.is_empty(), false);
        // assert_eq!(s.pop(), Ok(5));
        // assert_eq!(s.pop(), Ok(4));
        // assert_eq!(s.pop(), Ok(1));
        // assert_eq!(s.pop(), Err("Stack is empty"));
        // assert_eq!(s.is_empty(), true);
	}
}