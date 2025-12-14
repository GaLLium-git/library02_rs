fn main(){
    let A = Poly::new(vec![Mint::new(1),Mint::new(2),Mint::new(3)]); //1 + 2x + 3x^2
    let B = Poly::new(vec![Mint::new(4),Mint::new(5)]); //4 + 5x
    
    eprintln!("A={:?},B={:?}",A,B);
    eprintln!("A+B={:?}",A.clone()+B.clone()); 
    eprintln!("A-B={:?}",A.clone()-B.clone());
    eprintln!("A*B={:?}",A.clone()*B.clone());
    eprintln!("A/B={:?}",A.clone()/B.clone());
    eprintln!("A'={:?}",A.clone().bibun());
    eprintln!("∫A={:?}",A.clone().sekibun());
    eprintln!("log A={:?}",A.clone().log());
    eprintln!("A(2)={}",A.clone().assign(Mint::new(2)));
}


//多項式ライブラリ
type Mint = ac_library::ModInt998244353;
const MAX_DEG:usize = 1000000; //次数は < 1e6 で打ち切る

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Poly{
    pub seq:Vec<Mint>
}

impl Poly{
    fn new (seq:Vec<Mint>) -> Self{
        Self{seq}
    }
    
    //多項式逆元 O(NlogN)
    fn inv(self) -> Self{
        let mut res = Poly::new(vec![Mint::new(1)/self.seq[0]]);
        for i in 1..=MAX_DEG.ilog2()+1{
            res = res.clone()*(Poly::new(vec![Mint::new(2)]) - res.clone()*self.clone());
            res.seq.truncate(1<<i);
        }
        res.seq.truncate(MAX_DEG);
        res 
    }
    
    //微分 O(N)
    fn bibun(self) -> Self{
        let mut res = vec![];
        for i in 1..self.seq.len(){
            res.push(self.seq[i]*Mint::new(i));
        }
        Self{seq: res}
    }
    
    //積分 O(N)
    fn sekibun(self) -> Self{
        let mut res = vec![Mint::new(0)];
        for i in 0..self.seq.len(){
            res.push(self.seq[i] / Mint::new(i+1));
        }
        res.truncate(MAX_DEG);
        Self{seq: res}
    }
    
    //多項式のlog O(NlogN)
    fn log(self) -> Self{
        (self.clone().bibun() / self.clone()).sekibun()
    }
    
    //多項式のexp O(NlogN) f(0)=0
    fn exp(self) -> Self{
        let mut res = Poly::new(vec![Mint::new(1)]);
        for i in 1..=MAX_DEG.ilog2()+1{
            res = res.clone()*(Poly::new(vec![Mint::new(1)]) - res.clone().log() + self.clone());
            res.seq.truncate(1<<i);
        }
        res.seq.truncate(MAX_DEG);
        res 
    }
    
    //累乗 O(NlogN)
    fn pow(self, n:i64) -> Self{
        (self.log() * Mint::new(n)).exp() 
    }
    
    //値の代入 O(N)
    fn assign(self,c:Mint) -> Mint{
        let mut res = Mint::new(0);
        let mut pow = Mint::new(1);
        for i in 0..self.seq.len(){
            res += self.seq[i]*pow;
            pow *= c;
        }
        res
    }
    
    fn taylor_shift(self,c:Mint){
        
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
        self + rhs * Mint::new(-1)
    }
}

//定数倍 O(N) f*cの形のみ可
impl Mul<Mint> for Poly{
    type Output = Self;
    fn mul(self, c:Mint) -> Self {
        let mut res = self.seq;
        for i in 0..res.len(){
            res[i] *= c;
        }
        Self{seq: res}
    }
}

//乗法 O(NlogN)
impl Mul for Poly{
    type Output = Self;
    fn mul(self, rhs:Self) -> Self {
        let mut res = ac_library::convolution(&self.seq,&rhs.seq);
        res.truncate(MAX_DEG);
        Self{seq: res}
    }
}

//除法 O(NlogN);
impl Div for Poly{
    type Output = Self;
    fn div(self, rhs:Self) -> Self {
        self * rhs.inv()
    }
}
