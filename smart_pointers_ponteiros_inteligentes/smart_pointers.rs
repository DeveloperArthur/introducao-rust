/* Nós definimos a variável b como tendo o valor de um Box
que aponta para o valor 5, que está alocado no heap.
 sse programa irá imprimir b = 5 */
fn main1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

/* como a desreferencia funciona */
fn main2() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/* desreferenciando smart pointers */
fn main3() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // FUNCIONA
}