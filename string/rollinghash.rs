fn main(){
}

pub struct RollingHash {
    base: u64,
    mod1:u32,
    mod2:u32,
    hash1: Vec<StaticModInt<mod1>>,
    hash2: Vec<StaticModInt<mod2>>,
}

impl RollingHash {
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

    pub fn hash(&self, l: usize, r: usize) -> (StaticModInt<mod1>,StaticModInt<mod2>) {
        let res1=hash1[r-1]-hash1[l-1]*StaticModInt<mod1>::new(base).pow(r-l);
        let res2=hash2[r-1]-hash2[l-1]*StaticModInt<mod2>::new(base).pow(r-l);
        (res1,res2)
    }

}
