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

use crate::utilities::{
    boolean::Boolean,
    eq::{ConditionalEqGadget, EqGadget},
    int::*,
    num::Number,
};
use snarkvm_fields::PrimeField;
use snarkvm_r1cs::{errors::SynthesisError, ConstraintSystem};

macro_rules! cond_eq_int_impl {
    ($($gadget: ident),*) => ($(

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
                        &mut cs.ns(|| format!("{} equality check for the {}-th bit", <$gadget as Number>::SIZE, i)),
                        b,
                        condition,
                    )?;
                }

                Ok(())
            }

            fn cost() -> usize {
                <$gadget as Number>::SIZE * <Boolean as ConditionalEqGadget<F>>::cost()
            }
        }
    )*)
}

cond_eq_int_impl!(Int8, Int16, Int32, Int64, Int128);
