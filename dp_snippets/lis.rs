pub fn lis(A:&Vec<usize>) -> usize{
   let N = A.len();
   let mut lis = vec![usize::MAX;N];
   for &x in A.iter(){
     let pos = bsearch
     lis[pos] = i;
   }
   let mut res = 0;
   for i in 0..N{
       if lis[i] == usize::MAX {break;}
       res = i+1;
   }
   res
}
