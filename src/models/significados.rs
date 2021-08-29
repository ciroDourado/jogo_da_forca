use serde::Deserialize;
// puxando biblioteca de
// manipulação de arquivos Json

// aqui mandamos o compilador do Rust
// implementar automaticamente a função
// de deserializar para essa struct
#[derive(Deserialize)]
pub struct Significado {
    creator      : Option<String>,
    deleted      : Option<i32>,
    deletor      : Option<String>,
    derived_from : Option<String>,
    last_revision: Option<i32>,
    moderator    : Option<String>,
    normalized   : Option<String>,
    revision_id  : Option<i32>,
    sense        : Option<i32>,
    timestamp    : Option<String>,
    pub word     : String,
    word_id      : Option<i32>,
    pub xml      : String
} // struct Significado

// um conjunto de Significados nada mais
// é do que uma lista/vetor de Significado
type Significados = Vec<Significado>;
