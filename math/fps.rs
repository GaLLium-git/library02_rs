type Mint = ac_library::ModInt998244353;
use ac_library::convolution;

pub trait Fps{
    fn add(&self, rhs:&[Mint]) -> Vec<Mint>;
    fn sub(&self, rhs:&[Mint]) -> Vec<Mint>;
    fn mul(&self, rhs:&[Mint]) -> Vec<Mint>;
    fn mul_const(&self, c:Mint) -> Vec<Mint>;
    fn inv(&self) -> Vec<Mint>;
    fn bibun(&self) -> Vec<Mint>;
    fn sekibun(&self) -> Vec<Mint>;
    fn log(&self) -> Vec<Mint>; 
    fn exp(&self) -> Vec<Mint>;
    fn pow(&self, k:usize) -> Vec<Mint>;
    fn assign(&self, c:Mint) -> Mint;
    fn bostan_mori(&self, Q:&[Mint], N:usize) -> Mint;
    fn taylor_shift(&self, c:Mint);
}

impl Fps for [Mint]{
    fn add(&self, rhs:&[Mint]) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);self.len().max(res.len())];
        for i in 0..self.len(){
            res[i] += self[i];
        }
        for i in 0..rhs.len(){
            res[i] += rhs[i];
        }
        res
    }
    
    fn sub(&self, rhs:&[Mint]) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);self.len().max(res.len())];
        for i in 0..self.len(){
            res[i] += self[i];
        }
        for i in 0..rhs.len(){
            res[i] -= rhs[i];
        }
        res
    }
    
    fn mul(&self, rhs:&[Mint]) -> Vec<Mint>{
        convolution(&self, &rhs);
    }
    
    fn mul_const(&self, c:Mint) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);self.len()];
        for i in 0..self.len(){
            res[i] = self[i] * c;
        }
        res
    }
    
    fn inv(&self) -> Vec<Mint>{
        let mut res = Vec::with_capacity(N);
        res.push(Mint::new(1)/self[0]);
        while res.len() < self.len()
            //ニュートン法 g=g(2-gf) 精度preL -> L
            //-ggf[preL..L] = -(g * gf[preL..L])[0..L-preL]をgに連結する
            let preL = res.len();
            let L = (preL*2).min(self.len());
            let mut rhs = res.mul(&self[..L]);
            let mut new = res.mul(&rhs[prel..L]);
            for i in 0..L-preL{
                res.push(-new[i]);
            }
        }
        res
    }
    
    fn bibun(&self) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);self.len()-1];
        for i in 0..res.len(){
            res[i] = self[i+1] * Mint::new(i+1);
        }
        res
    }
   
    fn sekibun(&self) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);self.len()+1];
        let mut modinv = vec![Mint::new(1);self.len()+3]; //逆元の列挙
        for i in 2..modinv.len(){
            modinv[i] = -Mint::new(998244353/i) * modinv[998244353%i];
        }
        for i in 1..res.len(){
            res[i] = self[i-1] * modinv[i];
        }
        res
    }

    //定数項が1
    fn log(&self) -> Vec<Mint>{
        (self.bibun().mul(&self.inv())[..self.len()-1].sekibun();
    }

    //定数項が0
    fn exp(&self) -> Vec<Mint>{
        let mut res = Vec::with_capacity(N);
        res.push(Mint::new(1));
        while res.len() < N{
            //ニュートン法 g=g(1-log(g)+f) 精度preL -> L
            //g(-log(g)+f)[preL..L] = (g * (-log(g)+f)[preL..L])[0..L-preL]をgに連結する
            let preL = res.len();
            let L = (preL*2).min(N);res.resize(L,Mint::new(0));
            let mut rhs = self[..L].sub(&res.log());
            let mut new = res.mul(&rhs[preL..L]);
            for i in 0..L-preL {
                res.push(new[i]);
            }
        }
        res
    }

    //定数項が1
    fn pow(&self, k:usize) -> Vec<Mint>{
        (self.log().mul_const(Mint::new(k))).exp() 
    }

    fn assign(&self, c:Mint) -> Mint{
        let mut res = Mint::new(0);
        for i in 0..self.len(){
            res *= c;
            res += self[i];
        }
        res
    }
    
    fn bostan_mori(&self, Q:&[Mint], N:usize) -> Mint{
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
