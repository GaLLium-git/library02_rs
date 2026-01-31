pub fn mex(A:&Vec<usize>) -> usize{
    let len = A.len();
    let mut seen = vec![false;len];
    for &g in A.iter(){
        if g < len {seen[g] = true;}
    }
    for i in 0..len{
        if !seen[i] {return i;} 
    }
    len
}

fn main() {
    let mut sc = Scanner::new();
    let (N,K):(usize,usize) = (sc.next(),sc.next());
    let A:Vec<usize> = (0..N).map(|_| sc.next()).collect();
    
    //0が終了状態
    let mut gr = vec![0;K+1];
    
    for i in 0..=K{
        let mut ngr = vec![];
        for &a in A.iter(){
            if i >= a {ngr.push(gr[i-a]);}
        }
        gr[i] = mex(&ngr);
    }
    
    if gr[K] == 0{
        println!("Second");
    } else{
        println!("First");
    }
    //eprintln!("{:?}",gr);
}
