use windows::Win32::System::Console::{
    CONSOLE_MODE,
    GetStdHandle,
    STD_INPUT_HANDLE,
    GetConsoleMode
};

fn get_console_mode() -> windows::core::Result<CONSOLE_MODE> {
    unsafe{
        let mut console_mode = CONSOLE_MODE(0);

        let stdin = GetStdHandle( STD_INPUT_HANDLE )?;
        match GetConsoleMode( stdin , &mut console_mode ).ok() {
            Ok(_) => return Ok(console_mode),
            Err(err) => return Err(err),
        }
    }
}

fn main() {
    match get_console_mode() {
        Ok(mode) => {
            println!("success: value={}(0x{:X})",mode.0,mode.0)
        },
        Err(err) => {
            println!("error: {:?}",err)
        }
    }
}
