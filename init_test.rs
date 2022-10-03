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

    let name = program
        .meta_state::<Query, State>(Query::Name)
        .expect("failed to query name");
    assert_eq!(name, State::Name("gm".to_string()));

    let symbol = program
        .meta_state::<Query, State>(Query::Symbol)
        .expect("failed to query symbol");
    assert_eq!(symbol, State::Symbol("GM".to_string()));

    let base_uri = program
        .meta_state::<Query, State>(Query::BaseUri)
        .expect("failed to query base_uri");
    assert_eq!(base_uri, State::BaseUri("https://gm.dev/{}".to_string()));
}
