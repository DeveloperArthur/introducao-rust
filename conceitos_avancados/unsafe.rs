fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (v1, v2) = split_at_mut(&mut v, 3);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..]) //ERRO: cannot borrow `*values` as mutable more than once at a time
}

/* O verificador de empréstimo de Rust não consegue entender que estamos
emprestando diferentes partes da fatia; ele só sabe que estamos pegando
emprestado duas vezes da mesma fatia. Pegar emprestado partes diferentes
de uma fatia é fundamentalmente correto porque as duas fatias não estão
sobrepostas, mas Rust não é inteligente o suficiente para saber disso.

Quando sabemos que o código está correto, mas o Rust não, é hora de buscar
códigos inseguros: */

use std::slice;

fn split_at_mut_unsafe(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/* Este é um uso aceitável e apropriado de unsafe

 Com unsafe você pode abrir mão da segurança garantida em troca de maior
 desempenho ou da capacidade de interagir com outra linguagem, como
 a interface com código C, ou hardware onde as garantias do Rust
 não se aplicam.

 Outra razão pela qual Rust tem um alter ego inseguro é que o hardware
 do computador subjacente é inerentemente inseguro.
 Se Rust não permitisse que você realizasse operações inseguras, você
 não poderia realizar determinadas tarefas. Rust precisa permitir que
 você faça programação de sistemas de baixo nível, como interagir
 diretamente com o sistema operacional ou até mesmo escrever seu
 próprio sistema operacional. */