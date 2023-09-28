use windows::Win32::System::Console::{
    CONSOLE_MODE,
    GetStdHandle,
    STD_OUTPUT_HANDLE,
    GetConsoleMode,
    SetConsoleMode,
    // STD_HANDLE,
};

use windows::Win32::Foundation::HANDLE;

const ENABLE_VIRTUAL_TERMINAL_PROCESSING:u32 = 0x4;

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

    fn set_mode(&self, mode: CONSOLE_MODE) -> windows::core::Result<()> {
        unsafe{
            SetConsoleMode( self.handle  , mode );
            return Ok(());
        }
    }
}

fn enable_birtual_terminal_processing() -> windows::core::Result<u32> {
    let stdout = new_console_handle()?;
    let mode = stdout.get_mode()?;
    let _ = stdout.set_mode(
        CONSOLE_MODE( mode.0 | ENABLE_VIRTUAL_TERMINAL_PROCESSING));
    return Ok(mode.0)
}

fn main() {
    match enable_birtual_terminal_processing() {
        Ok(m) => println!("\x1B[36msuccess: value={}(0x{:X})\x1B[0m",m,m),
        Err(err) => {
            println!("error: {:?}",err);
        }
    }
}
