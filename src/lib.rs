use std::ffi::c_uint;
use std::time::Duration;

use tokio::time::sleep;

#[no_mangle]
pub extern "C" fn rust_function(n: std::ffi::c_uchar) -> c_uint {
    do_something(n)
}

fn do_something(n: std::ffi::c_uchar) -> c_uint {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(async {
        tokio::spawn(async move {
            let mut x: u32 = 0;
            sleep(Duration::from_secs(2)).await;
            for i in 0..(n as u32) {
                x += i;
            }
            x
        })
        .await
        .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = do_something(100);
        assert_eq!(result, 4950);
    }
}
