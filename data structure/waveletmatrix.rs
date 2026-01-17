//Wavelet Matrix
//座圧したほうがいい？
pub struct WaveletMatrix{
    data: Vec<BitVec>,
}

impl WaveletMatrix{
    pub fn new(val:&Vec<usize>) -> Self{
        let log = val.iter().max().unwrap().ilog2() as usize;
        let mut data = vec![BitVec::new(val.len());log+1];
        let mut cur = val.clone();
        for b in (0..=log).rev(){
            for i in 0..cur.len(){
                if cur[i] >> b == 1{
                    data[b].set(i);
                }
            }
            data[b].build();
            let mut nxt = cur.clone();
            for i in 0..cur.len(){
                if data[b].access(i) == 0{
                    nxt[data[b].rank0(i)] = cur[i];
                } else{
                    nxt[data[b].rank1(i) + data[b].zero] = cur[i];
                }
            }
            cur = nxt;
        }
        Self{data}
    }
    
    //T[i]を得る
    pub fn access(&self, i:usize) -> usize{
        let mut res = 0;
        let mut idx = i;
        for b in (0..self.data.len()).rev(){
            res = res*2 + self.data[b].access(idx);
            if self.data[b].access(idx) == 0{
                idx = self.data[b].rank0(idx)
            } else {
                idx = self.data[b].rank1(idx) + self.data[b].zero;
            }
        }
        res
    }
    
    //rangeでのx未満の値の出現回数
    pub fn rangefreq(&self, range: impl std::ops::RangeBounds<usize>, x:usize) -> usize{
        let (mut l, mut r) = get_bounds_usize(range);
        let mut res = 0;
        for b in (0..self.data.len()).rev(){
            if (x>>b)&1 == 1{
                res += self.data[b].rank0(l) - self.data[b].rank0(r);
                l = self.data[b].rank1(l) + self.data[b].zero;
                r = self. data[b].rank1(r) + self.data[b].zero;
            } else{
                l = self.data[b].rank0(l);
                r = self.data[b].rank0(r);
            }
        }
        res
    }
    
    //rangeでk番目に小さい数(0-indexed)
    pub fn kth_smallest(&self, range: impl std::ops::RangeBounds<usize>, mut k:usize) -> usize{
        let (mut l, mut r) = get_bounds_usize(range);
        let mut res = 0;
        for b in (0..self.data.len()).rev(){
            let cnt0 = self.data[b].rank0(l) -self. data[b].rank0(r);
            if cnt0 <= k{
                res |= 1<<b;
                k -= cnt0;
                l = self.data[b].rank1(l) + self.data[b].zero;
                r = self.data[b].rank1(r) + self.data[b].zero;
            } else{
                l = self.data[b].rank0(l);
                r = self.data[b].rank0(r);
            }
        }
        res
    }
    
    //rangeでk番目に大きい数(0-indexed)
    pub fn kth_largest(&self, range: impl std::ops::RangeBounds<usize>, k:usize) -> usize{
        let (l,r) = get_bounds_usize(range);
        self.kth_smallest(l..r, r-l-k-1)
    }
    
    //rangeに存在するx未満の最大値 (存在しないならusize::MAX)
    pub fn prev_value(&self, range: impl std::ops::RangeBounds<usize>, x:usize) -> usize{
        let (l,r) = get_bounds_usize(range);
        let cnt = self.rangefreq(l..r, x);
        if cnt == 0 {usize::MAX} else {self.kth_smallest(l..r, cnt-1)}
    }
    
    //rangeに存在するx以上の最小値
    pub fn next_value(&self, range: impl std::ops::RangeBounds<usize>, x:usize) -> usize{
        let (l,r) = get_bounds_usize(range);
        let cnt = self.rangefreq(l..r, x);
        self.kth_smallest(l..r, cnt)
    }
    
}

//ビット列に対する操作
#[derive(Clone)]
struct BitVec{
    bits: Vec<usize>,
    sum: Vec<usize>,
    zero: usize,
}

impl BitVec{
    pub fn new(n:usize) -> Self{
        Self{
            bits: vec![0;n/64+1],
            sum: vec![0;n/64+1],
            zero: n,
        }
    }
    
    pub fn set(&mut self, i:usize){
        self.bits[i/64] |= 1<<(i%64);
    }
    
    pub fn build(&mut self){
        self.sum[0] = self.bits[0].count_ones() as usize;
        for i in 1..=self.sum.len(){
            self.sum[i] = self.sum[i-1] + self.bits[i].count_ones() as usize; 
        }
        self.zero -= self.sum.last().unwrap();
    }
    
    pub fn access(&self, i:usize) -> usize{
        (self.bits[i/64] >> (i%64)) & 1
    }
    
    //[0,i) での 1の個数
    pub fn rank1(&self, i:usize) -> usize{
        self.sum[i/64] + (self.bits[i/64] & (1<<(i%64)-1)).count_ones() as usize
    }
    
    pub fn rank0(&self, i:usize) -> usize{
        i - self.rank1(i)
    }
}
