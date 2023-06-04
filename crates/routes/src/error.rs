use anyhow::Error;
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::{error::Error as StdError, fmt, panic::Location};

pub(crate) mod prelude {
    #[allow(unused)]
    pub(crate) use super::{ensure, route_error, throw, RouteResult, RouteResultContext};
}

// FIXME add pub(crate)
pub struct RouteError {
    inner: Error,
    location: &'static Location<'static>,
    status_code: StatusCode,
}

pub(crate) type RouteResult<T> = Result<T, RouteError>;

pub(crate) trait RouteResultContext<T, E> {
    fn status_code(self, status_code: StatusCode) -> Result<T, RouteError>;

    fn context<C>(self, context: C) -> Result<T, RouteError>
    where
        C: fmt::Display + Send + Sync + 'static;

    fn with_context<C, F>(self, context: F) -> Result<T, RouteError>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

#[allow(unused)]
impl RouteError {
    #[track_caller]
    pub(crate) fn msg<M>(message: M) -> Self
    where
        M: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        Self {
            inner: Error::msg(message),
            location: Location::caller(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub(crate) fn context<C>(mut self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.inner = self.inner.context(context);
        self
    }

    pub(crate) fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub(crate) fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = status_code;
    }

    pub(crate) fn is<E>(&self) -> bool
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.is::<E>()
    }

    pub(crate) fn downcast<E>(self) -> Result<E, Self>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        let Self {
            inner,
            location,
            status_code,
        } = self;
        inner.downcast::<E>().map_err(|inner| Self {
            inner,
            location,
            status_code,
        })
    }

    pub(crate) fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_ref::<E>()
    }

    pub(crate) fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_mut::<E>()
    }
}

#[allow(unused)]
macro_rules! route_error {
    ($msg:literal $(,)?) => {{
        let error: $crate::error::RouteError = anyhow::anyhow!($msg).into();
        error
    }};
    ($status_code:ident, $msg:literal $(,)?) => {{
        let mut error: $crate::error::RouteError = anyhow::anyhow!($msg).into();
        error.set_status_code(axum::http::StatusCode::$status_code);
        error
    }};
    ($err:expr $(,)?) => {{
        let error: $crate::error::RouteError = anyhow::anyhow!($err).into();
        error
    }};
    ($err:expr, $status_code:ident $(,)?) => {{
        let mut error: $crate::error::RouteError = anyhow::anyhow!($err).into();
        error.set_status_code(axum::http::StatusCode::$status_code);
        error
    }};
    ($status_code:ident, $fmt:expr, $($arg:tt)*) => {{
        let mut error: $crate::error::RouteError = anyhow::anyhow!($fmt, $($arg)*).into();
        error.set_status_code(axum::http::StatusCode::$status_code);
        error
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        let error: $crate::error::RouteError = anyhow::anyhow!($fmt, $($arg)*).into();
        error
    }};
}

#[allow(unused)]
macro_rules! throw {
    ($($arg:tt)*) => {
        return Err($crate::error::route_error!($($arg)*));
    };
}

#[allow(unused)]
macro_rules! ensure {
    ($condition:expr) => {
        $crate::error::ensure!($condition, concat!(stringify!($condition), " is falsy"));
    };
    ($condition:expr, $($arg:tt)*) => {
        if !$condition {
            $crate::error::throw!($($arg)*);
        }
    };
}

pub(crate) use {ensure, route_error, throw};

impl<T> RouteResultContext<T, RouteError> for RouteResult<T> {
    fn status_code(self, status_code: StatusCode) -> Result<T, RouteError> {
        self.map_err(|mut error| {
            error.status_code = status_code;
            error
        })
    }

    fn context<C>(self, context: C) -> Result<T, RouteError>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| error.context(context))
    }

    fn with_context<C, F>(self, context: F) -> Result<T, RouteError>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|mut error| {
            error.inner = error.inner.context(context());
            error
        })
    }
}

impl<T, E> RouteResultContext<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
    Self: anyhow::Context<T, E>,
{
    #[track_caller]
    fn status_code(self, status_code: StatusCode) -> Result<T, RouteError> {
        let location = Location::caller();
        self.map_err(|error| RouteError {
            inner: error.into(),
            location,
            status_code,
        })
    }

    #[track_caller]
    fn context<C>(self, context: C) -> Result<T, RouteError>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        let location = Location::caller();
        anyhow::Context::context(self, context).map_err(|inner| wrap_anyhow_error(inner, location))
    }

    #[track_caller]
    fn with_context<C, F>(self, context: F) -> Result<T, RouteError>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        let location = Location::caller();
        anyhow::Context::with_context(self, context)
            .map_err(|inner| wrap_anyhow_error(inner, location))
    }
}

