use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::ToString;

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    proxyType: String,
    proxyAutoconfigUrl: String,
    ftpProxy: String,
    httpProxy: String,
    noProxy: String,
    sslProxy: String,
    socksProxy: String,
    socksVersion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    browserName: String,
    browserVersion: String,
    platformName: String,
    platformVersion: String,
    specificationLevel: u16,
    raisesAccesibilityExceptions: bool,
    rotatable: bool,
    acceptSsslCerts: bool,
    takesElementScreenshot: bool,
    takesScreenshot: bool,
    proxy: Proxy,
    platform: String,
    xulAppId: String,
    device: String,
    version: String,
    command_id: u32,
}

pub enum By {
    Id,
    Name,
    ClassName,
    TagName,
    CssSelector,
    LinkText,
    PartialLinkText,
    Xpath,
    Anon,
    AnonAttribute,
}

impl ToString for By {
    fn to_string(&self) -> String {
        let res = match self {
            By::Id => "id",
            By::Name => "name",
            By::ClassName => "class name",
            By::TagName => "tag name",
            By::CssSelector => "css selector",
            By::LinkText => "link text",
            By::PartialLinkText => "partial link text",
            By::Xpath => "xpath",
            By::Anon => "anon",
            By::AnonAttribute => "anon attribute",
        };
        res.to_owned()
    }
}

pub enum Context {
    Chrome,
    Content,
}

impl ToString for Context {
    fn to_string(&self) -> String {
        let res = match self {
            Context::Chrome => "chrome",
            Context::Content => "content",
        };
        res.to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    secure: bool,
    expiry: u16,
    domain: String,
    httpOnly: bool,
    name: String,
    path: String,
    value: String,
}

// TODO: this should implement error?
#[derive(Debug, Serialize, Deserialize)]
pub struct DriverError {
    error: String,
    message: String,
    stacktrace: String,
}

pub enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}

pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    message_id: i32,
    size: i32,
    value: String,
    error: DriverError,
}

pub struct TransportError;

pub trait Transport {
    fn message_id(&self) -> i16;
    fn close(self) -> Result<(), TransportError>;
    fn send(&mut self, command: String, data: Value) -> Result<Response, TransportError>;
    fn receive(&mut self) -> Result<Vec<u8>, TransportError>;
}

pub trait Relay {
    fn send(&mut self, command: String, data: Value);
}

pub struct DecoderError;
pub struct EncoderError;

pub trait Decoder {
    fn decode(& self, buf: Vec<u8>) -> Result<Response, DecoderError>;
}

pub trait Encoder {
    fn encode(
        &self,
        t: impl Transport,
        command: String,
        value: Value,
    ) -> Result<Vec<u8>, EncoderError>;
}
