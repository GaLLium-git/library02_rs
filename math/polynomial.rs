#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::cell::RefCell;

thread_local! {
    static NTTCNT: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

fn main() {
    let mut sc=Scanner::new();
    let (D,N):(usize,usize) = (sc.next(),sc.next());
    if D > N {
        println!("{}",0);
        return;
    }
    let mut poly = Poly::new(vec![
        Mint::new(1),Mint::new(0),Mint::new(1),
        Mint::new(1),Mint::new(0),Mint::new(1)
    ]);
    let mut ans = poly.pow(D,N);
    println!("{}",ans.seq[N-D]);

    NTTCNT.with(|v| {
        let mut a = v.borrow().clone();
        a.sort();
        println!("{:?}", a);

        let mut op :f64= 0.0;
        for &sz in a.iter(){
            op += (sz as f64) * (sz as f64).log2();
        }
        println!("op={}",op);
    });
}

// Scanner
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        Scanner {buffer: std::collections::VecDeque::new()}
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.buffer.len() == 0 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().map(|s| s.to_string()).collect();
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
}

//多項式ライブラリ
type Mint = ac_library::ModInt998244353;
use ac_library::convolution;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Poly{
    pub seq:Vec<Mint>
}

impl Poly{
    fn new (seq:Vec<Mint>) -> Self{
        Self{seq}
    }
    
    //前N項を残す
    fn truncate(&self,N:usize) -> Self {
        let mut res = self.seq.clone();
        res.truncate(N);
        Self{seq: res}
    }
    
    //定数倍 O(N)
    fn mul_const(&self, c:Mint) -> Self {
        let mut res = self.seq.clone();
        for i in 0..res.len(){
            res[i] *= c;
        }
        Self{seq: res}
    }
    

    //逆元の前N項 O(NlogN)
    fn inv(&self,N:usize) -> Self{
        let mut res = Vec::with_capacity(N);
        res.push(Mint::new(1)/self.seq[0]);
        while res.len() < N{
            let mut nex_len = N.min(res.len()*2);
            let mut new = convolution(&convolution(&res,&res),&self.seq[..nex_len.min(self.seq.len())]);
            new.resize(nex_len,Mint::new(0));
            for i in res.len()..nex_len{
                res.push(-new[i]);
            }
        }
        Self {seq: res}
    }
    
    //微分 O(N)
    fn bibun(&self) -> Self{
        let mut res = vec![];
        for i in 1..self.seq.len(){
            res.push(self.seq[i]*Mint::new(i));
        }
        Self{seq: res}
    }
    
    //積分 O(N)
    fn sekibun(&self) -> Self{
        let mut res = vec![Mint::new(0)];
        let mut modinv = vec![Mint::new(1);self.seq.len() + 3]; //逆元の列挙
        for i in 2..modinv.len(){
            modinv[i] = -Mint::new(998244353/i) * modinv[998244353%i];
        }
        for i in 0..self.seq.len(){
            res.push(self.seq[i] * modinv[i+1]);
        }
        Self{seq: res}
    }
    
    //logの前N項 O(NlogN) 定数項が1
    fn log(&self,N:usize) -> Self{
        let mut res = (self.truncate(N+1).bibun() * self.inv(N)).sekibun();
        res.truncate(N)
    }
    
    //expの前N項 O(NlogN) 定数項が0
    fn exp(&self,N:usize) -> Self{
        let mut res = Poly::new(vec![Mint::new(1)]);
        for i in 1..=N.ilog2()+1{
            res = res.clone()*(Poly::new(vec![Mint::new(1)]) - res.clone().log(1<<i) + self.truncate(1<<i)).truncate(1<<i);
        }
        res.truncate(N)
    }
    
    //累乗の前N項 O(NlogN) 定数項が1
    fn pow(&self, k:usize,N:usize) -> Self{
        (self.log(N).mul_const(Mint::new(k))).exp(N) 
    }
    
    //値の代入 O(N)
    fn assign(&self,c:Mint) -> Mint{
        let mut res = Mint::new(0);
        let mut pow = Mint::new(1);
        for i in 0..self.seq.len(){
            res += self.seq[i]*pow;
            pow *= c;
        }
        res
    }
    
    fn taylor_shift(&self,c:Mint){
        
    }
}


use std::ops::{Add,Sub,Mul,Div};
//加法 O(N)
impl Add for Poly{
    type Output = Self;
    fn add(self, rhs:Self) -> Self {
        let len = self.seq.len().max(rhs.seq.len());
        let mut res = vec![Mint::new(0);len];
        for i in 0..len {
            if i < self.seq.len() { res[i] += self.seq[i];}
            if i < rhs.seq.len() { res[i] += rhs.seq[i];}
        }
        Self{seq: res}
    }
}

//減法 O(N)
impl Sub for Poly{
    type Output = Self;
    fn sub(self, rhs:Self) -> Self {
        self + rhs.mul_const(Mint::new(-1))
    }
}

//乗法 O(NlogN)
impl Mul for Poly{
    type Output = Self;
    fn mul(self, rhs:Self) -> Self {
        NTTCNT.with(|v| {
            v.borrow_mut().push(self.seq.len() + rhs.seq.len());
        });
        let res = ac_library::convolution(&self.seq,&rhs.seq);
        Self{seq: res}
    }
}
