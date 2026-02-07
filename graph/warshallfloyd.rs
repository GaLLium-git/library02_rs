pub fn warshall_floyd(N:usize, edges:&Vec<(usize,usize,usize)>) -> Vec<Vec<usize>>{
    let mut dist = vec![vec![usize::MAX/2;N+3];N+3];
    for i in 1..=N{
        dist[i][i] = 0;
    }
    for &(a,b,c) in edges.iter(){
        dist[a][b] = c;
        dist[b][a] = c;
    }
    for k in 1..=N{
        for i in 1..=N{
            for j in 1..=N{
                dist[i][j] = dist[i][j].min(dist[i][k]+dist[k][j]);
            }
        }
    }
    dist
}
