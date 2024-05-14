use std::{sync::{Mutex, Arc}, any::Any, net::TcpStream, io::{Read, Write}};
use deku::{DekuContainerRead, DekuContainerWrite};
use fyrox::{gui::core::reflect::FieldInfo, core::{reflect::Reflect, visitor::Visit}};
use crate::network::messages::{PlayerInput, PlayerOutput};

#[derive(Debug)]
pub struct NetworkPlayer {
    stream: Option<Arc<Mutex<TcpStream>>>
}

impl Clone for NetworkPlayer {
    fn clone(&self) -> Self {
        Self {
            stream: None
        }
    }
}

impl Default for NetworkPlayer {
    fn default() -> Self {
        Self {
            stream: None
        }
    }
}

impl Reflect for NetworkPlayer {
    fn type_name(&self) -> &'static str {
        "NetworkPlayer".into()
    }
    fn doc(&self) -> &'static str {
        "fodasekk"
    }
    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo<'_, '_>])) {}
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {

    }
    fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {

    }
    fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {

    }
    fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {

    }
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        Ok(value)
    }
}

impl Visit for NetworkPlayer {
    fn visit(&mut self, name: &str, visitor: &mut fyrox::core::visitor::prelude::Visitor) -> fyrox::core::visitor::prelude::VisitResult {
        Ok(())
    }
}

impl NetworkPlayer {
    pub fn new(stream: Arc<Mutex<TcpStream>>) -> Self {
        Self {
            stream: Some(stream)
        }
    }
    pub fn try_read_stream_input(&mut self) -> Option<PlayerInput> {
        let mut buf = [0; 1024];
        if let Ok(mut stream) = self.stream.as_mut().unwrap().lock() {
            if stream.read(&mut buf).is_err() {
                return None;
            }
        }
        match PlayerInput::from_bytes((&buf, 0)) {
            Ok((_, input)) => Some(input),
            _ => None
        }
    }

    pub fn try_write_stream_output(&mut self, output: PlayerOutput) {
        let bytes = output.to_bytes().unwrap();
        if let Ok(mut stream) = self.stream.as_mut().unwrap().lock() {
            stream.write(&bytes).unwrap();
        }
    }
}
