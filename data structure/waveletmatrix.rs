//Wavelet Matrix
//座圧したほうがいい？
pub struct WaveletMatrix{
    data: Vec<bitvec>,
}

impl WaveletMatrix{
    pub fn new(val:&Vec<usize>) -> Self{
        let log = val.iter().max().unwrap().ilog2();
        let mut data = vec![bitvec::new(val.len());log+1];
        for i in (0..=log).rev(){
            for j in 0..val.len(){
                if val[j] >> i == 1{
                    data[i].set(j);
                }
            }
            data[i].build();
        }
        Self{
            data,
        }
    }
    
    
    pub fn access(&self, i:usize) -> usize{
        let mut res = 0;
        let mut idx = i;
        for b in (0..data.len()).rev(){
            res = res*2 + self.data[b].access(idx);
            if self.data[b].access(idx) == 0{
                idx = self.data[b].rank0(idx)
            } else {
                idx = self.data[b].rank1(idx) + self.data[b].count_zeros();
            }
        }
        res
    }
    
    //rangeでのx未満の値の出現回数
    pub fn rangefreq(&self, range: impl std::ops::RangeBounds<usize>, x:usize) -> usize{
        let (mut l, mut r) = get_bounds_usize(range);
        let mut res = 0;
        for b in (0..data.len()).rev(){
            
            if (x>>b)&1 == 0{
                
            } else{
                
            }
        }
        
    }
    
    pub fn kth_smallest(&self, range: impl std::ops::RangeBounds<usize>, k:usize) -> usize{
        let (mut l, mut r) = get_bounds_usize(range);
    }
    
    pub fn kth_largest(&self, range: impl std::ops::RangeBounds<usize>, k:usize) -> usize{
        let (mut l, mut r) = get_bounds_usize(range);
        self.kth_smallest(range,r-l-k-1)
    }
    
    
}

//ビット列に対する操作
struct bitvec{
    bits: Vec<usize>,
    sum: Vec<usize>,
}

impl bitvec{
    pub fn new(n:usize) -> Self{
        Self{
            bits: vec![0;n/64+1],
            sum: vec![0;n/64+1],
        }
    }
    
    pub fn set(&mut self, i:usize){
        self.bits[i/64] |= 1<<(i%64);
    }
    
    pub fn build(&mut self){
        self.sum[0] = self.bits[0].counts_ones();
        for i in 1..=n/64{
            self.sum[i] = self.sum[i-1] + self.bits[i].counts_ones(); 
        }
    }
    
    pub fn access(&self, i:usize) -> usize{
        (self.bits[i/64] >> (i%64)) & 1
    }
    
    //[0,i) での 1の個数
    pub fn rank1(&self, i:usize) -> usize{
        self.sum[i/64] + (self.bits[i/64] & (1<<(i%64)-1)).count_ones()
    }
    
    pub fn rank0(&self, i:usize) -> usize{
        i - self.rank1(i)
    }
    
    //全体での0の個数
    pub fn count_zeros(&self) -> usize{
        self.rank0(self.bits.len()*64)
    }
}
