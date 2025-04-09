use std::process::Output;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[cfg(feature = "creusot")]
use creusot_contracts;

#[proc_macro_attribute]
pub fn requires(pred: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::requires(#pred)]
			#input
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

#[proc_macro_attribute]
pub fn ensures(pred: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::ensures(#pred)]
			#input
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

#[proc_macro_attribute]
pub fn invariant(pred: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::invariant(#pred)]
			#input
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

#[proc_macro_attribute]
pub fn trusted(_: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::trusted]
			#item
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

#[proc_macro_attribute]
pub fn pure(_: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::pure]
			#item
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

#[proc_macro_attribute]
pub fn predicate(_: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::predicate]
			#item
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

#[cfg(feature = "creusot")]
macro_rules! snapshot {
	($thing:expr) => {
		snapshot!($thing)
	};
}

#[cfg(not(feature = "creusot"))]
macro_rules! snapshot {
	($thing:expr) => {
		// No-op
	};
}

#[cfg(feature = "creusot")]
macro_rules! pearlite {
	($thing:expr) => {
		pearlite!($thing)
	};
}

#[cfg(not(feature = "creusot"))]
macro_rules! pearlite {
	($thing:expr) => {
		// No-op
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	// ======== Tests stolen from Creusot's test suite ======== 

	#[ensures(forall<i : Int> 0 <= i && i < (^v)@.len() ==> (^v)[i] == 0u32)]
	#[ensures(v@.len() == (^v)@.len())]
	pub fn all_zero(v: &mut Vec<u32>) {
		let old_v = snapshot! { v };
		#[invariant(v@.len() == old_v@.len())]
		#[invariant(forall<j : Int> 0 <= j && j < produced.len() ==> v[j] == 0u32)]
		for i in 0..v.len() {
			v[i] = 0;
		}
	}

	#[predicate]
	fn sorted_range<T: OrdLogic>(s: Seq<T>, l: Int, u: Int) -> bool {
		pearlite! {
			forall<i : Int, j : Int> l <= i && i < j && j < u ==> s[i] <= s[j]
		}
	}

	#[predicate]
	fn sorted<T: OrdLogic>(s: Seq<T>) -> bool {
		sorted_range(s, 0, s.len())
	}

	#[ensures(sorted((^v).deep_model()))]
	#[ensures((^v)@.permutation_of(v@))]
	pub fn gnome_sort<T: Ord + DeepModel>(v: &mut Vec<T>)
	where
		T::DeepModelTy: OrdLogic,
	{
		let old_v = snapshot! { v };
		let mut i = 0;
		#[invariant(sorted_range(v.deep_model(), 0, i@))]
		#[invariant(v@.permutation_of(old_v@))]
		while i < v.len() {
			if i == 0 || v[i - 1] <= v[i] {
				i += 1;
			} else {
				v.swap(i - 1, i);
				i -= 1;
			}
		}
	}

	#[trusted]
	#[requires(l@ <= u@)]
	#[ensures(l@ <= result@ && result@  < u@)]
	fn rand_in_range(l: usize, u: usize) -> usize {
		panic!()
	}

	#[ensures((^v)@.permutation_of(v@))]
	pub fn knuth_shuffle<T>(v: &mut Vec<T>) {
		let old_v = snapshot! { v };

		#[invariant(v@.permutation_of(old_v@))]
		for n in 0..v.len() {
			// We assign the length to a variable to work around a limitation with two-phase borrows
			// where we forget the value stored in the reference.
			let upper = v.len() - n;
			let i = rand_in_range(0, upper);
			v.swap(i, upper - 1);
		}
	}

	#[predicate]
	fn sorted_range(s: Seq<u32>, l: Int, u: Int) -> bool {
		pearlite! {
			forall<i : Int, j : Int> l <= i && i < j && j < u ==> s[i] <= s[j]
		}
	}

	#[predicate]
	fn sorted(s: Seq<u32>) -> bool {
		sorted_range(s, 0, s.len())
	}

	#[requires(arr@.len() <= usize::MAX@)]
	#[requires(sorted(arr@))]
	#[ensures(forall<x:usize> result == Ok(x) ==> arr[x@] == elem)]
	#[ensures(forall<x:usize> result == Err(x) ==>
		forall<i:usize>  i < x ==> arr[i@] <= elem)]
	#[ensures(forall<x:usize> result == Err(x) ==>
		forall<i:usize> x < i && i@ < arr@.len() ==> elem < arr[i@])]
	pub fn binary_search(arr: &Vec<u32>, elem: u32) -> Result<usize, usize> {
		if arr.len() == 0 {
			return Err(0);
		}
		let mut size = arr.len();
		let mut base = 0;

		#[invariant(0 < size@ && size@ + base@ <= arr@.len())]
		#[invariant(forall<i : usize> i < base ==> arr[i@] <= elem)]
		#[invariant(forall<i : usize> base@ + size@ < i@ && i@ < arr@.len() ==> elem < arr[i@])]
		while size > 1 {
			let half = size / 2;
			let mid = base + half;

			base = if arr[mid] > elem { base } else { mid };
			size -= half;
		}

		let cmp = arr[base];
		if cmp == elem {
			Ok(base)
		} else if cmp < elem {
			Err(base + 1)
		} else {
			Err(base)
		}
	}
	// #[test]
	// fn it_works() {
	// 	let result = add(2, 2);
	// 	assert_eq!(result, 4);
	// }
}
