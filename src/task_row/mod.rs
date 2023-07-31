mod imp;

use gtk::{prelude::*,subclass::prelude::*,glib,pango};
use glib::Object;
use pango::{AttrInt,AttrList};
use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
        @extends gtk::Box,gtk::Widget,
        @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;
}