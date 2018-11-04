#![allow(dead_code)]

extern crate stackvec;
use stackvec::prelude::*;

use ::std::iter::{
	self,
	Iterator,
};

static NUMBERS: [u8; 9] = [
	5, 8, 9, 10, 6, 7, 4, 6, 7
];

mod counted_instances {
	use ::std::cell::Cell;

	thread_local! {
		static INSTANCES_COUNT: Cell<isize> = Cell::new(0);
	}

	#[derive(Debug)]
	pub struct Instance(());

	impl Default for Instance {
		fn default() -> Self
		{
			INSTANCES_COUNT.with(|slf| slf.set(slf.get() + 1));
			Instance(())
		}
	}


	impl Instance {
		#[allow(non_upper_case_globals)]
		pub const new
			: fn() -> Self
			= Self::default;

		pub fn clone (&self) -> Self { Self::new() }

		pub fn total_count () -> isize
		{
			INSTANCES_COUNT.with(Cell::get)
		}

		pub fn count_assert_balanced ()
		{
			use ::std::cmp::Ordering::*;
			let instance_count = Self::total_count();
			match instance_count.cmp(&0) {
				Less => panic!(
					concat!(
						"Error, instance count {} < 0 ",
						r"=> /!\ double free /!\",
					),
					instance_count,
				),
				Greater => panic!(
					concat!(
						"Error, instance count {} > 0 ",
						r"=> /!\ memory leak /!\",
					),
					instance_count,
				),
				Equal =>(),
			} 
		}
	}

	impl Drop for Instance {
		fn drop (&mut self)
		{
			INSTANCES_COUNT.with(|slf| slf.set(slf.get() - 1))
		}
	}
}

#[test]
fn build ()
{
	let array = StackVec::<[u8; 10]>::from_iter( 
		NUMBERS.iter().cloned()
	);
    println!("{:?}", array);
}

#[test]
fn build_with_drop_full ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[Instance; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	}
	Instance::count_assert_balanced();
}

#[test]
fn build_with_drop_partial ()
{
	use counted_instances::*;
	{
		let mut array = StackVec::<[Instance; 3]>::default();
		array.try_push(
			Instance::new()
		).unwrap();
	    println!("{:?}", array);
	}
	Instance::count_assert_balanced();
}

#[test]
fn extend ()
{
	let mut array = StackVec::<[u8; 0x40]>::
		default();
    array.extend(Iterator::chain(
        (0 .. 56).map(|_| 0),
        b"Stackvec".iter().cloned(),
    ));
    println!("{:?}", array);
}


#[test]
fn iter ()
{
	let array = StackVec::<[u8; 10]>::from_iter(
		NUMBERS.iter().cloned()
	);
	for (value, expected_value) in Iterator::zip(array.iter(), &NUMBERS)
	{
	    assert_eq!(value, expected_value);
	};
}

#[test]
fn iter_mut ()
{
	let mut array = StackVec::from([0_u8; 10]);
	for (array_i, &value) in Iterator::zip(array.iter_mut(), &NUMBERS)
	{
		*array_i = value;
	};
	for (value, expected_value) in Iterator::zip(array.iter(), &NUMBERS)
	{
	    assert_eq!(value, expected_value);
	};
}

#[test]
fn into_iter ()
{
	let array = StackVec::<[u8; 10]>::from_iter(
		NUMBERS.iter().cloned()
	);
	assert_eq!(
		Vec::from_iter(array),
		Vec::from_iter(NUMBERS.iter().cloned()),
	);
}

#[test]
fn array_into_iter ()
{
	assert_eq!(
		Vec::from_iter(NUMBERS.into_iter()),
		Vec::from_iter(NUMBERS.iter().cloned()),
	);
}

#[test]
fn into_iter_with_drop_full ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[_; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	    for _ in array {} 
	}
	Instance::count_assert_balanced();
}

#[test]
fn into_iter_with_drop_partial_left ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[_; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	    let mut iterator = array.into_iter();
	    let _ = iterator.next();
	}
	Instance::count_assert_balanced();
}



#[test]
fn into_iter_with_drop_partial_right ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[_; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	    let mut iterator = array.into_iter();
	    let _ = iterator.next_back();
	}
	Instance::count_assert_balanced();
}

