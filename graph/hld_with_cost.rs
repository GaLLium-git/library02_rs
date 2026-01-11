//モノイドつきHLぶんかい
//頂点にコストがあるとする
//辺にコストがある場合は，cost[v] = (parent[v] と v　の辺のコスト）　として，頂点版からLCAを除外すればいける?
use ac_library::{Monoid,Segtree};
pub struct HLD<M:Monoid>{
    graph: Vec<Vec<usize>>,
    depth: Vec<usize>, //深さ
    parent: Vec<usize>, //親
    size: Vec<usize>,  //部分木のサイズ
    heavy: Vec<usize>, //heavyな子
    top: Vec<usize>,  //属するheavyパスの根
    index: Vec<usize>, //列に直したときのインデックス
    cur: usize, //オイラーツアー用
    seg: Segtree<M>,
}

impl<M:Monoid> HLD<M> {
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
            seg: Segtree::<M>::new(len),
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
                v = self.parent[self.top[v]];
            } else{
                u = self.parent[self.top[u]];
            }
        }
    }
    
    //セグ木の構築
    pub fn build_seg(&mut self, cost: &Vec<M::S>){
        for v in 0..cost.len(){
            self.seg.set(self.index[v],cost[v].clone());
        }
    }
    
    
    pub fn path(&self, mut u:usize, mut v:usize) -> M::S{
        let (mut prodl, mut prodr) = (M::identity(),M::identity());
        M::binary_operation(&prodl, &prodr)
    }
}

fn main(){
  eprint!("No bug");
}
