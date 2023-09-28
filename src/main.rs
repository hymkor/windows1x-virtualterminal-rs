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

struct RewindMode {
    handle: ConsoleHandle,
    mode: CONSOLE_MODE,
}

impl Drop for RewindMode {
    fn drop(&mut self){
        let _  = self.handle.set_mode(self.mode);
    }
}

fn enable_virtual_terminal_processing() -> windows::core::Result<RewindMode> {
    let stdout = new_console_handle()?;
    let mode = stdout.get_mode()?;
    let _ = stdout.set_mode(
        CONSOLE_MODE( mode.0 | ENABLE_VIRTUAL_TERMINAL_PROCESSING));
    return Ok(RewindMode{ handle:stdout , mode:mode })
}

fn main() {
    match enable_virtual_terminal_processing() {
        Ok(_) => {
            println!("\x1B[36msuccess\x1B[0m");
        }
        Err(err) => {
            println!("error: {:?}",err);
        }
    }
    println!("\x1B[36m(AFTER)\x1B[0m");
}
