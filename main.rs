use std::collections::HashMap;

fn main() {
    let mut produtos = HashMap::new();

    produtos.insert("notebook", "Dell");
    produtos.insert("celular", "Samsung");
    produtos.insert("tv", "LG");

    println!("Busca por notebook: {:?}", produtos.get("notebook"));
}
