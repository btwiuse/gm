//! contract tests

use crate::*;

#[cfg(test)]
use gtest::{Program, System};

#[test]
fn basic_query_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(
        42,
        Init {
            name: "gm".to_string(),
            symbol: "GM".to_string(),
            base_uri: "https://gm.dev/{}".to_string(),
        },
    );

    let name: State = program
        .meta_state(Query::Name)
        .expect("failed to query name");
    assert_eq!(name, State::Name("gm".to_string()));

    let symbol: State = program
        .meta_state(Query::Symbol)
        .expect("failed to query symbol");
    assert_eq!(symbol, State::Symbol("GM".to_string()));

    let base_uri: State = program
        .meta_state(Query::BaseUri)
        .expect("failed to query base_uri");
    assert_eq!(base_uri, State::BaseUri("https://gm.dev/{}".to_string()));
}
