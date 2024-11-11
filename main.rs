/// nucleol5/src/main.rs

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use cortex_v8m::sau;
use kernel::platform::sau::SAU;
use kernel::platform::sau::SauRegionAttribute;


#[entry]
unsafe fn main() -> ! {

    // Waiting for *a != 0 from debugger
    let a: *const u32 = 0x2000f000 as *const u32;
    let mut v = unsafe {*a};
    
    loop {
        v = unsafe{*a};
        if v != 0 {break;}
    }
    
    // Creating a variable to test the readability of its value if it's in a secure/non-secure region
    let address: *mut u32 = 0x2000ff00 as *mut u32;
    *address = 0xFF;
    
    
    // Sau and status creation
    let mut sau_ref: sau::Sau<8> = sau::Sau::new();
    let mut status: sau::SauStatus<8> = sau_ref.new_status();
    
    // Setting code region secure and loading the status into the physical sau
    sau_ref.set_region(&mut status, 0x08000000, 0x0805FFFF, SauRegionAttribute::Secure, 0 as usize);
    sau_ref.load_status(&status); 
 
    // Enabling sau
    sau_ref.enable_sau();
    
    // -------------------------------------------------------------------- //
    
    // Setting a SRAM region secure and loading the status into the physical sau
    sau_ref.set_region(&mut status, 0x2000fa00, 0x2000ffff, SauRegionAttribute::Secure, 1 as usize);
    sau_ref.load_status(&status);
    
    // Reading secure SRAM from non-secure code - 0xFF is expected
    let mut value = *address;
    
    // -------------------------------------------------------------------- //
    
    // Setting the SRAM region non-secure and loading the status into the physical sau
    sau_ref.set_region(&mut status, 0x2000fa00, 0x2000ffff, SauRegionAttribute::NonSecure, 1 as usize);
    sau_ref.load_status(&status);
    
    // Reading non-secure SRAM from non-secure code - 0x00 is expected
    value = *address;
    
    // -------------------------------------------------------------------- //
    
    // Setting the SRAM region secure and loading the status into the physical sau
    sau_ref.set_region(&mut status, 0x2000fa00, 0x2000ffff, SauRegionAttribute::Secure, 1 as usize);
    sau_ref.load_status(&status);
    
    // Reading non-secure SRAM from non-secure code - 0xFF is expected
    value = *address;

    loop {

    }
}
