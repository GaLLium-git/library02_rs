//もしかしたら関数にした方が使い勝手がいいかもしれない
type Mint = ac_library::ModInt998244353;
use ac_library::convolution;

pub trait fps{
    fn 
}

impl fps for Vec<Mint>{
    pub fn add(&self, rhs:&Vec<Mint>, N:usize) -> self{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.len()) {
            res[i] += self[i];
        }
        for i in 0..N.min(rhs.len()) {
            res[i] += rhs[i];
        }
        res
    }
    
    pub fn sub(&self, rhs:&Vec<Mint>, N:usize) -> self{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.len()) {
            res[i] += self[i];
        }
        for i in 0..N.min(rhs.len()) {
            res[i] -= rhs[i];
        }
        res
    }
    
    //乗法 O(NlogN)
    pub fn mul(&self, rhs:&Vec<Mint>, N:usize) -> self {
        let mut res = convolution(&self[..self.len().min(N)], &rhs[..rhs.len().min(N)]);
        res.resize(N,Mint::new(0));
        res
    }
    
    //定数倍 O(N)
    pub fn mul_const(&self, c:Mint) -> self{
        let mut res = vec![Mint::new(0);self.len()];
        for i in 0..self.len(){
            res[i] = self[i] * c;
        }
        res
    }
    

    //逆元の前N項 O(NlogN)
    pub fn inv(&self,N:usize) -> self{
        let mut res = Vec::with_capacity(N);
        res.push(Mint::new(1)/self[0]);
        while res.len() < N{
            //ニュートン法 g=g(2-gf) 精度preL -> L
            //-ggf[preL..L] = -(g * gf[preL..L])[0..L-preL]をgに連結する
            let preL = res.len();
            let L = (preL*2).min(N);
            let mut rhs = res.mul(&self,L);
            rhs.drain(..preL);
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL{
                res.push(-new[i]);
            }
        }
        res
    }
    
    //微分 O(N)
    pub fn bibun(&self,N:usize) -> self{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(self.len()-1);
        for i in 0..lim{
            res[i] = self[i+1] * Mint::new(i+1);
        }
        res
    }
    
    //積分 O(N)
    pub fn sekibun(&self,N:usize) -> self{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(self.len()+1);
        let mut modinv = vec![Mint::new(1);lim + 3]; //逆元の列挙
        for i in 2..modinv.len(){
            modinv[i] = -Mint::new(998244353/i) * modinv[998244353%i];
        }
        for i in 1..lim{
            res[i] = self[i-1] * modinv[i];
        }
        res
    }
    
    //logの前N項 O(NlogN) 定数項が1
    pub fn log(&self,N:usize) -> self{
        (self.bibun(N-1).mul(&self.inv(N-1),N-1)).sekibun(N)
    }
    
    //expの前N項 O(NlogN) 定数項が0
    pub fn exp(&self,N:usize) -> self{
        let mut res = Vec::with_capacity(N);
        res.push(Mint::new(1));
        while res.len() < N{
            //ニュートン法 g=g(1-log(g)+f) 精度preL -> L
            //g(-log(g)+f)[preL..L] = -(g * (log(g)-f)[preL..L])[0..L-preL]をgに連結する
            let preL = res.len();
            let L = (preL*2).min(N);
            let mut rhs = res.log(L).sub(&self,L);
            rhs.drain(..preL);
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL {
                res.push(-new[i]);
            }
        }
        res
    }
    
    //累乗の前N項 O(NlogN) 定数項が1
    pub fn pow(&self, k:usize, N:usize) -> self{
        (self.log(N).mul_const(Mint::new(k))).exp(N) 
    }
    
    //値の代入 O(N)
    pub fn assign(&self,c:Mint) -> Mint{
        let mut res = Mint::new(0);
        for i in 0..self.len(){
            res *= c;
            res += self[i];
        }
        res
    }
    
    //Bostan-Mori法 O(dlogdlogN)
    pub fn bostan_mori(&self, Q:&Vec<Mint>, N:usize) -> Mint{
        if N == 0 { return self[0]/Q[0]; }
        let d = Q.len();
        let mut Q_neg = Q.clone();
        for i in (1..d).step_by(2) {
            Q_neg[i] *= Mint::new(-1);
        }
        let mut V = Q.mul(&Q_neg,2*d);
        V = V.into_iter().step_by(2).collect();
        let mut U = self.mul(&Q_neg,2*d);
        if N % 2 == 0 { 
            U = U.into_iter().step_by(2).collect();
        } else {
            U = U.into_iter().skip(1).step_by(2).collect();
        }
        U.bostan_mori(&V,N/2)
    }
    
    fn taylor_shift(&self,c:Mint){
        
    } 
}
