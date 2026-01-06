pub fn eulertour(v:usize, graph:&Vec<Vec<usize>>, seen:&mut Vec<bool>, tour:&mut Vec<usize>){
    tour.push(v);
    seen[v]=true;
    for &nv in graph[v].iter(){
        if !(seen[nv]) {
           eulertour(nv,graph,seen,tour);
        }
    }
    tour.push(v);
}
