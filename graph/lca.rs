fn main(){
    let mut sc=Scanner::new();
    let N:usize=sc.next();
    let mut graph=vec![vec![];N+1];
    for _ in 0..N-1{
        let (x,y):(usize,usize)=(sc.next(),sc.next());
        graph[x].push(y);
        graph[y].push(x);
    }
    let mut lc=LCA::new(graph);
    lc.init(1);
    let Q:usize=sc.next();
    for _ in 0..Q{
        let (a,b):(usize,usize)=(sc.next(),sc.next());
        let ans=lc.depth[a]+lc.depth[b]-2*lc.depth[lc.lca(a,b)]+1;
        println!("{}",ans);
    }
}

struct LCA {
    graph: Vec<Vec<usize>>,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl LCA {
    pub fn new(graph :Vec<Vec<usize>>) -> Self {
        let n=graph.len();
        LCA {
            graph,
            parent: vec![vec![None;n];(n.ilog2()+2) as usize],
            depth: vec![0;n],
        }
    }

    fn dfs(&mut self, v: usize, p: Option<usize>, d: usize) {
        self.parent[0][v]= p;
        self.depth[v]=d;
        for i in 0..self.graph[v].len() {
            let nv=self.graph[v][i];
            if Some(nv)!=p {
                self.dfs(nv,Some(v),d+1);
            }
        }
    }

    pub fn init(&mut self, root: usize) {
        self.dfs(root,None,0);
        for k in 0..self.parent.len()-1 {
            for v in 0..self.graph.len() {
                if let Some(p) = self.parent[k][v]{
                    self.parent[k+1][v]=self.parent[k][p];
                }
            }
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize{
        if self.depth[u] < self.depth[v]{
            std::mem::swap(&mut u, &mut v);
        }

        for k in (0..self.parent.len()).rev(){
            if let Some(p)=self.parent[k][u]{
                if self.depth[p]>=self.depth[v]{
                    u = p;
                }
            }
        }
       
        if u==v {return u;}
        for k in (0..self.parent.len()).rev(){
            if self.parent[k][u].is_some() && self.parent[k][v].is_some() &&self.parent[k][u] != self.parent[k][v]{
                u = self.parent[k][u].unwrap();
                v = self.parent[k][v].unwrap();
            }
        }

        self.parent[0][u].unwrap()
    }
}
