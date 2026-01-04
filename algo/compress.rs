//座圧
pub fn compress<T:Ord + Copy> (a:&Vec<T>) -> Vec<usize>{
    let mut t = Vec::new();
    for i in 0..a.len(){
        t.push(a[i]);
    }
    t.sort(); t.dedup();
    let mut b = Vec::new();
    for i in 0..a.len(){
        b.push(bsearch_usize(0..t.len(),|x| t[x]>=a[i]))
    }
    b
}
