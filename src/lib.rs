#[macro_export]
macro_rules! extension_trait {
    (@finish_parsing [$($pub_token:tt)*]
        $(#[$attr:meta])*
        $(<$(
            $impl_gen_name:ident $(: [$($impl_gen_bound:tt)*])*
        ),*>)*
        $trait_name:ident for $type_name:ty
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
        trait $trait_name { $(
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
        compile_error!("Incorrect macro syntax");
    };

    (@normalize_block $parsed:tt $block_parsed:tt $pub_token:tt >> $($rest:tt)*) => {
        extension_trait!(@normalize_block $parsed $block_parsed $pub_token  > > $($rest)*);
    };

    (@normalize_block $parsed:tt $block_parsed:tt $pub_token:tt : $($rest:tt)*) => {
        extension_trait!(@parse_type_till_end [@normalize_block $parsed] $block_parsed $pub_token [] $($rest)*);
    };

    (@normalize_block {$($parsed:tt)*} $block_parsed:tt $pub_token:tt) => {
        extension_trait!(@finish_parsing $pub_token $($parsed)* $block_parsed);
    };

    (@normalize_block $parsed:tt {$($block_parsed:tt)*} $pub_token:tt $parsed_token:tt $($rest:tt)*) => {
        extension_trait!(@normalize_block $parsed {$($block_parsed)* $parsed_token} $pub_token $($rest)*);
    };

    (@normalize_expression $parsed:tt $pub_token:tt {$($contents:tt)*}) => {
        extension_trait!(@normalize_block $parsed {} $pub_token $($contents)*);
    };

    (@normalize_expression $parsed:tt $pub_token:tt >> $($rest:tt)*) => {
        extension_trait!(@normalize_expression $parsed $pub_token > > $($rest)*);
    };

    (@normalize_expression $parsed:tt [] pub $($rest:tt)*) => {
        extension_trait!(@normalize_expression $parsed [pub] $($rest)*);
    };

    (@normalize_expression $parsed:tt $pub_token:tt : $($rest:tt)*) => {
        extension_trait!(@parse_type_till_end [@normalize_expression] $parsed $pub_token [] $($rest)*);
    };

    (@parse_type_till_end [$($return_trait:tt)*] {$($parsed:tt)*} $pub_token:tt $type_name:tt , $($rest:tt)*) => {
        extension_trait!($($return_trait)* {$($parsed)* : $type_name,} $pub_token $($rest)*);
    };

    (@parse_type_till_end $return_trait:tt $parsed:tt $pub_token:tt [$($type_name:tt)*] $token:tt $($rest:tt)*) => {
        extension_trait!(@parse_type_till_end $return_trait $parsed $pub_token [$($type_name)* $token] $($rest)*);
    };

    (@normalize_expression {$($parsed:tt)*} $pub_token:tt $parsed_token:tt $($rest:tt)*) => {
        extension_trait!(@normalize_expression {$($parsed)* $parsed_token} $pub_token $($rest)*);
    };

    (@ $($rest:tt)*) => {
        compile_error!("Parsing has failed before generating final code, this is likely a bug");
    };

    ($($token:tt)*) => {
        extension_trait!(@normalize_expression {} [] $($token)*);
    };
}
