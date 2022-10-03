use crate::codec::Init;
use crate::codec::Input;
use crate::codec::Output;
use crate::codec::Query;
use crate::codec::State;
use gstd::ToString;

gstd::metadata! {
    title: "whoami",
    init:
        input: Init,
    handle:
        input: Input,
        output: Output,
    state:
        input: Query,
        output: State,
}
