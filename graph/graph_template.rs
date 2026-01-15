//グラフ受け取りを毎回書きたくない

//重み無し
fn main() {
    let mut sc = Scanner::new();
    let (N,M):(usize,usize) = (sc.next(),sc.next());
    let mut graph = vec![vec![];N+1];
    for i in 1..=M{
        let (u,v):(usize,usize) = (sc.next(),sc.next());
        graph[u].push(v);
        graph[v].push(u);  //有向ならいらない
    }
    //処理 
}


//重みあり
fn main() {
    let mut sc = Scanner::new();
    let (N,M):(usize,usize) = (sc.next(),sc.next());
    let mut graph = vec![vec![];N+1];
    for i in 1..=M{
        let (u,v,c):(usize,usize) = (sc.next(),sc.next());
        graph[u].push((v,c));
        graph[v].push((u,c));  //有向ならいらない
    }
    //処理 
}
