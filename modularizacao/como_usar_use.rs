/*pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
 
Como você pode ver acima, referir-se ao nome totalmente qualificado 
pode ficar bastante longo. Felizmente, Rust tem uma palavra-chave 
para tornar estas chamadas mais concisas.

ASSIM:
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("it work!");
            }
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}

OU ASSIM: */
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("it work!");
            }
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}