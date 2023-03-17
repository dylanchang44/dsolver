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
Command:
```sh
dsolver --help
```
OutPut:
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
Usage: 
```sh
Usage: dsolver --strategy <STRATEGY> pre-flop <HOLE> <POSITION> <CALLER> <LIMPER>
```
Command:
```
dsolver --strategy long-ball pre-flop ahkh 1 0 5
```
OutPut:
```
Move: Bet 9 BB
```
### flop
Usage: 
* <DISTRIBUTION> take char 'y' as positive to generate odd distribution, other char as negative.
```sh
Usage: dsolver --strategy <STRATEGY> flop <HOLE> <BOARD> <DISTRIBUTION>
```
Command:
```
dsolver --strategy small-ball flop ahkd 5c5h9s y
```
OutPut:
```
Move: Check/Fold

Hand Odd by River:
TwoPair-------35.0%
ThreeOfAKind--8.4%
FullHouse-----1.7%
FourOfAKind---0.1%
Straight------0.0%
Flush---------0.0%
```
