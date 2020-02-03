macro_rules! define_unsigned {
    ( $usertype:ident, $containertype:ident, $usersize:expr, #[$doc:meta] ) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        pub struct $usertype($containertype);
    };
}
