//トポロジカルソート
pub fn topological_sort(graph:&Vec<Vec<usize>>) -> Vec<usize>{
    let N = graph.len();
    //入次数
    let mut indeg = vec![0;N];
    for v in 0..N{
        for &nv in graph[v].iter(){
            indeg[nv] += 1;
        }
    }
    
    let mut que = VecDeque::new();
    for v in 0..N{
        if indeg[v] == 0{
            que.push_back(v);
        }
    }
    
    let mut res = vec![];
    while !(que.is_empty()) {
        let pos = que.pop_front().unwrap();
        res.push(pos);
        for &nex in graph[pos].iter(){
            indeg[nex] -= 1;
            if indeg[nex] == 0{
                que.push_back(nex);
            }
        }
    }
    res
}
