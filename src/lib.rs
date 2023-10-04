mod getkey;
pub use getkey::getkey;

mod mode;
pub use mode::{enable_stderr, enable_stdin, enable_stdout, make_raw, width_stderr, width_stdout};
