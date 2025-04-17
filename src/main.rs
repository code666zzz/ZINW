#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use my_os::println;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
	println!("Hello World{}","This is Zhang's operating system!");
	
	my_os::init();
	
	fn stack_overflow() {
		stack_overflow();
	}
	stack_overflow();

	#[cfg(test)]
	test_main();
	
	println!("Dia not crash!");
	loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}",info);
	loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}
	
