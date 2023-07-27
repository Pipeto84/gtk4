mod window;
use window::Window;
mod custom_button;
use gtk::prelude::*;
use gtk::{glib,gio, Application};
const APP_ID: &str = "org.gtk_rs.pipeto";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("pipeto.gresource")
        .expect("fallo el registro de recursos");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let window=Window::new(app);
    window.present();
}