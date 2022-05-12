#[macro_export]
macro_rules! error_code {
    (E0001) => {
        "todo"
    };
    ($ident:ident) => {
        compile_error!("unknown error code")
    };
}
