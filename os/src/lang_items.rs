// os/src/lang_items.rs
use core::panic::PanicInfo;

// panic handler for no_std programming, panic! calls this fn. Now loop
#[panic_handler]
fn panic_placeholder(_info: &PanicInfo) -> ! {
    loop {}
}
