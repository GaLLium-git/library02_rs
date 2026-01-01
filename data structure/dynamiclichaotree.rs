//Dynamic Li Chao Tree
//追加，取得ともにO(logN)
struct LiChaoNode<F>{
    func: F,
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
    x_min: T,
    x_max: T,
}

impl<F,T,Eval> LiChaoTree<F,T,Eval>
where
    F: Copy,
    T: Copy + Ord,
    Eval: Fn(F, T) -> T,
{
    pub fn new(eval:Eval,x_min:T,x_max:T) -> Self{
        Self {
            tree:
            eval,
            x_min,
            x_max,
        }
    }

    pub fn add(&mut self, f:F){
        
    }
    
    pub fn get_min(&self, x:T) -> T{
       
    }
}

