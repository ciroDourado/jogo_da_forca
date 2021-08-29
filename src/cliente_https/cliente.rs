use hyper::http::Error  as HttpError;
use std::result::Result as StdResult;
use hyper::Uri;

use super::requisicao::montar_requisicao;
use super::requisicao::fazer_requisicao;


pub struct Cliente<'a> {
    dominio: &'a str
} // struct Cliente


impl<'a> Cliente<'a> {
    pub fn conectar_no(dominio: &'a str) -> Self {
        Cliente { dominio }
    } // conectar_no


    pub fn get(&self, caminho: &'a str) -> String {
        self.montar_url(caminho)
            .and_then(montar_requisicao)
            .map     (fazer_requisicao)
            .unwrap_or(String::new())
    } // get


    fn montar_url(&self, caminho: &'a str)
        -> StdResult<Uri, HttpError>
    {
        Uri::builder()
            .scheme("https")
            .authority(self.dominio)
            .path_and_query(caminho)
            .build()
    } // montar_url

} // impl Cliente
