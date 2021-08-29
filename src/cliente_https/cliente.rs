use hyper::http::Result as HttpResult;
use hyper::http::Error  as HttpError;
use std::result::Result as StdResult;
use hyper::Request;
use hyper::Method;
use hyper::Body;
use hyper::Uri;


pub struct Cliente<'a> {
    dominio: &'a str
} // struct Cliente


impl<'a> Cliente<'a> {
    pub fn novo(dominio: &'a str) -> Self {
        Cliente { dominio }
    } // novo


    pub fn get(&self, caminho: &'a str) -> String {
        self.montar_url(caminho)
            .map(montar_requisicao)
            .map(validar_requisicao);
        "a".to_string()
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


fn montar_requisicao(url: Uri)
    -> HttpResult<Request<Body>>
{
    Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(Body::from(r#"{"library":"hyper"}"#))
} // montar_requisicao


fn validar_requisicao(requisicao: HttpResult<Request<Body>>)
    -> String
{
    match requisicao {
        Ok(req) => fazer_requisicao(req) ,
        Err(_)  => String::new() }
} // validar_requisicao
