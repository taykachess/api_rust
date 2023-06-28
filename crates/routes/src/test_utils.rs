use api_db::user::UserCredentialsDto;
use axum::Json;
use log::{Level, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};
use tokio::sync::OnceCell;

use crate::mw::jwt::Token;

fn init_logger() -> Result<(), SetLoggerError> {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

pub(crate) async fn init_test_utils() {
    static ONCE: std::sync::Once = std::sync::Once::new();

    ONCE.call_once(|| {
        dotenv::dotenv().ok();
        init_logger().unwrap();
    });
    api_db::init_db_test().await.expect("failed to init db");
}

// TODO! SignUp user(get token)  use once ceil

pub(crate) async fn create_user_and_get_token() -> Token {
    static ONCE: OnceCell<Token> = OnceCell::const_new();

    let token = ONCE
        .get_or_init(|| async {
            let Json(token) =
                crate::routes::auth::signup(Json(UserCredentialsDto::new("Vadim123", "12345")))
                    .await
                    .expect("failed to signup");
            token
        })
        .await;
    token.to_owned()
}
