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
