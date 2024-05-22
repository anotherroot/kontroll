use clap::{Parser, Subcommand};

use crate::{api, utils};

#[derive(Debug, Parser)]
#[command(name = "Kontroll", version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Kontroll demonstates how to control the Keymapp API, making it easy to control your ZSA keyboard from the command line and scripts.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List all available keyboards")]
    List,
    #[command(about = "Connect to a keyboard given the index returned by the list command")]
    Connect {
        #[arg(short, long, name = "keyboard index", required = true)]
        index: usize,
    },
    #[command(about = "Connect to the first keyboard detected by keymapp")]
    ConnectAny,
    #[command(about = "Set the layer of the currently connected keyboard")]
    SetLayer {
        #[arg(short, long, required = true)]
        index: usize,
    },
    #[command(about = "Sets the RGB color of a LED")]
    SetRGB {
        #[arg(short, long, required = true)]
        led: usize,
        #[arg(short, long, required = true)]
        color: String,
        #[arg(short, long, default_value = "0")]
        sustain: i32,
    },
    #[command(about = "Sets the RGB color of all LEDs")]
    SetRGBAll {
        #[arg(short, long, required = true)]
        color: String,
        #[arg(short, long, default_value = "0")]
        sustain: i32,
    },
    #[command(about = "Set / Unset a status LED")]
    SetStatusLed {
        #[arg(short, long, required = true)]
        led: usize,
        #[arg(short, long)]
        off: bool,
        #[arg(short, long, default_value = "0")]
        sustain: i32,
    },
    #[command(about = "Increase the brightness of the keyboard's LEDs")]
    IncreaseBrightness,
    #[command(about = "Decrease the brightness of the keyboard's LEDs")]
    DecreaseBrightness,
    #[command(about = "Disconnect from the currently connected keyboard")]
    Disconnect,
}

pub async fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => match api::list_keyboards().await {
            Ok(keyboards) => {
                for (_i, keyboard) in keyboards.iter().enumerate() {
                    let connected = if keyboard.is_connected {
                        "(connected)"
                    } else {
                        ""
                    };
                    println!("{}: {} {}", keyboard.id, keyboard.friendly_name, connected);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Commands::Connect { index } => match api::connect(index).await {
            Ok(_) => {
                println!("Connected to keyboard {}", index);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Commands::ConnectAny => match api::connect_any().await {
            Ok(_) => {
                println!("Connected to the first keyboard detected by keymapp");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Commands::Disconnect => match api::disconnect().await {
            Ok(_) => {
                println!("Disconnected from the currently connected keyboard");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Commands::SetLayer { index } => match api::set_layer(index).await {
            Ok(_) => {
                println!("Layer set to {}", index);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Commands::SetRGB {
            led,
            color,
            sustain,
        } => {
            let (r, g, b) = match utils::hex_to_rgb(&color) {
                Ok(rgb) => rgb,
                Err(_) => {
                    eprintln!("{} is not a valid hex color", color);
                    return;
                }
            };

            match api::set_rgb_led(led, r, g, b, sustain).await {
                Ok(_) => {
                    println!("LED {} set to color {}", led, color);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Commands::SetRGBAll { color, sustain } => {
            let (r, g, b) = match utils::hex_to_rgb(&color) {
                Ok(rgb) => rgb,
                Err(_) => {
                    eprintln!("{} is not a valid hex color", color);
                    return;
                }
            };

            match api::set_rgb_all(r, g, b, sustain).await {
                Ok(_) => {
                    println!("All LEDs set to color {}", color);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Commands::SetStatusLed { led, off, sustain } => {
            let on = !off;
            match api::set_status_led(led, on, sustain).await {
                Ok(_) => {
                    let state = if on { "on" } else { "off" };
                    println!("Status LED {} turned {}", led, state);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Commands::IncreaseBrightness => match api::update_brightness(true).await {
            Ok(_) => {
                println!("Brightness increased");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Commands::DecreaseBrightness => match api::update_brightness(false).await {
            Ok(_) => {
                println!("Brightness decreased");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
    }
}