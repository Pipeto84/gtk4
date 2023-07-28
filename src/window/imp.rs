use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use crate::custom_button::CustomButton;
// use std::cell::Cell;

#[derive(CompositeTemplate,Default)]
#[template(resource="/org/gtk_rs/example/window.ui")]
pub struct Window{
    #[template_child]
    pub button1: TemplateChild<CustomButton>,
    #[template_child]
    pub button2: TemplateChild<CustomButton>,
    // pub number: Cell<i32>,
}
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }
    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button1_clicked(button:&CustomButton) {
        button.set_label("hola pipeto");
    }
    #[template_callback]
    fn handle_button2_clicked(button:&CustomButton) {
        button.set_label("hola pipe");
    }
}
impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}