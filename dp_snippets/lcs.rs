fn main() {
    let mut sc = Scanner::new();
    let mut S:Vec<char> = sc.next::<String>().chars().collect();S.insert(0,'');
    let mut T:Vec<char> = sc.next::<String>().chars().collect();S.insert(0,'');
    
    let mut dp = vec![vec![0usize;T.len()];S.len()];
    for i in 1..S.len(){
        for j in 1..T.len(){
            if S[i]==T[j]{
                dp[i][j] = dp[i-1][j-1]+1;
            } else{
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }
    
    //復元
    let (mut i, mut j) = (S.len(),T.len());
    let mut ans = vec![];
    while (i > 0) && (j > 0){
        if S[i]==T[j]{
            ans.push(S[i]);
            i -= 1; j -= 1;
        } else if dp[i-1][j] > dp[i][j-1]{
            i -= 1;
        } else{
            j -= 1;
        }
    }
    ans.reverse();
    println!("{}", ans.iter().join(""));
    //eprintln!("{:?}",dp);
}
