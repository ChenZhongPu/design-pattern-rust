use bridge::{LG, SpecialRemote, Sony, RemoteControl, GenericRemote};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let sony = Sony::new();
    let remote_sony = SpecialRemote::new(
        Rc::new(RefCell::new(sony)));
    remote_sony.on();
    remote_sony.up();
    remote_sony.down();
    remote_sony.off();

    let lg = LG::new();
    let remote_lg = GenericRemote::new(
        Rc::new(RefCell::new(lg))
    );
    remote_lg.on();
    remote_lg.next_channel();
    remote_lg.prev_channel();
    remote_lg.off();
}