use testes_somador;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, testes_somador::add_two(2));
}