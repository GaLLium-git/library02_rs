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
        
    }

    fn dfs1(&mut self, v:usize, p:usize) -> usize{
        self.parent[v] = p;
        self.depth[v] = self.depth[p]+1;
        let mut maxsize = 0;
        for &nv in self.graph[v].iter(){
            if nv == self.parent[v] {continue;}
            let subsize = dfs1(self,nv,v);
            if subsize > maxsize{
                self.heavy[v] = nv;
                maxsize = subsize;
            }
        }
    }

    fn dfs2(&mut self, v:usize, t:usize){
        self.top[v] = t;
        if self.heavy[v] != usize::MAX{
            dfs2(self,self.heavy[v],self.top[v]);
        }
        for &nv in self.graph[v].iter(){
            if nv == self.parent[v] {continue;}
            dfs2(self,nv,nv);
        }
    }

    pub fn lca(&mut self, u:usize, v:usize) -> usize{
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
