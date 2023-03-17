# dsolver

<h4 align="center">Command line interface of Texas Holdem Beginner's Strategy for Pre-FLop & FLop</h4>

## Features

* Consume the input of cards information, and output the suggested move.
* At Pre-FLop stage, give consideration to player's position and number of callers before.
* Include Long-ball(tight-aggressive) and Small-Ball(loose-passive) route to choose.
* Optional odd distribution feature at FLop, calculate the odd of hands by river.

## Installation

```sh
brew install rustup
git clone git@github.com:reznorchang/dsolver.git <yourpath>
cd <yourpath>
cargo install --path .
```

## Usage

### --help
```sh
dsolver --help
```

```
A cli tool that generate long-ball/small-ball poker strategy at preflop/flop

Usage: dsolver --strategy <STRATEGY> <COMMAND>

Commands:
  pre-flop  
  flop      
  help      Print this message or the help of the given subcommand(s)

Options:
  -s, --strategy <STRATEGY>  [possible values: long-ball, small-ball]
  -h, --help                 Print help
  -V, --version              Print version
```
### pre-flop
### flop