impl<E: Into<Error>> From<E> for RouteError {
    #[track_caller]
    fn from(error: E) -> Self {
        Self {
            inner: error.into(),
            location: Location::caller(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Debug for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RouteError ({})\n  at {}\n  caused by: ",
            self.status_code, self.location
        )?;
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        let RouteError {
            status_code, inner, ..
        } = self;
        let body = Json(json!({
            "ok": false,
            "message": inner.to_string(),
        }));
        (status_code, body).into_response()
    }
}

fn wrap_anyhow_error(inner: anyhow::Error, location: &'static Location<'static>) -> RouteError {
    match inner.downcast::<RouteError>() {
        Ok(this) => this,
        Err(inner) => RouteError {
            inner,
            location,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}

#[cfg(test)]
mod test {
    use crate::error::prelude::*;

    fn make_error() -> std::io::Error {
        let kind = std::io::ErrorKind::NotFound;
        std::io::Error::new(kind, "Not found")
    }

    fn make_result() -> Result<(), std::io::Error> {
        Err(make_error())
    }

    #[test]
    fn simple_cast() {
        let line = line!() + 4;
        let column = 13;

        fn f() -> RouteResult<()> {
            make_result()?;
            Ok(())
        }

        let error = f().unwrap_err();
        assert_eq!(error.status_code().as_u16(), 500);

        let error_text = format!("{:?}", error);
        let at_file = format!("at {}:{}:{}", file!(), line, column);
        assert!(error_text.contains(&at_file));
        assert!(error_text.contains("caused by: Not found"));
    }

    #[test]
    fn with_status_code() {
        fn f() -> RouteResult<()> {
            make_result().status_code(axum::http::StatusCode::NOT_FOUND)?;
            Ok(())
        }

        let error = f().unwrap_err();
        assert_eq!(error.status_code().as_u16(), 404);
    }

    #[test]
    fn with_context() {
        const TEST_CONTEXT: &str = "TEST CONTEXT";

        fn f() -> RouteResult<()> {
            make_result().context(TEST_CONTEXT)?;
            Ok(())
        }

        let error = f().unwrap_err();
        let error_text = format!("{}", error);
        assert_eq!(error_text, TEST_CONTEXT);
    }

    #[test]
    fn with_context_and_status_code() {
        const TEST_CONTEXT: &str = "TEST CONTEXT";

        fn f1() -> RouteResult<()> {
            make_result()
                .status_code(axum::http::StatusCode::NOT_FOUND)
                .context(TEST_CONTEXT)?;
            Ok(())
        }

        fn f2() -> RouteResult<()> {
            make_result()
                .context(TEST_CONTEXT)
                .status_code(axum::http::StatusCode::NOT_FOUND)?;
            Ok(())
        }

        let error1 = f1().unwrap_err();
        let error2 = f2().unwrap_err();
        assert_eq!(error1.status_code().as_u16(), 404);
        assert_eq!(error2.status_code().as_u16(), 404);

        let error_text1 = format!("{}", error1);
        let error_text2 = format!("{}", error2);
        assert_eq!(error_text1, TEST_CONTEXT);
        assert_eq!(error_text2, TEST_CONTEXT);
    }

    #[test]
    fn route_error_macro() {
        let error = route_error!("TEXT");
        let error_text = format!("{}", error);
        assert_eq!(error_text, "TEXT");

        let error = route_error!("TEXT {}", "format");
        let error_text = format!("{}", error);
        assert_eq!(error_text, "TEXT format");

        let error = route_error!(make_error());
        assert_eq!(error.status_code().as_u16(), 500);
        assert!(error.downcast::<std::io::Error>().is_ok());

        let error = route_error!(NOT_FOUND, "TEXT");
        assert_eq!(error.status_code().as_u16(), 404);
        let error_text = format!("{}", error);
        assert_eq!(error_text, "TEXT");

        let error = route_error!(NOT_FOUND, "TEXT {}", "format");
        assert_eq!(error.status_code().as_u16(), 404);
        let error_text = format!("{}", error);
        assert_eq!(error_text, "TEXT format");

        let error = route_error!(make_error(), NOT_FOUND);
        assert_eq!(error.status_code().as_u16(), 404);
        assert!(error.downcast::<std::io::Error>().is_ok());
    }

    #[test]
    fn throw_macro() {
        fn f1() -> RouteResult<()> {
            throw!(make_error());
        }

        fn f2() -> RouteResult<()> {
            throw!(make_error(), NOT_FOUND);
        }

        let error1 = f1().unwrap_err();
        assert_eq!(error1.status_code().as_u16(), 500);
        assert!(error1.downcast::<std::io::Error>().is_ok());

        let error2 = f2().unwrap_err();
        assert_eq!(error2.status_code().as_u16(), 404);
        assert!(error2.downcast::<std::io::Error>().is_ok());
    }

    #[test]
    fn ensure_macro() {
        fn f1(a: u32, b: u32) -> RouteResult<()> {
            ensure!(a < b);
            Ok(())
        }

        fn f2(a: u32, b: u32) -> RouteResult<()> {
            ensure!(a < b, "a must be less than b");
            Ok(())
        }

        assert!(f1(1, 2).is_ok());
        assert!(f2(1, 2).is_ok());

        let error1 = f1(3, 2).unwrap_err();
        let error_text1 = format!("{}", error1);
        assert_eq!(error_text1, "a < b is falsy");

        let error2 = f2(3, 2).unwrap_err();
        let error_text2 = format!("{}", error2);
        assert_eq!(error_text2, "a must be less than b");
    }
}
