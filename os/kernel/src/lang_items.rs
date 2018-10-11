#[no_mangle]
#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt(
    fmt: ::std::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    static PANIC: &'static str = r#"
             (   )
          (   ) (
           ) _   )
            ( \_
          _(_\ \)__
         (____\___))  

       You fucked up.

---------- PANIC ----------
"#;
    use console::{kprint, kprintln};
    kprint!("{}", PANIC);
    kprintln!("file: {}", file);
    kprintln!("line: {}", line);
    kprintln!("col: {}\n", col);

    kprint!("{}", fmt);
    loop {
        unsafe { asm!("wfe") }
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}
