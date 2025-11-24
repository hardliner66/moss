use core::pin::Pin;

use alloc::{boxed::Box, collections::btree_map::BTreeMap};

use libkernel::error::Result;
use linkme::distributed_slice;

use crate::sync::OnceLock;

/// Type definition for syscall handler functions
pub type SyscallHandlerFn =
    fn(u64, u64, u64, u64, u64, u64) -> Pin<Box<dyn Future<Output = Result<usize>> + Send>>;

/// Structure to hold syscall handler information
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SyscallHandler {
    pub number: u32,
    pub hex: &'static str,
    pub name: &'static str,
    pub handler: SyscallHandlerFn,
}

#[distributed_slice]
pub static SYSCALLS: [SyscallHandler];

pub fn syscall_registry() -> &'static BTreeMap<u32, SyscallHandler> {
    static INSTANCE: OnceLock<BTreeMap<u32, SyscallHandler>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let mut map = BTreeMap::new();
        for handler in SYSCALLS {
            map.insert(handler.number, handler.clone());
        }
        map
    })
}
