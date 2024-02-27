/* Imagina que queremos uma função que recebe um vetor de numeros
 e retorna o maior, e uma função que recebe um vetor de char
 e retorna o maior, teriamos basicamente 2 funções com o mesmo
 corpo, mas que recebem dados diferentes, uma forma de evitar
 isso seria usando generics: */

fn maior<T>(lista: &[T]) -> T {
    let mut maior = lista[0];

    for &item in lista.iter() {
        if item > maior { // ERROR: binary operation `>` cannot be applied to type `T`
            maior = item;
        }
    }

    maior
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let resultado = maior(&lista_numero);
    println!("The maior number is {}", resultado);

    let lista_char = vec!['y', 'm', 'a', 'q'];

    let resultado = maior(&lista_char);
    println!("O maior char e {}", resultado);
}
//--------------------------------------------------------------------------------------------
//Consertando a Função maior com Limites de Traits

/* Para que possamos usar o operador maior-que, precisamos especificar
PartialOrd nos limites do trait para T para que a função maior funcione
em partes de qualquer tipo que possa ser comparada

Estamos restringindo nossa função para funcionar em qualquer tipo
genérico que implementa os traits PartialOrd e Copy, ou seja
estamos limitando os traits */
fn maior<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut maior = lista[0]; //(O tipo de list precisa implementar Copy se não vai quebrar aqui)

    for &item in lista.iter() {
        if item > maior { //(O tipo de list precisa implementar PartialOrd se não vai quebrar aqui)
            maior = item;
        }
    }

    maior
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let resultado = maior(&lista_numero);
    println!("The maior number is {}", resultado);

    let lista_char = vec!['y', 'm', 'a', 'q'];

    let resultado = maior(&lista_char);
    println!("O maior char e {}", resultado);
}

/* Traits e limites de traits nos deixam escrever código que usam
parâmetros de tipos genéricos para reduzir a duplicação, mas ainda
sim especificam para o compilador exatamente qual o comportamento
que nosso código precisa que o tipo genérico tenha. */

//--------------------------------------------------------------------------------------------
//Usando Tipos de Dados Genéros em Definições de Structs

struct Ponto<T> {
    x: T,
    y: T,
}

fn main() {
    let nao_funciona = Ponto { x: 5, y: 4.0 };
}
/* Quando atribuímos o valor de 5 para x, o compilador sabe que
para essa instância de Ponto o tipo genérico T será um número
inteiro. Então quando especificamos 4.0 para y, o qual é definido
para ter o mesmo tipo de x, nós temos um tipo de erro de incompatibilidade.

Se nós quisermos definir um struct de Ponto onde x e y têm tipos
diferentes e quisermos fazer com que esses tipos sejam genéricos,
nós podemos usar parâmetros múltiplos de tipos genéricos.

Abaixo mudamos a definição do Ponto para os tipos genéricos T e U.
O campo x é do tipo T, e o campo y do tipo U: */

struct Ponto<T, U> {
    x: T,
    y: U,
}

fn main() {
    let ambos_inteiros = Ponto {x: 5, y: 10};
    let ambos_floats = Ponto {x: 1.0, y: 4.0};
    let inteiro_e_float = Ponto {x: 5, y: 4.0};
}

/* Agora todos as instâncias de Ponto são permitidas! */

//--------------------------------------------------------------------------------------------
//Usando Tipos Genéricos de Dados em Definições de Métodos

/* definimos um método chamado x no Ponto<T> que retorna a
referência para o dado no campo x */

struct Ponto<T> {
    x: T,
    y: T,
}

impl<T> Ponto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/*  bloco de impl que só se aplica a uma struct com o
tipo específico usado pelo parâmetro de tipo genérico T */
impl Ponto<f32> {
    fn distancia_da_origem(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Ponto { x: 5, y: 10 };
    let t = Ponto { x: 5.0, y: 10.0 };

    println!("p.x = {}", p.x()); // x() funciona para todos
    println!("t.x = {}", t.x());

    // println!("p.distancia_da_origem = {}", p.distancia_da_origem()); ERRO method not found in `Ponto<{integer}>`

    println!("t.distancia_da_origem = {}", t.distancia_da_origem()); // funciona
}

//--------------------------------------------------------------------------------------------
/* Definindo um método mistura na estrutura Ponto<T, U>
O método recebe outro Ponto como parâmetro, que pode ter tipos
diferentes de self Ponto dos quais usamos no mistura.

O método cria uma nova instância de Ponto que possui o valor x
de self Ponto (que é um tipo de T) e o valor de y passado de
Ponto (que é do tipo W) */

struct Ponto<T, U> {
    x: T,
    y: U,
}

impl<T, U> Ponto<T, U> {
    fn mistura<V, W>(self, other: Ponto<V, W>) -> Ponto<T, W> {
        Ponto {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Ponto { x: 5, y: 10.4 };
    let p2 = Ponto { x: "Ola", y: 'c'};

    let p3 = p1.mistura(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}