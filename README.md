# flank

`flank` is a very basic CLI parsing application for CoH3 replays written using [vault](https://github.com/ryantaylor/vault).

Pre-built binaries are available in [releases](https://github.com/ryantaylor/flank/releases) for M1 Macs (ARM64), 64-bit Linux, and 64-bit Windows. Open an issue if you'd like another platform supported, or feel free to build from source.

## File & Version Support

`flank` supports all CoH3 replays regardless of version (same as `vault`).

## Company of Heroes 2

CoH2 replay support has been deprecated and will no longer be supported. If you still require CoH2 replay parsing, the last version to support it was [v0.6.0](https://github.com/ryantaylor/flank/tree/v0.6.0).

# Usage

[Download the binary for your platform](https://github.com/ryantaylor/flank/releases) and make sure it's in your path (alternatively you can [build from source](#building-from-source)). You should then be able to run `flank`:

```
$ flank
CLI parser for Company of Heroes replay files.

Usage: flank <FILE>

Arguments:
  <FILE>  Path to a CoH3 replay file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Parsing a file is easy, just pass the path to `flank`:

```
$ flank /path/to/replay.rec
```

The output is not currently pretty-printed, but its contents will look something like this:

```
{
  "version": 8369,
  "timestamp": "2023-02-23 21:18",
  "matchhistory_id": 150656,
  "map": {
    "filename": "data:scenarios\\multiplayer\\twin_beach_2p_mkii\\twin_beach_2p_mkii",
    "localized_name_id": "$11233976",
    "localized_description_id": "$11233977"
  },
  "players": [
    {
      "name": "calderon",
      "faction": "Wehrmacht",
      "team": "Second",
      "steam_id": 76561198307144220,
      "profile_id": 158704,
      "messages": [
        {
          "tick": 1821,
          "message": "..."
        },
        {
          "tick": 4064,
          "message": "bruh "
        },
        {
          "tick": 8445,
          "message": "ARE YOU ********** SURE ABOUT THAT"
        },
        {
          "tick": 8528,
          "message": "OH MY GHOD EHAT EVEN IS THIS"
        },
        {
          "tick": 8944,
          "message": "Kubel cant even shoot now"
        },
        {
          "tick": 9251,
          "message": "Wait your squads can pin????"
        }
      ]
    },
    {
      "name": "Inverse",
      "faction": "Americans",
      "team": "First",
      "steam_id": 76561197999799360,
      "profile_id": 8230,
      "messages": [
        {
          "tick": 8557,
          "message": "lmao"
        },
        {
          "tick": 8608,
          "message": "US is kinda busted tbh"
        },
        {
          "tick": 20965,
          "message": "ggwp"
        }
      ]
    }
  ],
  "length": 21001
}
```

# Building From Source

To build `flank`, you will need the latest version of the [Rust](https://www.rust-lang.org/) compiler. 

```bash
$ git clone https://github.com/ryantaylor/flank.git && cd flank
$ cargo build --release
```

# License

MIT
