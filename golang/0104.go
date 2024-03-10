func maxDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    l := maxDepth(root.Left) + 1
    r := maxDepth(root.Right) + 1
    if l > r {
        return l
    }
    return r
}
