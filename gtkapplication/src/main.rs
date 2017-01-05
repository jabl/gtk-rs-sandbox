extern crate gtk;
extern crate gio;

use gtk::prelude::*;

fn main() {

    let app = gtk::Application::new(Some("jabl.gtk-rs-sandbox.gtkapplication"),
                                    gio::APPLICATION_FLAGS_NONE).unwrap();

    app.connect_activate(move |app| {
        let widget = gtk::ApplicationWindow::new(&app);
        widget.show();
    });
                         
    let a: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = a.iter().map(|x| x.as_str()).collect();
    
    app.run(args.len() as i32, &args);


}

