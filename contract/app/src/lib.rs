#![no_std]

use sails_rs::gstd::gprogram;

pub mod service;

pub struct Program(());

#[gprogram]
impl Program {
    pub fn new() -> Self {
        service::Service::seed();
        Self(())
    }

    pub fn dns(&self) -> service::Service {
        service::Service::new()
    }
}
