use crate::speaking::Speaking;
pub struct Person {
    name: String,
    age: u32,
}

impl Person{
   pub fn new(name: String, age: u32) -> Self{
       Self{
           name,
           age,
       }
   }

   pub fn hello(&self) {
       println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
   }

   // fn display(&self) {
   //     println!("Name: {}, Age: {}", self.name, self.age);
   // }
}

impl Speaking for Person{
    fn speak(&self) {
        // Todo
        println!("{} is speaking...", self.name)
    }
}
