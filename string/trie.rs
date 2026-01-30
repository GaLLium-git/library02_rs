struct Node{
    children: Vec<usize>, //子のインデックスの配列
    //残りの状態
    cnt: usize,
}

impl Node{
    pub fn new() -> Self{
        Self{
            children: vec![usize::MAX;26],
            cnt: 0,
        }
    }
}

pub struct Trie{
    tree: Vec<Node>,
    //残りの状態
    ans: usize, 
}

impl Trie{
    pub fn new() -> Self{
        Self{
            tree: vec![Node::new()],
            ans: 0,
        }
    }
    
    pub fn insert(&mut self, word: &Vec<char>){
        let mut now = 0;
        for &c in word.iter(){
            let idx = c as usize - 'a' as usize;
            let mut nxt = self.tree[now].children[idx];
            if nxt == usize::MAX{
                nxt = self.tree.len();
                self.tree.push(Node::new());
                self.tree[now].children[idx] = nxt;
            }
            now = nxt;
            //処理
            self.ans += self.tree[now].cnt;
            self.tree[now].cnt += 1;
        }
    }
}
