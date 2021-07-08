use trillium_async_std::async_std::task;

pub fn app() -> impl trillium::Handler {
    |conn: trillium::Conn| async move {
        let response = task::spawn(async {
            task::sleep(std::time::Duration::from_millis(10)).await;
            "successfully spawned a task"
        })
        .await;

        conn.ok(response)
    }
}
pub fn main() {
    env_logger::init();
    trillium_async_std::run(app());
}

#[cfg(test)]
mod tests {
    use trillium_testing::prelude::*;
    #[test]
    fn test() {
        let app = super::app();
        assert_ok!(get("/").on(&app), "successfully spawned a task");
    }
}
