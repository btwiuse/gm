//! contract metadata declaration

use crate::*;

gstd::metadata! {
    title: "gm",
    init:
        input: Init,
    handle:
        input: Input,
        output: Output,
    state:
        input: Query,
        output: State,
}
