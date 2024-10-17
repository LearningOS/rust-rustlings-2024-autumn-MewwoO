// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.


use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {  
    let sun = Rc::new(Sun {});  
    println!("Initial reference count = {}", Rc::strong_count(&sun)); // 1 reference  
  
    let mercury = Planet::Mercury(Rc::clone(&sun));  
    println!("Reference count after Mercury = {}", Rc::strong_count(&sun)); // 2 references  
    mercury.details();  
  
    let venus = Planet::Venus(Rc::clone(&sun));  
    println!("Reference count after Venus = {}", Rc::strong_count(&sun)); // 3 references  
    venus.details();  
  
    let earth = Planet::Earth(Rc::clone(&sun));  
    println!("Reference count after Earth = {}", Rc::strong_count(&sun)); // 4 references  
    earth.details();  
  
    let mars = Planet::Mars(Rc::clone(&sun));  
    println!("Reference count after Mars = {}", Rc::strong_count(&sun)); // 5 references  
    mars.details();  
  
    let jupiter = Planet::Jupiter(Rc::clone(&sun));  
    println!("Reference count after Jupiter = {}", Rc::strong_count(&sun)); // 6 references  
    jupiter.details();  
  
    // Share the same sun instance with Saturn  
    let saturn = Planet::Saturn(Rc::clone(&sun));  
    println!("Reference count after Saturn = {}", Rc::strong_count(&sun)); // 7 references  
    saturn.details();  
  
    // Share the same sun instance with Uranus  
    let uranus = Planet::Uranus(Rc::clone(&sun));  
    println!("Reference count after Uranus = {}", Rc::strong_count(&sun)); // 8 references  
    uranus.details();  
  
    // Share the same sun instance with Neptune  
    let neptune = Planet::Neptune(Rc::clone(&sun));  
    println!("Reference count after Neptune = {}", Rc::strong_count(&sun)); // 9 references  
    neptune.details();  
  
    // Drop planets one by one  
    drop(neptune);  
    println!("Reference count after dropping Neptune = {}", Rc::strong_count(&sun)); // 8 references  
  
    drop(uranus);  
    println!("Reference count after dropping Uranus = {}", Rc::strong_count(&sun)); // 7 references  
  
    drop(saturn);  
    println!("Reference count after dropping Saturn = {}", Rc::strong_count(&sun)); // 6 references  
  
    drop(jupiter);  
    println!("Reference count after dropping Jupiter = {}", Rc::strong_count(&sun)); // 5 references  
  
    drop(mars);  
    println!("Reference count after dropping Mars = {}", Rc::strong_count(&sun)); // 4 references  
  
    drop(earth);  
    println!("Reference count after dropping Earth = {}", Rc::strong_count(&sun)); // 3 references  
  
    drop(venus);  
    println!("Reference count after dropping Venus = {}", Rc::strong_count(&sun)); // 2 references  
  
    drop(mercury);  
    println!("Reference count after dropping Mercury = {}", Rc::strong_count(&sun)); // 1 reference  
  
    assert_eq!(Rc::strong_count(&sun), 1);  
}