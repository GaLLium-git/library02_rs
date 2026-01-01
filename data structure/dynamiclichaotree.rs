//Dynamic Li Chao Tree
//追加，取得ともにO(logN)
struct Node<F>{
    f: F,
    left: Option<usize>,
    right: Option<usize>,
}

pub struct LiChaoTree<F,T,Eval>
where
    F: Copy,
    T: Copy + Ord,
    Eval: Fn(F, T) -> T,
{
    tree: Vec<LiChaoNode>,    
    eval: Eval,
    id: F,
    x_min: T,
    x_max: T,
}

impl<F,T,Eval> LiChaoTree<F,T,Eval>
where
    F: Copy,
    T: Copy + Ord,
    Eval: Fn(F, T) -> T,
{
    pub fn new(eval:Eval, id:F, x_min:T, x_max:T) -> Self{
        Self {
            tree: vec![Node{f:id,left:None,right:None}],
            eval,
            id,
            x_min,
            x_max,
        }
    }
    
    fn _add()
    pub fn add(&mut self, f:F){
        self._add(f,self.x_min,self.x_max);
    }
    
    
    fn _get()
    pub fn get(&self, x:T) -> T{
       self._get(x,self.x_min,self.x_max);
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
