use std::cell::RefCell;
use glib::{Properties,ParamSpec,Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use super::TaskData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::TaskObject)]
pub struct TaskObject {
    #[property(name = "completed", get, set, type = bool, member = completed)]
    #[property(name = "content", get, set, type = String, member = content)]
    pub data: RefCell<TaskData>,
}
#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
    type Type = super::TaskObject;
}
impl ObjectImpl for TaskObject {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }
    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }
    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }
}