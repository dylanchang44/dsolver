//Take care of the judgmental logic behind
use std::collections::BinaryHeap;

pub fn parse_hole(s:String)->BinaryHeap<(i8,char)>{
    let suits=vec!['s','h','d','c'];
    if s.len()!=4 {panic!("Unvalid hole card format")}
    let mut tuple=(1,'a'); //need to initiate tuple
    let mut hole_card=BinaryHeap::new();
    s.chars().enumerate().for_each(|(i,c)| {
        if i%2==0 {
                match c{
                    't' => tuple.0=10,
                    'j' => tuple.0=11,
                    'q' => tuple.0=12,
                    'k' => tuple.0=13,
                    '1'|'a'=> tuple.0=14,
                     _  => {if c.is_alphabetic() {panic!("Unvalid hole card rank");}
                            else {tuple.0=c.to_digit(10).unwrap() as i8}
                            }
                }
            }
        else{
            if suits.iter().any(|&x|x==c) {tuple.1=c;}
            else {panic!("Unvalid hole card suit");}
            hole_card.push(tuple);
            }
        
    });
    hole_card
}

pub fn parse_board(hole:String, board:String)->BinaryHeap<(i8,char)>{
    let suits=vec!['s','h','d','c'];
    if hole.len()!=4 {panic!("Unvalid hole card size")}
    if board.len()!=6 {panic!("Unvalid board card size")}
    let mut tuple=(1,'a');      //need to initiate tuple
    let mut hole_board=BinaryHeap::new();
    let hole_board_string=hole+&board;
    //panic!("{}",hole_board_string);
    hole_board_string.chars().enumerate().for_each(|(i,c)| {
        if i%2==0 {
                match c{
                    't' => tuple.0=10,
                    'j' => tuple.0=11,
                    'q' => tuple.0=12,
                    'k' => tuple.0=13,
                    '1'|'a'=> tuple.0=14,
                     _  => {if c.is_alphabetic() {panic!("Unvalid card rank");}
                            else {tuple.0=c.to_digit(10).unwrap() as i8}
                            }
                }
            }
        else{
            if suits.iter().any(|&x|x==c) {tuple.1=c;}
            else {panic!("Unvalid card suit");}
            hole_board.push(tuple);
            }
            
    });
    hole_board
}

pub fn hole_strength(cards: &BinaryHeap<(i8,char)>)->i8{
    let mut tier:i8=5; //tier range: 0-5
    let min=hole_min(cards);
    //identify tier
    if let Some(x)=check_pair(cards){    //pair condition
        match x{
            11..=14 => tier=0,
            8..=10 => tier=1,
            2..=7 => tier=2,
            _=>panic!("Unvalid pair value")
        }
    }else if min>=10{          //high cards condition
            match min{
                13=> tier=0,
                12=> tier=1,
                10|11=> tier=2,
                _=>()
            }
    }else if hole_suited(cards){            //suited condition
        if cards.peek().unwrap().0==14 {tier=3;}
        else if hole_connected(cards) {
            if min>7  {tier=3;}
            else {tier=4;}
    }
}
    tier
}

pub fn check_pair(cards: &BinaryHeap<(i8,char)>) -> Option<i8>{
    let mut heap=cards.clone();
    let mut prev:i8=0;
    while heap.len()!=0 {
        let num=heap.pop().unwrap().0;
        if prev==num {return Some(num);}
        else {prev=num;}
    }
    None
}

fn hole_suited(cards: &BinaryHeap<(i8,char)>)->bool{
    let mut prev='a';
    for card in cards{
        if prev==card.1 {return true;}
        else {prev=card.1;}
    }
    false
}

pub fn hole_min(cards: &BinaryHeap<(i8,char)>)->i8{
    let mut min:i8=14;
    for card in cards{
        if card.0<min {min=card.0;}
        }
    min
}

fn hole_connected(cards: &BinaryHeap<(i8,char)>)->bool{
    let mut prev=i8::MIN;
    for card in cards{
        if i8::abs(prev-card.0)<5 {return true;}
        else {prev=card.0;}
    }
    false
}

pub fn flop_top_pair(hole:&BinaryHeap<(i8,char)>,board:&BinaryHeap<(i8,char)>)->Option<bool>{
    //search for pair, if not return NONE
    let mut pair_rank=0;
    if let Some(rank)=check_pair(board){
        let mut have_pair=false;
        for card in hole{
            if rank==card.0 {
                have_pair=true;
                pair_rank=rank;
            }
        }
        if !have_pair {return None;}
    }else{
        return None;
    }
    //if hole have one pair, check it's top pair(true) or middle pair(false)
    let mut heap=board.clone();
    let top=heap.pop().unwrap().0;
    let mid=heap.pop().unwrap().0;
    if pair_rank==top {return Some(true)};
    if pair_rank==mid {return Some(false)};
    None
}