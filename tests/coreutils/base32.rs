use std::path::PathBuf;

use uutils_args::{Arguments, Initial, Options};

#[derive(Clone, Arguments)]
enum Arg {
    #[option("-d", "--decode")]
    Decode,

    #[option("-i", "--ignore-garbage")]
    IgnoreGarbage,

    #[option("-w COLS", "--wrap=COLS")]
    Wrap(usize),

    #[positional(..=1)]
    File(PathBuf),
}

#[derive(Initial)]
struct Settings {
    decode: bool,
    ignore_garbage: bool,
    #[field(default = Some(76))]
    wrap: Option<usize>,
    file: Option<PathBuf>,
}

impl Options for Settings {
    type Arg = Arg;
    fn apply(&mut self, arg: Arg) {
        match arg {
            Arg::Decode => self.decode = true,
            Arg::IgnoreGarbage => self.ignore_garbage = true,
            Arg::Wrap(0) => self.wrap = None,
            Arg::Wrap(x) => self.wrap = Some(x),
            Arg::File(f) => self.file = Some(f),
        }
    }
}

#[test]
fn wrap() {
    assert_eq!(Settings::parse(["base32"]).wrap, Some(76));
    assert_eq!(Settings::parse(["base32", "-w0"]).wrap, None);
    assert_eq!(Settings::parse(["base32", "-w100"]).wrap, Some(100));
    assert_eq!(Settings::parse(["base32", "--wrap=100"]).wrap, Some(100));
}
