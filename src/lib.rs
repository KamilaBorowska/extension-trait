/// Declares an extension trait
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate extension_trait;
///
/// extension_trait! { pub Double for str {
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
    (@finish_parsing [$($pub_token:tt)*]
        $(#[$attr:meta])*
        $(<$(
            $impl_gen_name:ident $(: [$($impl_gen_bound:tt)*])*
        ),*>)*
        $trait_name:ident for $type_name:ty
        $(
            where $(
                $where_impl_gen_name:ident $(: [$($where_impl_gen_bound:tt)*])*
            ),*
            $(,)*
        )*
        { $(
            fn $fn_name:ident
            $( < $(
                $fn_gen_name:ident $(: [$($fn_gen_bound:tt)*])*
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
            fn $fn_name
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
            fn $fn_name
            $( < $(
                $fn_gen_name $(: $($fn_gen_bound)*)*
            ),* > )*
            ( $($args)* ) $(-> $out)*
            $( where $( $where_gen_name: $($where_gen_bound)* ),* )*
            $code
        )* }
    };

    (@finish_parsing $($rest:tt)*) => {
        compile_error!(concat!("Incorrect macro syntax, debug info:\n", stringify!($($rest)*)));
    };

    (@normalize_block $parsed:tt $block_parsed:tt $pub_token:tt : $($rest:tt)*) => {
        extension_trait!(
            @parse_type_till_end
            [@normalize_block $parsed]
            $block_parsed
            []
            $pub_token
            []
            $($rest)*
        );
    };

    (@normalize_block {$($parsed:tt)*} $block_parsed:tt $pub_token:tt) => {
        extension_trait!(@finish_parsing $pub_token $($parsed)* $block_parsed);
    };

    (
        @normalize_block $parsed:tt {$($block_parsed:tt)*}
        $pub_token:tt $parsed_token:tt $($rest:tt)*
    ) => {
        extension_trait!(
            @normalize_block $parsed {$($block_parsed)* $parsed_token}
            $pub_token $($rest)*
        );
    };

    (@normalize_expression $parsed:tt $pub_token:tt {$($contents:tt)*}) => {
        extension_trait!(@normalize_block $parsed {} $pub_token $($contents)*);
    };

    (@normalize_expression $parsed:tt [] pub $($rest:tt)*) => {
        extension_trait!(@normalize_expression $parsed [pub] $($rest)*);
    };

    (@normalize_expression $parsed:tt $pub_token:tt : $($rest:tt)*) => {
        extension_trait!(
            @parse_type_till_end [@normalize_expression] $parsed []
            $pub_token [] $($rest)*
        );
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt [$($left_brackets:tt)*]
        $pub_token:tt [$($type_name:tt)*] < $($rest:tt)*
    ) => {
        extension_trait!(
            @parse_type_till_end $return_trait $parsed [$($left_brackets)* <]
            $pub_token [$($type_name)* <] $($rest)*
        );
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt $left_brackets:tt
        $pub_token:tt $type_name:tt >> $($rest:tt)*
    ) => {
        extension_trait!(
            @parse_type_till_end $return_trait $parsed $left_brackets
            $pub_token $type_name > > $($rest)*
        );
    };

    (
        @parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} []
        $pub_token:tt $type_name:tt > $($rest:tt)*
    ) => {
        extension_trait!($($return_trait)* {$($parsed)*: $type_name>} $pub_token $($rest)*);
    };

    (
        @parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} $left_brackets:tt
        $pub_token:tt $type_name:tt , $($rest:tt)*
    ) => {
        extension_trait!($($return_trait)* {$($parsed)*: $type_name,} $pub_token $($rest)*);
    };

    (
        @parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} []
        $pub_token:tt $type_name:tt {$($block:tt)*} $($rest:tt)*
    ) => {
        extension_trait!(
            $($return_trait)* {$($parsed)*: $type_name}
            $pub_token {$($block)*} $($rest)*
        );
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt [< $($left_brackets:tt)*]
        $pub_token:tt [$($type_name:tt)*] > $($rest:tt)*
    ) => {
        extension_trait!(@parse_type_till_end $return_trait $parsed
        [$($left_brackets)*] $pub_token [$($type_name)* >] $($rest)*);
    };

    (
        @parse_type_till_end $return_trait:tt $parsed:tt $left_brackets:tt
        $pub_token:tt [$($type_name:tt)*] $token:tt $($rest:tt)*
    ) => {
        extension_trait!(
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
        extension_trait!(@normalize_expression {$($parsed)* $parsed_token} $pub_token $($rest)*);
    };

    (@ $($rest:tt)*) => {
        compile_error!(concat!(
            "Parsing has failed before generating final code, this is likely a bug, debug info:\n",
            stringify!(@ $($rest)*)
        ) );
    };

    ($($token:tt)*) => {
        extension_trait!(@normalize_expression {} [] $($token)*);
    };
}
