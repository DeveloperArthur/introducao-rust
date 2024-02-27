/* macros são uma forma de escrever código que escreve outro código,
 são uma forma de encapsular lógica.

 vimos como podemos usar a vec! macro para criar um novo vetor com valores
 específicos. Por exemplo, a macro a seguir cria um novo vetor contendo
 três inteiros: */

fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
}

 /* Também poderíamos usar a vec!macro para criar um vetor de dois
 inteiros ou um vetor de cinco fatias de string. Não seríamos capazes
 de usar uma função para fazer o mesmo porque não saberíamos o número
 ou tipo de valores antecipadamente.

 Para definir uma macro, você usa a construção macro_rules!.
 Veja como a macro vec! é definida: */

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}