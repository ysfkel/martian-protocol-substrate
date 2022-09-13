#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::CheckedAdd, ArithmeticError};

pub trait SafeAdd: Sized {
	fn safe_add(&self, val: &Self) -> Result<Self, ArithmeticError>;
}

impl<T: CheckedAdd> SafeAdd for T {
	#[inline(always)]
	fn safe_add(&self, val: &Self) -> Result<Self, ArithmeticError> {
		self.checked_add(val).ok_or(ArithmeticError::Overflow)
	}
}
