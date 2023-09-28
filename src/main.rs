use windows::Win32::System::Console::{
    CONSOLE_MODE,
    GetStdHandle,
    STD_OUTPUT_HANDLE,
    GetConsoleMode,
    //SetConsoleMode,
    //STD_HANDLE,
};

use windows::Win32::Foundation::HANDLE;

struct ConsoleHandle {
    handle: HANDLE
}

fn new_console_handle() -> windows::core::Result<ConsoleHandle> {
    unsafe{
        let this = ConsoleHandle{
            handle: GetStdHandle( STD_OUTPUT_HANDLE )?
        };
        return Ok(this);
    }
}

impl ConsoleHandle {
    fn get_mode(&self) -> windows::core::Result<CONSOLE_MODE> {
        unsafe{
            let mut console_mode = CONSOLE_MODE(0);

            match GetConsoleMode( self.handle  , &mut console_mode ).ok() {
                Ok(_) => return Ok(console_mode),
                Err(err) => return Err(err),
            }
        }
    }
}

/*
fn set_console_mode(mode: CONSOLE_MODE) -> windows::core::Result<()> {
    unsafe{
        let stdout = GetStdHandle( STD_OUTPUT_HANDLE )?;
        SetConsoleMode( stdout , mode );
        return Ok(());
    }
}
*/

fn main() {
    if let Ok(stdout) = new_console_handle() {
        match stdout.get_mode() {
            Ok(mode) => {
                println!("success: value={}(0x{:X})",mode.0,mode.0)
            },
            Err(err) => {
                println!("error: {:?}",err)
            }
        }
    }
}
