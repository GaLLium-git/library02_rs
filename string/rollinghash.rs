pub fn rolling_hash(s:&Vec<char>,base:ModInt)->Vec<ModInt>{
    let mut ret=vec![ModInt::new(0);s.len()];
    ret[0]=ModInt::new(s[0] as usize);
    for i in 1..s.len(){
        ret[i]=ret[i-1]*base+ ModInt::new(s[i] as usize);
    }
    ret
}


fn main() {
    let mut sc=Scanner::new();
    type ModInt = ac_library::StaticModInt<1000000000000000009>;
    let n:usize=sc.next();
    let mut s:Vec<char>=sc.next::<String>().chars().collect();s.shift();
    // use rand::Rng;
    let mut rng = rand::thread_rng();
    let base=ModInt::new(rng.gen_range(1000..=100_000) | 1) ;
    let rori=rolling_hash(&s,base);
  
    let check = |k: usize| -> bool {
    let mut seen: HashMap<usize, usize> = HashMap::new(); 
    for l in 1..=n-k+1 {
        let r = l+k-1;
        let hash = (rori[r] - rori[l - 1] * base.pow(k)).val;
        if let Some(&prev_l) = seen.get(&hash) {
            if prev_l + k - 1 < l {
                return  false;
            }
        } else {
            seen.insert(hash, l);
        }
    }

    true
    };

    let ans=bsearch_irange(0,n+1,check)-1;
    println!("{}",ans);
}





  
    let ans=bsearch_irange(0,(n+1) as i64,check)-1;
    println!("{}",ans);
}
