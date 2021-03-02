// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_fields::PrimeField;

use crate::{
    utilities::{
        boolean::Boolean,
        eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
        uint::*,
    },
};
use snarkvm_r1cs::{errors::SynthesisError, ConstraintSystem};

macro_rules! eq_uint_impl {
    ($gadget: ident, $size: expr) => {
        impl<F: PrimeField> EvaluateEqGadget<F> for $gadget {
            fn evaluate_equal<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self,
            ) -> Result<Boolean, SynthesisError> {
                let mut result = Boolean::constant(true);
                for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
                    let equal = a.evaluate_equal(
                        &mut cs.ns(|| format!("{} evaluate equality for {}-th bit", $size, i)),
                        b,
                    )?;

                    result = Boolean::and(
                        &mut cs.ns(|| format!("{} and result for {}-th bit", $size, i)),
                        &equal,
                        &result,
                    )?;
                }

                Ok(result)
            }
        }

        impl<F: PrimeField> EqGadget<F> for $gadget {}

        impl<F: PrimeField> ConditionalEqGadget<F> for $gadget {
            fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self,
                condition: &Boolean,
            ) -> Result<(), SynthesisError> {
                for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
                    a.conditional_enforce_equal(
                        &mut cs.ns(|| format!("{} equality check for {}-th bit", $size, i)),
                        b,
                        condition,
                    )?;
                }
                Ok(())
            }

            fn cost() -> usize {
                const MULTIPLIER: usize = if $size == 128 { 128 } else { 8 };

                MULTIPLIER * <Boolean as ConditionalEqGadget<F>>::cost()
            }
        }
    };
}

eq_uint_impl!(UInt8, 8);
eq_uint_impl!(UInt16, 16);
eq_uint_impl!(UInt32, 32);
eq_uint_impl!(UInt64, 64);
eq_uint_impl!(UInt128, 128);
