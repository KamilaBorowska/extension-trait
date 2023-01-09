extern crate proc_macro;
#[macro_use]
extern crate darling;

use proc_macro::TokenStream;
use quote::quote;
use std::borrow::Cow;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, FnArg, Ident, ImplItem, ImplItemConst, ImplItemMethod,
    ImplItemType, ItemImpl, Pat, PatBox, PatIdent, PatReference, PatTuple, PatType, Signature,
    Visibility, AttributeArgs,
};
use darling::FromMeta;

struct ItemImplWithVisibility {
    attrs: Vec<Attribute>,
    visibility: Visibility,
    impl_item: ItemImpl,
}

impl Parse for ItemImplWithVisibility {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let impl_item = input.parse()?;
        Ok(Self {
            attrs,
            visibility,
            impl_item,
        })
    }
}

#[derive(Default, Debug, FromMeta)]
struct MacroArgs {
    async_trait: Option<bool>,
}

impl MacroArgs {
    fn async_trait(&self) -> bool {
        if !cfg!(feature = "async_trait") {
            if self.async_trait.is_some() {
                panic!("async_trait feature is not enabled!");
            }
            return false;
        }
        self.async_trait.unwrap_or(true)
    }
}

/// Declares an extension trait
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate extension_trait;
///
/// #[extension_trait]
/// pub impl DoubleExt for str {
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
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let attr_args = MacroArgs::from_list(&attr_args).unwrap_or_else(|_| MacroArgs::default());
    let ItemImplWithVisibility {
        attrs,
        visibility,
        impl_item,
    } = parse_macro_input!(input as ItemImplWithVisibility);
    let ItemImpl {
        impl_token,
        unsafety,
        trait_,
        items,
        ..
    } = &impl_item;
    let items = items.iter().map(|item| match item {
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
                        let ident = extract_ident(pat).unwrap_or_else(|| Cow::Owned(Ident::new("_", span)));
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
        _ => syn::Error::new(item.span(), "unsupported item type").to_compile_error(),
    });
    let async_trait = if attr_args.async_trait() { "#[async_trait::async_trait]" } else { "" };
    let async_trait: proc_macro2::TokenStream = async_trait.parse().unwrap();
    if let Some((None, path, _)) = trait_ {
        (quote! {
            #(#attrs)*
            #async_trait
            #visibility #unsafety trait #path {
                #(#items)*
            }
            #async_trait
            #impl_item
        })
        .into()
    } else {
        syn::Error::new(impl_token.span(), "extension trait name was not provided")
            .to_compile_error()
            .into()
    }
}

fn extract_ident(pat: &Pat) -> Option<Cow<'_, Ident>> {
    match pat {
        Pat::Box(PatBox { pat, .. }) | Pat::Reference(PatReference { pat, .. }) => {
            extract_ident(pat)
        }
        Pat::Ident(PatIdent { ident, .. }) => Some(Cow::Borrowed(ident)),
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
                Some(Cow::Owned(Ident::new(&joined, span)))
            }
        }
        _ => None,
    }
}
