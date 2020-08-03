#![feature(let_chains)]
#[macro_use]extern crate structopt;
use artifice_manager::{ArtificeDB, Manager};
use artifice_manager::database::Database;
use structopt::StructOpt;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use std::error::Error;
use networking::asyncronous::{AsyncHost, AsyncStream};
use std::io::{Read, Write};

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "artifice")]
pub struct StartArgs{
    #[structopt(short, long, parse(from_os_str))]
    path: Option<PathBuf>,
    #[structopt(short, long)]
    start: Option<bool>,
    #[structopt(long)]
    password: Option<String>,
}
impl StartArgs{
    pub fn path(&self) -> Option<PathBuf>{
        self.path.clone()
    }
    pub fn start(&self) -> Option<bool>{
        self.start
    }
    pub fn password(&self) -> Option<String>{
        self.password.clone()
    }
}
pub fn login() -> std::io::Result<String>{
    let mut first_key: [u8; 65535] = [0; 65535];
    std::io::stdout().lock().write(b"enter encryption key: ")?;
    std::io::stdout().lock().flush()?;
    std::io::stdin().lock().read(&mut first_key)?;
    let fkey = String::from_utf8(first_key.to_vec()).unwrap();
    Ok(fkey)
}

pub fn start(password: String) -> Result<(), Box<dyn Error>>{
    let database = ArtificeDB::default();
    println!("database created");
    let manager = Manager::load(database, &password.into_bytes())?;
    
    let runtime = Runtime::new()?;

    let config = manager.config();
    println!("config: {:?}", config);
    runtime.spawn(async move {
        let mut host = AsyncHost::from_host_config(&config).await.unwrap();
        while let Some(stream) = host.incoming().unwrap().await {
            
        }
    });
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = StartArgs::from_args();
    let password = args.password();
    if args.start().is_some() && password != None{
        start(password.unwrap())?;
    }else if args.start().is_some() && password == None {
        start(login()?)?;
    }
    
    Ok(())
}