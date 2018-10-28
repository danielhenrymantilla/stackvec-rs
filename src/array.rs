pub trait Sealed {}

pub trait Array: Sealed {
    type Item;

    const N: usize;

    fn as_ptr (
        self: &Self,
    ) -> *const Self::Item;

    fn as_mut_ptr (
        self: &mut Self,
    ) -> *mut Self::Item;
}

macro_rules! impl_array {
    ($N: expr) => (
        impl<T> Sealed for [T; $N] {}
        impl<T> Array for [T; $N] {
            type Item = T;

            const N: usize = $N;

            #[inline(always)]
            fn as_ptr (
                self: &Self,
            ) -> *const Self::Item
            {
                <[Self::Item]>::as_ptr(self)
            }

            #[inline(always)]
            fn as_mut_ptr (
                self: &mut Self,
            ) -> *mut Self::Item
            {
                <[Self::Item]>::as_mut_ptr(self)
            }
        }
    )
}

impl_array!(0x0);
impl_array!(0x1);
impl_array!(0x2);
impl_array!(0x3);
impl_array!(0x4);
impl_array!(0x5);
impl_array!(0x6);
impl_array!(0x7);
impl_array!(0x8);
impl_array!(0x9);
impl_array!(0xa);
impl_array!(0xb);
impl_array!(0xc);
impl_array!(0xd);
impl_array!(0xe);
impl_array!(0xf);
impl_array!(0x10);
impl_array!(0x20);
impl_array!(0x40);
impl_array!(0x80);
impl_array!(0x100);
impl_array!(0x200);
impl_array!(0x400);
impl_array!(0x800);
impl_array!(0x1000);
impl_array!(0x2000);
impl_array!(0x4000);
impl_array!(0x8000);
impl_array!(20);
impl_array!(30);
impl_array!(50);
impl_array!(80);
impl_array!(100);
impl_array!(120);
impl_array!(150);
impl_array!(200);
impl_array!(250);
impl_array!(300);
impl_array!(350);
impl_array!(400);
impl_array!(450);
impl_array!(500);
impl_array!(600);
impl_array!(700);
impl_array!(800);
impl_array!(900);
impl_array!(1000);
impl_array!(1100);
impl_array!(1200);
impl_array!(1400);
impl_array!(1600);
impl_array!(1800);
impl_array!(2000);
impl_array!(2500);
impl_array!(3000);
impl_array!(3500);
impl_array!(4000);
impl_array!(4500);
impl_array!(5000);
impl_array!(10000);
impl_array!(15000);
impl_array!(20000);
impl_array!(25000);
impl_array!(30000);
