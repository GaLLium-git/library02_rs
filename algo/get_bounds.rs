//usizeのrange型を[l,r)に直す関数]
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
