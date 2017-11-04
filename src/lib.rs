#[macro_export]
macro_rules! extension_trait {
    (@finish_parsing [$($pub_token:tt)*] [
        $(#[$attr:meta])*
        $(<$(
            $impl_gen_name:ident $(: $impl_gen_bound:ty)*
        ),*>)*
        $trait_name:ident for $type_name:ty
        { $(
            fn $fn_name:ident
            $( < $(
                $fn_gen_name:ident $(: $fn_gen_bound:ty)*
            ),* > )*
            ( $($args:tt)* ) $(-> $out:ty)*
            $(
                where $(
                    $where_gen_name:ty : $bound:ty
                ),*
                $(,)*
            )*
            $code:block
        )* }
    ]) => {
        $(#[$attr])*
        $($pub_token)*
        trait $trait_name { $(
            fn $fn_name
            $( < $(
                $fn_gen_name $(: $fn_gen_bound)*
            ),* > )*
            ( $($args)* ) $(-> $out)*;
        )* }

        impl
        $(<$(
            $impl_gen_name $(: $impl_gen_bound)*
        ),*>)*
        $trait_name for $type_name
        { $(
            fn $fn_name
            $( < $(
                $fn_gen_name $(: $fn_gen_bound)*
            ),* > )*
            ( $($args)* ) $(-> $out)* $code
        )* }
    };

    (@normalize_expression $parsed:tt $pub_token:tt) => {
        extension_trait!(@finish_parsing $pub_token $parsed);
    };

    (@normalize_expression $parsed:tt $pub_token:tt >> $($rest:tt)*) => {
        extension_trait!(@normalize_expression $parsed $pub_token > > $($rest)*);
    };

    (@normalize_expression $parsed:tt [] pub $($rest:tt)*) => {
        extension_trait!(@normalize_expression $parsed [pub] $($rest)*);
    };

    (@normalize_expression [$($parsed:tt)*] $pub_token:tt $parsed_token:tt $($rest:tt)*) => {
        extension_trait!(@normalize_expression [$($parsed)* $parsed_token] $pub_token $($rest)*);
    };

    ($($token:tt)*) => {
        extension_trait!(@normalize_expression [] [] $($token)*);
    };
}
