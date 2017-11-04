#![no_std]

#[doc(hidden)]
#[macro_export]
macro_rules! extension_trait_internal {
    (@finish_parsing [$($pub_token:tt)*]
        $(#[$attr:meta])*
        $(<$(
            $impl_gen_name:tt $(: [$($impl_gen_bound:tt)*])*
        ),*>)*
        $trait_name:ident for $type_name:ty
        $(
            where $(
                $where_impl_gen_name:tt $(: [$($where_impl_gen_bound:tt)*])*
            ),*
            $(,)*
        )*
        { $(
            [$($fn_keywords:tt)*] fn $fn_name:ident
            $( < $(
                $fn_gen_name:tt $(: [$($fn_gen_bound:tt)*])*
            ),* > )*
            ( $($args:tt)* ) $(-> $out:ty)*
            $(
                where $(
                    $where_gen_name:ty : [$($where_gen_bound:tt)*]
                ),*
                $(,)*
            )*
            $code:block
        )* }
    ) => {
        $(#[$attr])*
        $($pub_token)*
        trait $trait_name
        { $(
            $($fn_keywords)* fn $fn_name
            $( < $(
                $fn_gen_name $(: $($fn_gen_bound)*)*
            ),* > )*
            ( $($args)* ) $(-> $out)*
            $( where $( $where_gen_name: $($where_gen_bound)* ),* )*;
        )* }

        impl
        $(<$(
            $impl_gen_name $(: $($impl_gen_bound)*)*
        ),*>)*
        $trait_name for $type_name
        $(
            where
            $(
                $where_impl_gen_name $(: $($where_impl_gen_bound)*)*
            ),*
        )*
        { $(
            $($fn_keywords)* fn $fn_name
            $( < $(
                $fn_gen_name $(: $($fn_gen_bound)*)*
            ),* > )*
            ( $($args)* ) $(-> $out)*
            $( where $( $where_gen_name: $($where_gen_bound)* ),* )*
            $code
        )* }
    };

    (@normalize_block $parsed:tt $block_parsed:tt $pub_token:tt : $($rest:tt)*) => {
        extension_trait_internal!(
            @parse_type_till_end
            [@normalize_block $parsed]
            $block_parsed
            []
            $pub_token
            []
            $($rest)*
        );
    };

    (@normalize_block $parsed:tt {$($block_parsed:tt)*} $pub_token:tt fn $($rest:tt)*) => {
        extension_trait_internal!(
            @normalize_block $parsed {$($block_parsed)* [] fn}
            $pub_token $($rest)*
        );
    };

    (@normalize_block $parsed:tt {$($block_parsed:tt)*} $pub_token:tt unsafe fn $($rest:tt)*) => {
        extension_trait_internal!(
            @normalize_block $parsed {$($block_parsed)* [unsafe] fn}
            $pub_token $($rest)*
        );
    };

    (@normalize_block {$($parsed:tt)*} $block_parsed:tt $pub_token:tt) => {
        extension_trait_internal!(@finish_parsing $pub_token $($parsed)* $block_parsed);
    };

    (
        @normalize_block $parsed:tt {$($block_parsed:tt)*}
        $pub_token:tt $parsed_token:tt $($rest:tt)*
    ) => {
        extension_trait_internal!(
            @normalize_block $parsed {$($block_parsed)* $parsed_token}
            $pub_token $($rest)*
        );
    };

    (@normalize_expression $parsed:tt $pub_token:tt {$($contents:tt)*}) => {
        extension_trait_internal!(@normalize_block $parsed {} $pub_token $($contents)*);
    };

    (@normalize_expression $parsed:tt [] pub $($rest:tt)*) => {
        extension_trait_internal!(@normalize_expression $parsed [pub] $($rest)*);
    };

    (@normalize_expression $parsed:tt $pub_token:tt : $($rest:tt)*) => {
        extension_trait_internal!(
            @parse_type_till_end [@normalize_expression] $parsed []
            $pub_token [] $($rest)*
        );
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt [$($left_brackets:tt)*]
        $pub_token:tt [$($type_name:tt)*] < $($rest:tt)*
    ) => {
        extension_trait_internal!(
            @parse_type_till_end $return_trait $parsed [$($left_brackets)* <]
            $pub_token [$($type_name)* <] $($rest)*
        );
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt $left_brackets:tt
        $pub_token:tt $type_name:tt >> $($rest:tt)*
    ) => {
        extension_trait_internal!(
            @parse_type_till_end $return_trait $parsed $left_brackets
            $pub_token $type_name > > $($rest)*
        );
    };

    (
        @parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} []
        $pub_token:tt $type_name:tt > $($rest:tt)*
    ) => {
        extension_trait_internal!(
            $($return_trait)* {$($parsed)*: $type_name>}
            $pub_token $($rest)*
        );
    };

    (
        @parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} $left_brackets:tt
        $pub_token:tt $type_name:tt , $($rest:tt)*
    ) => {
        extension_trait_internal!(
            $($return_trait)* {$($parsed)*: $type_name,}
            $pub_token $($rest)*
        );
    };

    (
        @parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} []
        $pub_token:tt $type_name:tt {$($block:tt)*} $($rest:tt)*
    ) => {
        extension_trait_internal!(
            $($return_trait)* {$($parsed)*: $type_name}
            $pub_token {$($block)*} $($rest)*
        );
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt [< $($left_brackets:tt)*]
        $pub_token:tt [$($type_name:tt)*] > $($rest:tt)*
    ) => {
        extension_trait_internal!(@parse_type_till_end $return_trait $parsed
        [$($left_brackets)*] $pub_token [$($type_name)* >] $($rest)*);
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt $left_brackets:tt
        $pub_token:tt [$($type_name:tt)*] $token:tt $($rest:tt)*
    ) => {
        extension_trait_internal!(
            @parse_type_till_end
            $return_trait
            $parsed
            $left_brackets
            $pub_token
            [$($type_name)* $token]
            $($rest)*
        );
    };

    (@normalize_expression {$($parsed:tt)*} $pub_token:tt $parsed_token:tt $($rest:tt)*) => {
        extension_trait_internal!(
            @normalize_expression {$($parsed)* $parsed_token}
            $pub_token $($rest)*
        );
    };
}

/// Declares an extension trait
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate extension_trait;
///
/// extension_trait! { pub DoubleExt for str {
///    fn double(&self) -> String {
///        self.repeat(2)
///    }
/// } }
///
/// fn main() {
///     assert_eq!("Hello".double(), "HelloHello");
/// }
/// ```
#[macro_export]
macro_rules! extension_trait {
    ($($token:tt)+) => {
        extension_trait_internal!(@normalize_expression {} [] $($token)+);
    };
}
