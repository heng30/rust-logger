#[macro_use]
extern crate logger as mylogger;

use std::thread;
use mylogger::log;

#[derive(Debug)]
struct A {
    a: u32,
}

fn main() {
    log::set_level(log::A_LEVEL);
    log::set_size(1024);
    // log::set_filepath("/tmp/rs.dat".to_owned());
    log::dump();
    let count = 1024;

    let hdl = thread::spawn(move || {
        for i in 0..count {
            traceln!();
            traceln!("{}", "trace");
            debugln!("{}", "debug");
            infoln!("{}", "info");
            warnln!("{}", "warn");
            errorln!("{}", "error");
            fatalln!("{}-{} {}", "fatal", i, "world");
            debugln!("{:?}", A { a: 12 });
        }
    });

    for i in 0..count {
        traceln!();
        traceln!("{}", "trace");
        debugln!("{}", "debug");
        infoln!("{}{}", "info", i);
        warnln!("{}", "warn");
        errorln!("{}", "error");
        fatalln!("{}-{}", "fatal", "world");
        debugln!("{:?}", A { a: 12 });
    }

    hdl.join().unwrap();
}
