//Li Chao Tree
pub struct LiChaoTree<T, F, Eval>
where
    Eval: Fn(&F, T) -> T,
{
    val:Vec<T>
    best:Vec<F>
}

impl LiChaoTree{
    pub fn new(val:&Vec<T>) -> Self{
        let log = (val.len().ilog2() +1) as usize;
        let len = (1<<log);
        Self{
            value,
            best: vec![;len];
        }
    }
    
    pub fn add(&mut self,f:F){
        
    }
    
    pub fn get_min(&mut self,idx:usize) -> T{
        let mut ans = T::MAX;
        while 
    }
}
