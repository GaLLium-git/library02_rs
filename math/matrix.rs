//正方行列
#[derive(Clone)]
pub struct Matrix{
    mat: Vec<Vec<Mint>>,
}

impl Matrix{
    pub fn new(A:&Vec<Vec<Mint>>) -> Self{
        Matrix{mat: A.clone()}
    }
    
    //サイズNの単位行列
    pub fn one(N:usize) -> Self{
        let mut res = vec![vec![Mint::new(0);N];N];
        for i in 0..N{
            res[i][i] = Mint::new(1);
        }
        Matrix{mat: res}
    }
    
    pub fn mul(&self, rhs:&Matrix) -> Self{
        let N = self.mat.len();
        let mut res = vec![vec![Mint::new(0);N];N];
        for i in 0..N{
            for j in 0..N{
                for k in 0..N{
                    res[i][j] += self.mat[i][k] * rhs.mat[k][j];
                }
            }
        }
        Matrix{mat: res}
    }
    
    pub fn pow(&self, mut k:usize) -> Self{
        let mut M = self.clone();
        let mut res = Self::one(self.mat.len());
        for i in 0..=k.ilog2(){
            if k & (1<<i) != 0{
                res = res.mul(&M);
            }
            M = M.mul(&M);
        }
        res
    }
}
