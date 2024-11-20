#[derive(Debug)]
struct Triangle {
    base : u32,
    heigth : u32
}

impl Triangle {

    fn get_area(&self) -> u32{
        (self.base * self.heigth) / 2
    }

    fn can_hold(&self, triangle : &Triangle) -> bool {
        self.get_area() > triangle.get_area()
    }
}

fn main() {
    
    let triangle_a = Triangle {
        base : 40,
        heigth : 20
    };

    let triangle_b = Triangle {
        base : 20,
        heigth : 30
    };

    let area = dbg!(triangle_a.get_area());

    println!("A área do triângulo é {}", area);
    println!("O triângulo B consegue ter dentro de si o triângulo A? {}", 
    triangle_b.can_hold(&triangle_a))
}