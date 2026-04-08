//正方行列
type Mint = ac_library::ModInt998244353;
pub trait Matrix{
    fn one(N:usize) -> Vec<Vec<Mint>>;
    fn mul(&self, B:&Vec<Vec<Mint>>) -> Vec<Vec<Mint>>;
    fn pow(&self, k:usize) -> Vec<Vec<Mint>>;
}

impl Matrix for Vec<Vec<Mint>>{
    //サイズNの単位行列
    fn one(N:usize) -> Vec<Vec<Mint>>{
        let mut res = vec![vec![Mint::new(0);N];N];
        for i in 0..N{
            res[i][i] = Mint::new(1);
        }
        res
    }
    
    fn mul(&self, rhs:&Vec<Vec<Mint>>) -> Vec<Vec<Mint>>{
        let N = self.len();
        let mut res = vec![vec![Mint::new(0);N];N];
        for i in 0..N{
            for j in 0..N{
                for k in 0..N{
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
    
    fn pow(&self, mut k:usize) -> Vec<Vec<Mint>>{
        let mut M = self.clone();
        let mut res = Self::one(self.len());
        while k > 1{
            if (k&1) != 0{
                res = res.mul(&M)
            }
            k >>= 1;
            M = M.mul(M);
        }
        res
    }
}
