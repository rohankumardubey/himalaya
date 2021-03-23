mod app;
mod config;
mod input;
mod smtp;
mod table;
mod output {
    pub(crate) mod cli;
    pub(crate) mod utils;
}
mod imap {
    pub(crate) mod cli;
    pub(crate) mod model;
}
mod flag {
    pub(crate) mod cli;
    pub(crate) mod model;
}
mod msg {
    pub(crate) mod cli;
    pub(crate) mod model;
}
mod mbox {
    pub(crate) mod cli;
    pub(crate) mod model;
}

use crate::app::App;

fn main() {
    if let Err(ref errs) = App::new().run() {
        let mut errs = errs.iter();
        match errs.next() {
            None => (),
            Some(err) => {
                eprintln!("{}", err);
                errs.for_each(|err| eprintln!(" ↳ {}", err));
            }
        }
    }
}
