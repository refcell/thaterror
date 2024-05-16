#![cfg_attr(not(feature = "std"), feature(error_in_core))]

use anyhow::anyhow;
use thiserror::error::Error;
use thiserror::Error;

#[cfg(feature = "std")]
#[test]
fn test_transparent_struct() {
    use std::io;

    #[derive(Error, Debug)]
    #[error(transparent)]
    struct Error(ErrorKind);

    #[derive(Error, Debug)]
    enum ErrorKind {
        #[error("E0")]
        E0,
        #[error("E1")]
        E1(#[from] io::Error),
    }

    let error = Error(ErrorKind::E0);
    assert_eq!("E0", error.to_string());
    assert!(error.source().is_none());

    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = Error(ErrorKind::from(io));
    assert_eq!("E1", error.to_string());
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}

#[test]
fn test_transparent_enum() {
    #[derive(Error, Debug)]
    enum MyError {
        #[error("this failed")]
        This,
        #[error(transparent)]
        Other(anyhow::Error),
    }

    let error = MyError::This;
    assert_eq!("this failed", error.to_string());

    let error = MyError::Other(anyhow!("inner").context("outer"));
    assert_eq!("outer", error.to_string());
    assert_eq!("inner", error.source().unwrap().to_string());
}

#[test]
fn test_anyhow() {
    #[derive(Error, Debug)]
    #[error(transparent)]
    struct Any(#[from] anyhow::Error);

    let error = Any::from(anyhow!("inner").context("outer"));
    assert_eq!("outer", error.to_string());
    assert_eq!("inner", error.source().unwrap().to_string());
}

#[test]
fn test_non_static() {
    #[derive(Error, Debug)]
    #[error(transparent)]
    struct MyError<'a> {
        inner: ErrorKind<'a>,
    }

    #[derive(Error, Debug)]
    enum ErrorKind<'a> {
        #[error("unexpected token: {:?}", token)]
        Unexpected { token: &'a str },
    }

    let error = MyError {
        inner: ErrorKind::Unexpected { token: "error" },
    };
    assert_eq!("unexpected token: \"error\"", error.to_string());
    assert!(error.source().is_none());
}
