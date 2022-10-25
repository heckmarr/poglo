use gtk;
use gtk::prelude::*;
use gdk;
use gtk::{Button, Builder, Widget, Dialog, StyleContext, CssProvider, ApplicationWindow};

fn main() {

    let application = gtk::Application::new(Some("com.silverpaw.dodecahedral.space"), Default::default(),
    );

    application.connect_activate(build_ui);



    application.run();


}



fn build_ui(application: &gtk::Application) {
        let builder = Builder::from_string(include_str!("../glade/login-modal.ui"));


        let main_window: ApplicationWindow = builder.object::<ApplicationWindow>("main").expect("Couldn't initialize main window");
        let window: Dialog = builder.object::<Dialog>("login").expect("Couldn't initialize login window.");
        window.set_application(Some(application));
        main_window.set_application(Some(application));
        //let butt: Button = builder.object::<Button>("exit-modal").expect("Couldn't get button widget");
        let provider = CssProvider::new();
        let style = include_bytes!("../glade/style.css");
        provider.load_from_data(style).expect("Failed loading style data.");

        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().expect("Failed initializing gdk."),
        &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,);

        window.show_all();
        main_window.show_all();



}
