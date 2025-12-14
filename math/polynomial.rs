fn main() {
    let mut sc=Scanner::new();
    let N:usize=sc.next();
    let A:Vec<Mint>=(0..N).map(|_| sc.next()).collect();
    
    let mut ans = Poly::new(A).exp().seq;
    ans.truncate(N);
    
    for &val in ans.iter(){
        print!("{} ",val);
    }
}

//多項式ライブラリ
type Mint = ac_library::ModInt998244353;

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
        let mut res = Poly::new(vec![Mint::new(1)/self.seq[0]]);
        for i in 1..=N.ilog2()+1{
            res = (res.clone()*(Poly::new(vec![Mint::new(2)]) - res.clone()*self.truncate(1<<i))).truncate(1<<i);
        }
        res.truncate(N)
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
    fn pow(&self, k:i64,N:usize) -> Self{
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
        let mut res = ac_library::convolution(&self.seq,&rhs.seq);
        Self{seq: res}
    }
               }
