use macaw::ColorRgba8;
use mirror_mirror_macros::__private_derive_reflect_foreign;
use fixed_type_id::{prelude::*, type_name, type_id};

__private_derive_reflect_foreign! {
    #[reflect(crate_name(crate), opt_out(Default))]
    pub struct ColorRgba8(pub [u8; 4]);
}
