mod imp;

use glib::{clone, Object};
use gio::Settings;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory,prelude::*,subclass::prelude::*,ListItem,
        CustomFilter,FilterListModel};
use crate::task_object::TaskObject;
use crate::task_row::TaskRow;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl Window {
    pub fn new(app: &Application) -> Self {
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
        self.imp().tasks_list.set_model(Some(&selection_model));

        self.settings().connect_changed(
            Some("filter"), 
            clone!(@weak self as window,@weak filter_model=>move|_,_|{
                filter_model.set_filter(window.filter().as_ref());
            }),
        );
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
    fn setup_factory(&self) {
        let factory=SignalListItemFactory::new();
        factory.connect_setup(move|_,list_item|{
            let task_row=TaskRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .set_child(Some(&task_row));
        });
        factory.connect_bind(move|_,list_item|{
            let task_object=list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .item()
                .and_downcast::<TaskObject>()
                .expect("tiene que ser TaskObject");
            let task_row=list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("tiene que ser TaskRow");
            task_row.bind(&task_object);
        });
        factory.connect_unbind(move|_,list_item|{
            let task_row=list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("tiene que ser TaskRow");
            task_row.unbind();
        });
        self.imp().tasks_list.set_factory(Some(&factory));
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
}