use hyper::http::Result as HttpResult;
use hyper::http::Error as HttpError;
use std::result::Result as StdResult;
use hyper::Request;
use hyper::Body;
use hyper::Uri;


pub struct Cliente {
    dominio: &str
} // struct Cliente


impl Cliente {
    pub fn novo(dominio: &str) -> Self {
        Cliente { dominio }
    } // novo


    pub fn get(&self, caminho: &str) -> String {
        self.montar_url(caminho)
            .map(montar_requisicao)
    } // get


    fn montar_url(&self, caminho: &str)
        -> StdResult<Uri, HttpError>
    {
        Uri::builder()
            .scheme("https")
            .authority(self.dominio)
            .path_and_query(caminho)
            .build()
    } // montar_url


    fn montar_requisicao(url: Uri)
        -> HttpResult<Request<Body>>
    {
        Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::from(r#"{"library":"hyper"}"#))
    } // request_build


    // fn fazer_request() -> i32
    //
    // {
    //     "a"
    // } // fazer_request


} // impl Cliente
