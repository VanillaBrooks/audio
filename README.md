# Audio

Configure volume and output device (speakers, headphones) from the linux command line using `pactl`.

## System Dependencies 

You must have pulse audio controller (`pactl`) installed and available in your `$PATH`

## Installing

If you have `cargo` installed, you may build from source:

```
cargo install --git https://github.com/VanillaBrooks/audio
```

## Usage


```bash
audio --help
```

``` audio 0.1.0
Brooks
parse `pactl` output to easily set sink and its respective volume

USAGE:
    audio <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    device    Set the output device. Alias: `d`
    help      Print this message or the help of the given subcommand(s)
    volume    Set the volume to a specific percent (<100). Alias: `v`
```

## Examples

Set to headphones:

```
audio device headphones
```

```
audio d h
```

Set to speakers:

```
audio device speakers
```

```
audio d s
```

Set the volume to a specific percent (capped to 100%)

```
audio volume 30
```

```
audio v 30
```
