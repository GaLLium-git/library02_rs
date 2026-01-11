struct Node{
    children: Vec<char>, //子の配列
    common: usize, //このノードまでを共有する文字列の数
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
