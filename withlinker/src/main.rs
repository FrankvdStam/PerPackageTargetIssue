#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[entry]
fn main() -> !
{
	loop{}
}

#[panic_handler]
fn my_panic(panic_info: &core::panic::PanicInfo) -> !
{
	loop{}
}