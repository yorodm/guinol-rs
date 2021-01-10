use crate::types::{By, Relay, TransportError};
use serde::{Deserialize, Serialize};

pub struct Point {
    x: f32,
    y: f32,
}

pub struct Size {
    width: f64,
    height: f64,
}

pub struct WindowRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub struct ElementRect {
    point: Point,
    size: Size,
}

pub struct WebElement<T: Relay> {
    id: String,
    transport: T,
}

impl<'de, T: Relay> Deserialize<'de> for WebElement<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl<T: Relay> Serialize for WebElement<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

pub trait Finder<T: Relay>{
	fn findElement(&self, by: By, value: String) -> Result<WebElement<T>, TransportError> ;

    fn findElements(
        &self,
        by: By,
        value: String,
    ) -> Result<Vec<WebElement<T>>, TransportError>;
}

impl <T: Relay> Finder<T> for WebElement<T> {
    fn findElement(&self, by: By, value: String) -> Result<WebElement<T>, TransportError>  {
        todo!()
    }

    fn findElements(
        &self,
        by: By,
        value: String,
    ) -> Result<Vec<WebElement<T>>, TransportError> {
        todo!()
    }
}

impl<T: Relay> WebElement<T> {
    pub fn id(&self) -> String {
        todo!();
    }

    pub fn enabled(&self) -> bool {
        todo!()
    }

    pub fn selected(&self) -> bool {
        todo!()
    }

    pub fn tag_name(&self) -> String {
        todo!()
    }

    pub fn text(&self) -> String {
        todo!()
    }

    pub fn attribute(&self, name: String) -> String {
        todo!()
    }

    pub fn css_value(&self, property: String) -> String {
        todo!()
    }

    pub fn rect(&self, property: String) -> Result<ElementRect, TransportError> {
        todo!()
    }

    pub fn click(&self) {
        todo!()
    }

    pub fn send_keys(&self, keys: String) -> Result<(), TransportError> {
        todo!()
    }

    pub fn clear(&self) {
        todo!()
    }

    pub fn location(&self) -> Result<Point, TransportError> {
        todo!()
    }

    pub fn size(&self) -> Result<Size, TransportError> {
        todo!()
    }

    pub fn screenshot(&self) -> Result<String, TransportError> {
        todo!()
    }
}
