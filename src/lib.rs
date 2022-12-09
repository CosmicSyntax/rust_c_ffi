use std::ffi::c_uint;

#[no_mangle]
pub extern "C" fn rust_function() -> c_uint {
    do_something()
}

fn do_something() -> c_uint {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio::spawn(async {
            let mut x = 0;
            for i in 0..100 {
                x += i;
            }
            x
        }).await.unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = do_something();
        assert_eq!(result, 4950);
    }
}
