use windows::Win32::System::Console::{
    STD_HANDLE,
    CONSOLE_MODE,
    STD_INPUT_HANDLE,
    STD_OUTPUT_HANDLE,
    STD_ERROR_HANDLE,
    GetStdHandle,
    GetConsoleMode,
    SetConsoleMode,
};

use windows::Win32::Foundation::HANDLE;

// for stdin
const ENABLE_VIRTUAL_TERMINAL_INPUT     :u32 = 0x0200;
// for stdout / stderr
const ENABLE_VIRTUAL_TERMINAL_PROCESSING:u32 = 0x4;

// for raw mode of stdin
const ENABLE_ECHO_INPUT       :u32 = 0x0004;
const ENABLE_PROCESSED_INPUT  :u32 = 0x0001;
const ENABLE_LINE_INPUT       :u32 = 0x0002;

// for raw mode of stdout
const ENABLE_PROCESSED_OUTPUT :u32 = 0x0001;

const RAW_MODE: u32 = !(ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_OUTPUT);

struct ConsoleHandle(HANDLE);

fn new_console_handle(handle: STD_HANDLE) -> windows::core::Result<ConsoleHandle> {
    unsafe{
        return Ok(ConsoleHandle(GetStdHandle( handle )?));
    }
}

impl ConsoleHandle {
    fn get_mode(&self) -> windows::core::Result<CONSOLE_MODE> {
        unsafe{
            let mut console_mode = CONSOLE_MODE(0);
            match GetConsoleMode( self.0  , &mut console_mode ).ok() {
                Ok(_) => return Ok(console_mode),
                Err(err) => return Err(err),
            }
        }
    }

    fn set_mode(&self, mode: CONSOLE_MODE) -> windows::core::Result<()> {
        unsafe{
            match SetConsoleMode( self.0  , mode ).ok() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }
}

pub struct RewindMode(ConsoleHandle,CONSOLE_MODE);

impl Drop for RewindMode {
    fn drop(&mut self){
        let _  = self.0.set_mode(self.1);
    }
}

fn change(handle: STD_HANDLE,and_value: u32,or_value: u32) -> windows::core::Result<RewindMode> {
    let stdout = new_console_handle(handle)?;
    let mode = stdout.get_mode()?;
    stdout.set_mode(CONSOLE_MODE( mode.0 & and_value | or_value ))?;
    return Ok(RewindMode(stdout , mode))
}


#[allow(dead_code)]
pub fn enable_stdin() -> windows::core::Result<RewindMode> {
    return change(STD_INPUT_HANDLE, !0, ENABLE_VIRTUAL_TERMINAL_INPUT);
}

#[allow(dead_code)]
pub fn enable_stdout() -> windows::core::Result<RewindMode> {
    return change(STD_OUTPUT_HANDLE, !0, ENABLE_VIRTUAL_TERMINAL_PROCESSING);
}

#[allow(dead_code)]
pub fn enable_stderr() -> windows::core::Result<RewindMode> {
    return change(STD_ERROR_HANDLE, !0, ENABLE_VIRTUAL_TERMINAL_PROCESSING);
}

#[allow(dead_code)]
pub fn make_raw() -> windows::core::Result<RewindMode> {
    return change(STD_INPUT_HANDLE, RAW_MODE, 0);
}
