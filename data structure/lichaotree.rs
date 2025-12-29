//Li Chao Tree
pub struct LiChaotree<F,T,Eval>
where
    F: Copy,
    T: Copy + Ord,
    Eval: Fn(F, T) -> T,
{
    val: Vec<T>,              
    tree: Vec<Option<F>>,    
    eval: Eval,
}

impl<F,T,Eval> LiChaotree<F,T,Eval>
where
    F: Copy,
    T: Copy + Ord,
    Eval: Fn(F, T) -> T,
{
    pub fn new(val:&Vec<T>, eval:Eval) -> Self{
        let log = (val.len().ilog2() +1) as usize;
        let len = (1<<log);
        Self{
            val: val.clone(),
            tree: vec![None;len*2],
            eval
        }
    }
    
    pub fn add(&mut self, f:F){
        
    }
    
    pub fn get_min(&self, x:T) -> T{
        let mut idx = bsearch_usize(0..self.val.len(),|i| self.val[i]>=x);
        let mut res = (self.eval)(self.tree[idx].unwrap(),x);
        while idx != 1{
            idx >>= 1;
            res = res.min((self.eval)(self.tree[idx].unwrap(),x));
        }
        res
    }
}
