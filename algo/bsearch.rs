//二分探索

//[l.r) で fがtrueとなる最小を返す
pub fn bsearch_irange<F>(mut l: usize, mut r: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
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
pub fn bsearch_frange<F>(mut l: f64, mut r: f64, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> bool,
{
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
