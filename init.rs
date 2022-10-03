//! contract state initialization / constructor

use crate::*;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let Init {
        name,
        symbol,
        base_uri,
    } = msg::load().expect("Invalid init message");
    /*
    match payload {
        Init::Payload(msg) => {
            debug!("init(): msg = {}", msg);
            debug!("init(): id = {:?}", id);
            state::STATE.push(id);
        },
    }
    */
    let id: ActorId = msg::source();
    STATE = Some(Contract::<GearConfig>::new(&id));
    STATE.as_mut().unwrap().name = name;
    STATE.as_mut().unwrap().symbol = symbol;
    STATE.as_mut().unwrap().base_uri = base_uri;
}
