use serde::Deserialize;
// puxando biblioteca de
// manipulação de arquivos Json

// aqui mandamos o compilador do Rust
// implementar automaticamente a função
// de deserializar para essa struct
#[derive(Deserialize)]
pub struct PalavraAleatoria {
    sense: i32,
    wid  : i32,
    pub word : String
} // struct PalavraAleatoria
