struct Node{
    children: Vec<char>, //子の配列
    accept: Vec<usize>, //受理する文字列のインデックス
}

pub struct Trie{
    tree: Vec<Node>,
}
impl Trie{
    pub fn new() -> Self{
        Self{tree: Vec::new()},
    }
    
    pub fn insert(&mut self, word: &Vec<char>){
        
    }
}
