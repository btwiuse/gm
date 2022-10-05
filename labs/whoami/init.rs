use crate::codec::Init;
use crate::state::STATE;
use gstd::{debug, msg, ActorId};

#[no_mangle]
pub unsafe extern "C" fn init() {
    let payload: Init = msg::load().expect("Invalid init message");
    let id: ActorId = msg::source();
    match payload {
        Init::Payload(msg) => {
            debug!("init(): msg = {}", msg);
            debug!("init(): id = {:?}", id);
            STATE.push(id);
        }
    }
}
