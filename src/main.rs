use clap::{Args, Parser, Subcommand};
use log::{error, info, trace, warn};
use pretty_env_logger;
use sonos;

//Clap Args Structure
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from cargo.tomli
#[command(propagate_version = true)]
struct SonoWireCLI {
    #[command(subcommand)]
    command: SonoWireCommmand,
}

#[derive(Subcommand)]
enum SonoWireCommand {
    ///Top Level Commands
    ListDevices,
}

fn main() {
    //Initialise Logger
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::init();

    //Parse Command Line Arguments
    let sw_cli = SonoWireCLI::parse();

    match &sw_cli.command {
        SonoWireCommand::ListDevices => {
            //Discover Devices
            println!("Fetching Devices, Please Wait...");
            let devices = sonos::discover();
            if devices.is_ok() {
                //List the devices
                println!("Successfully fetched {} devices:" devices.unwrap().len());
                for device in devices.unwrap() {
                    println!(device.to_string());
                }
            } else {
                let error = devices.unwrap_err();
                error!(
                    "{} occurred when attempting to discover devices:\n{}",
                    error.kind(),
                    error.description()
                );
            }
        }
    }
}
