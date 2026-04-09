
use std::collections::HashMap;

fn main() {
    let mut produtos: HashMap<String, String> = HashMap::new();

    // Inserindo produtos
    produtos.insert("notebook".to_string(), "Dell".to_string());
    produtos.insert("celular".to_string(), "Samsung".to_string());
    produtos.insert("tv".to_string(), "LG".to_string());

    // Busca de produto
    buscar_produto(&produtos, "notebook");
    buscar_produto(&produtos, "celular");
    buscar_produto(&produtos, "tablet"); // não existe
}

// Função de busca
fn buscar_produto(produtos: &HashMap<String, String>, nome: &str) {
    match produtos.get(nome) {
        Some(marca) => println!("Produto encontrado: {} - Marca: {}", nome, marca),
        None => println!("Produto '{}' não encontrado.", nome),
    }
}Melhoria no sistea de busca 
