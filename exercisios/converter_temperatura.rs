const FATOR_DE_CONVERSAO: f64 = 1.8;
const AJUSTE_ADICIONAL: i8 = 32;

fn main() {
    println!("{} °F", converte_fahrenheit_para_celsius(0));
    println!("{} °C", converte_celsius_para_fahrenheit(113));
}

fn converte_fahrenheit_para_celsius(f: i8) -> f64 { 
    ((f as f64) * FATOR_DE_CONVERSAO) + (AJUSTE_ADICIONAL as f64)
}

fn converte_celsius_para_fahrenheit(c: i8) -> f64 {
    ((c as f64) - (AJUSTE_ADICIONAL as f64)) / FATOR_DE_CONVERSAO
}