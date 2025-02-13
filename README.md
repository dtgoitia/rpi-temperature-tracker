# RPi5 temperature tracker

Logs CPU temperature to `~/.local/share/rpi-temperature-tracker`.

## Usage

Run:

```shell
nix run
```

Build:

```shell
nix build
```

## Development

To update flake dependencies:

```shell
nix flake update
```

Before deploying, make sure to run flake checks - see `checks` ouptut in [flake.nix](./flake.nix) for details:

```shell
nix flakes check
```
