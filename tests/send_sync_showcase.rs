use std::{thread, time::Duration};

#[derive(Clone)]
struct User {
    id: u64,
    username: String,
}

unsafe impl Send for User {}
unsafe impl Sync for User {}

#[test]
fn send_sync_showcase_1() {
    let user = User {
        id: 10,
        username: "user1".to_string(),
    };

    let user_clone = user.clone();
    thread::spawn(move || run_me("th1".to_string(), user));
    thread::spawn(move || run_me("th2".to_string(), user_clone));

    thread::sleep(Duration::from_secs(2));
}

fn run_me(thread_name: String, user: User) {
    for i in 0..=10 {
        println!(
            "{} user id: {} , username: {}",
            thread_name, user.id, user.username
        );
        thread::sleep(Duration::from_millis(100));
    }
}
