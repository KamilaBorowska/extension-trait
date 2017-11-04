#[macro_export]
macro_rules! extension_trait {
    (
        $(#[$attr:meta])*
        $trait_name:ident for $impl_ty:ty { $(
            fn $fn_name:ident $args:tt $(-> $out_type:ty)* $code:block
        )* }
    ) => {
        $(#[$attr])*
        trait $trait_name { $(
            fn $fn_name $args $(-> $out_type)*;
        )* }

        impl $trait_name for $impl_ty { $(
            fn $fn_name $args $(-> $out_type)* $code
        )* }
    };

    (
        $(#[$attr:meta])*
        pub $trait_name:ident for $impl_ty:ty { $(
            fn $fn_name:ident $args:tt $(-> $out_type:ty)* $code:block
        )* }
    ) => {
        $(#[$attr])*
        pub trait $trait_name { $(
            fn $fn_name $args $(-> $out_type)*;
        )* }

        impl $trait_name for $impl_ty { $(
            fn $fn_name $args $(-> $out_type)* $code
        )* }
    };
}
