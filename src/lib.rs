use std::io::Read;

use windows::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE, STD_ERROR_HANDLE, STD_HANDLE,
    STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};

use windows::Win32::Foundation::HANDLE;

// for stdin
const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;
// for stdout / stderr
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x4;

// for raw mode of stdin
const ENABLE_ECHO_INPUT: u32 = 0x0004;
const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
const ENABLE_LINE_INPUT: u32 = 0x0002;

// for raw mode of stdout
const ENABLE_PROCESSED_OUTPUT: u32 = 0x0001;

const RAW_MODE: u32 =
    !(ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_OUTPUT);

pub type Error = windows::core::Error;

pub type Result<T> = std::result::Result<T, Error>;

struct ConsoleHandle(HANDLE);

fn new_console_handle(handle: STD_HANDLE) -> Result<ConsoleHandle> {
    unsafe {
        return Ok(ConsoleHandle(GetStdHandle(handle)?));
    }
}

impl ConsoleHandle {
    fn get_mode(&self) -> Result<CONSOLE_MODE> {
        unsafe {
            let mut console_mode = CONSOLE_MODE(0);
            match GetConsoleMode(self.0, &mut console_mode).ok() {
                Ok(_) => return Ok(console_mode),
                Err(err) => return Err(err),
            }
        }
    }

    fn set_mode(&self, mode: CONSOLE_MODE) -> Result<()> {
        unsafe {
            match SetConsoleMode(self.0, mode).ok() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }

    pub fn width(&self) -> Result<i16> {
        use windows::Win32::System::Console::{
            GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO,
        };
        let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = Default::default();
        unsafe {
            GetConsoleScreenBufferInfo(self.0, &mut csbi);
            return Ok(csbi.dwSize.X);
        }
    }
}

pub struct OldState(ConsoleHandle, CONSOLE_MODE);

impl OldState {
    pub fn restore(&mut self) {
        let _ = self.0.set_mode(self.1);
    }
}

impl Drop for OldState {
    fn drop(&mut self) {
        self.restore();
    }
}

fn change(handle: STD_HANDLE, and_value: u32, or_value: u32) -> Result<OldState> {
    let stdout = new_console_handle(handle)?;
    let mode = stdout.get_mode()?;
    stdout.set_mode(CONSOLE_MODE(mode.0 & and_value | or_value))?;
    return Ok(OldState(stdout, mode));
}

pub fn enable_stdin() -> Result<OldState> {
    return change(STD_INPUT_HANDLE, !0, ENABLE_VIRTUAL_TERMINAL_INPUT);
}

pub fn enable_stdout() -> Result<OldState> {
    return change(STD_OUTPUT_HANDLE, !0, ENABLE_VIRTUAL_TERMINAL_PROCESSING);
}

pub fn enable_stderr() -> Result<OldState> {
    return change(STD_ERROR_HANDLE, !0, ENABLE_VIRTUAL_TERMINAL_PROCESSING);
}

pub fn make_raw() -> Result<OldState> {
    return change(STD_INPUT_HANDLE, RAW_MODE, 0);
}

pub enum GetKeyError {
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
}

pub fn getkey() -> std::result::Result<String, GetKeyError> {
    let mut buffer: Vec<u8> = vec![0; 256];
    let mut stdin = std::io::stdin();

    let n = match stdin.read(&mut buffer) {
        Ok(n) => n,
        Err(err) => return Err(GetKeyError::Io(err)),
    };
    buffer.truncate(n);
    match String::from_utf8(buffer) {
        Ok(s) => return Ok(s),
        Err(err) => return Err(GetKeyError::FromUtf8(err)),
    }
}

pub fn width_stdout() -> Result<i16> {
    let stdout = new_console_handle(STD_OUTPUT_HANDLE)?;
    return stdout.width();
}

pub fn width_stderr() -> Result<i16> {
    let stdout = new_console_handle(STD_ERROR_HANDLE)?;
    return stdout.width();
}
