use core::clone::Clone;
use core::iter::IntoIterator;
use itertools;
use itertools::Itertools as _;

pub fn permutations<I>(
	iter: &I,
	len: usize,
) -> itertools::MultiProduct<<I as IntoIterator>::IntoIter>
where
	I: IntoIterator + Clone,
	<I as IntoIterator>::Item: Clone,
	<I as IntoIterator>::IntoIter: Clone,
{
	(0..len)
		.map(|_| iter.clone().into_iter())
		.multi_cartesian_product()
}
