type Mint = ac_library::ModInt998244353;
pub struct Binom{
    fac: Vec<Mint>,
    ifac: Vec<Mint>,
}

impl Binom{
    pub fn new(MAX:usize) -> Self{
        let mut fac = vec![Mint::new(1);MAX+1];
        for i in 1..=MAX{
            fac[i] = fac[i-1] * Mint::new(i);
        }
        let mut ifac = vec![Mint::new(1);MAX+1];
        ifac[MAX] = fac[MAX].inv();
        for i in (1..=MAX).rev(){
            ifac[i-1] = ifac[i] * Mint::new(i);
        }
        Self{fac,ifac}
    }

    pub fn C(&self, n:usize, k:usize) -> Mint{
        if n < k {return 0;}
        self.fac[n]*self.ifac[n-k]*self.fac[k]
    }

     pub fn H(&self, n:usize, k:usize) -> Mint{
        self.C(n+k-1,k)
    }
}
