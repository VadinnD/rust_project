use std::convert::Into;
use druid::{Data, Lens};
use druid::EventCtx;
use druid::Env;
use im::Vector;
use std::string::ToString;
use chrono::{Datelike, Timelike, NaiveDate, Utc};
use postgres::{Client, NoTls, Error};

use crate::delegate::MENU;


#[derive(Clone, Data, Lens)]
pub struct TodoList {
    tasks: Vector<Task>,
    new_task: String,
    new_deadline: String,
    new_importance: String,
}

impl TodoList {
    pub fn new(tasks: Vec<Task>) -> Self {

        Self {
            new_task: "".into(),
            new_deadline: "".into(),
            new_importance: "".into(),
            tasks: Vector::from(tasks),
        }
    }

    pub fn load_from_bd() -> Self {
        let mut todos: Vec<Task> = vec![];

        let mut client = Client::connect(&"postgresql://postgres:root456@localhost/ToDo", NoTls).unwrap();

        client.execute("CREATE TABLE IF NOT EXISTS ToDo  (
             task_id         SERIAL PRIMARY KEY,
             name            TEXT NOT NULL,
             description     TEXT DEFAULT '...',
             deadline        TEXT DEFAULT '365',
             importance      TEXT DEFAULT '1',
             done            BOOL DEFAULT false
             );", &[]).unwrap();

        for row in client.query("SELECT * FROM ToDo", &[]).unwrap() {

            let t1 = Task {
                task_id: row.get(0),
                name: row.get(1),
                deadline: row.get(3),
                importance: row.get(4),
                days: Task::calc(row.get(3)),
                done: row.get(5),
                description: row.get(2),
            };

            todos.push(t1);
        }

        Self {
            tasks: Vector::from(todos),
            new_task: "".to_string(),
            new_deadline: "".to_string(),
            new_importance: "".to_string(),
        }
    }

    pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let index = data.save_to_bd();

        data.tasks.push_front(Task::new(index, &data.new_task, &data.new_deadline,
                                        &data.new_importance, "...", false));

        data.new_task = "".into();
        data.new_deadline = "".into();
        data.new_importance = "".into();
    }

    pub fn save_to_bd(&self) -> i32 {
        let mut client = Client::connect(&"postgresql://postgres:root456@localhost/ToDo", NoTls).unwrap();

        client.execute(
                "INSERT INTO ToDo (name, deadline, importance, done) \
                VALUES ($1, $2, $3, false)",
                &[&self.new_task, &self.new_deadline, &self.new_importance],
        ).unwrap();

        let mut i: i32 = 0;

        for row in &client.query("SELECT MAX(task_id) FROM ToDo", &[]).unwrap() {
            i = row.get(0);
        }

        i
    }

    pub fn delete_from_bd(&self) {
        let mut client = Client::connect(&"postgresql://postgres:root456@localhost/ToDo", NoTls).unwrap();

        client.execute("DELETE FROM ToDo WHERE done = true;", &[]).unwrap();
    }

    pub fn delete_all_from_bd(&mut self) {
        let mut client = Client::connect(&"postgresql://postgres:root456@localhost/ToDo", NoTls).unwrap();

        client.execute("DELETE FROM ToDo;", &[]).unwrap();
        self.tasks = Vector::from(vec![]);
    }

    pub fn clear_completed(&mut self) {
        self.tasks.retain(|item| !item.done);
        self.delete_from_bd();
    }

    pub fn hide_completed(&mut self) {
        self.tasks.retain(|item| !item.done);
    }
}


#[derive(Clone, Data, Lens)]
pub struct Task {
    pub task_id: i32,
    pub name: String,
    pub deadline: String,
    pub importance: String,
    pub days: String,
    pub done: bool,
    pub description: String,
}

impl Task {
    pub fn new(id: i32, name: &str, deadline: &str, importance: &str, description: &str,
               checked: bool) -> Self {
        Self {
            task_id: id,
            name: name.to_string(),
            deadline: deadline.to_string(),
            importance: importance.to_string(),
            days: Self::calc(deadline),
            done: checked,
            description: description.to_string(),
        }
    }

    fn calc(deadline: &str) -> String {
        let now = Utc::now().date().naive_utc();
        let naive_date = NaiveDate::parse_from_str(deadline, "%Y-%m-%d").unwrap();
        (naive_date - now).num_days().to_string() + &" days left"
    }

    pub fn click_check(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let mut client = Client::connect(&"postgresql://postgres:root@localhost/ToDo", NoTls).unwrap();

        data.done = !data.done;

        client.execute("UPDATE ToDo
                        SET done = $2
                        WHERE task_id = $1;", &[&data.task_id, &data.done]).unwrap();
    }

    pub fn click_menu(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        ctx.submit_command(MENU);
    }

    pub fn click_delete(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let mut client = Client::connect(&"postgresql://postgres:root456@localhost/ToDo", NoTls).unwrap();

        client.execute(
                "DELETE FROM ToDo
                WHERE task_id = $1;",
                &[&data.task_id],
        ).unwrap();
    }
}
