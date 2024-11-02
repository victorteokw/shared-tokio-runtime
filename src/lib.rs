use lazy_static::lazy_static;
use tokio;
pub use shared_tokio_runtime_macros::runtime;

pub fn rt() -> &'static tokio::runtime::Runtime {
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Should create a tokio runtime");
    }
    &RT
}
