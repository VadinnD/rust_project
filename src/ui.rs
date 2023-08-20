use druid::Widget;
use druid::widget::Button;
use druid::widget::Flex;
use druid::widget::List;
use druid::widget::Checkbox;
use druid::widget::Label;
use druid::WidgetExt;
use druid::widget::TextBox;
use druid::Color;
use druid::widget::Scroll;


use crate::data::*;

pub fn build_ui() -> impl Widget<TodoList> {
    Scroll::new(
    Flex::column()
        .with_child(new_todo_textbox())
        .with_child(List::new(todo_item).lens(TodoList::tasks))
        .with_flex_spacer(1.0)
    )
        .vertical()
        .expand()
}


fn new_todo_textbox() -> impl Widget<TodoList> {
    let new_todo_textbox_name = TextBox::new()
        .with_placeholder("TODO")
        .expand_width()
        .padding(20.0)
        .lens(TodoList::new_task);

    let new_todo_textbox_deadline = TextBox::new()
        .with_placeholder("DEADLINE (Y-M-D)")
        .expand_width()
        .padding(10.0)
        .lens(TodoList::new_deadline);

    let new_todo_textbox_importance = TextBox::new()
        .with_placeholder("IMPORTANCE")
        .expand_width()
        .padding(10.0)
        .lens(TodoList::new_importance);

    let add_todo_button = Button::new("➕ ADD")
        .on_click(TodoList::click_add)
        .padding(10.0);

    Flex::row()
        .with_flex_child(new_todo_textbox_name, 2.4)
        .with_flex_spacer(1.0)
        .with_flex_child(new_todo_textbox_deadline, 1.4)
        .with_flex_spacer(1.0)
        .with_flex_child(new_todo_textbox_importance, 0.85)
        .with_flex_spacer(1.0)
        .with_child(add_todo_button)
        .border(Color::grey(0.25), 2.0)
}


fn todo_item() -> impl Widget<Task> {
    let checkbox = Checkbox::new("").lens(Task::done).on_click(Task::click_check);
    let label_name = Label::raw().lens(Task::name).fix_width(400.0);
    let label_deadline = Label::raw().lens(Task::days).fix_width(240.0);
    let label_importance = Label::raw().lens(Task::importance).fix_width(180.0);

    let delete_button = Button::new("≡").on_click(Task::click_menu).padding(10.0);

    Flex::row()
        .with_child(checkbox)
        .with_child(label_name)
        .with_flex_spacer(1.0)
        .with_child(label_deadline)
        .with_flex_spacer(1.0)
        .with_child(label_importance)
        .with_flex_spacer(1.0)
        .with_child(delete_button)
}


pub fn build_new_win() -> impl Widget<TodoList> {
    Flex::column()
        .with_child(new_todo_textbox())
        .with_child(List::new(todo_item).lens(TodoList::tasks))
        .with_flex_spacer(1.0)
}
