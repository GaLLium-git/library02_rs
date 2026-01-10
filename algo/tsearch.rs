//三分探索
fn main() {
    let f = |x: usize| (x - 5) * (x - 5);
    let x = tsearch_max_usize(0..11, f); 
    println!("imax: x = {}, f(x) = {}", x, f(x));

    let g = |x: f64| (x - 2.0).powi(2) + 1.0;
    let x = tsearch_min_f64(0.0..5.0, g, 1e-9);
    println!("fmin: x = {:.9}, f(x) = {:.9}", x, g(x));
}


pub fn tsearch_max_usize<F>(range: impl std::ops::RangeBounds<usize>, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let (mut l, mut r) = get_bounds_usize(range);
    while r - l > 3 {
        let m1 = l + (r - l) / 3;
        let m2 = r - (r - l) / 3;
        if f(m1) < f(m2) {
            l = m1;
        } else {
            r = m2;
        }
    }
    (l..r).max_by_key(|&x| f(x)).unwrap()
}

pub fn tsearch_min_usize<F>(range: impl std::ops::RangeBounds<usize>, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let (mut l, mut r) = get_bounds_usize(range);
    while r - l > 3 {
        let m1 = l + (r - l) / 3;
        let m2 = r - (r - l) / 3;
        if f(m1) > f(m2) {
            l = m1;
        } else {
            r = m2;
        }
    }
    (l..r).min_by_key(|&x| f(x)).unwrap()
}

pub fn tsearch_max_f64<F>(range: impl std::ops::RangeBounds<f64>, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let (mut l, mut r) = get_bounds_f64(range);
    while r - l > eps {
        let m1 = l + (r - l) / 3.0;
        let m2 = r - (r - l) / 3.0;
        if f(m1) < f(m2) {
            l = m1;
        } else {
            r = m2;
        }
    }
    (l + r) / 2.0
}

pub fn tsearch_min_f64<F>(range: impl std::ops::RangeBounds<f64>, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let (mut l, mut r) = get_bounds_f64(range);
    while r - l > eps {
        let m1 = l + (r - l) / 3.0;
        let m2 = r - (r - l) / 3.0;
        if f(m1) > f(m2) {
            l = m1;
        } else {
            r = m2;
        }
    }
    (l + r) / 2.0
}
