/* De vez em quando, nossa empresa de camisetas distribui uma
camisa exclusiva de edição limitada para alguém de nossa lista
de e-mails como promoção. As pessoas na lista de e-mail podem,
opcionalmente, adicionar sua cor favorita ao perfil.
Se a pessoa escolhida para a camisa grátis tiver sua cor preferida
definida, ela ganha a camisa dessa cor. Se a pessoa não especificou
uma cor favorita, ela obtém a cor que a empresa mais possui atualmente. */

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        /* Closure é uma função anônima que pode ser usada como
        argumento em Rust. No caso desta função, o closure
        || self.most_stocked() é usado.

        Isso significa que, se user_preference for None, o método
        most_stocked da própria instância será chamado para
        determinar a cor da camiseta a ser usada.

        Quando você define o closure || self.most_stocked(), ele
        captura uma referência imutável (&self) à instância de
        Inventory onde o método giveaway está sendo chamado.

        Isso significa que o closure tem acesso à instância de
        Inventory e pode chamar métodos nela, como most_stocked()

        O uso de closures permite capturar e usar implicitamente o
        self (a instância atual) de um objeto em um contexto onde
        uma função simples não poderia fazer isso, a mesma função
        sem closure ficaria assim:

        fn giveaway(inventory: &Inventory, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| inventory.most_stocked())
        }

        Teriamos que receber a instância atual por parâmetro, usar self
        só é possível graças ao closure */
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue,
                     ShirtColor::Red,
                     ShirtColor::Blue],
    };

    let user_pref1 = ShirtColor::Red;
    let giveaway1 = store.giveaway(Some(user_pref1));
    println!("The user with preference {:?} gets {:?}",
        user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",
             user_pref2, giveaway2);
}