//多項式ライブラリ
type Mint = ac_library::ModInt998244353;
use ac_library::convolution;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Poly{
    pub seq:Vec<Mint>
}

impl Poly{
    pub fn new (seq:Vec<Mint>) -> Self{
        Self{seq}
    }
    
    //加法 O(N)
    pub fn add(&self, rhs:&Self, N:usize) -> Self{
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
    pub fn sub(&self, rhs:&Self, N:usize) -> Self{
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
    pub fn mul(&self, rhs: &Self, N: usize) -> Self {
        let mut res = convolution(&self.seq[..self.seq.len().min(N)], &rhs.seq[..rhs.seq.len().min(N)]);
        res.resize(N,Mint::new(0));
        Self {seq: res }
    }
    
    //定数倍 O(N)
    pub fn mul_const(&self, c:Mint) -> Self{
        let mut res = vec![Mint::new(0);self.seq.len()];
        for i in 0..self.seq.len(){
            res[i] = self.seq[i] * c;
        }
        Self{seq: res}
    }
    

    //逆元の前N項 O(NlogN)
    pub fn inv(&self,N:usize) -> Self{
        let mut res = Poly::new(Vec::with_capacity(N));
        res.seq.push(Mint::new(1)/self.seq[0]);
        while res.seq.len() < N{
            //ニュートン法 g=g(2-gf) 精度preL -> L
            //-ggf[preL..L] = -(g * gf[preL..L])[0..L-preL]をgに連結する
            let preL = res.seq.len();
            let L = (preL*2).min(N);
            let mut rhs = res.mul(&self,L);
            rhs.seq.drain(..preL);
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL {
                res.seq.push(-new.seq[i]);
            }
        }
        res
    }
    
    //微分 O(N)
    pub fn bibun(&self,N:usize) -> Self{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(self.seq.len()-1);
        for i in 0..lim{
            res[i] = self.seq[i+1] * Mint::new(i+1);
        }
        Self{seq: res}
    }
    
    //積分 O(N)
    pub fn sekibun(&self,N:usize) -> Self{
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
    pub fn log(&self,N:usize) -> Self{
        (self.bibun(N-1).mul(&self.inv(N-1),N-1)).sekibun(N)
    }
    
    //expの前N項 O(NlogN) 定数項が0
    pub fn exp(&self,N:usize) -> Self{
        let mut res = Poly::new(Vec::with_capacity(N));
        res.seq.push(Mint::new(1));
        while res.seq.len() < N{
            //ニュートン法 g=g(1-log(g)+f) 精度preL -> L
            //g(-log(g)+f)[preL..L] = -(g * (log(g)-f)[preL..L])[0..L-preL]をgに連結する
            let preL = res.seq.len();
            let L = (preL*2).min(N);
            let mut rhs = res.log(L).sub(&self,L);
            rhs.seq.drain(..preL);
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL {
                res.seq.push(-new.seq[i]);
            }
        }
        res
    }
    
    //累乗の前N項 O(NlogN) 定数項が1
    pub fn pow(&self,k:usize,N:usize) -> Self{
        (self.log(N).mul_const(Mint::new(k))).exp(N) 
    }
    
    //値の代入 O(N)
    pub fn assign(&self,c:Mint) -> Mint{
        let mut res = Mint::new(0);
        for i in 0..self.seq.len(){
            res *= c;
            res += self.seq[i];
        }
        res
    }
    
    //Bostan-Mori法 O(dlogdlogN)
    pub fn bostan_mori(&self,Q:&Self,N:usize) -> Mint{
        if N == 0 { return self.seq[0]/Q.seq[0]; }
        let d = Q.seq.len();
        let mut Q_neg = Q.clone();
        for i in (1..d).step_by(2) {
            Q_neg.seq[i] *= Mint::new(-1);
        }
        let mut V = Q.mul(&Q_neg,2*d);
        V.seq = V.seq.into_iter().step_by(2).collect();
        let mut U = self.mul(&Q_neg,2*d);
        if N % 2 == 0 { 
            U.seq = U.seq.into_iter().step_by(2).collect();
        } else {
            U.seq = U.seq.into_iter().skip(1).step_by(2).collect();
        }
        U.bostan_mori(&V,N/2)
    }
    
    fn taylor_shift(&self,c:Mint){
        
    } 
}
