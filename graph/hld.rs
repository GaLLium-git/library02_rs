//HL分解 (可換モノイド前提)
pub struct HLD{
    graph: Vec<Vec<usize>>,
    depth: Vec<usize>, //深さ
    parent: Vec<usize>, //親
    size: Vec<usize>,  //部分木のサイズ
    heavy: Vec<usize>, //heavyな子
    top: Vec<usize>,  //属するheavyパスの根
    index: Vec<usize>, //列に直したときのインデックス
    cur: usize, //indexの計算用
}

impl HLD{
    //構築 O(V)
    pub fn new(graph:&Vec<Vec<usize>>, root:usize) -> Self{
        let len = graph.len();
        let mut hld = Self{
            graph:graph.clone(),
            depth:vec![0;len],
            parent:vec![0;len],
            size:vec![0;len],
            heavy:vec![usize::MAX;len],
            top:vec![0;len],
            index:vec![0;len],
            cur:0,
        };
        hld.dfs1(root,root);
        hld.dfs2(root,root);
        hld
    }
    
    //depth,parent,size,heavyを作る
    fn dfs1(&mut self, v:usize, p:usize){
        self.parent[v] = p;
        if v!=p {self.depth[v] = self.depth[p]+1;}
        let mut maxsize = 0;
        let mut size = 1;
        let children = self.graph[v].clone();
        for &nv in children.iter(){
            if nv == self.parent[v] {continue;}
            self.dfs1(nv,v);
            size += self.size[nv];
            if self.size[nv] > maxsize{
                self.heavy[v] = nv;
                maxsize = self.size[nv];
            }
        }
        self.size[v] = size
    }
    
    //top,indexを作る(heavy辺を優先するDFS)
    fn dfs2(&mut self, v:usize, t:usize){
        self.index[v] = self.cur;
        self.cur += 1;
        self.top[v] = t;
        if self.heavy[v] != usize::MAX{
            self.dfs2(self.heavy[v],self.top[v]);
        }
        let children = self.graph[v].clone();
        for &nv in children.iter(){
            if nv == self.parent[v] || nv == self.heavy[v] {continue;}
            self.dfs2(nv,nv);
        }
    }
    
    //LCA O(VlogV)
    pub fn lca(&self, mut u:usize, mut v:usize) -> usize{
        loop{
            if self.top[u] == self.top[v]{
                return if self.depth[u] < self.depth[v] {u} else {v};
            }
            if self.depth[self.top[u]] < self.depth[self.top[v]] {
                (u,v) = (v,u);
            }
            u = self.parent[self.top[u]];
        }
    }
    
    //indexでのu->vのパス上の半開区間[l,r)の列を返す
    pub fn path_ranges(&self, mut u:usize, mut v:usize) -> Vec<(usize,usize)>{
        let mut ranges = Vec::new();
        loop{
            if self.top[u] == self.top[v]{
                let l = self.index[u].min(self.index[v]);
                let r = self.index[u].max(self.index[v])+1;
                ranges.push((l,r));
                return ranges;
            }
            if self.depth[self.top[u]] < self.depth[self.top[v]]{
                (u,v) = (v,u);
            }
            let l = self.index[self.top[u]];
            let r = self.index[u]+1;
            ranges.push((l,r));
            u = self.parent[self.top[u]];
       }
    }
    
    //indexでのvの部分木の半開区間[l,r) を返す
    pub fn subtree_range(&self, v:usize) -> (usize,usize){
        (self.index[v], self.index[v]+self.size[v])
    }
}
