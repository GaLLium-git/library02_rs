

//HLぶんかい
pub struct HLD{
    graph: Vec<Vec<usize>>,
    top: Vec<usize>,
    depth: Vec<usize>,
    parent: Vec<usize>,
    index: Vec<usize>,
    heavy: Vec<usize>,
}

impl HLD{
    pub fn new(graph:Vec<Vec<usize>>, root:usize) -> Self{
        let len = graph.len();
        let mut hld = Self{
           graph,
           top:vec![0;len],
           depth:vec![0;len],
           parent:vec![0;len],
           index:vec![0;len],
           heavy:vec![usize::MAX;len],
        };
        hld.dfs1(root,root);
        hld.dfs2(root,root);
        hld
    }
    
    //depth,parent,heavyを作る
    fn dfs1(&mut self, v:usize, p:usize) -> usize{
        self.parent[v] = p;
        self.depth[v] = self.depth[p]+1;
        let mut maxsize = 0;
        let mut size = 0;
        let children = self.graph[v].clone();
        for &nv in children.iter(){
            if nv == self.parent[v] {continue;}
            let subsize = self.dfs1(nv,v);
            size += subsize;
            if subsize > maxsize{
                self.heavy[v] = nv;
                maxsize = subsize;
            }
        }
        size
    }
    
    //topを作る(heavy辺を優先するDFS)
    fn dfs2(&mut self, v:usize, t:usize){
        self.top[v] = t;
        if self.heavy[v] != usize::MAX{
            self.dfs2(self.heavy[v],self.top[v]);
        }
        let children = self.graph[v].clone();
        for &nv in children.iter(){
            if nv == self.parent[v] {continue;}
            self.dfs2(nv,nv);
        }
    }

    pub fn lca(&self, mut u:usize, mut v:usize) -> usize{
        loop{
            if self.top[u] == self.top[v]{
                return if self.depth[u] > self.depth[v]{u} else {v};
            }
            if self.depth[u] > self.depth[v] {
                v = self.parent[self.top[v]];
            } else{
                u = self.parent[self.top[v]];
            }
        }
    }
}
