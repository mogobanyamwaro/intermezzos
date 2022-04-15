#![no_std]
#![allow(unused_variables)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn main() {
    #[no_mangle]
    pub extern "C" fn kmain() -> ! {
        loop {}
    }
}
