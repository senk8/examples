use std::fmt::Debug;
#[derive(Debug)]
struct IntPack(i32);
trait Pack<A> {	
	fn get(&self)->A;
	fn set(&mut self,content:A)->();
}

impl Pack<i32> for IntPack {
	fn get(&self) -> i32 {
		self.0
	}
	fn set(&mut self, content: i32)->(){
		self.0 = content;
	}
}

impl Pack<i64> for IntPack {
	fn get(&self) -> i64 {
		self.0 as i64
	}
	fn set(&mut self, content: i64)->(){
		self.0 = content as i32;
	} 
}

fn debug<A:Pack<i32>+Debug>(a:A){
	dbg!(a);
}

fn debug64<A:Pack<i64>+Debug>(a:A){
	dbg!(a);
}

fn main() {
	let int_pack = IntPack(10);
    debug(int_pack);
}