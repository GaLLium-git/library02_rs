fn main(){
}

pub struct RollingHash {
    hash1: Vec<StaticModInt>,
    hash2: Vec<StaticModInt>,
}

impl RollingHash {
    const MOD1=1000000007;
    const MOD2=998244353;
    pub fn new(s: &Vec<char>, base: u64) -> Self {
        let n = s.len();
        let mut hash1 = vec![0; n + 1];
        
        for i in 0..n{
            hash1[i + 1] = (hash1[i] * base + s[i] as usize
        }
        for (i, c) in s.bytes().enumerate() {
            hash[i + 1] = (hash[i] * base + c as u64) % modulo;
            power[i + 1] = (power[i] * base) % modulo;
        }

        RollingHash {
            base,
            hash1,
            hash2,
        }
    }

    pub fn hash(&self, l: usize, r: usize) -> (StaticModInt<MOD1>,StaticModInt<MOD2>) {
        let res1=hash1[r-1]-hash1[l-1]*StaticModInt<MOD1>::new(BASE).pow(r-l);
        let res2=hash2[r-1]-hash2[l-1]*StaticModInt<MOD2>::new(BASE).pow(r-l);
        (res1,res2)
    }

}
