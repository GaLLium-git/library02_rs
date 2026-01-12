/* from
https://github.com/GaLLium-git/library02_rs/edit/main/template.rs
*/
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use template::*;
use itertools::*;
use ordered_float::*;
use std::collections::*;

fn main() {
    let mut sc=Scanner::new();
    let a:usize=sc.next();
    let b:usize=sc.next();
    let ans= if a*b%2==0{"Even"} else {"Odd"};
    println!("{}",ans);
}


pub mod template {

//Scanner
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        Scanner {buffer: std::collections::VecDeque::new()}
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.buffer.len() == 0 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().map(|s| s.to_string()).collect();
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
}

//Shift vec
pub trait Shift<T>
    where
        T: Default + Copy,
    {
        fn shift(&mut self);
    }
impl<T> Shift<T> for Vec<T>
    where
        T: Default + Copy,
    {
        fn shift(&mut self) {
            self.insert(0, T::default());
        }
    }
    
   
pub trait Shift2D<T>
    where
        T: Default + Copy,
    {   
        fn shift(&mut self);
        fn shift_2d(&mut self);
    }
impl<T> Shift2D<T> for Vec<Vec<T>>
    where
        T: Default + Copy,
    {
         fn shift(&mut self) {
            self.insert(0, vec![T::default();self[0].len()]);
        }
        fn shift_2d(&mut self) {
            for i in 0..self.len() {
                self[i].shift();
            }
            self.shift();
        }
    }


//range型を[l,r)に直す関数
pub fn get_bounds_usize(range: impl std::ops::RangeBounds<usize>) -> (usize,usize){
    let l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => *l+1,
        std::ops::Bound::Unbounded => 0,
    };
    let r = match range.end_bound() {
        std::ops::Bound::Included(r) => *r+1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => usize::MAX,
    };
    (l,r)
}



pub fn get_bounds_f64(range: impl std::ops::RangeBounds<f64>) -> (f64,f64){
    const EPS:f64 = 1e-9;
    let l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => *l+EPS,
        std::ops::Bound::Unbounded => 0.0,
    };
    let r = match range.end_bound() {
        std::ops::Bound::Included(r) => *r+1e-9,
        std::ops::Bound::Excluded(r) => *r+EPS,
        std::ops::Bound::Unbounded => f64::MAX,
    };
    (l,r)
}


//二分探索
//range で fがtrueとなる最小を返す
pub fn bsearch_usize<F>(range: impl std::ops::RangeBounds<usize>, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let (mut l, mut r) = get_bounds_usize(range);
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}


//浮動小数点バージョン
pub fn bsearch_f64<F>(range: impl std::ops::RangeBounds<f64>, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> bool,
{
    let (mut l, mut r) = get_bounds_f64(range);
    while r - l > eps {
        let m = (l + r) / 2.0;
        if f(m) {
            r = m;
        } else {
            l = m;
        }
    }
    l
}
    
 //cumulate,cumlate_rev
pub trait Cumulate<T> 
    where
      T:Clone,
    {
    fn cumulate<F>(&self, f: F) -> Vec<T>
    where
        F: Fn(&T,&T) -> T;
     fn cumulate_rev<F>(&self, f: F) -> Vec<T>
     where
        F: Fn(&T,&T) -> T;
    }
impl<T> Cumulate<T> for Vec<T>
    where
      T:Clone,
    {
     fn cumulate<F>(&self, f: F) ->Vec<T>
     where
        F: Fn(&T,&T) ->T, 
      {
        let mut cumvec= self.clone();
        for i in 1..self.len() {
          cumvec[i]=f(&cumvec[i-1],&self[i]);
        }
        cumvec
      }  
     fn cumulate_rev<F>(&self, f: F) ->Vec<T>
     where
        F: Fn(&T,&T) ->T, 
      {
        let mut cumvec = self.clone();
        for i in (0..self.len()-1).rev() {
          cumvec[i]=f(&cumvec[i+1],&self[i]);
        }
        cumvec
      }
    }
    
  //cumlate_2d
pub trait Cumulate2D<T> 
    where
      T:Clone,
    {
    fn cumulate_2d<F>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(&T,&T) -> T;
    }
impl<T> Cumulate2D<T> for Vec<Vec<T>>
    where
      T:Clone,
    {
     fn cumulate_2d<F>(&self, f: F) ->Vec<Vec<T>>
     where
        F: Fn(&T,&T) ->T, 
      {
        let mut cumvec = self.clone();
        for i in 0..self.len() {
          for j in 1..self[i].len(){
            cumvec[i][j]=f(&cumvec[i][j],&cumvec[i][j-1]);
          }
        }
        for i in 1..self.len() {
          for j in 0..self[i].len(){
            cumvec[i][j]=f(&cumvec[i][j],&cumvec[i-1][j]);
          }
        }
        cumvec
      }  
    }
    
}
