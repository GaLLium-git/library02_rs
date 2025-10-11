//三分探索
//中身は関数名の通り、型は問題に応じて変えよう
fn main() {
    let f = |x: usize| -(x - 5) * (x - 5);
    let x = tsearch_imax(0, 11, f); // [0, 11) → x ∈ [0,10]
    println!("imax: x = {}, f(x) = {}", x, f(x));

    let g = |x: f64| (x - 2.0).powi(2) + 1.0;
    let x = tsearch_fmin(0.0, 5.0, g, 1e-9);
    println!("fmin: x = {:.9}, f(x) = {:.9}", x, g(x));
}


pub fn tsearch_imax<F>(mut l: usize, mut r:usize, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
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

pub fn tsearch_imin<F>(mut l: usize, mut r: usize, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
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

pub fn tsearch_fmax<F>(mut l: f64, mut r: f64, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> f64,
{
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

pub fn tsearch_fmin<F>(mut l: f64, mut r: f64, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> f64,
{
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
