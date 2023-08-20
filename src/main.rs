use druid::{AppLauncher, WindowDesc};

mod data;
use data::Task;
use data::TodoList;
mod ui;
use ui::build_ui;
mod delegate;
use delegate::Delegate;
use delegate::make_menu;

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_ui())
        .title("Todo App")
        .resizable(false)
        .menu(make_menu)
        .window_size((1200.0, 600.0));


    let initial_state = TodoList::load_from_bd();;


    // start the application
    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}
