use gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Dialog};


fn main() {

    let application = gtk::Application::new(Some("com.silverpaw.dodecahedral.space"), Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();


}



fn build_ui(application: &gtk::Application) {
        let builder = Builder::from_string(include_str!("../glade/login-modal.ui"));


        let window: Dialog = builder.object("login").expect("Couldn't initialize login window.");
        window.set_application(Some(application));


        window.show_all();




}
