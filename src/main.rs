use gtk;
use gtk::prelude::*;
use gdk;
use poglo::ThreadPool;
use gtk::{Button, Builder, Entry, Grid, Widget, Box, Adjustment, Dialog, StyleContext, CssProvider, ApplicationWindow, ScrolledWindow};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::{BufReader};

fn main() {

    let application = gtk::Application::new(Some("com.silverpaw.dodecahedral.space"), Default::default(),
    );

    application.connect_activate(build_ui);


    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Bound!");
    pool.execute(move || -> (){
    for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
            //pool.execute(|| -> (){
                handle_connection(stream);

            //});
    }
    });

    application.run();

}

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        println!("Request {:#?}", http_request);
    let response = "HTTP/1.1 200 OK \r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn build_ui(application: &gtk::Application) {
        let builder = Builder::from_string(include_str!("../glade/login-modal.ui"));


        let main_window: ApplicationWindow = builder.object::<ApplicationWindow>("main").expect("Couldn't initialize main window");
        let window: Dialog = builder.object::<Dialog>("login").expect("Couldn't initialize login window.");
        window.set_application(Some(application));
        main_window.set_application(Some(application));

        let jackon: Button = builder.object::<Button>("connection").expect("Couldnd't get login button widget.");
        let butt: Button = builder.object::<Button>("exit-modal").expect("Couldn't get exit button widget");
        let provider = CssProvider::new();
        let style = include_bytes!("../glade/style.css");
        provider.load_from_data(style).expect("Failed loading style data.");

        //let entry_0: Entry = Entry::new();

        let entry_0: Entry = builder.object::<Entry>("entry_0").expect("Couldn't get first entry.");

        let entry_box: Box = builder.object::<Box>("row_1").expect("Couldn't find the box!.");
        entry_0.set_placeholder_text(Some("Enter your message..."));
        entry_box.pack_end(&entry_0, false, false, 0);

        entry_0.connect_activate(move |entry_0| -> () {
            let text = entry_0.text();



            let new_entry: Entry = Entry::new();
            new_entry.set_text(&text);
            entry_box.pack_start(&new_entry, true, true, 0);
            new_entry.show();
            {
                entry_0.set_text("");
                let scrolled_window: ScrolledWindow = builder.object::<ScrolledWindow>("scrolling_window").expect("Couldn't get the scrolled window!");
                let vadje: Adjustment = scrolled_window.vadjustment();
                println!("{:?}",vadje.upper());
                let vaadje: Adjustment = Adjustment::new(vadje.step_increment()+vadje.upper(), vadje.lower(), vadje.upper(), vadje.step_increment(), vadje.page_increment(), vadje.page_size());
                scrolled_window.set_vadjustment(Some(&vaadje));
                scrolled_window.show_all();
            }
            entry_box.show_all();
        });
        window.show_all();

        //Set the exit button to close the application
        //and gracefully shut down the program
        jackon.connect_clicked(move |_| -> () {
            window.close();
            main_window.show_all();
        });

        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().expect("Failed initializing gdk."),
        &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        


}
