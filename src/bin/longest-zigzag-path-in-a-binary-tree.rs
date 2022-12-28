//
// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/
//

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

#[allow(unused)]
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn left_subtree(parent: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if parent.is_none() {
            return depth - 1;
        }

        let parent = parent.unwrap();
        let parent = (Rc::as_ref(&parent)).borrow();

        if parent.left.is_none() && parent.right.is_none() {
            return depth;
        }

        //let d = i32::max(Self::longest_zig_zag(parent.right.clone()), Self::longest_zig_zag(parent.left.clone()));

        let d = Self::left_subtree(parent.left.clone(), 1);

        let r = i32::max(Self::right_subtree(parent.right.clone(), depth + 1), d);
        r
    }

    fn right_subtree(parent: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if parent.is_none() {
            return depth - 1;
        }

        let parent = parent.unwrap();
        let parent = (Rc::as_ref(&parent)).borrow();

        if parent.left.is_none() && parent.right.is_none() {
            return depth;
        }

        //let d = i32::max(Self::longest_zig_zag(parent.right.clone()), Self::longest_zig_zag(parent.left.clone()));

        let d = Self::right_subtree(parent.right.clone(), 1);

        let r = i32::max(Self::left_subtree(parent.left.clone(), depth + 1), d);
        r
    }

    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let root = (Rc::as_ref(&root)).borrow();

        if root.left.is_none() && root.right.is_none() {
            return 0;
        }

        let left_depth = Self::left_subtree(root.left.clone(), 1);
        let right_depth = Self::right_subtree(root.right.clone(), 1);

        let r = i32::max(left_depth, right_depth);
        r
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    
    use crate::TreeNode;

    use super::Solution;

    fn both(
        //val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left,
            right
        })))
    }

    fn left(
        //val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        both(left, None)
    }

    fn right(
        //val: i32,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        both(None, right)
    }

    fn empty() -> Option<Rc<RefCell<TreeNode>>> {
        both(None, None)
    }

    #[test]
    fn test1() {
        let root = right(
            both(
                empty(),
                both(
                    right(
                        right(
                            empty()
                        ),
                    ),
                    empty(),
                )
            )
        );

        assert_eq!(
            Solution::longest_zig_zag(root),
            3
        );
    }

    #[test]
    fn test2() {
        let root = both(
            right(
                both(
                    right(
                        empty()
                    ),
                    empty()
                )
            ),
            empty(),
        );

        assert_eq!(
            Solution::longest_zig_zag(root),
            4
        );
    }

    #[test]
    fn test3() {
        let root = empty();

        assert_eq!(
            Solution::longest_zig_zag(root),
            0
        );
    }

    #[test]
    fn test4() {
        let root = left(left(empty()));

        assert_eq!(
            Solution::longest_zig_zag(root),
            1
        );
    }
}

fn main() { }
