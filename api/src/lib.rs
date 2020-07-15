#[macro_use]extern crate serde_derive;
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use std::env::{var, VarError};
#[derive(Debug, Serialize, Deserialize)]
pub struct API{
    send: IpcSender,
    recv: IpcReceiver,
}
impl API {
    pub fn init() -> Result<Self, VarError>{
        let sender = var("sender")?;
        let receiver = var("receiver")?;
        let send = serde_json::from_str(sender).unwrap();
        let recv = serde_json::from_str(receiver).unwrap();
        Ok(Self{send, recv})
    }
    pub fn channel(self) -> (IpcSender, IpcReceiver){
        (self.send, self.recv)
    }
}