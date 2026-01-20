pub fn dijkstra(graph:&Vec<Vec<usize>>, root:usize) -> Vec<usize>{
    use std::cmp::Reverse;
    let mut check = vec![false;N+1];
    let mut cur = vec![usize::MAX;N+1];
    let mut pq = BinaryHeap::new();
    cur[root] = 0;
    pq.push(Reverse((cur[root],root)));
    
    while !(pq.is_empty()) {
        let Reverse((_,pos)) = pq.pop().unwrap();
        if check[pos]{continue;}
        check[pos] = true;
        for &(nex,cost) in graph[pos].iter(){
            if cur[nex] > cur[pos]+cost{
                cur[nex] = cur[pos]+cost;
                pq.push(Reverse((cur[nex],nex)));
            }            
        }
    }
    cur
}

