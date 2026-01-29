pub struct Mo{
    querys: Vec<(usize,usize,usize)>,
    //残りの状態
    A: Vec<usize>,
    ans: Vec<usize>,
    cnt: Vec<usize>,
    cur:usize,
}

impl Mo{
    pub fn new(A:Vec<usize>) -> Self{
        Self{
            querys: vec![],
            A,
            ans: vec![0;200005],
            cnt: vec![0;200005],
            cur: 0,
        }
    }
    
    //クエリ[l,r) の追加
    pub fn insert(&mut self, range: impl std::ops::RangeBounds<usize>){
        let (l, r) = get_bounds_usize(range);
        let i = self.querys.len();
        self.querys.push((l,r,i));
    }
    
    fn inc_l(&mut self, l:usize){
        self.cur -= C3(self.cnt[self.A[l]]);
        self.cnt[self.A[l]] -= 1;
        self.cur += C3(self.cnt[self.A[l]]);
    }
    
    fn dec_l(&mut self, l:usize){
        self.cur -= C3(self.cnt[self.A[l-1]]);
        self.cnt[self.A[l-1]] += 1;
        self.cur += C3(self.cnt[self.A[l-1]]);
    }
    
    fn inc_r(&mut self, r:usize){
        self.cur -= C3(self.cnt[self.A[r]]);
        self.cnt[self.A[r]] += 1;
        self.cur += C3(self.cnt[self.A[r]]);
    }
    
    fn dec_r(&mut self, r:usize){
        self.cur -= C3(self.cnt[self.A[r-1]]);
        self.cnt[self.A[r-1]] -= 1;
        self.cur += C3(self.cnt[self.A[r-1]]);
    }
    
    fn calc(&mut self, i:usize){
        self.ans[i] = self.cur;
    }
    
    pub fn solve(&mut self){
        let B = self.querys.len().isqrt();
        self.querys.sort_by_key(|&(l,r,_)| (l/B,r));
        let (mut nl, mut nr) = (0,0);
        
        let querys = self.querys.clone();
        for &(l,r,i) in querys.iter(){
            while nl < l{
                self.inc_l(nl);
                nl += 1;
            }
            while nl > l{
                self.dec_l(nl);
                nl -= 1;
            }
            while nr < r{
                self.inc_r(nr);
                nr += 1;
            }
            while nr > r{
                self.dec_r(nr);
                nr -= 1;
            }
            self.calc(i);
        }
    }
}
