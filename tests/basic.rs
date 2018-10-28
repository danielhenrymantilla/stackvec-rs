extern crate stackvec;
use stackvec::prelude::*;

use ::std::iter::{
	self,
	Iterator,
};

static NUMBERS: &[u8] = &[
	5, 8, 9, 10, 6, 7, 4, 6, 7
];

mod counted_instances {
	use ::std::cell;

	thread_local! {
		static INSTANCES_COUNT: cell::Cell<isize> = cell::Cell::new(0);
	}

	#[derive(Debug)]
	pub struct Instance(());

	impl Instance {
		pub fn new () -> Self
		{
			INSTANCES_COUNT.with(|slf| slf.set(slf.get() + 1));
			Instance(())
		}

		pub fn total_count () -> isize
		{
			INSTANCES_COUNT.with(cell::Cell::get)
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
fn build()
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
    assert_eq!(Instance::total_count(), 0, "Instance count is balanced");
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
    assert_eq!(Instance::total_count(), 0, "Instance count is balanced");
}

#[test]
fn extend()
{
	let mut array = StackVec::<[u8; 0x40]>::
		default();
    array.extend(Iterator::chain(
        (0 .. 56).map(|_| 0),
        b"Shrewk".iter().cloned(),
    ));
    println!("{:?}", array);
}


#[test]
fn iter()
{
	let array = StackVec::<[u8; 10]>::from_iter(
		NUMBERS.iter().cloned()
	);
	for (value, expected_value) in Iterator::zip(array.iter(), NUMBERS)
	{
	    assert_eq!(value, expected_value);
	};
}

#[test]
fn iter_mut()
{
	let mut array = StackVec::<[u8; 10]>::from_iter(
		iter::repeat(0)
	);
	for (array_i, &value) in Iterator::zip(array.iter_mut(), NUMBERS)
	{
		*array_i = value;
	};
	for (value, expected_value) in Iterator::zip(array.iter(), NUMBERS)
	{
	    assert_eq!(value, expected_value);
	};
}

#[test]
fn into_iter()
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
fn into_iter_with_drop_full ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[Instance; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	    for _ in array {} 
	}
    assert_eq!(Instance::total_count(), 0, "Instance count is balanced");
}

#[test]
fn into_iter_with_drop_partial_left ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[Instance; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	    let mut iterator = array.into_iter();
	    let _ = iterator.next();
	}
    assert_eq!(Instance::total_count(), 0, "Instance count is balanced");
}



#[test]
fn into_iter_with_drop_partial_right ()
{
	use counted_instances::*;
	{
		let array = StackVec::<[Instance; 3]>::from_iter( 
			iter::repeat_with(Instance::new)
		);
	    println!("{:?}", array);
	    let mut iterator = array.into_iter();
	    let _ = iterator.next_back();
	}
    assert_eq!(Instance::total_count(), 0, "Instance count is balanced");
}

