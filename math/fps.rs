//もしかしたら関数にした方が使い勝手がいいかもしれない
type Mint = ac_library::ModInt998244353;
use ac_library::convolution;

pub trait fps{
    
}

impl fps for Vec<Mint>{
    pub fn add(&Vec<Mint>, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(self.len()) {
            res[i] += Vec<Mint>[i];
        }
        for i in 0..N.min(rhs.len()) {
            res[i] += rhs[i];
        }
        res
    }
    
    pub fn sub(&Vec<Mint>, rhs:&Vec<Mint>, N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        for i in 0..N.min(Vec<Mint>.len()) {
            res[i] += Vec<Mint>[i];
        }
        for i in 0..N.min(rhs.len()) {
            res[i] -= rhs[i];
        }
        Vec<Mint>{seq: res}
    }
    
    //乗法 O(NlogN)
    pub fn mul(&Vec<Mint>, rhs: &Vec<Mint>, N: usize) -> Vec<Mint> {
        let mut res = convolution(&Vec<Mint>[..Vec<Mint>.len().min(N)], &rhs[..rhs.len().min(N)]);
        res.resize(N,Mint::new(0));
        Vec<Mint> {seq: res }
    }
    
    //定数倍 O(N)
    pub fn mul_const(&Vec<Mint>, c:Mint) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);Vec<Mint>.len()];
        for i in 0..Vec<Mint>.len(){
            res[i] = Vec<Mint>[i] * c;
        }
        Vec<Mint>{seq: res}
    }
    

    //逆元の前N項 O(NlogN)
    pub fn inv(&Vec<Mint>,N:usize) -> Vec<Mint>{
        let mut res = Poly::new(Vec::with_capacity(N));
        res.push(Mint::new(1)/Vec<Mint>[0]);
        while res.len() < N{
            //ニュートン法 g=g(2-gf) 精度preL -> L
            //-ggf[preL..L] = -(g * gf[preL..L])[0..L-preL]をgに連結する
            let preL = res.len();
            let L = (preL*2).min(N);
            let mut rhs = res.mul(&Vec<Mint>,L);
            rhs.drain(..preL);
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL {
                res.push(-new[i]);
            }
        }
        res
    }
    
    //微分 O(N)
    pub fn bibun(&Vec<Mint>,N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(Vec<Mint>.len()-1);
        for i in 0..lim{
            res[i] = Vec<Mint>[i+1] * Mint::new(i+1);
        }
        Vec<Mint>{seq: res}
    }
    
    //積分 O(N)
    pub fn sekibun(&Vec<Mint>,N:usize) -> Vec<Mint>{
        let mut res = vec![Mint::new(0);N];
        let lim = N.min(Vec<Mint>.len()+1);
        let mut modinv = vec![Mint::new(1);lim + 3]; //逆元の列挙
        for i in 2..modinv.len(){
            modinv[i] = -Mint::new(998244353/i) * modinv[998244353%i];
        }
        for i in 1..lim{
            res[i] = Vec<Mint>[i-1] * modinv[i];
        }
        Vec<Mint>{seq: res}
    }
    
    //logの前N項 O(NlogN) 定数項が1
    pub fn log(&Vec<Mint>,N:usize) -> Vec<Mint>{
        (Vec<Mint>.bibun(N-1).mul(&Vec<Mint>.inv(N-1),N-1)).sekibun(N)
    }
    
    //expの前N項 O(NlogN) 定数項が0
    pub fn exp(&Vec<Mint>,N:usize) -> Vec<Mint>{
        let mut res = Poly::new(Vec::with_capacity(N));
        res.push(Mint::new(1));
        while res.len() < N{
            //ニュートン法 g=g(1-log(g)+f) 精度preL -> L
            //g(-log(g)+f)[preL..L] = -(g * (log(g)-f)[preL..L])[0..L-preL]をgに連結する
            let preL = res.len();
            let L = (preL*2).min(N);
            let mut rhs = res.log(L).sub(&Vec<Mint>,L);
            rhs.drain(..preL);
            let mut new = res.mul(&rhs,L-preL);
            for i in 0..L-preL {
                res.push(-new[i]);
            }
        }
        res
    }
    
    //累乗の前N項 O(NlogN) 定数項が1
    pub fn pow(&Vec<Mint>,k:usize,N:usize) -> Vec<Mint>{
        (Vec<Mint>.log(N).mul_const(Mint::new(k))).exp(N) 
    }
    
    //値の代入 O(N)
    pub fn assign(&Vec<Mint>,c:Mint) -> Mint{
        let mut res = Mint::new(0);
        for i in 0..Vec<Mint>.len(){
            res *= c;
            res += Vec<Mint>[i];
        }
        res
    }
    
    //Bostan-Mori法 O(dlogdlogN)
    pub fn bostan_mori(&Vec<Mint>,Q:&Vec<Mint>,N:usize) -> Mint{
        if N == 0 { return Vec<Mint>[0]/Q[0]; }
        let d = Q.len();
        let mut Q_neg = Q.clone();
        for i in (1..d).step_by(2) {
            Q_neg[i] *= Mint::new(-1);
        }
        let mut V = Q.mul(&Q_neg,2*d);
        V = V.into_iter().step_by(2).collect();
        let mut U = Vec<Mint>.mul(&Q_neg,2*d);
        if N % 2 == 0 { 
            U = U.into_iter().step_by(2).collect();
        } else {
            U = U.into_iter().skip(1).step_by(2).collect();
        }
        U.bostan_mori(&V,N/2)
    }
    
    fn taylor_shift(&Vec<Mint>,c:Mint){
        
    } 
}
