#[macro_use] pub mod bitflags;

macro_rules! as_item {
    ($($i:item)*) => ($($i)*);
}

macro_rules! ext_impl {
    (
        $impl_ty:ty as $trait_ident:ident {
            $(fn $fn_name:ident [$($gen_args:tt)*] $fn_args:tt $(-> $fn_return:ty)* {$($fn_block:tt)*})*
        }
    ) => {
        as_item! {
            pub trait $trait_ident {
                $(fn $fn_name <$($gen_args)*> $fn_args $(-> $fn_return)*;)*
            }

            impl $trait_ident for $impl_ty {
                $(fn $fn_name <$($gen_args)*> $fn_args $(-> $fn_return)* {$($fn_block)*})*
            }
        }
    };
}

macro_rules! impl_Display {
    (for $t:ty, ($self_:ident, $fmt:ident) $body:block) => {
        impl ::std::fmt::Display for $t {
            #[allow(unused_imports)]
            fn fmt(&self, $fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                use ::std::fmt::Display;
                let $self_ = self;
                $body
            }
        }
    };
}
