use hashbrown::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use super::request::Request;
use super::response::Response;

pub trait Process {
    fn process(request: Request) -> Response;
}

pub type Storage = HashMap<String, String>;

#[allow(dead_code)]
pub type UnSharableStorage = Storage;

#[allow(dead_code)]
pub fn new_unsharable() -> UnSharableStorage {
    Default::default()
}

#[allow(dead_code)]
pub type SharableStorage = Rc<RefCell<Storage>>;

#[allow(dead_code)]
pub fn new_sharable() -> SharableStorage {
    Rc::new(RefCell::new(Default::default()))
}
