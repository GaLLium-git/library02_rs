fn main(){
    let mut sc = Scanner::new();
    let (N,Q):(usize,usize) = (sc.next(),sc.next());
    type F = (i64,i64);
    let eval = |f:F,x:T| -> T{
        f.0 * x + f.1
    };
    let id = (0,i64::MAX);
    let x_min = -1_000_000_005;
    let x_max = 1_000_000_005;
    let mut Li = LiChaoTree::new(eval,id,x_min,x_max);
    
    for _ in 0..N{
        let (a,b):(i64,i64) = (sc.next(),sc.next());
        Li.add((a,b));
    }
    
    for _ in 0..Q{
        let qu:usize = sc.next();
        if qu==0{
            let (a,b):(i64,i64) = (sc.next(),sc.next());
            Li.add((a,b));
        } else{
            let x:i64 = sc.next();
            println!("{}",Li.get(x));
        }
    }
}

//Dynamic Li Chao Tree
//追加，取得ともにO(logN)

type T = i64; //xの型 
#[derive(Copy, Clone)]
struct Node<F>{
    f: F,
    lc: Option<usize>,
    rc: Option<usize>,
}

pub struct LiChaoTree<F,Eval>
where
    F: Copy,
    Eval: Fn(F, T) -> T,
{
    tree: Vec<Node<F>>,    
    eval: Eval,
    id: F,
    x_min: T,
    x_max: T,
}

impl<F,Eval> LiChaoTree<F,Eval>
where
    F: Copy,
    Eval: Fn(F, T) -> T,
{
    pub fn new(eval:Eval, id:F, x_min:T, x_max:T) -> Self{
        Self {
            tree: vec![Node{f:id,lc:None,rc:None}],
            eval,
            id,
            x_min,
            x_max,
        }
    }
    
    //[l,r)を覆うノードvでの更新
    fn _add(&mut self, mut f:F, v:usize, l:T, r:T){
        let m = l+(r-l)/2;
        let new_m = (self.eval)(f,m);
        let pre_m = (self.eval)(self.tree[v].f,m);
        
        if new_m < pre_m{
            (self.tree[v].f,f) = (f,self.tree[v].f);
        }
        
        if r - l == 1{return;}
        
        let new_l = (self.eval)(f,l);
        let new_r = (self.eval)(f,r-1);
        let pre_l = (self.eval)(self.tree[v].f,l);
        let pre_r = (self.eval)(self.tree[v].f,r-1);
        
        if new_l >= pre_l && new_r >= pre_r{return;}
        
        else if new_l < pre_l{
            if self.tree[v].lc.is_none(){
                self.tree[v].lc = Some(self.tree.len());
                self.tree.push(Node{f:self.id,lc:None,rc:None});
            }
            self._add(f, self.tree[v].lc.unwrap(), l, m);
        }
        
        else if new_r < pre_r{
            if self.tree[v].rc.is_none(){
                self.tree[v].rc = Some(self.tree.len());
                self.tree.push(Node{f:self.id,lc:None,rc:None});
            }
            self._add(f, self.tree[v].rc.unwrap(), m, r);
        }
        
    }
    pub fn add(&mut self, f:F){
        self._add(f,0,self.x_min,self.x_max);
    }
    
    //[l,r)を覆うノードでのｘの代入
    fn _get(&self, x:T, v:usize, l:T, r:T) -> T{
        let m = l+(r-l)/2;
        let mut res = (self.eval)(self.tree[v].f, x);
        //左側
        if l <= x && x < m && self.tree[v].lc.is_some(){
            res = res.min(self._get(x,self.tree[v].lc.unwrap(),l,m));
        }
        //右側
        if m <= x && x < r && self.tree[v].rc.is_some(){
            res = res.min(self._get(x,self.tree[v].rc.unwrap(),m,r));
        }
        res
    }
    
    pub fn get(&self, x:T) -> T{
       self._get(x,0,self.x_min,self.x_max)
    }
}
