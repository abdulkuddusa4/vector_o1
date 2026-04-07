#![allow(warnings)]

use core::time;
use std::{cmp, fmt::{self, Display}, ops::{self, Index, IndexMut}, time::Instant};

pub fn print_type_of<T>(obj: &T){
	println!("{}", std::any::type_name::<T>());
}
pub struct VecO1<T, I: IndexType = usize>{
	id: Vec<I>,
	index: Vec<I>,
	data: Vec<T>
}
trait IndexType: Copy+ops::Add+ops::Sub+cmp::Eq+cmp::Ord+cmp::Ord+cmp::PartialOrd{
	fn get_max()->usize;
	fn as_usize(self)->usize;
	fn from_usize(value: usize)->Self;
}

impl IndexType for u8{
	fn get_max()->usize {
		u8::MAX as usize
	}

	fn as_usize(self)->usize {
	    self as usize
	}

	fn from_usize(value: usize)->Self {
	    value as Self
	}
}

impl IndexType for u16{
	fn get_max()->usize {
	    u16::MAX as usize
	}

	fn as_usize(self)->usize{
		self as usize
	}

	fn from_usize(value: usize)->Self {
	    value as Self
	}
}

impl IndexType for u32{
	fn get_max()->usize {
	    u32::MAX as usize
	}

	fn as_usize(self)->usize{
		self as usize
	}

	fn from_usize(value: usize)->Self {
	    value as Self
	}
}

impl IndexType for usize{
	fn get_max()->usize {
	    usize::MAX
	}

	fn as_usize(self)->usize{
		self as usize
	}

	fn from_usize(value: usize)->Self {
	    value as Self
	}
}

impl<T, I:IndexType> VecO1<T, I>{
	pub fn new()->VecO1<T,I>{
		VecO1 { id: Vec::new(), index: Vec::new(), data: Vec::new() }
	}

	pub unsafe fn get_unchecked(&self, id: usize)->T{
		self.get_unchecked(self.index[id].as_usize())
	}

	pub fn push(&mut self, value: T)->Result<(), String>{
		if self.data.len() == I::get_max()+1{
			return Err(format!("maximum capacity reached for index type"));
		}

		if self.data.len() < self.id.len(){
			let new_idx = self.data.iter().len();
			self.data.push(value);
			self.index[self.id[new_idx].as_usize()] = I::from_usize(new_idx);
		}
		else{
			self.index.push(I::from_usize(self.data.len()));
			self.id.push(I::from_usize(self.data.len()));
			self.data.push(value);
		}
		Ok(())
	}

	pub fn remove(&mut self, idx: usize)where T: fmt::Debug{
		if self.index[idx].as_usize() >= self.data.len() || self.data.len() == 0{
			return;
		}


		let _idx = self.index[idx].as_usize();
		let _idx_swp = self.data.len()-1;

		let _id = self.id[_idx].as_usize();
		let _id_swp = self.id[_idx_swp].as_usize();

		// swap data

		self.data.swap_remove(_idx);
		
		let temp = self.id[_idx];
		self.id[_idx] = self.id[_idx_swp];
		self.id[_idx_swp] = temp;

		let temp = self.index[_id];
		self.index[_id] = self.index[_id_swp];
		self.index[_id_swp] = temp;
	}

	pub fn len(&self)->usize{
		self.data.len()
	}

	pub fn print(&self) where T:Display, I:fmt::Debug{
		println!("{:?}", &self.index);
		print!("[");
		for idx in &self.index{
			if self.data.len() == 0{
				break;
			}
			if idx.as_usize()>=self.data.len(){
				continue;
			}
			print!("{}, ", self.data[idx.as_usize()]);
		}
		println!("]");
	}
}


impl<T, I: IndexType> Index<usize> for VecO1<T, I>{
    type Output=T;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx>= self.data.len(){
        	panic!("index out of bound");
        }

        &self.data[self.index[idx].as_usize()]
    }
}

impl<T, I: IndexType> IndexMut<usize> for VecO1<T, I>{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx>= self.data.len(){
        	panic!("index out of bound");
        }

        &mut self.data[self.index[idx].as_usize()]
    }
}
