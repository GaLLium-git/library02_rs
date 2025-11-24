/* from
https://github.com/GaLLium-git/library02_rs/edit/main/template.rs
*/
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use library::*;
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


pub mod library {

//Scanner
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        Scanner {buffer: std::collections::VecDeque::new()}
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if buffer.len() == 0 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().map(|s| s.to_string()).collect();
        }
        self.buffer.pop_front().parse::<T>().unwrap();
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

 //bsearch
pub fn bsearch_irange<F>(mut l: usize, mut r: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
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

pub fn bsearch_frange<F>(mut l: f64, mut r: f64, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> bool,
{
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
