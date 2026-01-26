fn main(){
    let mut sc = Scanner::new();
    let (N,Q):(usize,usize) = (sc.next(),sc.next());
    let mut A:Vec<usize> = (0..N).map(|_| sc.next()).collect();
    
    let mut mo = Mo::new();
    
    for _ in 0..Q{
        let (l,r):(usize,usize) = (sc.next(),sc.next());
        mo.insert(l..=r);
    }
    
    let mut inc_l = |&l:usize| {};
    let mut dec_l = |&l:usize| {};
    let mut inc_r = |&r:usize| {};
    let mut dec_r = |&r:usize| {};
    let mut calc = |&i:usize| {};
    mo.solve(inc_l,dec_l,inc_r,del_r,calc);
}

//Scanner
pub struct Scanner{
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self{
        Scanner {buffer: std::collections::VecDeque::new()}
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T{
        if self.buffer.len() == 0{
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().map(|s| s.to_string()).collect();
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
}

//range型を[l,r)に直す関数
pub fn get_bounds_usize(range: impl std::ops::RangeBounds<usize>) -> (usize,usize){
    let l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => *l+1,
        std::ops::Bound::Unbounded => 0,
    };
    let r = match range.end_bound() {
        std::ops::Bound::Included(r) => *r+1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => usize::MAX,
    };
    (l,r)
}

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
    pub fn insert(&mut self, range: impl std::ops::RangeBounds<usize>){
        let (l, r) = get_bounds_usize(range);
        let i = self.querys.len();
        self.querys.push((l,r,i));
    }
    
    pub fn solve(&mut self, mut inc_l:impl FnMut(usize), mut dec_l:impl FnMut(usize),
                            mut inc_r:impl FnMut(usize), mut dec_r:impl FnMut(usize), mut calc:impl FnMut(usize)){
        let B = self.querys.len().isqrt();
        self.querys.sort_by_key(|&(l,r,_)| (l/B,r));
        
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
            calc(i);
        }
    }
}
