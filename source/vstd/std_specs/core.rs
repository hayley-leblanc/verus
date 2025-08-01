use super::super::prelude::*;

use verus as verus_;

verus_! {

#[verifier::external_trait_specification]
pub trait ExDeref {
    type ExternalTraitSpecificationFor: core::ops::Deref;

    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

#[verifier::external_trait_specification]
pub trait ExIndex<Idx> where Idx: ?Sized {
    type ExternalTraitSpecificationFor: core::ops::Index<Idx>;

    type Output: ?Sized;
}

#[verifier::external_trait_specification]
pub trait ExIndexMut<Idx>: core::ops::Index<Idx> where Idx: ?Sized {
    type ExternalTraitSpecificationFor: core::ops::IndexMut<Idx>;
}

#[verifier::external_trait_specification]
pub trait ExInteger: Copy {
    type ExternalTraitSpecificationFor: Integer;
}

#[verifier::external_trait_specification]
pub trait ExSpecOrd<Rhs> {
    type ExternalTraitSpecificationFor: SpecOrd<Rhs>;
}

#[verifier::external_trait_specification]
pub trait ExAllocator {
    type ExternalTraitSpecificationFor: core::alloc::Allocator;
}

#[verifier::external_trait_specification]
pub trait ExFreeze {
    type ExternalTraitSpecificationFor: core::marker::Freeze;
}

#[verifier::external_trait_specification]
pub trait ExDebug {
    type ExternalTraitSpecificationFor: core::fmt::Debug;
}

#[verifier::external_trait_specification]
pub trait ExDisplay {
    type ExternalTraitSpecificationFor: core::fmt::Display;
}

#[verifier::external_trait_specification]
pub trait ExFrom<T>: Sized {
    type ExternalTraitSpecificationFor: core::convert::From<T>;

    fn from(v: T) -> (ret: Self);
}

#[verifier::external_trait_specification]
pub trait ExInto<T>: Sized {
    type ExternalTraitSpecificationFor: core::convert::Into<T>;

    fn into(self) -> (ret: T);
}

pub assume_specification<T, U: From<T>>[ T::into ](a: T) -> (ret: U)
    ensures
        call_ensures(U::from, (a,), ret),
;

#[verifier::external_trait_specification]
#[verifier::external_trait_extension(PartialEqSpec via PartialEqSpecImpl)]
pub trait ExPartialEq<Rhs: ?Sized = Self> {
    type ExternalTraitSpecificationFor: core::cmp::PartialEq<Rhs>;

    spec fn obeys_spec_eq() -> bool;

    spec fn spec_eq(&self, other: &Rhs) -> bool;

    fn eq(&self, other: &Rhs) -> (r: bool)
        ensures
            Self::obeys_spec_eq() ==> r == self.spec_eq(other);

    fn ne(&self, other: &Rhs) -> (r: bool)
        ensures
            Self::obeys_spec_eq() ==> r == !self.spec_eq(other),
        default_ensures
            call_ensures(Self::eq, (self, other), !r);
}

#[verifier::external_trait_specification]
pub trait ExEq: PartialEq {
    type ExternalTraitSpecificationFor: core::cmp::Eq;
}

#[verifier::external_trait_specification]
#[verifier::external_trait_extension(PartialOrdSpec via PartialOrdSpecImpl)]
pub trait ExPartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    type ExternalTraitSpecificationFor: core::cmp::PartialOrd<Rhs>;

    spec fn obeys_spec_partial_cmp() -> bool;

    spec fn spec_partial_cmp(&self, other: &Rhs) -> Option<core::cmp::Ordering>;

    fn partial_cmp(&self, other: &Rhs) -> (r: Option<core::cmp::Ordering>)
        ensures
            Self::obeys_spec_partial_cmp() ==> r == self.spec_partial_cmp(other);

    fn lt(&self, other: &Rhs) -> (r: bool)
        ensures
            Self::obeys_spec_partial_cmp() ==>
                (r <==> self.spec_partial_cmp(other) == Some(core::cmp::Ordering::Less)),
        default_ensures
            exists|o: Option<core::cmp::Ordering>|
                {
                    &&& #[trigger] call_ensures(Self::partial_cmp, (self, other), o)
                    &&& r <==> o == Some(core::cmp::Ordering::Less)
                }
    ;

    fn le(&self, other: &Rhs) -> (r: bool)
        ensures
            Self::obeys_spec_partial_cmp() ==>
                (r <==> self.spec_partial_cmp(other) matches Some(
                    core::cmp::Ordering::Less
                    | core::cmp::Ordering::Equal,
                )),
        default_ensures
            exists|o: Option<core::cmp::Ordering>|
                {
                    &&& #[trigger] call_ensures(Self::partial_cmp, (self, other), o)
                    &&& r <==> o matches Some(
                        core::cmp::Ordering::Less
                        | core::cmp::Ordering::Equal,
                    )
                }
    ;

    fn gt(&self, other: &Rhs) -> (r: bool)
        ensures
            Self::obeys_spec_partial_cmp() ==>
                (r <==> self.spec_partial_cmp(other) == Some(core::cmp::Ordering::Greater)),
        default_ensures
            exists|o: Option<core::cmp::Ordering>|
                {
                    &&& #[trigger] call_ensures(Self::partial_cmp, (self, other), o)
                    &&& r <==> o == Some(core::cmp::Ordering::Greater)
                }
    ;

    fn ge(&self, other: &Rhs) -> (r: bool)
        ensures
            Self::obeys_spec_partial_cmp() ==>
                (r <==> self.spec_partial_cmp(other) matches Some(
                    core::cmp::Ordering::Greater
                    | core::cmp::Ordering::Equal,
                )),
        default_ensures
            exists|o: Option<core::cmp::Ordering>|
                {
                    &&& #[trigger] call_ensures(Self::partial_cmp, (self, other), o)
                    &&& r <==> o matches Some(
                        core::cmp::Ordering::Greater
                        | core::cmp::Ordering::Equal,
                    )
                }
    ;
}

#[verifier::external_trait_specification]
pub trait ExOrd: Eq + PartialOrd {
    type ExternalTraitSpecificationFor: Ord;
}

#[verifier::external_trait_specification]
pub trait ExHash {
    type ExternalTraitSpecificationFor: core::hash::Hash;
}

#[verifier::external_trait_specification]
pub trait ExPtrPointee {
    type ExternalTraitSpecificationFor: core::ptr::Pointee;

    type Metadata:
        Copy + Send + Sync + Ord + core::hash::Hash + Unpin + core::fmt::Debug + Sized + core::marker::Freeze;
}

#[verifier::external_trait_specification]
pub trait ExIterator {
    type ExternalTraitSpecificationFor: core::iter::Iterator;

    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[verifier::external_trait_specification]
pub trait ExIntoIterator {
    type ExternalTraitSpecificationFor: core::iter::IntoIterator;
}

#[verifier::external_trait_specification]
pub trait ExIterStep: Clone + PartialOrd + Sized {
    type ExternalTraitSpecificationFor: core::iter::Step;
}

#[verifier::external_trait_specification]
pub trait ExBorrow<Borrowed> where Borrowed: ?Sized {
    type ExternalTraitSpecificationFor: core::borrow::Borrow<Borrowed>;
}

#[verifier::external_trait_specification]
pub trait ExStructural {
    type ExternalTraitSpecificationFor: Structural;
}

pub assume_specification<T>[ core::mem::swap::<T> ](a: &mut T, b: &mut T)
    ensures
        *a == *old(b),
        *b == *old(a),
    opens_invariants none
    no_unwind
;

#[verifier::external_type_specification]
pub struct ExOrdering(core::cmp::Ordering);

#[verifier::external_type_specification]
#[verifier::accept_recursive_types(V)]
#[verifier::ext_equal]
pub struct ExOption<V>(core::option::Option<V>);

#[verifier::external_type_specification]
#[verifier::accept_recursive_types(T)]
#[verifier::reject_recursive_types_in_ground_variants(E)]
pub struct ExResult<T, E>(core::result::Result<T, E>);

pub open spec fn iter_into_iter_spec<I: Iterator>(i: I) -> I {
    i
}

#[verifier::when_used_as_spec(iter_into_iter_spec)]
pub assume_specification<I: Iterator>[ I::into_iter ](i: I) -> (r: I)
    ensures
        r == i,
;

// I don't really expect this to be particularly useful;
// this is mostly here because I wanted an easy way to test
// the combination of external_type_specification & external_body
// in a cross-crate context.
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExDuration(core::time::Duration);

#[verifier::external_type_specification]
#[verifier::accept_recursive_types(V)]
pub struct ExPhantomData<V: ?Sized>(core::marker::PhantomData<V>);

pub assume_specification[ core::intrinsics::likely ](b: bool) -> (c: bool)
    ensures
        c == b,
;

pub assume_specification[ core::intrinsics::unlikely ](b: bool) -> (c: bool)
    ensures
        c == b,
;

pub assume_specification<T, F: FnOnce() -> T>[ bool::then ](b: bool, f: F) -> (ret: Option<T>)
    ensures
        if b {
            ret.is_some() && f.ensures((), ret.unwrap())
        } else {
            ret.is_none()
        },
;

#[verifier::external_type_specification]
#[verifier::external_body]
#[verifier::reject_recursive_types_in_ground_variants(V)]
pub struct ExManuallyDrop<V: ?Sized>(core::mem::ManuallyDrop<V>);

// A private seal trait to prevent a trait from being implemented outside of vstd.
pub(crate) trait TrustedSpecSealed {}

#[allow(private_bounds)]
pub trait IndexSetTrustedSpec<Idx>: core::ops::IndexMut<Idx> + TrustedSpecSealed {
    spec fn spec_index_set_requires(&self, index: Idx) -> bool;

    spec fn spec_index_set_ensures(
        &self,
        new_container: &Self,
        index: Idx,
        val: Self::Output,
    ) -> bool where Self::Output: Sized;
}

// TODO(uutaal): Do not need index_set once mutable reference support lands.
// Use index_set to replace IndexMut in assign-operator.
// Users must provide IndexSetTrustedSpec to use it.
// It could be replaced after mutable reference is fully supported
// Avoid call it explicitly.
#[verifier(external_body)]
pub fn index_set<T, Idx, E>(container: &mut T, index: Idx, val: E) where
    T: ?Sized + core::ops::IndexMut<Idx> + core::ops::Index<Idx, Output = E> + IndexSetTrustedSpec<
        Idx,
    >,

    requires
        old(container).spec_index_set_requires(index),
    ensures
        old(container).spec_index_set_ensures(container, index, val),
    no_unwind
{
    container[index] = val;
}

} // verus!
macro_rules! impl_from_spec {
    ($from: ty => [$($to: ty)*]) => {
        verus!{
        $(
        pub assume_specification[ <$to as core::convert::From<$from>>::from ](a: $from) -> (ret: $to)
            ensures
                ret == a as $to,
        ;
        )*
        }
    };
}

impl_from_spec! {u8 => [u16 u32 u64 usize u128]}
impl_from_spec! {u16 => [u32 u64 usize u128]}
impl_from_spec! {u32 => [u64 u128]}
impl_from_spec! {u64 => [u128]}

#[verifier::external_type_specification]
#[verifier::external_body]
#[verifier::accept_recursive_types(T)]
pub struct ExAssertParamIsClone<T: Clone + ?Sized>(core::clone::AssertParamIsClone<T>);
