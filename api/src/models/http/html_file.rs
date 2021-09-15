use std::io::Read;

use rocket::response::{self, Responder, Stream};
use rocket::Request;

pub struct HtmlFile<T>
    where
        T: Read,
{
    pub content: T,
}

impl<'r, T> Responder<'r> for HtmlFile<T>
    where
        T: Read + 'r,
{
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let mut content_response = Stream::chunked(self.content, 256).respond_to(request)?;
        content_response.set_raw_header("Content-Type", "text/html; charset=UTF-8");

        response::Result::Ok(content_response)
    }
}
