//下位集合のゼータ変換
pub fn zeta_sub(A:&Vec<usize>) -> Vec<usize>{
    let mut res = A.clone();
    let log = A.len().trailing_zeros() as usize;
    for i in 0..log {
        for mask in 0..res.len() {
            if (mask & (1 << i)) != 0 {
                res[mask] += res[mask ^ (1 << i)];
            }
        }
    }
    res
}

//下位集合のメビウス変換
pub fn mobius_sub(A:&Vec<usize>) -> Vec<usize>{
    let mut res = A.clone();
    let log = A.len().trailing_zeros() as usize;
    for i in 0..log {
        for mask in 0..res.len() {
            if (mask & (1 << i)) != 0 {
                res[mask] -= res[mask ^ (1 << i)];
            }
        }
    }
    res
}


//上位集合のゼータ変換
pub fn zeta_sup(A:&Vec<usize>) -> Vec<usize>{
    let mut res = A.clone();
    let log = A.len().trailing_zeros() as usize;
    for i in 0..log {
        for mask in 0..res.len() {
            if (mask & (1 << i)) == 0 {
                res[mask] += res[mask ^ (1 << i)];
            }
        }
    }
    res
}

//上位集合のメビウス変換
pub fn mobius_sup(A:&Vec<usize>) -> Vec<usize>{
    let mut res = A.clone();
    let log = A.len().trailing_zeros() as usize;
    for i in 0..log {
        for mask in 0..res.len() {
            if (mask & (1 << i)) == 0 {
                res[mask] -= res[mask ^ (1 << i)];
            }
        }
    }
    res
}

