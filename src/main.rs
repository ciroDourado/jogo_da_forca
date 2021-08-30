#![allow(dead_code)]
pub mod cliente_https;
pub mod models;

use cliente_https::cliente::Cliente;

use models::deserializacao::deserializar;
use models::palavra_aleatoria::PalavraAleatoria;
use models::significados::Significados;


#[tokio::main]
async fn main() -> () {
    let dominio = "api.dicionario-aberto.net";
    let cliente = Cliente::conectar_no(dominio);

    let json_parte1 = cliente.get("/random");
    let aleatoria   = deserializar::<PalavraAleatoria>(&json_parte1);
    let caminho     = vec!["word", &aleatoria.word];
    let buscada     = formatar_url( caminho );

    let json_parte2  = cliente.get(&buscada);
    let significados = deserializar::<Significados>(&json_parte2);
} // main


// código experimental!
fn formatar_url(partes: Vec<&str>) -> String {
    let cada_parte = partes.into_iter();

    let caminho = cada_parte
        .map(aplicar_codificacao)
        .collect::<TudoNumaLista>()
        .join("/");
    f!("/{caminho}/")
} // formatar_url


#[macro_use]
extern crate fstrings;
// apenas para facilitar a leitura
type TudoNumaLista = Vec<String>;


fn aplicar_codificacao(data: &str)
    -> String
{
    urlencoding::encode(data)
        .to_owned()
        .to_string()
} // aplicar_codificacao
