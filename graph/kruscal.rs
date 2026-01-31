pub fn kruscal(N:usize, edges:&Vec<(usize,usize,usize)>) -> usize{
    let mut U = vec![]; let mut V = vec![]; let mut C = vec![];
    for i in 0..edges.len(){
        let (u,v,cost) = edges[i];
        U.push(u);
        V.push(v);
        C.push(cost,i);
    }
    C.sort();
    
    let mut uf = ac_library::DSU::new(N+1);
    let mut res = 0;
    for &(cost,i) in c.iter(){
        if !(uf.same(U[i],V[i])) {
            res += cost;
            uf.merge(U[i),V[i]);
        }
    }
    res
}

    
