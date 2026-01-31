fn main() {
    let mut sc = Scanner::new();
    let N:usize = sc.next();
    let A:Vec<i64> = (0..N).map(|_| sc.next()).collect();
    
    let mut dp = vec![vec![0i64;N];N];
    
    for d in 0..N{
        for l in 0..N-d{
            if d == 0 {dp[l][l] = A[l];}
            else {dp[l][l+d] = (A[l] - dp[l+1][l+d]).max(A[l+d] - dp[l][l+d-1]);}
        }
    }
    
    println!("{}",dp[0][N-1]);
    //eprintln!("{:?}",dp);
}
