// criando meu proprio smart pointer
struct MeuBox<T>(T); // <- struct-tupla de 1 elemento do tipo T

impl<T> MeuBox<T> {
    fn new(x: T) -> MeuBox<T> {
        MeuBox(x)
    }
}

// tentando desreferenciar MeuBox:
fn main4() {
    let x = 5;
    let y = MeuBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // type `MeuBox<{integer}>` cannot be dereferenced
}

/* Nosso tipo MeuBox<T> não pode ser desreferenciado porque
não implementamos essa habilidade nele.
Para habilitar desreferenciamento com o operador *, temos que
implementar a trait Deref. */

use std::ops::Deref;

impl<T> Deref for MeuBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 //para que deref retorne uma referência ao valor que queremos acessar com o operador *
    }
}

/* Sem a trait Deref, o compilador só consegue desreferenciar referências &.
O método deref dá ao compilador a habilidade de tomar um valor de qualquer
tipo que implemente Deref e chamar o método deref para pegar uma referência
&, que ele sabe como desreferenciar.

Quando entramos *y na Listagem 15-9, por trás dos panos o Rust na verdade
rodou este código: *(y.deref()) */

/* O Rust substitui o operador * com uma chamada ao método deref e em
seguida uma desreferência comum, de modo que nós programadores não
precisamos pensar sobre se temos ou não que chamar o método deref */