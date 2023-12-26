use gui_lib::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {

    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };
    
    screen.run();
}