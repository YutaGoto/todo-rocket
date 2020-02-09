use diesel::{self, prelude::*};

mod schema {
    table! {
        todos {
            id -> Nullable<Integer>,
            description -> Text,
            completed -> Bool,
        }
    }
}

use self::schema::todos;
use self::schema::todos::dsl::{todos as all_todos, completed as todo_completed};

#[table_name="todos"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Todo {
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool
}

#[derive(FromForm)]
pub struct Task {
    pub description: String,
}

impl Todo {
    pub fn all(conn: &SqliteConnection) -> Vec<Todo> {
        all_todos.order(todos::id.desc()).load::<Todo>(conn).unwrap()
    }

    pub fn insert(task: Task, conn: &SqliteConnection) -> bool {
        let t = Todo { id: None, description: task.description, completed: false };
        diesel::insert_into(todos::table).values(&t).execute(conn).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let todo = all_todos.find(id).get_result::<Todo>(conn);
        if todo.is_err() {
            return false;
        }

        let new_status = !todo.unwrap().completed;
        let updated_todo = diesel::update(all_todos.find(id));
        updated_todo.set(todo_completed.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_todos.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_todos).execute(conn).is_ok()
    }
}
