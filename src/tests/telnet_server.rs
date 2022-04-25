/* use crate::telnet_messaging::telnet_server::{
    MessageWrapper
}; */

use mockall_double::double;
use mockall::{automock, mock, predicate::*};

#[test]
fn it_can_construct_message_wrapper() {
    let wrapper = MessageWrapper {
        message: Vec::from("Hello".as_bytes()),
        sender_id: 1
    };
    assert_eq!(wrapper.sender_id, 1);
}
