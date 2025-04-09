extern crate proc_macro;


#[cfg(feature = "creusot")]
use creusot_contracts;

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn requires(elem1: TS1, tokens: TS1) -> TS1 {
	creusot_contracts::requires(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn ensures(elem1: TS1, tokens: TS1) -> TS1 {
	creusot_contracts::ensures(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn invariant(elem1: TS1, tokens: TS1) -> TS1 {
	creusot_contracts::invariant(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn variant(elem1: TS1, tokens: TS1) -> TS1 {
	creusot_contracts::variant(elem1, tokens)
}


#[cfg(feature = "creusot")]
#[proc_macro]
pub fn proof_assert(elem1: TS1) -> TS1 {
	creusot_contracts::proof_assert(elem1)
}

#[cfg(feature = "creusot")]
#[proc_macro]
pub fn snapshot(elem: TS1) -> TS1 {
    creusot_contracts::snapshot(elem)
}

#[cfg(feature = "creusot")]
#[proc_macro]
pub fn ghost(body: TS1) -> TS1 {
	creusot_contracts::ghost(body) 
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn terminates(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts::terminates(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn pure(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts::pure(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn logic(elem1: TS1, elem2: TS1) -> TS1 {
    creusot_contracts::logic(elem1, elem2)
}

#[cfg(feature = "creusot")]
#[proc_macro]
pub fn pearlite(elem1: TS1) -> TS1 {
    creusot_contracts::pearlite(elem1)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn predicate(elem1: TS1, elem2: TS1) -> TS1 {
    creusot_contracts::predicate(elem1, elem2)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn law(elem1: TS1, elem2: TS1) -> TS1 {
    creusot_contracts::law(elem1, elem2)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn trusted(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts::trusted(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro]
pub fn extern_spec(elem1: TS1) -> TS1 {
    creusot_contracts::extern_spec(elem1)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn maintains(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts::maintains(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn open(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts::open(elem1, token)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn open_inv_result(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts(elem1, tokens)
}

#[cfg(feature = "creusot")]
#[proc_macro_attribute]
pub fn bitwise_proof(elem1: TS1, tokens: TS1) -> TS1 {
    creusot_contracts::bitwise_proof(elem1, tokens)
}


// Define a macro to conditionally compile the attributes
// #[cfg(feature = "creusot")]
// macro_rules! invariant {
//	 ($predicate:expr) => {
//		 #[invariant($predicate)]
//	 };
// }
//
// #[cfg(not(feature = "creusot"))]
// macro_rules! invariant {
//	 ($predicate:expr) => {
//		 // No-op for the invariant
//	 };
// }
//
// #[cfg(feature = "creusot")]
// macro_rules! requires {
//	 ($predicate:expr) => {
//		 #[requires($predicate)]
//	 };
// }
//
// #[cfg(not(feature = "creusot"))]
// macro_rules! requires {
//	 ($predicate:expr) => {
//		 // No-op for the requires
//	 };
// }
//
// #[cfg(feature = "creusot")]
// macro_rules! ensures {
//	 ($predicate:expr) => {
//		 #[ensures($predicate)]
//	 };
// }
//
// #[cfg(not(feature = "creusot"))]
// macro_rules! ensures {
//	 ($predicate:expr) => {
//		 // No-op for the ensures
//	 };
// }
//
// #[cfg(feature = "creusot")]
// macro_rules! trusted {
//	 () => {
// 		#[trusted]
// 	}
// }
//
// #[cfg(not(feature = "creusot"))]
// macro_rules! trusted {
//	 () => {
// 		// No op when creusot is not enabled
// 	}
// }

#[cfg(feature = "creusot")]
macro_rules! snapshot {
	($thing:expr) => {
		snapshot!($thing)
	};
}

#[cft(not(feature = "creusot"))]
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

#[cft(not(feature = "creusot"))]
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
