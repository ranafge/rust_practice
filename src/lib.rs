
pub struct Inventory {
    current_state: Option<Box<dyn State>>,
    item: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            current_state: Some(Box::new(InStock)),
            item: 3,
        }
    }
    pub fn add_item(&mut self, item: u32) {
        self.item += item
    }
    pub fn display_item(&self) -> u32 {
        self.item
    }
    pub fn sold(&mut self) -> u32 {
        if self.item > 0 {
            self.item -= 1;
            println!("sold item {}", self.item );
            self.item
            
        }else {
            self.go_to_outofstock();
            return self.item;
        }
    }

    pub fn go_to_outofstock(&mut self) {
        if self.display_item() == 0 {
            if let Some(s) = self.current_state.take() {
                self.current_state = Some(s.go_to_outofstock());
                println!("Out of stock state");
            }
        } else {
            println!("You have {} item. You are not out of stock", self.item)
        }
    }
    pub fn go_to_discontinued(&mut self) {
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.go_to_discontinued());
            println!("We are in discontinued state")
        }
    }
    pub fn go_to_instock(&mut self) {
        if self.display_item() > 0 {
            if let Some(s) = self.current_state.take() {
                self.current_state = Some(s.go_to_instock());
                println!("We are in instock state")
            }
        } else {
            println!("You have out of stock, add some item");
        }
    }
}
trait State {
    fn go_to_outofstock(self: Box<Self>) -> Box<dyn State>;
    fn go_to_discontinued(self: Box<Self>) -> Box<dyn State>;
    fn go_to_instock(self: Box<Self>) -> Box<dyn State>;
}
struct InStock;
impl State for InStock {
    fn go_to_outofstock(self: Box<Self>) -> Box<dyn State> {
        Box::new(OutOfStock)
    }
    fn go_to_discontinued(self: Box<Self>) -> Box<dyn State> {
        Box::new(Discontiued)
    }
    fn go_to_instock(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct OutOfStock;
impl State for OutOfStock {
    fn go_to_outofstock(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn go_to_discontinued(self: Box<Self>) -> Box<dyn State> {
        Box::new(Discontiued)
    }
    fn go_to_instock(self: Box<Self>) -> Box<dyn State> {
        Box::new(InStock)
    }
}
struct Discontiued;

impl State for Discontiued {
    fn go_to_outofstock(self: Box<Self>) -> Box<dyn State> {
        Box::new(OutOfStock)
    }
    fn go_to_discontinued(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn go_to_instock(self: Box<Self>) -> Box<dyn State> {
        Box::new(InStock)
    }
}
