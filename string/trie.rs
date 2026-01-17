struct Node{
    c: Option<char>, //そのノードの文字
    children: Vec<usize>, //子のインデックスの配列
}

impl Node{
    pub fn new(c: Option<char>) -> Self{
        Self{
            c,
            children: vec![usize::MAX;26],
        }
    }
}

pub struct Trie{
    tree: Vec<Node>,
}
impl Trie{
    pub fn new() -> Self{
        Self{tree: vec![Node::new(None)]}
    }
    
    pub fn insert(&mut self, word: &Vec<char>){
        let mut now = 0;
        for &c in word.iter(){
            let idx = c as usize - 'a' as usize;
            let mut nxt = self.tree[now].children[idx];
            if nxt == usize::MAX{
                nxt = self.tree.len();
                self.tree.push(Node::new(Some(c)));
                self.tree[now].children[idx] = nxt;
            }
            now = nxt;
        }
    }
}
