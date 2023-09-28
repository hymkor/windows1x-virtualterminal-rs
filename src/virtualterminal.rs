use windows::Win32::System::Console::{
    CONSOLE_MODE,
    STD_ERROR_HANDLE,
    STD_HANDLE,
    STD_OUTPUT_HANDLE,
    GetConsoleMode,
    GetStdHandle,
    SetConsoleMode,
};

use windows::Win32::Foundation::HANDLE;

const ENABLE_VIRTUAL_TERMINAL_PROCESSING:u32 = 0x4;

struct ConsoleHandle {
    handle: HANDLE
}

fn new_console_handle(handle: STD_HANDLE) -> windows::core::Result<ConsoleHandle> {
    unsafe{
        let this = ConsoleHandle{
            handle: GetStdHandle( handle )?
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

pub struct RewindMode {
    handle: ConsoleHandle,
    mode: CONSOLE_MODE,
}

impl Drop for RewindMode {
    fn drop(&mut self){
        let _  = self.handle.set_mode(self.mode);
    }
}

fn enable(handle: STD_HANDLE) -> windows::core::Result<RewindMode> {
    let stdout = new_console_handle(handle)?;
    let mode = stdout.get_mode()?;
    let _ = stdout.set_mode(
        CONSOLE_MODE( mode.0 | ENABLE_VIRTUAL_TERMINAL_PROCESSING));
    return Ok(RewindMode{ handle:stdout , mode:mode })
}

#[allow(dead_code)]
pub fn enable_stdout() -> windows::core::Result<RewindMode> {
    return enable(STD_OUTPUT_HANDLE);
}

#[allow(dead_code)]
pub fn enable_stderr() -> windows::core::Result<RewindMode> {
    return enable(STD_ERROR_HANDLE);
}
