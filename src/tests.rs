use super::*;

use self::prelude::*;

#[test]
fn build_stackvec ()
{
	let array = StackVec::<[_; 10]>::from_iter(
		[3, 4, 6, 8].iter().cloned()
	);
    println!("{:?}", array);
}

#[test]
fn build_array ()
{
	let array: [_; 4] =
		[3, 4, 6, 8]
		.iter().cloned()
		.try_collect()
		.expect("Missing elements to collect");
    println!("{:?}", array);
}

#[test]
#[should_panic]
fn build_array_but_incomplete ()
{
	let _array: [_; 10] =
		[3, 4, 6, 8]
		.iter().cloned()
		.try_collect()
		.expect("Missing elements to collect");
}

#[test]
fn array_into_iter ()
{
	assert_eq!(
		[3, 4, 6, 8].into_iter().collect::<Vec<_>>(),
		vec![3, 4, 6, 8],
	);
}

#[test]
fn array_into_iter_dropped ()
{
	let _ = [3, 4, 6, 8].into_iter();
}

#[test]
fn array_map ()
{
	let array: [_; 4] =
		[3, 4, 6, 8]
		.into_iter()
		.map(|x| 2 * x)
		.try_collect()
		.expect("Missing elements to collect");
	let mut array_map_in_place = [3, 4, 6, 8];
	array_map_in_place.iter_mut().for_each(|x| *x *= 2);
	assert_eq!(
		array,
		array_map_in_place,
	);
}

#[test]
#[should_panic]
fn try_push_to_full_stackvec ()
{
	let mut stackvec: StackVec<[_; 1]> = Default::default();
	stackvec.try_push(0).unwrap();
	stackvec.try_push(0).unwrap();
}
