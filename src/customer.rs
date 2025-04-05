pub struct Customer{
    name: String,
    tel: String,
}

impl Customer{
    pub fn new(name: String, tel: String) -> Self{
        Self{
            name,
            tel,
        }
    }

    pub fn hello(&self) {
        println!("Hello, my name is {} and my phone number is {}.", self.name, self.tel);
    }
}