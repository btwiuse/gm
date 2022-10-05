//! contract tests

use crate::*;

#[cfg(test)]
use gtest::{Program, System};

#[test]
fn init_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = Init {
        name: "gm".to_string(),
        symbol: "GM".to_string(),
        base_uri: "https://gm.dev/{}".to_string(),
    };

    let res = program.send(42, init_msg);
    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), InitOk.encode());
}
