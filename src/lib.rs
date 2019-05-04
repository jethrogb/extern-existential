#![no_std]

#[macro_export]
macro_rules! extern_existential {
    ( extern existential type $i:ident: $tr:path = $ty:path; ) => {
        #[no_mangle]
        pub static $i: &(dyn $tr + Send + Sync + 'static) = &$ty;
    };
    ( pub extern existential type $i:ident: $tr:path; ) => {
        pub struct $i;

        impl core::ops::Deref for $i {
            type Target = dyn $tr + 'static;

            // make sure the use of the extern symbol appears in another crate
            // so the undefined symbol appears in the right place in the link
            // order
            #[inline(always)]
            fn deref(&self) -> &(dyn $tr + 'static) {
                #[allow(improper_ctypes)]
                extern "C" {
                    static $i: &'static (dyn $tr + Send + Sync + 'static);
                }

                unsafe { $i }
            }
        }
    };
}
