use std::sync::{Arc, Mutex};

use pkg::Pkg;
use tokio::sync::broadcast;

pub struct Pakcage {
    pub pkg: Arc<Mutex<Pkg>>,
    pub sender: broadcast::Sender<Pkg>,
}

impl Pakcage {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(10);

        Self {
            pkg: Arc::new(Mutex::new(Pkg::new())),
            sender,
        }
    }
    pub fn update_pkg(&self, pkg: Pkg) -> Result<usize, broadcast::error::SendError<Pkg>> {
        let mut pkg_lock = self.pkg.lock().unwrap();

        *pkg_lock = pkg.clone();

        self.sender.send(pkg)
    }
    pub fn get_pkg(&self) -> Pkg {
        self.pkg.lock().unwrap().clone()
    }
}
