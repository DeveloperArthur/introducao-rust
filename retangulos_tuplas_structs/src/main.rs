//projetinho que calcula area de um retangulo:
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

//length e width estao relacionados
//nao deveriam ser recebimos sepadados no método
fn area(length: u32, width: u32) -> u32 {
    length * width
}

//------------------------------------------------------------------------
//mesmo projeto usando tuplas
fn main() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    //nao fica descritivo o que é 0 e 1
    dimensions.0 * dimensions.1
}

//------------------------------------------------------------------------
//mesmo projeto usando struct

//essa anotação é para poder imprimir certinho
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { 
        length: 50, 
        width: 30 
    };

    //{:?} é para poder imprimir a struct
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
//------------------------------------------------------------------------
//mesmo projeto mas com a funcao sendo pertencente da struct
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { 
        length: 50, 
        width: 30 
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
//------------------------------------------------------------------------
//Métodos com Mais Parametros + Funções associadas
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    //função associada (basicamente funcionam 
        //como os métodos estáticos)
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            length: size, 
            width: size 
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30
    };

    let rect2 = Rectangle {
        length: 40,
        width: 10
    };

    let rect3 = Rectangle {
        length: 45,
        width: 60
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //chamando função associada:
    let sq = Rectangle::square(3);

    // rect1.square(4); ERROR: this is an associated function, not a method
}