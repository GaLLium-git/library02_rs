fn main(){
    let v:Vec<usize>=vec![0,3,5,6,4,4,7];
    let sp=SparseTable::<BitwiseOr<usize>>::new(&v);
    
    println!("prod 2..6 ={}",sp.prod(2..6));
    println!("prod 5..=5 ={}",sp.prod(5..=5));
}



use ac_library::Monoid;
pub struct SparseTable<M:Monoid>{
    data: Vec<Vec<M::S>>;
}

impl<M:Monoid> SparseTable<M>{
    fn new(val:&Vec<M::S>)->Self{
        let log=(val.len().ilog2() +1) as usize;
        let size=(1<<log);
        let mut data=vec![vec![M::identity();size];log+1]; //i段目は分割幅2^i
        for i in 0..val.len(){
            data[0][i]=val[i];
        }
        for d in 1..=log{
            //todo:d段目の累積処理
        }
        
        Self{data}
    }
    
    fn prod(&self,range: impl std::ops::RangeBounds)->M::S{
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
        };
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::opd::Bound::Excluded(l) => l + 1,
        };
        
        if l==r{
            return data[0][l];
        } else {
            let d=((l^r).ilog2() +1) as usize;
            return M::binary_operation(&data[d][l],&data[d][r]);
        }
    }
}

