extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, FnArg, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType,
    ItemImpl, Pat, PatBox, PatIdent, PatReference, PatTuple, PatType, Signature, Visibility,
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
        impl_token,
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
                let span = arg.span();
                match arg {
                    FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                        let ident = extract_ident(*pat).unwrap_or_else(|| Ident::new("_", span));
                        quote! { #(#attrs)* #ident: #ty }
                    },
                    FnArg::Receiver(_) => quote! { #arg }
                }
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
        _ => return syn::Error::new(item.span(), "unsupported item type").to_compile_error().into(),
    });
    if let Some((None, path, _)) = trait_ {
        let input = proc_macro2::TokenStream::from(input);
        (quote! {
            #(#attrs)*
            #visibility #unsafety trait #path {
                #(#items)*
            }
            #input
        })
        .into()
    } else {
        syn::Error::new(impl_token.span(), "extension trait name was not provided")
            .to_compile_error()
            .into()
    }
}

fn extract_ident(pat: Pat) -> Option<Ident> {
    match pat {
        Pat::Box(PatBox { pat, .. }) | Pat::Reference(PatReference { pat, .. }) => {
            extract_ident(*pat)
        }
        Pat::Ident(PatIdent { ident, .. }) => Some(ident),
        Pat::Tuple(PatTuple { elems, .. }) => {
            if elems.len() <= 1 {
                extract_ident(elems.into_iter().next()?)
            } else {
                let span = elems.span();
                let elems = elems
                    .into_iter()
                    .map(extract_ident)
                    .map(|o| o.map(|ident| ident.unraw().to_string()))
                    .collect::<Option<Vec<String>>>()?;
                let joined = elems.join("_");
                Some(Ident::new(&joined, span))
            }
        }
        _ => None,
    }
}
