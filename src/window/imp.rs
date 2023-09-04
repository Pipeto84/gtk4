use std::{cell::{RefCell,OnceCell},fs::File};

use gtk::{gio,glib,prelude::*,subclass::prelude::*,CompositeTemplate, Entry, ListView};
use glib::subclass::InitializingObject;
use gio::Settings;

use crate::task_object::{TaskData, TaskObject};
use crate::utils::data_path;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/Todo1/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<gio::ListStore>>,
    pub settings:OnceCell<Settings>,
}
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }
    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.setup_tasks();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_factory();
        obj.setup_actions();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        let backup_data:Vec<TaskData>=self
            .obj()
            .tasks()
            .iter::<TaskObject>()
            .filter_map(Result::ok)
            .map(|task_object|task_object.task_data())
            .collect();

        let file=File::create(data_path()).expect("no se pudo crear el json");
        serde_json::to_writer(file, &backup_data)
            .expect("no pudo escribir los datos en el archivo json");

        self.parent_close_request()
    }
}
impl ApplicationWindowImpl for Window {}