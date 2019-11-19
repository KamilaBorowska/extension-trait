extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, FnArg, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, ItemImpl, Pat,
    PatIdent, PatType, Signature, Visibility,
};

/// Declares an extension trait
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate extension_trait;
///
/// #[extension_trait(pub)]
/// impl DoubleExt for str {
///    fn double(&self) -> String {
///        self.repeat(2)
///    }
/// }
///
/// fn main() {
///     assert_eq!("Hello".double(), "HelloHello");
/// }
/// ```
#[proc_macro_attribute]
pub fn extension_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let visibility = parse_macro_input!(args as Visibility);
    let input_cloned = input.clone();
    let ItemImpl {
        attrs,
        unsafety,
        trait_,
        items,
        ..
    } = parse_macro_input!(input_cloned as ItemImpl);
    let items = items.into_iter().map(|item| match item {
        ImplItem::Const(ImplItemConst {
            attrs, ident, ty, ..
        }) => quote! { #(#attrs)* const #ident: #ty; },
        ImplItem::Method(ImplItemMethod { attrs, sig: Signature {
            constness, asyncness, unsafety, abi, ident, generics, inputs, variadic, output, ..
        }, .. }) => {
            let inputs = inputs.into_iter().map(|arg| {
                if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = &arg {
                    match **pat {
                        Pat::Ident(PatIdent {
                            by_ref: None,
                            mutability: None,
                            subpat: None,
                            ..
                        }) => {},
                        _ => return quote! { #(#attrs)* _: #ty },
                    }
                }
                quote! { #arg }
            });
            let where_clause = &generics.where_clause;
            quote! {
                #(#attrs)*
                #constness #asyncness #unsafety #abi fn #ident #generics (#(#inputs,)* #variadic) #output #where_clause;
            }
        },
        ImplItem::Type(ImplItemType {
            attrs,
            ident,
            generics,
            ..
        }) => quote! { #(#attrs)* type #ident #generics; },
        _ => panic!("Unsupported item type in impl"),
    });
    if let Some((None, path, _)) = trait_ {
        let input = proc_macro2::TokenStream::from(input);
        (quote! {
            #(#attrs)*
            #visibility #unsafety trait #path {
                #(#items)*
            }
            #input
        }
        .into())
    } else {
        panic!("Extension trait name was not provided");
    }
}
