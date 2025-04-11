
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[cfg(feature = "creusot")]
use creusot_contracts;

#[cfg(feature = "prusti")]
use prusti_contracts;

// Attributes which take a predicate

#[proc_macro_attribute]
pub fn requires(pred: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let pred2 = TS::from(pred);
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::requires(#pred2)]
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
		let pred2 = TS::from(pred);
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::ensures(#pred2)]
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
		let pred2 = TS::from(pred);
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::invariant(#pred2)]
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
pub fn variant(pred: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let pred2 = TS::from(pred);
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::variant(#pred2)]
			#input
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

// Attributes which do not take a predicate

#[proc_macro_attribute]
pub fn terminates(_: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::terminates]
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
pub fn pure(_: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::pure]
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
pub fn predicate(_: TokenStream, item: TokenStream) -> TokenStream {
	#[cfg(feature = "creusot")]
	{
		let input = parse_macro_input!(item as ItemFn);
		let output = quote! {
			#[creusot_contracts::predicate]
			#input
		};
		output.into()
	}
	#[cfg(not(feature = "creusot"))]
	{
		item // Do not modify anything
	}
}

// Macros

#[cfg(feature = "creusot")]
#[macro_export]
macro_rules! snapshot {
	($thing:expr) => {
		prusti_contracts::snapshot!($thing)
	};
}

#[cfg(not(feature = "creusot"))]
#[macro_export]
macro_rules! snapshot {
	($thing:expr) => {
		// No-op
	};
}

#[cfg(feature = "creusot")]
#[macro_export]
macro_rules! pearlite {
	($thing:expr) => {
		prusti_contracts::pearlite!($thing)
	};
}

#[cfg(not(feature = "creusot"))]
#[macro_export]
macro_rules! pearlite {
	($thing:expr) => {
		// No-op
	};
}

#[cfg(feature = "creusot")]
#[macro_export]
macro_rules! pure {
    ($thing:expr) => {
		prusti_contracts::ghost!($thing)
    };
}

#[cfg(not(feature = "creusot"))]
#[macro_export]
macro_rules! pure {
	($thing:expr) => {
		// No-op
	};
}

#[cfg(feature = "creusot")]
#[macro_export]
macro_rules! proof_assert {
    ($thing:expr) => {
		prusti_contracts::ghost!($thing)
    };
}

#[cfg(not(feature = "creusot"))]
#[macro_export]
macro_rules! proof_assert {
	($thing:expr) => {
		// No-op
	};
}
