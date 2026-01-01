//Li Chao Tree
//追加，取得ともにO(logN)
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
    
    //tree[k],範囲[l,r)への追加
    fn _add(&mut self, f:F, k:usize, l:usize, r:usize){
        
        
        
        
    }

    pub fn add(&mut self, f:F){
        self._add(f,1,0,self.tree.len()/2);
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


//range で fがtrueとなる最小を返す
pub fn bsearch_usize<F>(range: impl std::ops::RangeBounds<usize>, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => l+1,
        std::ops::Bound::Unbounded => 0,
    };
    let mut r = match range.end_bound() {
        std::ops::Bound::Included(r) => r+1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => usize::MAX,
    };
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}
