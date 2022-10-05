//! contract metadata declaration

use crate::*;

gstd::metadata! {
    title: "gm",
    init:
        input: Init,
        output: InitOk,
    handle:
        input: Action,
        output: Event,
    state:
        input: Query,
        output: State,
}
