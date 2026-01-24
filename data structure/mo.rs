pub struct Mo{
    querys: Vec<(usize,usize,usize)>,
}

impl Mo{
    pub fn new() -> Self{
        Self{
            querys: vec![],
        }
    }
    
    //クエリ[l,r) の追加
    pub fn insert(&mut self, l:usize, r:usize){
        let i = self.querys.len();
        self.querys.push((l,r,i));
    }
    
   
    pub fn solve(&mut self, mut inc_l:impl FnMut(usize), mut inc_r:impl FnMut(usize),
                            mut dec_l:impl FnMut(usize), mut dec_r:impl FnMut(usize), calc:impl Fn() -> usize) -> Vec<usize>{
        let B = self.querys.len().isqrt();
        self.querys.sort_by_key(|&(l,r,_)| (l/B,r));
        
        let mut res = vec![0;self.querys.len()];
        let (mut nl, mut nr) = (0,0);
        
        for &(l,r,i) in self.querys.iter(){
            while nl < l{
                inc_l(nl);
                nl += 1;
            }
            while nl > l{
                dec_l(nl);
                nl -= 1;
            }
            while nr < r{
                inc_r(nr);
                nr += 1;
            }
            while nr > r{
                dec_r(nr);
                nr -= 1;
            }
            res[i] = calc();
        }
        res
    }
}
