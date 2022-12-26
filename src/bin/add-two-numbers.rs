//
// https://leetcode.com/problems/add-two-numbers/
//

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::collections::VecDeque;

        let mut result = None;

        let mut lp1 = l1;
        let mut lp2 = l2;
        let mut carry = false;
        let mut end;

        let mut deq = VecDeque::new();
        
        loop {
            end = false;

            let sum = if carry { 1 } else { 0 } + match (lp1.take(), lp2.take()) {
                (Some(mut n1), Some(mut n2)) => {
                    lp1 = n1.next.take();
                    lp2 = n2.next.take();
                    n1.val + n2.val
                }

                (None, Some(mut n2)) => {
                    lp2 = n2.next.take();
                    n2.val
                }

                (Some(mut n1), None) => {
                    lp1 = n1.next.take();
                    n1.val
                }

                _ => {
                    end = true;
                    0
                }
            };

            if !carry && !deq.is_empty() && end {
                break;
            }

            deq.push_back(sum % 10);


            carry = sum >= 10;
        }

        while let Some(val) = deq.pop_back() {
            result = Some(Box::new(ListNode { val, next: result }));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, ListNode};

    #[test]
    fn test() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(1)));
        let result = Some(Box::new(ListNode::new(1)));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            result
        );
    }

    #[test]
    fn test2() {
        let l1 = Some(Box::new(ListNode::new(1)));
        let l2 = Some(Box::new(ListNode::new(1)));
        let result = Some(Box::new(ListNode::new(2)));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            result
        );
    }

    #[test]
    fn test3() {
        let l1 = Some(Box::new(ListNode { 
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));

        let l2 = Some(Box::new(ListNode { 
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let result = Some(Box::new(ListNode { 
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: None
                }))
            }))
        }));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            result
        );
    }

    #[test]
    fn test4() {
        let l1 = Some(Box::new(ListNode { 
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: None
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }));

        let l2 = Some(Box::new(ListNode { 
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: None
                    }))
                }))
            }))
        }));

        let result = Some(Box::new(ListNode { 
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 1,
                                        next: None
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            result
        );
    }
}

fn main() { }
