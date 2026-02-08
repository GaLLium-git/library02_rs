type Mint = ac_library::ModInt998244353;
use ac_library::convolution;

pub trait Fps{
    fn add(&self, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>;
    fn sub(&self, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>;
    fn mul(&self, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>;
    fn mul_const(&self, c:Mint) -> Vec<Mint>;
    fn inv(&self, N:usize) -> Vec<Mint>;
    fn bibun(&self, N:usize) -> Vec<Mint>;
    fn sekibun(&self, N:usize) -> Vec<Mint>;
    fn log(&self, N:usize) -> Vec<Mint>; 
    fn exp(&self, N:usize) -> Vec<Mint>;
    fn pow(&self, k:usize, N:usize) -> Vec<Mint>;
    fn assign(&self, c:Mint) -> Mint;
    fn bostan_mori(&self, Q:&Vec<Mint>, N:usize) -> Mint;
    fn taylor_shift(&self, c:Mint);
}

impl Fps for Vec<Mint>{
    fn add(&self, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.len()) {
            res[i] += self[i];
        }
        for i in 0..N.min(rhs.len()) {
            res[i] += rhs[i];
        }
        res
    }
    
    fn sub(&self, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.len()) {
            res[i] += self[i];
        }
        for i in 0..N.min(rhs.len()) {
            res[i] -= rhs[i];
        }
        res
    }
    
    fn mul(&self, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>{
        let mut res = convolution(&self[..self.len().min(N)], &rhs[..rhs.len().min(N)]);
        res.resize(N,Mint::new(0));
        res
    }
    
    fn mul_const(&self, c:Mint) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);self.len()];
        for i in 0..self.len(){
            res[i] = self[i] * c;
        }
        res
    }
    
    fn inv(&self, N:usize) -> Vec<Mint>{
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
    
    fn bibun(&self, N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        let L = N.min(self.len()-1);
        for i in 0..L{
            res[i] = self[i+1] * Mint::new(i+1);
        }
        res
    }
    
    fn sekibun(&self, N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        let L = N.min(self.len()+1);
        let mut modinv = vec![Mint::new(1);N+3]; //逆元の列挙
        for i in 2..modinv.len(){
            modinv[i] = -Mint::new(998244353/i) * modinv[998244353%i];
        }
        for i in 1..L{
            res[i] = self[i-1] * modinv[i];
        }
        res
    }
    
    fn log(&self, N:usize) -> Vec<Mint>{
        (self.bibun(N-1).mul(&self.inv(N-1),N-1)).sekibun(N)
    }
    
    fn exp(&self, N:usize) -> Vec<Mint>{
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
    
    fn pow(&self, k:usize, N:usize) -> Vec<Mint>{
        (self.log(N).mul_const(Mint::new(k))).exp(N) 
    }

    fn assign(&self, c:Mint) -> Mint{
        let mut res = Mint::new(0);
        for i in 0..self.len(){
            res *= c;
            res += self[i];
        }
        res
    }
    
    fn bostan_mori(&self, Q:&Vec<Mint>, N:usize) -> Mint{
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
    
    fn taylor_shift(&self, c:Mint){
        
    } 
}
