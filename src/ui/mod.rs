pub fn gtk_main() {
    use gtk;
    use gtk::prelude::*;
    use gtk::{Builder, Button, Window};
    use mkdir;

    let _ = mkdir("AppContent");
    let _ = mkdir("AppContent/Logo");

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../glade/MainWindow.ui");
    let builder = Builder::new_from_string(glade_src);
    let window: Window = builder.get_object("MainWindow").unwrap();

    use controls::all_colored::*;
    let colored_all: Button = builder.get_object("AllColorsButton").expect("boned");
    colored_all.connect_clicked(move |_| {
        all_colored();
    });

    use controls::colored_no_logo::*;
    let colored_no_logo_button: Button = builder.get_object("ColoredNoLogoButton").expect("boned");
    colored_no_logo_button.connect_clicked(move |_| {
        colored_no_logo();
    });

    use controls::all::*;
    let all_button: Button = builder.get_object("GenerateAllButton").expect("boned");
    all_button.connect_clicked(move |_| {
        all();
    });

    use controls::outlined_all::*;
    let all_outline_button: Button = builder.get_object("OutlinedAllButton").expect("boned");
    all_outline_button.connect_clicked(move |_| {
        outlined_all();
    });

    let outline_no_logo_button: Button = builder.get_object("OutlinedNoLogoButton").expect("boned");
    outline_no_logo_button.connect_clicked(move |_| {
        outlined_no_logo();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.set_title("Christoffen Assets");
    window.show_all();
    gtk::main();
}
