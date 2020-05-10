extern crate hex;
extern crate im_rc;
extern crate leb128;
extern crate maplit;
extern crate rand;
extern crate sha2;
extern crate uuid;
extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

mod backend;
mod columnar;
mod concurrent_operations;
mod encoding;
mod error;
mod object_store;
mod op_handle;
mod op_set;
mod ordered_set;
mod pending_diff;
mod undo_operation;
mod serialize;
mod time;
mod actor_map;

pub use automerge_protocol::{ActorID, ChangeHash, ObjType, ObjectID, ElementID, Key, DataType, Value, OpType, Operation, Change, ChangeRequest, ChangeRequestType, DiffEdit, Patch, Diff, MapDiff, SeqDiff, ObjDiff};
pub use backend::Backend;
pub use error::AutomergeError;
