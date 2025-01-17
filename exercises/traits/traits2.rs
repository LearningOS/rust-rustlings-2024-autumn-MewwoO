// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar<T> {  
    fn append_bar(self) -> Vec<T> where T: Sized + Clone;  
}  

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar<String> for Vec<String> {  
    fn append_bar(self) -> Vec<String> {  
        let mut vec = self; // 直接接收 self 的所有权  
        vec.push(String::from("Bar")); // 向向量末尾添加 "Bar"  
        vec // 返回修改后的新向量  
    }  
}  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
