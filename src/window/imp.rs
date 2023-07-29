use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use crate::custom_button::CustomButton;
use std::cell::Cell;

#[derive(CompositeTemplate,Default)]
#[template(resource="/org/gtk_rs/example/window.ui")]
pub struct Window{
    // #[template_child]
    // pub button1: TemplateChild<CustomButton>,
    pub number1: Cell<i32>,
    // #[template_child]
    // pub button2: TemplateChild<CustomButton>,
    pub number2: Cell<i32>,
}
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;
    fn class_init(klass: &mut Self::Class) {
        CustomButton::ensure_type();
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
    fn handle_button1_clicked(&self,button:&CustomButton) {
        let aumentar=self.number1.get()+1;
        self.number1.set(aumentar);
        button.set_label(&aumentar.to_string());
    }
    #[template_callback]
    fn handle_button2_clicked(&self,button:&CustomButton) {
        let aumentar=self.number2.get()+2;
        self.number2.set(aumentar);
        button.set_label(&aumentar.to_string());
    }
}
impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}