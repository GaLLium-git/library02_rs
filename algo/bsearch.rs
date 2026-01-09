//二分探索

//range で fがtrueとなる最小を返す
pub fn bsearch_usize<F>(range: impl std::ops::RangeBounds<usize>, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let (mut l, mut r) = get_bounds_usize(range);
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

//浮動小数点バージョン
pub fn bsearch_f64<F>(range: impl std::ops::RangeBounds<f64>, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> bool,
{
    let mut l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => l+eps,
        std::ops::Bound::Unbounded => f64::MIN,
    };
    let mut r = match range.end_bound() {
        std::ops::Bound::Included(r) => r+eps,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => f64::MAX,
    };
    while r - l > eps {
        let m = (l + r) / 2.0;
        if f(m) {
            r = m;
        } else {
            l = m;
        }
    }
    l
}
