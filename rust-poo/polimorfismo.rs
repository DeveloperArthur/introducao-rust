pub trait Animal {
    fn forma_de_se_comunicar(&self) -> String;
}

pub struct Cachorro {
    nome: String,
    idade: i32
}

impl Animal for Cachorro {
    fn forma_de_se_comunicar(&self) -> String {
        String::from("au au")
    }
}

pub struct Gato {
    nome: String,
    idade: i32
}

impl Animal for Gato {
    fn forma_de_se_comunicar(&self) -> String {
        String::from("miau miau")
    }
}

fn main() {
    let gato = Gato {
        nome: String::from("Sofia"),
        idade: 7
    };
    recebe_animal(&gato);

    let cachorro = Cachorro {
        nome: String::from("costelinha"),
        idade: 1
    };
    recebe_animal(&cachorro);
}

fn recebe_animal(animal: &Animal) {
    println!("{}", animal.forma_de_se_comunicar());
}