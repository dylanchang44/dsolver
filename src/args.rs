use std::collections::HashSet;
use substring::Substring;
use clap::{Parser,Subcommand, Args};

#[derive(Debug,Parser,Clone)]
#[command(author="Dylan Chang",version,about="A cli tool that generate long-ball/small-ball poker strategy at preflop/flop")]
pub struct DsolverArgs{
    #[clap(subcommand)]
    pub stage: Stage,
    #[arg(value_enum, short, long)]
    pub strategy:Strategy,
}

#[derive(clap::ValueEnum, Clone, Debug, strum_macros::Display)]
pub enum Strategy{
    LongBall,
    SmallBall,
}

#[derive(Subcommand, Debug, Clone, strum_macros::Display)]
pub enum Stage{
    PreFlop(PreFlopCommand),
    Flop(FlopCommand),
    // Turn,
    // River,
}

#[derive(Debug,Args,Clone)]
pub struct PreFlopCommand{
    pub hole: String,
    
    pub position: i32,
    
    pub caller: i32,
    
    pub limper: i32,
}

#[derive(Debug,Args,Clone)]
pub struct FlopCommand{
    pub hole: String,
    
    pub board: String,
    
    pub distribution: char
}

impl PreFlopCommand {
    pub fn validate_int(&self) -> Result<(), String> {
        if self.caller > 0 && self.limper > 0 {
            Err(String::from("caller and limper cannot both be positive"))
        } else if self.caller < 0 || self.limper < 0{
            Err(String::from("caller or limper cannot be negative"))
        }
        else {
            Ok(())
        }
    }
}

impl FlopCommand {
    pub fn validate_cards(&self) -> Result<(), String> {
        //hashset to keep track of cards
        let mut set = HashSet::new();
        //construct card str slice
        let hole_slice=self.hole.clone();
        let board_slice=self.board.clone();
        let cards=hole_slice+&board_slice;
        for i in 0..=5 {
            if set.contains(cards.substring(2*i, 2*i+2)){
                return Err(String::from("Sensed duplicated cards"));
            }
            else {set.insert(cards.substring(2*i, 2*i+2));}
        }
        Ok(())
        }
}