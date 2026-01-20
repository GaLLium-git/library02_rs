pub fn bellmanford(N:usize, edges: &Vec<(usize,usize,i64)>, root:usize) -> Vec<i64>{
    let mut dist = vec![i64::MAX;N+1];
    dist[root]=0;
    for _ in 0..N-1{
        for &(u,v,cost) in edges.iter(){
            if dist[u] != i64::MAX && dist[v] > dist[u]+cost{
                dist[v] = dist[u]+cost;
            }
        }
    }
    
    for _ in 0..N-1{
        for &(u,v,cost) in edges.iter(){
            if dist[u] == i64::MIN || (dist[u] != i64::MAX && dist[v] > d[u]+cost){
                dist[v]=i64::MIN;
            }
        }
    }
    dist
}
