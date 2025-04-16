
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, Expr};

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

#[proc_macro]
pub fn snapshot(input: TokenStream) -> TokenStream {
    let thing = parse_macro_input!(input as Expr);

    // Check if the "creusot" feature is enabled
    let expanded = if cfg!(feature = "creusot") {
        quote! {
            creusot_contracts::snapshot!(#thing)
        }
    } else {
        quote! {
            // No-op
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn pearlite(input: TokenStream) -> TokenStream {
    let thing = parse_macro_input!(input as Expr);

    // Check if the "creusot" feature is enabled
    let expanded = if cfg!(feature = "creusot") {
        quote! {
            creusot_contracts::pearlite!(#thing)
        }
    } else {
        quote! {
            // No-op
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn proof_assert(input: TokenStream) -> TokenStream {
    let thing = parse_macro_input!(input as Expr);

    // Check if the "creusot" feature is enabled
    let expanded = if cfg!(feature = "creusot") {
        quote! {
            creusot_contracts::proof_assert!(#thing)
        }
    } else {
        quote! {
            // No-op
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn ghost(input: TokenStream) -> TokenStream {
    let thing = parse_macro_input!(input as Expr);

    // Check if the "creusot" feature is enabled
    let expanded = if cfg!(feature = "creusot") {
        quote! {
            creusot_contracts::ghost!(#thing)
        }
    } else {
        quote! {
            // No-op
        }
    };

    TokenStream::from(expanded)
}
