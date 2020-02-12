use super::todo::Todo;

use parking_lot::Mutex;
use rand::{Rng, thread_rng, distributions::Alphanumeric};

use rocket::local::Client;
use rocket::http::{Status, ContentType};

static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();
        let rocket = super::rocket();
        let db = super::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        assert!(Todo::delete_all(&$conn), "failed to delete all tasks for testing");

        $block
    })
}

#[test]
fn test_insertion_deletion() {
    run_test!(|client, conn| {
        let init_todos = Todo::all(&conn);

        // Issue a request to insert a new task.
        client.post("/todo")
            .header(ContentType::Form)
            .body("description=My+first+task")
            .dispatch();

        // Ensure we have one more task in the database.
        let new_todos = Todo::all(&conn);
        assert_eq!(new_todos.len(), init_todos.len() + 1);

        // Ensure the task is what we expect.
        assert_eq!(new_todos[0].description, "My first task");
        assert_eq!(new_todos[0].completed, false);

        // Issue a request to delete the task.
        let id = new_todos[0].id.unwrap();
        client.delete(format!("/todo/{}", id)).dispatch();

        // Ensure it's gone.
        let final_todos = Todo::all(&conn);
        assert_eq!(final_todos.len(), init_todos.len());
        if final_todos.len() > 0 {
            assert_ne!(final_todos[0].description, "My first task");
        }
    })
}

#[test]
fn test_toggle() {
    run_test!(|client, conn| {
        client.post("/todo")
            .header(ContentType::Form)
            .body("description=test_for_completion")
            .dispatch();

        let todo = Todo::all(&conn)[0].clone();
        assert_eq!(todo.completed, false);

        client.put(format!("/todo/{}", todo.id.unwrap())).dispatch();
        assert_eq!(Todo::all(&conn)[0].completed, true);

        client.put(format!("/todo/{}", todo.id.unwrap())).dispatch();
        assert_eq!(Todo::all(&conn)[0].completed, false);
    })
}

#[test]
fn test_many_insertions() {
    const ITER: usize = 100;

    let rng = thread_rng();
    run_test!(|client, conn| {
        let init_num = Todo::all(&conn).len();
        let mut descs = Vec::new();

        for i in 0..ITER {
            let desc: String = rng.sample_iter(&Alphanumeric).take(12).collect();
            client.post("/todo")
                .header(ContentType::Form)
                .body(format!("description={}", desc))
                .dispatch();

            descs.insert(0, desc);

            let todos = Todo::all(&conn);
            assert_eq!(todos.len(), init_num + i + 1);

            for j in 0..i {
                assert_eq!(descs[j], todos[j].description);
            }
        }
    })
}

#[test]
fn test_bad_form_submissions() {
    run_test!(|client, _conn| {
        let res = client.post("/todo")
            .header(ContentType::Form)
            .dispatch();

        let mut cookies = res.headers().get("Set-Cookie");
        assert_eq!(res.status(), Status::UnprocessableEntity);
        assert!(!cookies.any(|value| value.contains("error")));

        let res = client.post("/todo")
            .header(ContentType::Form)
            .body("description=")
            .dispatch();

        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|value| value.contains("error")));

        let res = client.post("/todo")
            .header(ContentType::Form)
            .body("evil=smile")
            .dispatch();

        let mut cookies = res.headers().get("Set-Cookie");
        assert_eq!(res.status(), Status::UnprocessableEntity);
        assert!(!cookies.any(|value| value.contains("error")));
    })
}
