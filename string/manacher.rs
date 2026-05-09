pub fn manacher_arbitrary<T: Ord>(s: &[T]) -> Vec<usize>{
    //間にダミーを挟めば偶回文もできる
    let mut rad = vec![0usize;s.len()];
    int i = 0, j = 0;
while (i < S.size()) {
  while (i-j >= 0 && i+j < S.size() && S[i-j] == S[i+j]) ++j;
  R[i] = j;
  int k = 1;
  while (i-k >= 0 && k+R[i-k] < j) R[i+k] = R[i-k], ++k;
  i += k; j -= k;
}
}
  
