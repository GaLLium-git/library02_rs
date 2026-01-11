fn main(){
    //階乗，階乗逆元
    let MAX = 40000000;
    let mut fac = vec![Mint::new(1);MAX+1];
    for i in 1..=MAX{
        fac[i] = fac[i-1] * Mint::new(i);
    }
    let mut ifac = vec![Mint::new(1);MAX+1];
    ifac[MAX] = fac[MAX].inv();
    for i in (1..=MAX).rev(){
        ifac[i-1] = ifac[i] * Mint::new(i);
    }
    let C = |n:usize, k:usize| -> Mint{
        fac[n]*ifac[n-k]*ifac[k]
    };
}
