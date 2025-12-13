fn main(){
    let A = Poly::new(vec![Mint::new(1),Mint::new(2),Mint::new(3)]); //1 + 2x + 2x^2
    let B = Poly::new(vec![Mint::new(4),Mint::new(5)]); //4 + 5x
    
    eprintln!("{:?}",(A.clone()+B.clone()).seq);
    eprintln!("{:?}",(A.clone()*B.clone()).seq);
    eprintln!("{:?}",(A.clone()*Mint::new(-1)).seq);
}


//多項式ライブラリ
type Mint = ac_library::ModInt998244353;
const MAX_DEG:usize = 1000000; //数列の長さは1e6で打ち切る

#[derive(Clone,PartialEq,Eq)]
pub struct Poly{
    pub seq:Vec<Mint>
}

impl Poly{
    fn new (seq:Vec<Mint>) -> Self{
        Self{seq}
    }
}


use std::ops::{Add,Sub,Mul,Div};
//和 O(N)
impl Add for Poly{
    type Output = Self;
    fn add(self, rhs:Self) -> Self {
        let len = self.seq.len().max(rhs.seq.len());
        let mut res = vec![Mint::new(0);len];
        for i in 0..len {
            if i < self.seq.len() {
                res[i] += self.seq[i];
            }
            if i < rhs.seq.len() {
                res[i] += rhs.seq[i];
            }
        }
        Self{seq: res}
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

//積 O(NlogN)
impl Mul for Poly{
    type Output = Self;
    fn mul(self, rhs:Self) -> Self {
        let mut res = ac_library::convolution(&self.seq,&rhs.seq);
        res.truncate(MAX_DEG);
        Self{seq: res}
    }
}
