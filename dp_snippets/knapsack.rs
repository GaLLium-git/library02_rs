fn main() {
    let mut sc = Scanner::new();
    let (N,W):(usize,usize) = (sc.next(),sc.next());
    let mut w = vec![0usize;N+1];
    let mut v = vec![0usize;N+1];
    for i in 1..=N{
        (w[i],v[i]) = (sc.next(),sc.next());
    }
    
    let mut dp = vec![vec![0usize;W+1];N+1];
    for i in 1..=N{
        for j in 0..=W{
            if j < w[i]{
                dp[i][j] = dp[i-1][j];
            }
            else{
                dp[i][j] = dp[i-1][j].max(dp[i-1][j-w[i]]+v[i]);
            }
        }
    }
    println!("{}",dp[N].iter().max().unwrap());
    //eprintln!("{:?}",dp);
}
 
