# flank

`flank` is a very basic CLI parsing application for CoH2 replays written using [vault](https://github.com/ryantaylor/vault). It is currently powering [COH2.ORG's replay section](http://coh2.org/replays).

I've only tested `flank` on Linux, but it should work fine on other platforms provided you are able to build it. The remainder of this documentation assumes you're running Linux.

## File & Version Support

`flank` supports every filetype and CoH2 version that [vault](https://github.com/ryantaylor/vault) does. Currently that means `.rec` files, `.zip` files, and directories, and replays from CoH2 versions 19545 and higher.

# Usage

To build `flank`, you will need the latest version of the [Rust](https://www.rust-lang.org/) compiler. Only Rust stable has been tested, but the beta and nightly compilers should work fine as well.

```bash
$ git clone https://github.com/ryantaylor/flank.git && cd flank
$ cargo build --release
```

Once `flank` has been built, you'll have to add the output directory to `$PATH`:

```bash
$ export PATH="${PATH}:/path/to/flank/target/release"
```

You should then be able to run `flank`:

```bash
$ flank
Usage: flank [options] FILE

Options:
    -l, --log           enable logging to stdout
    -a, --array         output replays as array inside wrapper JSON object
    -v, --version       print version information
    -h, --help          print this help menu
```

Parsing a file is easy, just pass the path to `flank`:

```bash
$ flank /path/to/replay.rec
```

The output is not currently pretty-printed, but its contents will look something like this:

```
{
   "error":null,
   "file":{
      "name":"game_1.rec",
      "data":[

      ],
      "cursor":1719336
   },
   "version":19654,
   "game_type":"COH2_REC",
   "date_time":"21-9-2015 2:10",
   "map":{
      "file":"DATA:scenarios\\mp\\2p_angoville_farms\\2p_angoville_farms",
      "name":"$11085926",
      "description":"$11085927",
      "description_long":"",
      "width":320,
      "height":320,
      "players":2
   },
   "players":[
      {
         "name":"IIPLIICaptainSPrice",
         "steam_id":76561198069018038,
         "team":0,
         "faction":"aef",
         "items":[
            {
               "id":384483191710633501,
               "item_type":"Skin"
            },
            {
               "id":327596199551258140,
               "item_type":"Skin"
            },
            {
               "id":82914536923155070,
               "item_type":"Skin"
            },
            {
               "id":0,
               "item_type":"FacePlate"
            },
            {
               "id":243665099703974684,
               "item_type":"VictoryStrike"
            },
            {
               "id":698857544894308312,
               "item_type":"Decal"
            },
            {
               "id":77523824685537325,
               "item_type":"Commander"
            },
            {
               "id":77523790325798959,
               "item_type":"Commander"
            },
            {
               "id":77523794620766254,
               "item_type":"Commander"
            },
            {
               "id":0,
               "item_type":"Bulletin"
            },
            {
               "id":0,
               "item_type":"Bulletin"
            },
            {
               "id":0,
               "item_type":"Bulletin"
            }
         ]
      },
      {
         "name":"Symbiosis",
         "steam_id":76561198035487085,
         "team":1,
         "faction":"german",
         "items":[
            {
               "id":86279841367726836,
               "item_type":"Skin"
            },
            {
               "id":86279880022432499,
               "item_type":"Skin"
            },
            {
               "id":289762253547248734,
               "item_type":"Skin"
            },
            {
               "id":86279807007988842,
               "item_type":"FacePlate"
            },
            {
               "id":453399532567654104,
               "item_type":"VictoryStrike"
            },
            {
               "id":698857261426466776,
               "item_type":"Decal"
            },
            {
               "id":452177056025811705,
               "item_type":"Commander"
            },
            {
               "id":86279721108641576,
               "item_type":"Commander"
            },
            {
               "id":86279927267324747,
               "item_type":"Commander"
            },
            {
               "id":0,
               "item_type":"Bulletin"
            },
            {
               "id":0,
               "item_type":"Bulletin"
            },
            {
               "id":0,
               "item_type":"Bulletin"
            }
         ]
      }
   ],
   "duration":32000,
   "rng_seed":1412721418,
   "opponent_type":1,
   "chat":[
      {
         "tick":76,
         "name":"IIPLIICaptainSPrice",
         "message":"gl hf"
      },
      {
         "tick":102,
         "name":"Symbiosis",
         "message":"u2"
      },
      {
         "tick":127,
         "name":"Symbiosis",
         "message":"be nice pls ^^"
      },
      {
         "tick":15199,
         "name":"Symbiosis",
         "message":"no damge from teller!?"
      },
      {
         "tick":31654,
         "name":"IIPLIICaptainSPrice",
         "message":"gardening at wall"
      },
      {
         "tick":31686,
         "name":"Symbiosis",
         "message":"yeye"
      },
      {
         "tick":31701,
         "name":"Symbiosis",
         "message":"spam more"
      }
   ]
}
```

# Logging

If you wish to enable logging in `flank`, you must first create a log configuration. This file goes in your home directory:

```bash
$ cd ~
$ mkdir .flank && cd .flank
$ touch log.toml
```

You can then edit log.toml with a logging configuration. `flank` uses [log4rs](https://github.com/sfackler/log4rs), so any valid `log4rs` configurations will work, such as this simple config that outputs logging to stdout:

```toml
refresh_rate = 99999

[appender.stdout]
kind = "console"
pattern = "%d{%H:%M:%S} [%l] %f: %m"

[root]
level = "info"
appenders = ["stdout"]
```

You can then run `flank` with the `-l` flag to enable logging:

```bash
$ flank -l /path/to/replay.rec
```