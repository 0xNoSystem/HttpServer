use std::hash::Hash;
use std::str::FromStr;
use crate::error::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Conn{
    Keep,
    ync fn index(session: Session) -> HttpResponse {
    let count = session.get::<i32>("counter").unwrap_or(Some(0)).unwrap_or(0);
    session.insert("counter", count + 1).unwrap();

    HttpResponse::Ok().body(format!("Counter: {}", count))
}Close,
    Custom(String)
}

impl FromStr for Conn{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s.trim(){
            "keep-alive" => Ok(Conn::KeepAlive),
            "close" => Ok(Conn::Close),
            other => Conn::Custom(other.to_string()),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum ContType {
    Json,
    Xml,
    FormUrlEncoded,
    MultipartFormData,
    Html,
    PlainText,
    Css,
    Javascript,
    OctetStream,
    Pdf,
    ImagePng,
    ImageJpeg,
    ImageWebp,
    Other(String),
}

impl ContType {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "application/json" => ContentType::Json,
            "application/xml" => ContentType::Xml,
            "application/x-www-form-urlencoded" => ContentType::FormUrlEncoded,
            "multipart/form-data" => ContentType::MultipartFormData,
            "text/html" => ContentType::Html,
            "text/plain" => ContentType::PlainText,
            "text/css" => ContentType::Css,
            "text/javascript" | "application/javascript" => ContentType::Javascript,
            "application/octet-stream" => ContentType::OctetStream,
            "application/pdf" => ContentType::Pdf,
            "image/png" => ContentType::ImagePng,
            "image/jpeg" => ContentType::ImageJpeg,
            "image/webp" => ContentType::ImageWebp,
            other => ContentType::Other(other.to_string()),
        }
    }
}






