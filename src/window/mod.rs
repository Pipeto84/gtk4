mod imp;
use std::fs::File;
use glib::{clone, Object};
use gio::Settings;
use gtk::{gio,glib,Align,CheckButton,NoSelection,CustomFilter,FilterListModel};
use adw::{prelude::*,subclass::prelude::*,ActionRow};
use crate::task_object::{TaskObject, TaskData};
use crate::utils::data_path;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
    fn setup_settings(&self) {
        let settings=Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("setup_settings debe estar primero que settings");
    }
    fn settings(&self)->&Settings {
        self.imp()
            .settings
            .get()
            .expect("settings esta configurada en setup_settings")
    }
    fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }
    fn filter(&self)->Option<CustomFilter> {
        let filter_state:String=self.settings().get("filter");

        let filter_open=CustomFilter::new(|obj|{
            let task_object=obj
                .downcast_ref::<TaskObject>()
                .expect("tiene que ser TaskObject");
            !task_object.is_completed()
        });
        let filter_done=CustomFilter::new(|obj|{
            let task_object=obj
                .downcast_ref::<TaskObject>()
                .expect("tiene que ser TaskObject");
            task_object.is_completed()
        });
        match filter_state.as_str() {
            "All"=>None,
            "Open"=>Some(filter_open),
            "Done"=>Some(filter_done),
            _ =>unreachable!(),
        }
    }
    fn setup_tasks(&self) {
        let model = gio::ListStore::new::<TaskObject>();
        self.imp().tasks.replace(Some(model));

        let filter_model=FilterListModel::new(Some(self.tasks()),self.filter());
        let selection_model = NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.bind_model(
            Some(&selection_model), 
            clone!(@weak self as window=>@default-panic,move|obj|{
                let task_object=obj.downcast_ref().expect("el objeto tiene que ser TaskObject");
                let row=window.create_task_row(task_object);
                row.upcast()
            })
        );

        self.settings().connect_changed(
            Some("filter"), 
            clone!(@weak self as window,@weak filter_model=>move|_,_|{
                filter_model.set_filter(window.filter().as_ref());
            }),
        );

        self.set_task_list_visible(&self.tasks());
        self.tasks().connect_items_changed(
            clone!(@weak self as window =>move|tasks,_,_,_|{
                window.set_task_list_visible(tasks);
            })
        );
    }
    fn set_task_list_visible(&self,tasks:&gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }
    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            let backup_data:Vec<TaskData>=serde_json::from_reader(file)
                .expect("Deberia ser posible leer backup_data del archivo json");
            let task_objects:Vec<TaskObject>=backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();
            self.tasks().extend_from_slice(&task_objects);
        }
    }
    fn setup_callbacks(&self) {
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));
        self.imp()
            .entry
            .connect_icon_release( clone!(@weak self as window => move |_,_| {
                window.new_task();
            }));
    }
    fn new_task(&self) {
        let buffer = self.imp().entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");
        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }
    fn setup_actions(&self) {
        let action_filter=self.settings().create_action("filter");
        self.add_action(&action_filter);

        let action_remove_done_tasks=
            gio::SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(clone!(@weak self as window=>move|_,_|{
            let tasks=window.tasks();
            let mut position=0;
            while let Some(item) = tasks.item(position) {
                let task_object=item
                    .downcast_ref::<TaskObject>()
                    .expect("tiene que ser TaskObject");
                if task_object.is_completed() {
                    tasks.remove(position);
                }else {
                    position += 1;
                }
            }
        }));
        self.add_action(&action_remove_done_tasks);
    }
    fn create_task_row(&self,task_object:&TaskObject)->ActionRow {
        let checkbutton=CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let row=ActionRow::builder()
            .activatable_widget(&checkbutton)
            .build();
        row.add_prefix(&checkbutton);

        task_object
            .bind_property("completed", &checkbutton, "active")
            .bidirectional()
            .sync_create()
            .build();
        task_object
            .bind_property("content", &row, "title")
            .sync_create()
            .build();
        row
    }
}