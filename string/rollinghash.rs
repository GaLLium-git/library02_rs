fn main(){
    let s:Vec<char>="atcatc".chars().collect();
    let mut rol=RollingHash::new(&s,10);
    if rol.hash(0,3)==rol.hash(3,6){
        println!("ok");
    }
}

type Mint1 = ac_library::ModInt1000000007;
type Mint2 = ac_library::ModInt998244353;
pub struct RollingHash{
    base:usize,
    hash1: Vec<Mint1>,
    hash2: Vec<Mint2>,
}
impl RollingHash {
    pub fn new(s: &Vec<char>, base: usize) -> Self{
        let n = s.len();
        let mut hash1 = vec![Mint1::new(0);n+1];
        for i in 1..=n{
            hash1[i] = hash1[i-1]*Mint1::new(base)+Mint1::new(s[i-1] as u32);
        }
        let mut hash2 = vec![Mint2::new(0);n+1];
        for i in 1..=n{
            hash2[i] = hash2[i-1]*Mint2::new(base)+Mint2::new(s[i-1] as u32);
        }
        RollingHash {
            base,
            hash1,
            hash2,
        }
    }

    //sの[l,r)のハッシュ(0indexed)
    pub fn hash(&self, l: usize, r: usize) -> (Mint1,Mint2){
        let res1 = self.hash1[r]-self.hash1[l]*Mint1::new(self.base).pow((r-l) as u64);
        let res2 = self.hash2[r]-self.hash2[l]*Mint2::new(self.base).pow((r-l) as u64);
        (res1,res2)
    }
}
