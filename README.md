# iptvrs

![build](https://github.com/vihu/iptvrs/actions/workflows/rust.yml/badge.svg) ![release](https://github.com/vihu/iptvrs/actions/workflows/release.yml/badge.svg)

Playlist parser and search support for m3u4u playlists locally!

## What does this do?

- Gets your [m3u4u](https://m3u4u.com/) playlists.
- Stores the playlists locally in a sqlite database.
- Allows searching for channels (and playlists).
- Supports playing a channel with your choice of local player.

## Installation

You can find pre-built release for your system [here](https://github.com/vihu/iptvrs/releases).
Alternatively you can install using `cargo install iptvrs`

## Requirements

- [m3u4u](https://m3u4u.com) playlist.
- [Rust](https://www.rust-lang.org/) (rustc 1.70.0+).

## Setup

- Clone this repo:

```
$ git clone https://github.com/vihu/iptvrs.git
```

- Create the config directory:

```
$ mkdir -p $HOME/.config/iptvrs
```

- Copy settings:

```
$ cp settings.template.toml $HOME/.config/iptvrs/settings.toml
```

Edit the `settings.toml` as necessary (it is commented for reference).

## Build

In the repo root folder:

```
$ cargo build --release
```

Optional (but recommended), copy the binary to somewhere in your path:

```
$ cp ./target/release/iptvrs ~/.bin
```

## Run

The first thing you'd want to do is "hydrate" the database from your configured
playlist. Run this once in a while to fetch and rehydrate database as necessary
(I recommend once per day). Feel free to set a cron job to do this.

```
$ iptvrs hydrate
```

You can now search for channels in your playlist:

```
$ iptvrs search "<channel name>"
```

Assuming you've set a local player in your `settings.toml`, simply run:

```
$ iptvrs play <channel_index_integer>
```

## CLI

The `iptvrs` CLI is basically as follows:

```
iptvrs

Usage: iptvrs [OPTIONS] <COMMAND>

Commands:
  hydrate  Hydrate the database with the contents of playlist
  search   Search the database for a channel
  play     Play a channel using the configured player
  list     List all channels in the database
  help     Print this message or the help of the given subcommand(s)

Options:
  -c <CONFIG>
  -h, --help       Print help
  -V, --version    Print version
```
