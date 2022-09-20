#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    _ = libreauth::pass::HashBuilder::from_phc(data);
});
