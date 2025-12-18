#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]


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
    
    //加法 O(N)
    fn add(&self, rhs:&Self, N:usize) -> Self{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.seq.len()) {
            res[i] += self.seq[i];
        }
        for i in 0..N.min(rhs.seq.len()) {
            res[i] += rhs.seq[i];
        }
        Self{seq: res}
    }
    
    //減法 O(N)
    fn sub(&self, rhs:&Self, N:usize) -> Self{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.seq.len()) {
            res[i] += self.seq[i];
        }
        for i in 0..N.min(rhs.seq.len()) {
            res[i] -= rhs.seq[i];
        }
        Self{seq: res}
    }
    
    //乗法 O(NlogN)
    fn mul(&self, rhs: &Self, N: usize) -> Self {
        let mut res = convolution(&self.seq[..self.seq.len().min(N)], &rhs.seq[..rhs.seq.len().min(N)]);
        res.resize(N,Mint::new(0));
        Self {seq: res }
    }
    
    //定数倍 O(N)
    fn mul_const(&self, c:Mint) -> Self{
        let mut res = vec![Mint::new(0);self.seq.len()];
        for i in 0..self.seq.len(){
            res[i] = self.seq[i] * c;
        }
        Self{seq: res}
    }
    

    //逆元の前N項 O(NlogN)
    fn inv(&self,N:usize) -> Self{
        let mut res = Poly::new(Vec::with_capacity(N));
        res.seq.push(Mint::new(1)/self.seq[0]);
        while res.seq.len() < N{
            // g = g(2-gf)
            let preL = res.seq.len();
            let L = (preL*2).min(N);
            let mut rhs = res.mul(&self,L);
            rhs.seq = rhs.seq[preL..L].to_vec();
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL {
                res.seq.push(-new.seq[i]);
            }
        }
        res
    }
    
    //微分 O(N)
    fn bibun(&self,N:usize) -> Self{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(self.seq.len()-1);
        for i in 0..lim{
            res[i] = self.seq[i+1] * Mint::new(i+1);
        }
        Self{seq: res}
    }
    
    //積分 O(N)
    fn sekibun(&self,N:usize) -> Self{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(self.seq.len()+1);
        let mut modinv = vec![Mint::new(1);lim + 3]; //逆元の列挙
        for i in 2..modinv.len(){
            modinv[i] = -Mint::new(998244353/i) * modinv[998244353%i];
        }
        for i in 1..lim{
            res[i] = self.seq[i-1] * modinv[i];
        }
        Self{seq: res}
    }
    
    //logの前N項 O(NlogN) 定数項が1
    fn log(&self,N:usize) -> Self{
        (self.bibun(N-1).mul(&self.inv(N-1),N-1)).sekibun(N)
    }
    
    //expの前N項 O(NlogN) 定数項が0
    fn exp(&self,N:usize) -> Self{
        let mut res = Poly::new(vec![Mint::new(1)]);
        while res.seq.len() < N{
            let mut L = N.min(res.seq.len()*2);
            let mut rhs = res.log(L).mul_const(Mint::new(-1)).add(&self,L);
            rhs.seq[0] += Mint::new(1);
            res = res.mul(&rhs,L);  //g = g(1-log(g)+f)
        }
        res
    }
    
    //累乗の前N項 O(NlogN) 定数項が1
    fn pow(&self, k:usize,N:usize) -> Self{
        (self.log(N).mul_const(Mint::new(k))).exp(N) 
    }
    
    //値の代入 O(N)
    fn assign(&self,c:Mint) -> Mint{
        let mut res = Mint::new(0);
        for i in 0..self.seq.len(){
            res *= c;
            res += self.seq[i];
        }
        res
    }
    
    fn taylor_shift(&self,c:Mint){
        
    }
}
