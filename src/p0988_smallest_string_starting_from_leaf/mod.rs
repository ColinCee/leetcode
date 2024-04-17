use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
        // BFS solution
        // Use BFS to find the shortest paths to leaf nodes
        // When the first leaf node is found we know that is the shortest string length
        // Keep track of the current path with ref to the node
        //      This will allow you to keep track of the depth
        // At the end we will have a list of paths

        // Each path in list is the same size
        // If only one path exists return it
        // Starting from index 1, get the paths with min of all index 1 chars
        //      If we equal get min of i - 1
        // Increment i + 1 until only one path remains
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        // DFS
        // Keep track of current string for until first leaf, reverse it
        // Recursively find next leaf, return early if depth is > current min string length
        // If string len is shorter than min replace with current
        // If all characters are less than min, replace with current
    }
}

pub fn solve() {
    // Implement the solution here
}
