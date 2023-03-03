use std::{collections::BinaryHeap, vec};
use bio::stats::combinatorics::combinations;
use strum_macros::Display;
use std::cmp::max;

//All kind of hand type
#[derive(Debug, PartialEq, Clone, Copy, Hash, PartialOrd, Eq, Display)]
pub enum HandType {
    //HighCard,
    //Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
}

pub fn flop_distribution(cards: &BinaryHeap<(i8,char)>)->Vec<f32>{
    //for odd of following handtype
    //2pair 1
    //ThreeOfAKind 1
    //Straight 2
    //Flush 2
    //FullHouse 1
    //FourOfAKind 1
    let m_odds=multicard_odds(process_multicards(cards));
    let sf_odds=straight_flush_odds(cards);
    let mut odd_vec=vec![m_odds.0,m_odds.1,sf_odds.0,sf_odds.1,m_odds.2,m_odds.3];
    odd_vec.iter_mut().for_each(|e| *e= *e * 100.0);
    odd_vec
}

//generate odd by out
fn flop_out_to_perc(out:f32)->f32{
    let left:f32=52.0-5.0;
    1.0 - (left-out)/left * (left-1.0-out)/(left-1.0)
}

//process straight, flush odd 
fn straight_flush_odds(cards: &BinaryHeap<(i8,char)>)-> (f32, f32){
    //return (straight_odds,flush_odds)
    let mut straight_odds = 0.0;
    let mut flush_odds = 0.0;

    let mut flop: Vec<(i8, char)> = cards.clone().into_sorted_vec();
    //process A for straight judgment
    if flop[4].0==14 {flop.insert(0, (1,flop[4].1));}   
    
    // Check if straight exist or open-ended straight draws
    let mut pre=flop[0].0;
    let mut straight_vec=vec![flop[0].0];
    for i in 1..flop.len(){
        //element is pre+1
        if flop[i].0==pre+1 {
            straight_vec.push(flop[i].0);
        }
        //element is pre
        else if flop[i].0==pre {continue;}
        //element is none, reset
        else {
            if straight_vec.len()>=4 {break;}
            straight_vec=vec![flop[i].0];
        }
        //update pre
        pre=flop[i].0;
    }

    if straight_vec.len()==5 {straight_odds=1.0;}
    else if straight_vec.len()==4 {
        if straight_vec[0]==1 || straight_vec[3]==14 {straight_odds=flop_out_to_perc(4.0);}
        else {straight_odds=flop_out_to_perc(8.0);}
    }

    // Check for gutshot straight draws or backdoor straight draws
    if straight_odds==0.0 {
        let mut pre=flop[0].0;
        let mut straight_vec=vec![flop[0].0];
        let mut miss_card=0;
        for i in 1..flop.len(){
            //element is pre+2
            if flop[i].0==pre+2 {
                miss_card+=1;
                if miss_card>2 {break;}
                straight_vec.push(flop[i].0);
            }
            //element is pre+1
            else if flop[i].0==pre+1 {
                straight_vec.push(flop[i].0);
            }
            //element is pre
            else if flop[i].0==pre {continue;}
            //element is none, reset
            else {
                if straight_vec.len()>=3 {break;}
                straight_vec=vec![flop[i].0];
                miss_card=0;
            }
            //update pre
            pre=flop[i].0;
        }
    
        if straight_vec.len()>=4 && straight_vec[straight_vec.len()-1]-straight_vec[0]<5 {straight_odds=flop_out_to_perc(4.0);}
        else if straight_vec.len()>=3 {straight_odds=0.03;}
    }

// Check for flush draws
    if flop[0].0==1 {flop.pop();}
    let mut max_suit_count=0;
    for suit in ['c', 'd', 'h', 's'].iter() {
        let suit_count = flop.iter().filter(|&&(_, s)| &s == suit).count();
        max_suit_count=max(max_suit_count,suit_count);
    }
    match max_suit_count{
        5=>flush_odds=1.0,
        4=>flush_odds=flop_out_to_perc(9.0),
        3=>flush_odds=10.0/47.0 * 9.0/46.0,
        _=>()
    }

    (straight_odds,flush_odds)
}

//process 2pair,ThreeOfAKind,FullHouse,FourOfAKind off
fn multicard_odds(hand_status:(i8,bool,bool))->(f32,f32,f32,f32){
    let mut odd=(0.0,0.0,0.0,0.0);
    if hand_status.2 {odd.3=1.0;}
    else if hand_status.1{
        if hand_status.0>0{
            odd.2=1.0;
        }else{
            odd.1=1.0;
            odd.2=flop_out_to_perc(6.0);
        }
        odd.3=flop_out_to_perc(1.0);
    }else{
        match hand_status.0{
            2=>{
                odd.0=1.0;
                odd.2=flop_out_to_perc(4.0);
                odd.3=2.0 * 2.0/47.0 * 1.0/46.0;
            },
            1=>{
                odd.0=flop_out_to_perc(9.0);
                odd.1=flop_out_to_perc(2.0);
                odd.2=2.0 * 2.0/47.0 * 9.0/46.0;
                odd.3=2.0/47.0 * 1.0/46.0;
            },
            _=>{
                odd.0=combinations(5,2) as f32 * 2.0* 3.0/47.0 * 3.0/46.0;
                odd.1=5.0 * 3.0/47.0 * 2.0/46.0;
            }
        }
    }
    odd
}

//return (pairs,trips,quads)
fn process_multicards(cards: &BinaryHeap<(i8,char)>)->(i8,bool,bool){
    let mut heap=cards.clone();
    let mut type_count=(0,false,false);
    let mut type_counter=vec![1,1,1];
    let mut prev:i8=0;
    for _ in 0..5{
        let num=heap.pop().unwrap().0;
        if prev==num {
            type_counter.iter_mut().for_each(|x| *x+=1);
            
            if type_counter[2]==4 {        //if we have a quads, end fn
                type_count=(0,false,true);
                break;
            }else if type_counter[1]==3 {  //if we have a trips, trips=true
                type_count.0-=1;
                type_count.1=true;
               type_counter_reset(&mut type_counter,2);
            }else{                        //if we have a trips, pair+1
                type_count.0+=1;
                type_counter_reset(&mut type_counter,1);
            }
        }else {type_counter_reset(&mut type_counter,3);}
        prev=num;
    }
    // println!("type_counter is {:?}", type_counter);
     //println!("tuple is {:?}", type_count);
    type_count
}

fn type_counter_reset(counter:&mut Vec<i8>,range:usize){
    for i in 0..range{
        counter[i]=1;
    }
}
