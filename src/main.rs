use anyhow::Context;
use anyhow::Result;

use std::fmt;

use clap::Parser;

use xshell::{cmd, Shell};

#[derive(Parser)]
#[clap(
    author = "Brooks",
    version,
    about = "parse `pactl` output to easily set sink and its respective volume"
)]
enum Args {
    #[clap(alias="v")]
    /// Set the volume to a specific percent (<100). Alias: `v`
    Volume(Volume),
    #[clap(subcommand)]
    #[clap(alias="d")]
    /// Set the output device. Alias: `d`
    Device(Device),
}

#[derive(Parser)]
struct Volume {
    volume: usize,
}

#[derive(Parser, Clone)]
enum Device {
    #[clap(alias="h")]
    /// Use headphone audio. Alias: `h`
    Headphones,
    #[clap(alias="s")]
    /// Use speaker audio. Alias: `s`
    Speakers,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Self::Headphones => write!(f, "headphones"),
            Self::Speakers => write!(f, "speakers")
        }
    }
}

struct Sink {
    id: usize,
    desc: String,
}

impl Sink {
    fn new(desc: String) -> Result<Self> {
        let id = desc
            .as_str()
            .chars()
            .take_while(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .with_context(|| {
                format!(
                    "failed to parse leading numerical characters from pactl description {desc}"
                )
            })?;

        Ok(Self { id, desc })
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args {
        Args::Volume(v) => set_volume(v),
        Args::Device(d) => set_device(d),
    }?;

    Ok(())
}

fn set_device(device: Device) -> Result<()> {
    let sh = Shell::new()?;

    let sink_string_list = cmd!(sh, "pactl list short sinks").read()?;

    let sinks = sink_string_list
        .split('\n')
        .map(|x| Sink::new(x.into()))
        .filter_map(|sink_result| match sink_result {
            Ok(sink) => Some(sink),
            Err(e) => {
                println!("warning: failed to parse sink: {e}");
                None
            }
        });

    let sink = match device {
        Device::Speakers => {
            sinks
                .filter(|sink| sink.desc.contains("analog-stereo") && sink.desc.contains("pci"))
                .next()
        }
        Device::Headphones => {
            sinks
                .filter(|sink| sink.desc.contains("analog-stereo") && sink.desc.contains("Hyper"))
                .next()
        }
    };

    if let Some(sink) = sink {
        let id = sink.id.to_string();
        let mut cmd = cmd!(sh, "pactl set-default-sink {id}");
        cmd.set_quiet(true);
        cmd.run()
            .with_context(|| format!("failed to swap sink to id {id}"))?;
    } else {
        anyhow::bail!("failed to find corresponding sink for {device}, is it connected?");
    }

    Ok(())
}

fn set_volume(volume: Volume) -> Result<()> {
    let sh = Shell::new()?;

    if volume.volume > 100 {
        anyhow::bail!("cant set volume higher than 100%");
    }

    let vol_percent = volume.volume.to_string();

    let mut cmd = cmd!(sh, "pactl set-sink-volume @DEFAULT_SINK@ {vol_percent}%");
    cmd.set_quiet(true);
    cmd.run()
        .with_context(|| format!("failed to change volume to {vol_percent}%"))?;

    Ok(())
}
