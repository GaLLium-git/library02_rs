pub fn bfs(graph: &Vec<Vec<usize>>, root:usize) -> Vec<usize>{
    let mut que = VecDeque::new();
    let mut dist = vec![usize::MAX;graph.len()];
    que.push_back(root);
    dist[root]=0;
    while !(que.is_empty()) {
        let pos = que.pop_front().unwrap();
        for &nex in graph[pos].iter(){
            if dist[nex] == usize::MAX{
                dist[nex] = dist[pos]+1;
                que.push_back(nex);
            }
        }
    }
    dist
}

fn main() {
    let mut sc = Scanner::new();
    let (n,m):(usize,usize) = (sc.next(),sc.next());
    let mut graph=vec![vec![];n+1];
    for i in 1..=m{
      let (a,b):(usize,usize)=(sc.next(),sc.next());
      graph[a].push(b);
      graph[b].push(a);
    }
    let dist = bfs(&graph,1);
    for i in 1..=n{
        println!("{}",dist[i]);
    }
    
}
    
