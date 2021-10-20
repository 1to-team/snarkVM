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

use snarkvm_fields::Field;
use snarkvm_r1cs::{errors::SynthesisError, ConstraintSystem};

use crate::integers::uint::UInt8;

// LE
pub trait ToBytesLEGadget<F: Field> {
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError>;

    /// Additionally checks if the produced list of booleans is 'valid'.
    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError>;
}

impl<'a, F: Field, T: 'a + ToBytesLEGadget<F>> ToBytesLEGadget<F> for &'a T {
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        (*self).to_bytes_le(cs)
    }

    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.to_bytes_le(cs)
    }
}

impl<'a, F: Field, T: 'a + ToBytesLEGadget<F>> ToBytesLEGadget<F> for &'a [T] {
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_le(cs.ns(|| format!("to_bytes_le_{}", i)))?);
        }
        Ok(vec)
    }

    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_le_strict(cs.ns(|| format!("to_bytes_le_{}", i)))?);
        }
        Ok(vec)
    }
}

impl<F: Field, T: ToBytesLEGadget<F>> ToBytesLEGadget<F> for [T] {
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_le(cs.ns(|| format!("to_bytes_le_{}", i)))?);
        }
        Ok(vec)
    }

    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_le_strict(cs.ns(|| format!("to_bytes_le_{}", i)))?);
        }
        Ok(vec)
    }
}

impl<F: Field, T: ToBytesLEGadget<F>> ToBytesLEGadget<F> for Vec<T> {
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_le(cs.ns(|| format!("to_bytes_le_{}", i)))?);
        }
        Ok(vec)
    }

    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_le_strict(cs.ns(|| format!("to_bytes_le_{}", i)))?);
        }
        Ok(vec)
    }
}

// todo (@collinc97): uncomment or remove these
// impl<F: Field> ToBytesLEGadget<F> for [UInt8] {
//     fn to_bytes_le<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         Ok(self.to_vec())
//     }
//
//     fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         self.to_bytes_le(cs)
//     }
// }
//
// impl<'a, F: Field> ToBytesLEGadget<F> for &'a [UInt8] {
//     fn to_bytes_le<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         Ok(self.to_vec())
//     }
//
//     fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         self.to_bytes_le(cs)
//     }
// }
//
// impl<F: Field> ToBytesLEGadget<F> for Vec<UInt8> {
//     fn to_bytes_le<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         Ok(self.to_vec())
//     }
//
//     fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         self.to_bytes_le(cs)
//     }
// }

// BE
pub trait ToBytesBEGadget<F: Field> {
    fn to_bytes_be<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError>;

    /// Additionally checks if the produced list of boobeans is 'valid'.
    fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError>;
}

impl<'a, F: Field, T: 'a + ToBytesBEGadget<F>> ToBytesBEGadget<F> for &'a T {
    fn to_bytes_be<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        (*self).to_bytes_be(cs)
    }

    fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.to_bytes_be(cs)
    }
}

impl<'a, F: Field, T: 'a + ToBytesBEGadget<F>> ToBytesBEGadget<F> for &'a [T] {
    fn to_bytes_be<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_be(cs.ns(|| format!("to_bytes_be_{}", i)))?);
        }
        Ok(vec)
    }

    fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_be_strict(cs.ns(|| format!("to_bytes_be_{}", i)))?);
        }
        Ok(vec)
    }
}

impl<F: Field, T: ToBytesBEGadget<F>> ToBytesBEGadget<F> for [T] {
    fn to_bytes_be<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_be(cs.ns(|| format!("to_bytes_be_{}", i)))?);
        }
        Ok(vec)
    }

    fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_be_strict(cs.ns(|| format!("to_bytes_be_{}", i)))?);
        }
        Ok(vec)
    }
}

impl<F: Field, T: ToBytesBEGadget<F>> ToBytesBEGadget<F> for Vec<T> {
    fn to_bytes_be<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_be(cs.ns(|| format!("to_bytes_be_{}", i)))?);
        }
        Ok(vec)
    }

    fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let mut vec = vec![];
        for (i, elem) in self.iter().enumerate() {
            vec.append(&mut elem.to_bytes_be_strict(cs.ns(|| format!("to_bytes_be_{}", i)))?);
        }
        Ok(vec)
    }
}

// todo (@collinc97): uncomment or remove these
// Implement gadgets for byte array methods
// impl<F: Field> ToBytesBEGadget<F> for [UInt8] {
//     fn to_bytes_be<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         Ok(self.to_vec())
//     }
//
//     fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         self.to_bytes_be(cs)
//     }
// }
//
// impl<'a, F: Field> ToBytesBEGadget<F> for &'a [UInt8] {
//     fn to_bytes_be<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         let mut vec = self.to_vec();
//         vec.reverse();
//         Ok(vec)
//     }
//
//     fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         self.to_bytes_be(cs)
//     }
// }
//
// impl<F: Field> ToBytesBEGadget<F> for Vec<UInt8> {
//     fn to_bytes_be<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         let mut vec = self.to_vec();
//         vec.reverse();
//         Ok(vec)
//     }
//
//     fn to_bytes_be_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
//         self.to_bytes_be(cs)
//     }
// }
