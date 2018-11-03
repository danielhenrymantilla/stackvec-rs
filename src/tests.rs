use super::*;

use self::prelude::*;

#[test]
fn build()
{
	let array = StackVec::<[u8; 10]>::from_iter(
		[3, 4, 6, 8].iter().cloned()
	);
    println!("{:?}", array);
}
