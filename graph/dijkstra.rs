fn main() {
    let mut sc = Scanner::new();
    let (N,M):(usize,usize) = (sc.next(),sc.next());
    let mut graph = vec![vec![];N+1];
    for i in 1..=M{
        let (a,b,c):(usize,usize,usize)=(sc.next(),sc.next(),sc.next());
        graph[a].push((b,c));
        graph[b].push((a,c));
    }

    use std::cmp::Reverse;
    let mut kakutei = vec![false;N+1];
    let mut cur = vec![usize::MAX;N+1];
    let mut pq = BinaryHeap::new();
    cur[1] = 0;
    pq.push(Reverse((cur[1],1)));
    
    while !(pq.is_empty()) {
        let Reverse((_,pos)) = pq.pop().unwrap();
        if kakutei[pos] == true{
            continue;
        }
        
        kakutei[pos]=true;
        for &(nex,cost) in graph[pos].iter(){
            if cur[nex] > cur[pos]+cost{
                cur[nex] = cur[pos]+cost;
                pq.push(Reverse((cur[nex],nex)));
            }            
        }
    }
    
    for i in 1..=n{
        if cur[i] == usize::MAX{
            println!("{}",-1);
        } else{
            println!("{}",cur[i]);
        }
    }
    
}
    
    
