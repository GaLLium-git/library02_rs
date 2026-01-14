//いろんなdfs

//seenを持つやつ
pub fn dfs(graph:&Vec<Vec<usize>>, v:usize, seen:&mut Vec<bool>){
    seen[v]=true;
    for &nv in graph[v].iter(){
        if !(seen[nv]) {
          dfs(nv,graph,seen);
        }
    }
}

//親を持つやつ
pub fn dfs(graph:&Vec<Vec<usize>>, v:usize, p:usize){
    for &nv in graph[v].iter(){
        if nv == p {continue;}
        dfs(nv,graph,seen);
    }
}


//例:木のオイラーツアー
pub fn eulertour(graph:&Vec<Vec<usize>>, root:usize) -> Vec<usize>{
    tour.push(v);
    seen[v]=true;
    for &nv in graph[v].iter(){
        if !(seen[nv]) {
           eulertour(nv,graph,seen,tour);
        }
    }
    tour.push(v);
}
