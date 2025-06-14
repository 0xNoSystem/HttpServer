use std::collections::HashMap;
use std::str::FromStr;
use crate::error::Error;
use std::hash::Hash;
//use crate::types::*;


#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: HttpVersion,
    headers: HashMap<Header, Vec<String>>,
}

impl HttpRequest {
    pub fn parse(raw: &str) -> Result<Self, Error> {
        println!("PARSING");
        let mut lines = raw.split("\r\n");
        let first = lines.next().ok_or(Error::IncompleteRequest)?;
        let mut parts = first.split_whitespace();
        let method = parts.next().ok_or(Error::InvalidRequestLine)?.parse()?;
        let path = parts.next().ok_or(Error::InvalidRequestLine)?;
        let version = parts.next().ok_or(Error::InvalidRequestLine)?.parse()?;
        
        let mut headerMap = HashMap::new();

        while let Some(headers) = lines.next(){
            let mut line = headers.split_once(':');
            if let Some((header, value)) = line{
                let htype = Header::from_str(header).unwrap();
                headerMap
                    .entry(htype)
                    .or_insert_with(Vec::new)
                    .push(value.to_string());
            //return Err(Error::InvalidRequestLine);
            }
        }

        // Header parsing to be implemented
        Ok(HttpRequest {
            method,
            path: path.to_string(),
            version,
            headers: headerMap,
        })
    }
}


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Header{
    Connection,
    Host,
    ContentLength,
    ContentType,
    Other,
}


impl FromStr for Header{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>{

        let s_trimmed = s.trim().to_ascii_lowercase(); 
        let htype = match s_trimmed.as_str(){
            "connection" => Header::Connection,
            "host" => Header::Host,
            "content-length" => Header::ContentLength,
            "content-type" => Header::ContentType,
            _ => Header::Other,
    };
        Ok(htype)
    }
}





#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl FromStr for HttpMethod {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(Error::InvalidMethod),
        }
    }
}




#[derive(Debug, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
}

impl FromStr for HttpVersion {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2.0" => Ok(HttpVersion::Http20),
            _ => Err(Error::UnsupportedHttpVersion(s.to_string())),
        }
    }
}

