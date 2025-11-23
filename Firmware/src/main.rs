use rppal;
use tokio;
use bluer::{Adapter, AdapterEvent};

mod bluetooth;
mod controllers;


#[tokio::main]
async fn main() {

    println!("Test");
    
    
    let systems = tokio::spawn(systems_handler());
    let controllers = tokio::spawn(controller_handler());
    let host_comms = tokio::spawn(host_communication_handler());
    let slammer = tokio::spawn(slam());

}


async fn systems_handler()
{
    loop {
        
    }


}


async fn controller_handler()
{

}

async fn host_communication_handler()
{

}

async fn slam()
{

}