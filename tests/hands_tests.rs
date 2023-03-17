#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use dsolver::hands::*;

    #[test]
    fn test_flop_out_to_perc() {
        let sample2=flop_out_to_perc(2.0);
        let sample5=flop_out_to_perc(5.0);
        assert_eq!(sample2,0.08418131);
        assert_eq!(sample5,0.20351523);
    }

    #[test]
    fn test_process_multicards(){
        let mut fullhouse=BinaryHeap::new();
        fullhouse.push((2,'c'));
        fullhouse.push((2,'d'));
        fullhouse.push((2,'h'));
        fullhouse.push((10,'s'));
        fullhouse.push((10,'c'));
        let tuple1=process_multicards(&fullhouse);
        let mut two_pair=BinaryHeap::new();
        two_pair.push((14,'d'));
        two_pair.push((14,'s'));
        two_pair.push((6,'d'));
        two_pair.push((6,'h'));
        two_pair.push((8,'c'));
        let tuple2=process_multicards(&two_pair);
        assert_eq!(tuple1,(1,true,false));
        assert_eq!(tuple2,(2,false,false));
    }

    #[test]
    fn test_multicard_odds(){  //2pair,ThreeOfAKind,FullHouse,FourOfAKind odd
        assert_eq!(multicard_odds((2,false,false)),(1.0,0.0,0.1646623, 0.0018501387));
        assert_eq!(multicard_odds((0,true,false)),(0.0, 1.0, 0.24144316, 0.042553186));
    }

    #[test]
    fn test_straight_flush_odds(){
        let mut openend=BinaryHeap::new();
        openend.push((7,'d'));
        openend.push((8,'d'));
        openend.push((14,'d'));
        openend.push((10,'h'));
        openend.push((9,'c'));
        let mut gutshot_flushdraw=BinaryHeap::new();
        gutshot_flushdraw.push((7,'c'));
        gutshot_flushdraw.push((4,'h'));
        gutshot_flushdraw.push((3,'c'));
        gutshot_flushdraw.push((5,'c'));
        gutshot_flushdraw.push((13,'c'));
        assert_eq!(straight_flush_odds(&openend),(0.31452358, 0.041628122));
        assert_eq!(straight_flush_odds(&gutshot_flushdraw),(0.1646623, 0.3496762));
    }
}