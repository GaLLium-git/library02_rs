//range型を[l,r)に直す関数
pub fn get_bounds_usize(range: impl std::ops::RangeBounds<usize>) -> (usize,usize){
    let l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => *l+1,
        std::ops::Bound::Unbounded => 0,
    };
    let r = match range.end_bound() {
        std::ops::Bound::Included(r) => *r+1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => usize::MAX,
    };
    (l,r)
}



pub fn get_bounds_f64(range: impl std::ops::RangeBounds<f64>) -> (f64,f64){
    const EPS:f64 = 1e-9;
    let l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => *l+EPS,
        std::ops::Bound::Unbounded => 0.0,
    };
    let r = match range.end_bound() {
        std::ops::Bound::Included(r) => *r+1e-9,
        std::ops::Bound::Excluded(r) => *r+EPS,
        std::ops::Bound::Unbounded => f64::MAX,
    };
    (l,r)
}
