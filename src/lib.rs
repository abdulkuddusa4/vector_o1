#![allow(warnings)]

use std::ops::{self, Index, IndexMut};


pub struct VecO1<T, I: IndexType = usize>{
	id: Vec<I>,
	index: Vec<I>,
	data: Vec<T>
}
trait IndexType: Copy+ops::Add+ops::Sub<Output=Self>{}

impl IndexType for u8{}
impl IndexType for u16{}
impl IndexType for u32{}
impl IndexType for usize{}

impl<T, I:IndexType> VecO1<T, I>{
	pub fn new()->VecO1<T,I>{
		todo!()
	}

	pub unsafe fn get_unchecked(idx: I){
		todo!()
	}

	pub fn add(value: T){
		todo!()
	}

	pub fn remove(idx: I){
		todo!()
	}
}


impl<T, I: IndexType> Index<I> for VecO1<T, I>{
    type Output=T;

    fn index(&self, index: I) -> &Self::Output {
        todo!()
    }
}

impl<T, I: IndexType> IndexMut<I> for VecO1<T, I>{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        todo!()
    }
}