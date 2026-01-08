//Wavelet Matrix
//座圧したほうがいい？
pub struct WaveletMatrix{
    data: Vec<bitvec>,
    count0: Vec<usize>,
}

impl WaveletMatrix{
    pub fn new(val:&Vec<usize>) -> Self{
        let log = val.iter().max().unwrap().ilog2();
        let mut data = vec![];
        let mut count0 = vec![];
        for i in (0..=log).rev(){
            let bits_i = vec![];
            for &value in val.iter(){
                bits_i.push(value>>i);
            }
            data.push(bitvec::new(&bits_i));
        }
        Self{
            data,
            count0,
        }
    }
    
    pub fn access(i:usize) -> usize{
        let mut res = 0;
        let mut idx = i;
        for b in 0..data.len(){
            res = res*2 + data[b].access(idx);
            if data[b].access(idx) == 0{
                idx = data[b].rank0(idx)
            } else {
                idx = data[b].rank1(idx) + count0[b];
            }
        }
        res
    }
}

struct bitvec{
    
}

impl bitvec{
    
}
