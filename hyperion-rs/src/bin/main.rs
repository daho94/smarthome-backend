use hyperion_rs::palette::named;
use hyperion_rs::Hyperion;
use palette::rgb::Srgb;
use palette::Component;
use std::io::Error as IOError;
use structopt::StructOpt;

type HyperionResult = Result<(), IOError>;

#[derive(Debug, StructOpt)]
struct ColorRgb {
    /// Amount of red [0-255]
    red: u8,
    /// Amount of green [0-255]
    green: u8,
    /// Amount of blue [0-255]
    blue: u8,
}

#[derive(Debug, StructOpt)]
struct ColorNamed {
    /// Name of color e.g. "red", "whitesmoke" (See https://www.w3.org/TR/css-color-3/#svg-color for all possible values)
    name: String,
}

#[derive(Debug, StructOpt)]
enum Effect {
    /// Set effect e.g. "Rainbow swirl", "Blue mood blobs"
    #[structopt(name = "set")]
    Set { name: String },
    /// List all available effects
    #[structopt(name = "list")]
    List,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Set color from RGB values
    #[structopt(name = "rgb")]
    ColorRgb(ColorRgb),
    /// Set color
    #[structopt(name = "color")]
    ColorNamed(ColorNamed),
    /// Set effect or list available effects
    #[structopt(name = "effect")]
    Effect(Effect),
    /// Clear all current effects or colors
    #[structopt(name = "clear")]
    ClearAll,
    /// Get current serverinfo
    #[structopt(name = "serverinfo")]
    ServerInfo,
    /// Lighten active color
    #[structopt(name = "lighten")]
    Lighten,
    /// Darken active color
    #[structopt(name = "darken")]
    Darken,
}

impl Command {
    fn execute(&self, address: &str) -> HyperionResult {
        use Command::*;

        let hyperion = Hyperion::new(address);

        match self {
            ColorRgb(values) => {
                let srgb = Srgb::new(
                    values.red.convert(),
                    values.green.convert(),
                    values.blue.convert(),
                )
                .into_linear();
                hyperion.set_color(srgb.into())
            }
            ColorNamed(color) => {
                let color_named = named::from_str(&color.name)
                    .expect("Not a valid color")
                    .into_format::<f32>()
                    .into_linear();
                hyperion.set_color(color_named.into())
            }
            Effect(option) => {
                use crate::Effect::*;
                match option {
                    Set { name } => hyperion.set_effect(&name),
                    List => {
                        if let Some(effects) = hyperion.get_effects()
                        {
                            println!("-----Available effects-----");
                            effects.iter().for_each(|e| println!("{}", e));
                        }

                        Ok(())                        
                    },
                }
            }
            ClearAll => hyperion.clear_all(),
            ServerInfo => {
                let info = hyperion.get_serverinfo()?;
                println!("{}", info);
                Ok(())
            }
            Lighten => hyperion.lighten(0.1),
            Darken => hyperion.darken(0.1),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "hyperion-remote")]
struct ApplicationArguments {
    #[structopt(subcommand)]
    command: Command,
    /// Address to hyperion server
    #[structopt(short = "a", long = "address", default_value = "192.168.178.48:19444")]
    address: String,
}

fn main() {
    let opt = ApplicationArguments::from_args();

    match opt.command.execute(&opt.address) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
