#[cfg(test)]
mod tests {
    use dsolver::helper::*;
    use std::collections::BinaryHeap;

    #[test]
    fn test_parse_hole(){
        let mut case1=BinaryHeap::new();
        case1.push((14,'c'));
        case1.push((5,'h'));
        let string1="ac5h".to_string();
        assert_eq!(parse_hole(string1).pop(),case1.pop());
    }

    #[test]
    fn test_hole_strength(){
        let mut hole1=BinaryHeap::new();
        hole1.push((14,'c'));
        hole1.push((5,'c'));
        assert_eq!(hole_strength(&hole1),3);
        let mut hole2=BinaryHeap::new();
        hole2.push((14,'c'));
        hole2.push((10,'h'));
        assert_eq!(hole_strength(&hole2),2);
    }

    #[test]
    fn test_check_pair(){
        let mut hole1=BinaryHeap::new();
        hole1.push((14,'c'));
        hole1.push((5,'c'));
        assert_eq!(check_pair(&hole1),None);
        let mut hole2=BinaryHeap::new();
        hole2.push((10,'c'));
        hole2.push((10,'h'));
        assert_eq!(check_pair(&hole2),std::option::Option::Some(10));
    }

    #[test]
    fn test_hole_suited(){
        let mut hole1=BinaryHeap::new();
        hole1.push((14,'c'));
        hole1.push((5,'c'));
        assert_eq!(hole_suited(&hole1),true);
        let mut hole2=BinaryHeap::new();
        hole2.push((10,'c'));
        hole2.push((10,'h'));
        assert_eq!(hole_suited(&hole2),false);
    }

    #[test]
    fn test_flop_top_pair(){
        let mut hole=BinaryHeap::new();
        hole.push((10,'c'));
        hole.push((5,'c'));
        let mut board=BinaryHeap::new();
        board.push((10,'c'));
        board.push((10,'h'));
        board.push((4,'s'));
        board.push((14,'c'));
        board.push((5,'c'));
        assert_eq!(flop_top_pair(&hole,&board),std::option::Option::Some(false));
    }
}