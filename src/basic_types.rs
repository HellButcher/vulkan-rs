use std::marker::PhantomData;

pub type EnumRepr = u32;

pub trait Enum {
    fn to_repr(&self) -> EnumRepr;
    unsafe fn from_repr(EnumRepr) -> Self;
}

impl Enum for EnumRepr {
    fn to_repr(&self) -> EnumRepr {
        *self
    }
    unsafe fn from_repr(v: EnumRepr) -> Self {
        v
    }
}
macro_rules! vk_enum_impl {
    ($name:ident) => {
        impl $crate::basic_types::Enum for $name {
          fn to_repr(&self) -> $crate::basic_types::EnumRepr {
            *self as $crate::basic_types::EnumRepr
          }
          unsafe fn from_repr(v: $crate::basic_types::EnumRepr) -> Self {
            mem::transmute(v)
          }
        }
    };
}
macro_rules! vk_bitmask {
    ($(#[$attr:meta])* pub $flag:ident for $bits:path) => {
        $(#[$attr])*
        #[repr(C)]
        pub struct $flag {
            bits: $crate::basic_types::EnumRepr,
        }
        vk_bitmask!(impl $flag for $bits);
    };
    ($(#[$attr:meta])* $flag:ident for $bits:path) => {
        $(#[$attr])*
        #[repr(C)]
        struct $flag {
            bits: $crate::basic_types::EnumRepr,
        }
        vk_bitmask!(impl $flag for $bits);
    };
    (impl $flag:ident for $bits:path) => {
        impl $flag {
            /// Returns an empty set.
            #[inline]
            pub fn new() -> $flag {
                $flag {
                    bits: 0,
                }
            }
            #[inline]
            pub fn of(value: $bits) -> $flag {
                $flag {
                    bits: value as $crate::basic_types::EnumRepr,
                }
            }
            /// Returns true if no bit is set.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.bits == 0
            }
            /// Returns the union of both sets of bits.
            #[inline]
            pub fn union(&self, other: $flag) -> $flag {
                $flag {
                    bits: self.bits | other.bits,
                }
            }

            /// Returns the intersection of both sets of bits.
            #[inline]
            pub fn intersection(&self, other: $flag) -> $flag {
                $flag {
                    bits: self.bits & other.bits,
                }
            }

            /// Adds an enum, and returns `true` if it wasn't there before
            #[inline]
            pub fn insert(&mut self, value: $bits) -> bool {
                let result = !self.contains(value);
                self.bits |= value as $crate::basic_types::EnumRepr;
                result
            }

            /// Removes an enum from the EnumSet
            #[inline]
            pub fn remove(&mut self, value: $bits) -> bool {
                let result = self.contains(value);
                self.bits &= !(value as $crate::basic_types::EnumRepr);
                result
            }

            /// Returns `true` if the given bit is contained.
            #[inline]
            pub fn contains(&self, value: $bits) -> bool {
                (self.bits & (value as $crate::basic_types::EnumRepr)) != 0
            }

            /// Returns an iterator over the bits of the flags.
            pub fn iter(&self) -> $crate::basic_types::FlagsIter<$bits> {
                $crate::basic_types::FlagsIter::new(self.bits)
            }
        }
        impl ::std::ops::Sub for $flag {
            type Output = $flag;

            #[inline]
            fn sub(self, e: $flag) -> $flag {
                $flag {
                    bits: self.bits & !e.bits,
                }
            }
        }
        impl ::std::ops::BitOr for $flag {
            type Output = $flag;

            #[inline]
            fn bitor(self, e: $flag) -> $flag {
                $flag {
                    bits: self.bits | e.bits,
                }
            }
        }
        impl ::std::ops::BitAnd for $flag {
            type Output = $flag;

            #[inline]
            fn bitand(self, e: $flag) -> $flag {
                $flag {
                    bits: self.bits & e.bits,
                }
            }
        }
        impl ::std::ops::BitXor for $flag {
            type Output = $flag;

            #[inline]
            fn bitxor(self, e: $flag) -> $flag {
                $flag {
                    bits: self.bits ^ e.bits,
                }
            }
        }
        impl ::std::iter::FromIterator<$bits> for $flag {
            fn from_iter<I: IntoIterator<Item = $bits>>(iter: I) -> $flag {
                let mut ret = $flag::new();
                ret.extend(iter);
                ret
            }
        }
        impl<'a> IntoIterator for &'a $flag {
            type Item = $bits;
            type IntoIter = $crate::basic_types::FlagsIter<$bits>;

            fn into_iter(self) -> $crate::basic_types::FlagsIter<$bits> {
                $crate::basic_types::FlagsIter::new(self.bits)
            }
        }
        impl Extend<$bits> for $flag {
            fn extend<I: IntoIterator<Item = $bits>>(&mut self, iter: I) {
                for element in iter {
                    self.insert(element);
                }
            }
        }
        impl<'a> Extend<&'a $bits> for $flag {
            fn extend<I: IntoIterator<Item = &'a $bits>>(&mut self, iter: I) {
                self.extend(iter.into_iter().cloned());
            }
        }
    };
}
macro_rules! vk_union_enty_pad {
    ($value:ident ; ) => {};
    ($value:ident ; $($padding:expr),+) => {
        let $value = ($value, [ $($padding),* ]);
    };
}
macro_rules! vk_union_entries {
    ($sname:ident ;) => {};
    ($s_name:ident ; $(#[$attr:meta])* pub $fromname:ident / $asname:ident : $variant:ty ; $($tail:tt)*) => {
        vk_union_entries!($s_name; $(#[$attr])* pub $fromname / $asname : $variant [] ; $($tail)*);
    };
    ($s_name:ident ; $(#[$attr:meta])* $fromname:ident / $asname:ident : $variant:ty ; $($tail:tt)*) => {
        vk_union_entries!($s_name; $(#[$attr])* $fromname / $asname : $variant [] ; $($tail)*);
    };
    ($sname:ident ; $(#[$attr:meta])* pub $fromname:ident / $asname:ident : $variant:ty [$($padding:expr),*]; $($tail:tt)*) => {
        $(#[$attr])*
        #[inline]
        pub fn $fromname (value: $variant) -> $sname {
            vk_union_enty_pad!(value ; $($padding),*);
            $sname {
                data: unsafe { ::std::mem::transmute(value) }
            }
        }
        $(#[$attr])*
        #[inline]
        pub unsafe fn $asname (&self) -> &$variant {
            ::std::mem::transmute(&self.data)
        }
        vk_union_entries!($sname ; $($tail)*);
    };
    ($sname:ident ; $(#[$attr:meta])* $fromname:ident / $asname:ident : $variant:ty [$($padding:expr),*]; $($tail:tt)*) => {
        $(#[$attr])*
        #[inline]
        fn $fromname (value: $variant) -> $sname {
            vk_union_enty_pad!(value ; $($padding),*);
            $sname {
                data: unsafe { ::std::mem::transmute(value) }
            }
        }
        $(#[$attr])*
        #[inline]
        unsafe fn $asname (&self) -> &$variant {
            ::std::mem::transmute(&self.data)
        }
        vk_union_entries!($sname ; $($tail)*);
    };
}
macro_rules! vk_union {
    ($(#[$attr:meta])* pub $name:ident : $base:ty {
        $($tail:tt)*
    }) => {
        $(#[$attr])*
        #[repr(C)]
        pub struct $name {
          data: $base,
        }
        impl $name {
            vk_union_entries!($name ; $($tail)*);
        }
    };
    ($(#[$attr:meta])* $name:ident : $base:ty {
        $($tail:tt)*
    }) => {
        $(#[$attr])*
        #[repr(C)]
        struct $name {
          data: $base,
        }
        impl $name {
            vk_union_entries!($name ; $($tail)*);
        }
    };
}

#[derive(Copy,Clone)]
pub struct FlagsIter<B> {
    index: u32,
    bits: EnumRepr,
    marker: PhantomData<B>,
}

impl<B: Enum> FlagsIter<B> {
    pub fn new(bits: EnumRepr) -> FlagsIter<B> {
        FlagsIter {
            index: 0,
            bits: bits,
            marker: PhantomData,
        }
    }
}

impl<B: Enum> Iterator for FlagsIter<B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        if self.bits == 0 {
            return None;
        }

        while (self.bits & 1) == 0 {
            self.index += 1;
            self.bits >>= 1;
        }
        unsafe {
            let elem = B::from_repr(1 << self.index);
            self.index += 1;
            self.bits >>= 1;
            Some(elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.bits.count_ones() as usize;
        (exact, Some(exact))
    }
}


#[repr(C)]
pub struct Handle {
    value: usize,
}
#[repr(C)]
pub struct DispatchableHandle {
    value: u32,
}


#[cfg(test)]
mod tests {
    use super::{Enum, EnumRepr};
    use std::mem;

    #[derive(Debug,Copy,Clone)]
    #[repr(u32)]
    enum TestBits {
        Value1 = 1,
        Value2 = 2,
        Value3 = 4,
    }
    impl Enum for TestBits {
        fn to_repr(&self) -> EnumRepr {
            *self as EnumRepr
        }
        unsafe fn from_repr(v: EnumRepr) -> Self {
            mem::transmute(v)
        }
    }
    vk_bitmask!(TestFlags for TestBits);

    vk_union!(pub TestUnion : [u32; 4] {
        pub from_float / as_float : [f32; 4];
        pub from_int / as_int : [u32; 4];
        pub from_long / as_long : [u64; 2];
        pub from_single / as_single : u64 [0u64];
    });

    #[test]
    fn test_flags() {
        let mut v = TestFlags::new();
        assert!(v.is_empty());
        assert!(!v.contains(TestBits::Value1));
        assert!(!v.contains(TestBits::Value2));
        assert!(!v.contains(TestBits::Value3));
        assert!(v.insert(TestBits::Value1));
        assert!(!v.is_empty());
        assert!(v.contains(TestBits::Value1));
        assert!(!v.contains(TestBits::Value2));
        assert!(!v.contains(TestBits::Value3));
        assert!(!v.insert(TestBits::Value1));
        assert!(!v.is_empty());
        assert!(v.contains(TestBits::Value1));
        assert!(!v.contains(TestBits::Value2));
        assert!(!v.contains(TestBits::Value3));
        assert!(v.insert(TestBits::Value2));
        assert!(!v.is_empty());
        assert!(v.contains(TestBits::Value1));
        assert!(v.contains(TestBits::Value2));
        assert!(!v.contains(TestBits::Value3));
        assert!(v.remove(TestBits::Value1));
        assert!(!v.is_empty());
        assert!(!v.contains(TestBits::Value1));
        assert!(v.contains(TestBits::Value2));
        assert!(!v.contains(TestBits::Value3));
        assert!(!v.remove(TestBits::Value3));
        assert!(!v.is_empty());
        assert!(!v.contains(TestBits::Value1));
        assert!(v.contains(TestBits::Value2));
        assert!(!v.contains(TestBits::Value3));
    }

    #[test]
    fn test_union1() {
        unsafe {
            let v = TestUnion::from_single(1337);
            let v2 = v.as_int();
            assert!(v2[0]==1337);
            assert!(v2[1]==0);
            assert!(v2[2]==0);
            assert!(v2[3]==0);
        }
    }
    #[test]
    fn test_union2() {
        unsafe {
            let v = TestUnion::from_long([1337, 666]);
            let v2 = v.as_int();
            assert!(v2[0]==1337);
            assert!(v2[1]==0);
            assert!(v2[2]==666);
            assert!(v2[3]==0);
        }
    }
}
