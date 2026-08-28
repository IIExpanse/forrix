pub mod proto {
    pub mod finam {
        include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    }
}
pub mod application;
