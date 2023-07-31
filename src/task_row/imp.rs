use std::cell::RefCell;
use gtk::{subclass::prelude::*,glib,CheckButton,CompositeTemplate,Label};
use glib::Binding;

#[derive(Default,CompositeTemplate)]
#[template(resource="/org/gtk_rs/example/task_row.ui")]
pub struct TaskRow{
    #[template_child]
    pub completed_button:TemplateChild<CheckButton>,
    #[template_child]
    pub content_label:TemplateChild<Label>,
    pub bindings:RefCell<Vec<Binding>>,
}
#[glib::object_subclass]
impl ObjectSubclass for TaskRow {
    const NAME: &'static str = "TodoTaskRow";
    type Type = super::TaskRow;
    type ParentType = gtk::Box;

    fn class_init(_klass: &mut Self::Class) {
        _klass.bind_template();
    }
    fn instance_init(_obj: &glib::subclass::InitializingObject<Self>) {
        _obj.init_template();
    }
}
impl ObjectImpl for TaskRow {}
impl BoxImpl for TaskRow {}
impl WidgetImpl for TaskRow {}