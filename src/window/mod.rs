mod imp;
use gtk::{gio,glib,Application};
use glib::Object;


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Window,gtk::ApplicationWindow,gtk::Widget,
        @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Native,
                    gtk::Root,gtk::ShortcutManager,gio::ActionGroup,gio::ActionMap;
}
impl Window {
    pub fn new(app:&Application)->Self {
        Object::builder().property("application", app).build()
    }
}