fn main(){
    let v:Vec<usize>=vec![0,3,5,6,4,4,7];
    let sp=SparseTable::<ac_library::Max<usize>>::new(&v);
    
    println!("prod 2..6 ={}",sp.prod(2..6));
    println!("prod 5..=5 ={}",sp.prod(5..=5));
}



use ac_library::Monoid;
pub struct SparseTable<M:Monoid>{
    data: Vec<Vec<M::S>>,
}

impl<M:Monoid> SparseTable<M>{
    fn new(val:&Vec<M::S>)->Self{
        let log=(val.len().ilog2() +1) as usize;
        let len=(1<<log);
        let mut data=vec![vec![M::identity();len];log+1]; //i段目は分割幅2^i
        for i in 0..val.len(){
            data[0][i]=val[i].clone();
        }
        
        for i in 1..=log{
            //i段目の累積処理
            for pa in 0..(1<<(log-i)){
                let (left,right)=(pa*(1<<i),(pa+1)*(1<<i)); //区間[l,r) を担当
                let mid=(left+right)/2; //累積の始点の仕切りの右側
                
                //右方向への累積
                let mut accr=M::identity();
                for j in mid..right{
                    accr=M::binary_operation(&accr,&data[0][j]);
                    data[i][j]=accr.clone();
                }
                //左方向への累積
                let mut accl=M::identity();
                for j in (left..mid).rev(){
                    accl=M::binary_operation(&accl,&data[0][j]);
                    data[i][j]=accl.clone();
                }
            }
        }
        
        Self{data}
    }
    
    fn prod(&self,range: impl std::ops::RangeBounds<usize>)->M::S{
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(r) => *r,
            std::ops::Bound::Excluded(r) => r - 1,
            std::ops::Bound::Unbounded => self.data[0].len() - 1,
        };
        let mut l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        
        if l==r{
            return self.data[0][l].clone();
        } else {
            let d=((l^r).ilog2() +1) as usize;
            return M::binary_operation(&self.data[d][l],&self.data[d][r]);
        }
    }
}
