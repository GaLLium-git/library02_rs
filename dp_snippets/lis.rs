pub fn lis(A:&Vec<usize>) -> usize{
   let N = A.len();
   let mut dp = vec![usize::MAX;N];
   for &x in A.iter(){
     let pos = bsearch_usize(0..N,|i| dp[i] > x);
     dp[pos] = x;
   }
   let mut res = 0;
   for i in 0..N{
       if dp[i] == usize::MAX {break;}
       res = i+1;
   }
   res
}
