use ipc_channel::{IpcBytesSender, IpcBytesReceiver};
use std::error::Error;
use std::future::Future;

use networking::AsyncStream;

pub struct Callback<O, T: impl Future<Ouput = Result<O, Box<dyn Error>>> {
    size: usize,
    func: *mut T,
}
