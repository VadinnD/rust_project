use druid::{AppDelegate, Command, DelegateCtx, Handled, Selector, Target};

use crate::TodoList;
use crate::Task;
use druid::Env;
use druid::Data;
use druid::WindowId;
use druid::Menu;
use druid::LocalizedString;

use druid::commands;
use druid::{ MenuItem, SysMods};


use crate::ui::build_new_win;

use crate::WindowDesc;


pub const DELETE: Selector = Selector::new("Delete1");
pub const DELETE_COMPLETED: Selector = Selector::new("Delete2");
pub const DELETE_ALL: Selector = Selector::new("Delete3");
pub const MENU: Selector = Selector::new("New_Menu");
pub const NUM: Selector = Selector::new("TODO");

pub struct Delegate;

impl AppDelegate<TodoList> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut TodoList,
        _env: &Env,
    ) -> Handled {
        if cmd.is(DELETE) {
            data.hide_completed();
            Handled::Yes
        } else if cmd.is(DELETE_COMPLETED) {
            data.clear_completed();
            Handled::Yes
        } else if cmd.is(DELETE_ALL) {
            data.delete_all_from_bd();
            Handled::Yes
        } else if cmd.is(MENU) {
            let new_win = WindowDesc::new(build_new_win())
                .window_size((300.0, 500.0));
            ctx.new_window(new_win);
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}

/*
pub fn make_context_menu<T: Data>(_window: Option<WindowId>, _data: &TodoList, _env: &Env) -> Menu<T> {

}
*/

#[allow(unused_assignments, unused_mut)]
pub fn make_menu<T: Data>(_window: Option<WindowId>, _data: &TodoList, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();

    base = base.entry(
        Menu::new(LocalizedString::new("Main"))
            .entry(MenuItem::new(LocalizedString::new("Hide completed tasks"))
                .command(DELETE))

            .entry(MenuItem::new(LocalizedString::new("Delete all completed tasks"))
                .command(DELETE_COMPLETED))

            .entry(MenuItem::new(LocalizedString::new("Delete all tasks"))
                .command(DELETE_ALL))

            .separator()

            .entry(druid::platform_menus::common::cut())
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste())

            .separator()

            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())

            .separator()

            .entry(MenuItem::new(LocalizedString::new("Quit"))
                .command(commands::QUIT_APP)
                .hotkey(SysMods::Cmd, "q")),
    );

    base.entry(
        Menu::new(LocalizedString::new("Edit"))
            .entry(MenuItem::new(LocalizedString::new("First"))
                .command(NUM))

            .entry(MenuItem::new(LocalizedString::new("Second"))
                .command(NUM))

            .entry(MenuItem::new(LocalizedString::new("Third"))
                .command(NUM))
    )
}

