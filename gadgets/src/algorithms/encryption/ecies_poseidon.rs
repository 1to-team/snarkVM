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

use crate::{
    algorithms::crypto_hash::{CryptographicSpongeVar, PoseidonSpongeGadget},
    AllocGadget,
    Boolean,
    ConditionalEqGadget,
    EncryptionGadget,
    EqGadget,
    FieldGadget,
    FpGadget,
    GroupGadget,
    Integer,
    ToBitsLEGadget,
    ToBytesLEGadget,
    UInt8,
};
use itertools::Itertools;
use snarkvm_algorithms::{crypto_hash::PoseidonDefaultParametersField, encryption::ECIESPoseidonEncryption};
use snarkvm_curves::{
    templates::twisted_edwards_extended::{Affine as TEAffine, Projective as TEProjective},
    AffineCurve,
    Group,
    ProjectiveCurve,
    TwistedEdwardsParameters,
};
use snarkvm_fields::{FieldParameters, PrimeField};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};
use snarkvm_utilities::{borrow::Borrow, to_bytes_le, ToBytes};

use anyhow::Result;
use std::marker::PhantomData;

type TEAffineGadget<TE, F> = crate::curves::templates::twisted_edwards::AffineGadget<TE, F, FpGadget<F>>;

/// ECIES encryption private key gadget
#[derive(Derivative)]
#[derivative(
    Clone(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    PartialEq(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    Eq(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    Debug(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField")
)]
pub struct ECIESPoseidonEncryptionPrivateKeyGadget<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField>(
    pub Vec<UInt8>,
    PhantomData<TE>,
    PhantomData<F>,
);

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> AllocGadget<TE::ScalarField, F>
    for ECIESPoseidonEncryptionPrivateKeyGadget<TE, F>
{
    fn alloc_constant<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<TE::ScalarField>,
        CS: ConstraintSystem<F>,
    >(
        _cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let private_key = to_bytes_le![value_gen()?.borrow()].unwrap();
        Ok(ECIESPoseidonEncryptionPrivateKeyGadget(
            UInt8::constant_vec(&private_key),
            PhantomData,
            PhantomData,
        ))
    }

    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<TE::ScalarField>, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let private_key = to_bytes_le![value_gen()?.borrow()].unwrap();
        let bytes = UInt8::alloc_vec(cs.ns(|| "allocate the private key as bytes"), &private_key)?;
        Ok(ECIESPoseidonEncryptionPrivateKeyGadget(bytes, PhantomData, PhantomData))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<TE::ScalarField>, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let private_key = to_bytes_le![value_gen()?.borrow()].unwrap();
        let bytes = UInt8::alloc_input_vec_le(cs.ns(|| "allocate the private key as bytes"), &private_key)?;
        Ok(ECIESPoseidonEncryptionPrivateKeyGadget(bytes, PhantomData, PhantomData))
    }
}

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> ToBytesLEGadget<F>
    for ECIESPoseidonEncryptionPrivateKeyGadget<TE, F>
{
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.to_bytes_le(&mut cs.ns(|| "to_bytes_le"))
    }

    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.to_bytes_le_strict(&mut cs.ns(|| "to_bytes_le_strict"))
    }
}

/// ECIES encryption randomness gadget
#[derive(Derivative)]
#[derivative(
    Clone(bound = "TE: TwistedEdwardsParameters"),
    PartialEq(bound = "TE: TwistedEdwardsParameters"),
    Eq(bound = "TE: TwistedEdwardsParameters"),
    Debug(bound = "TE: TwistedEdwardsParameters")
)]
pub struct ECIESPoseidonEncryptionRandomnessGadget<TE: TwistedEdwardsParameters>(pub Vec<UInt8>, PhantomData<TE>);

impl<TE: TwistedEdwardsParameters> AllocGadget<TE::ScalarField, TE::BaseField>
    for ECIESPoseidonEncryptionRandomnessGadget<TE>
where
    TE::BaseField: PrimeField,
{
    fn alloc_constant<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<TE::ScalarField>,
        CS: ConstraintSystem<TE::BaseField>,
    >(
        _cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let randomness = to_bytes_le![value_gen()?.borrow()].unwrap();
        Ok(ECIESPoseidonEncryptionRandomnessGadget(
            UInt8::constant_vec(&randomness),
            PhantomData,
        ))
    }

    fn alloc<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<TE::ScalarField>,
        CS: ConstraintSystem<TE::BaseField>,
    >(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let randomness = to_bytes_le![value_gen()?.borrow()].unwrap();
        Ok(ECIESPoseidonEncryptionRandomnessGadget(
            UInt8::alloc_vec(cs, &randomness)?,
            PhantomData,
        ))
    }

    fn alloc_input<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<TE::ScalarField>,
        CS: ConstraintSystem<TE::BaseField>,
    >(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let randomness = to_bytes_le![value_gen()?.borrow()].unwrap();
        Ok(ECIESPoseidonEncryptionRandomnessGadget(
            UInt8::alloc_input_vec_le(cs, &randomness)?,
            PhantomData,
        ))
    }
}

/// ECIES Poseidon encryption gadget
#[derive(Derivative)]
#[derivative(
    Clone(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    PartialEq(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    Eq(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    Debug(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField")
)]
pub struct ECIESPoseidonEncryptionGadget<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField>
where
    TE::BaseField: PoseidonDefaultParametersField,
{
    encryption: ECIESPoseidonEncryption<TE>,
    f_phantom: PhantomData<F>,
}

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> AllocGadget<ECIESPoseidonEncryption<TE>, F>
    for ECIESPoseidonEncryptionGadget<TE, F>
where
    TE::BaseField: PoseidonDefaultParametersField,
{
    fn alloc_constant<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<ECIESPoseidonEncryption<TE>>,
        CS: ConstraintSystem<F>,
    >(
        _cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        Ok(Self {
            encryption: (*value_gen()?.borrow()).clone(),
            f_phantom: PhantomData,
        })
    }

    fn alloc<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<ECIESPoseidonEncryption<TE>>,
        CS: ConstraintSystem<F>,
    >(
        _cs: CS,
        _value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        unimplemented!()
    }

    fn alloc_input<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<ECIESPoseidonEncryption<TE>>,
        CS: ConstraintSystem<F>,
    >(
        _cs: CS,
        _value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        unimplemented!()
    }
}

/// ECIES Poseidon public key gadget
#[derive(Derivative)]
#[derivative(
    Clone(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    PartialEq(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    Eq(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField"),
    Debug(bound = "TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField")
)]
pub struct ECIESPoseidonEncryptionPublicKeyGadget<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField>(
    TEAffineGadget<TE, F>,
);

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> AllocGadget<TEAffine<TE>, F>
    for ECIESPoseidonEncryptionPublicKeyGadget<TE, F>
where
    TEAffineGadget<TE, F>: GroupGadget<TEAffine<TE>, F>,
{
    fn alloc_constant<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<TEAffine<TE>>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let public_key = value_gen()?.borrow().clone();
        Ok(Self(TEAffineGadget::<TE, F>::alloc_constant(cs, || Ok(public_key))?))
    }

    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<TEAffine<TE>>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let public_key = value_gen()?.borrow().clone();
        Ok(Self(TEAffineGadget::<TE, F>::alloc_checked(cs, || Ok(public_key))?))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<TEAffine<TE>>, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let point = if let Ok(pk) = value_gen() {
            pk.borrow().clone()
        } else {
            TEAffine::<TE>::default()
        };

        let x_coordinate_gadget =
            FpGadget::<TE::BaseField>::alloc_input(cs.ns(|| "input x coordinate"), || Ok(point.x))?;
        let allocated_gadget =
            TEAffineGadget::<TE, F>::alloc_checked(cs.ns(|| "input the allocated point"), || Ok(point.clone()))?;

        allocated_gadget
            .x
            .enforce_equal(cs.ns(|| "check x consistency"), &x_coordinate_gadget)?;

        Ok(Self(allocated_gadget))
    }
}

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> ConditionalEqGadget<F>
    for ECIESPoseidonEncryptionPublicKeyGadget<TE, F>
{
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        self.0.conditional_enforce_equal(cs, &other.0, condition)
    }

    fn cost() -> usize {
        <TEAffineGadget<TE, F> as ConditionalEqGadget<F>>::cost()
    }
}

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> ToBytesLEGadget<F>
    for ECIESPoseidonEncryptionPublicKeyGadget<TE, F>
{
    fn to_bytes_le<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.x.to_bytes_le(cs)
    }

    fn to_bytes_le_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.x.to_bytes_le_strict(cs)
    }
}

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField> EqGadget<F>
    for ECIESPoseidonEncryptionPublicKeyGadget<TE, F>
{
}

impl<TE: TwistedEdwardsParameters<BaseField = F>, F: PrimeField + PoseidonDefaultParametersField>
    EncryptionGadget<ECIESPoseidonEncryption<TE>, F> for ECIESPoseidonEncryptionGadget<TE, F>
{
    type PrivateKeyGadget = ECIESPoseidonEncryptionPrivateKeyGadget<TE, F>;
    type PublicKeyGadget = ECIESPoseidonEncryptionPublicKeyGadget<TE, F>;
    type RandomnessGadget = ECIESPoseidonEncryptionRandomnessGadget<TE>;

    fn check_public_key_gadget<CS: ConstraintSystem<TE::BaseField>>(
        &self,
        mut cs: CS,
        private_key: &Self::PrivateKeyGadget,
    ) -> Result<Self::PublicKeyGadget, SynthesisError> {
        let private_key_bits = private_key.0.iter().flat_map(|b| b.to_bits_le()).collect::<Vec<_>>();
        let mut public_key = <TEAffineGadget<TE, F> as GroupGadget<TEAffine<TE>, F>>::zero(cs.ns(|| "zero"))?;

        let num_powers = private_key_bits.len();

        let generator_powers: Vec<TEAffine<TE>> = {
            let mut generator_powers = Vec::new();
            let mut generator = self.encryption.generator.into_projective();
            for _ in 0..num_powers {
                generator_powers.push(generator.clone());
                generator.double_in_place();
            }
            TEProjective::<TE>::batch_normalization(&mut generator_powers);
            generator_powers.into_iter().map(|v| v.into()).collect()
        };

        public_key.scalar_multiplication(
            cs.ns(|| "check_public_key_gadget"),
            private_key_bits.iter().zip_eq(&generator_powers),
        )?;

        Ok(ECIESPoseidonEncryptionPublicKeyGadget::<TE, F> { 0: public_key })
    }

    fn check_encryption_gadget<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        randomness: &Self::RandomnessGadget,
        public_key: &Self::PublicKeyGadget,
        message: &[UInt8],
    ) -> Result<Vec<UInt8>, SynthesisError> {
        let affine_zero: TEAffineGadget<TE, F> =
            <TEAffineGadget<TE, F> as GroupGadget<TEAffine<TE>, F>>::zero(cs.ns(|| "affine zero")).unwrap();

        // Compute the ECDH value.
        let randomness_bits = randomness.0.iter().flat_map(|b| b.to_bits_le()).collect::<Vec<_>>();
        let ecdh_value = <TEAffineGadget<TE, F> as GroupGadget<TEAffine<TE>, F>>::mul_bits(
            &public_key.0,
            cs.ns(|| "compute_ecdh_value"),
            &affine_zero,
            randomness_bits.iter().copied(),
        )?;

        // Prepare the sponge.
        let params =
            <TE::BaseField as PoseidonDefaultParametersField>::get_default_poseidon_parameters(4, false).unwrap();
        let mut sponge = PoseidonSpongeGadget::<TE::BaseField>::new(cs.ns(|| "sponge"), &params);
        sponge.absorb(cs.ns(|| "absorb"), [ecdh_value.x].iter())?;

        // Squeeze one element for the commitment randomness.
        let commitment_randomness =
            sponge.squeeze_field_elements(cs.ns(|| "squeeze field elements for polyMAC"), 1)?[0].clone();

        // Add a commitment to the public key.
        let public_key_commitment = {
            let mut sponge =
                PoseidonSpongeGadget::<TE::BaseField>::new(cs.ns(|| "sponge for public key commitment"), &params);
            sponge.absorb(
                cs.ns(|| "absorb for public key commitment"),
                [commitment_randomness, public_key.0.x.clone()].iter(),
            )?;
            sponge.squeeze_field_elements(cs.ns(|| "squeeze for public key commitment"), 1)?[0].clone()
        };

        // Convert the message into bits.
        let mut bits = Vec::with_capacity(message.len() * 8);
        for byte in message.iter() {
            bits.extend_from_slice(&byte.to_bits_le());
        }
        bits.push(Boolean::Constant(true));
        // The last bit indicates the end of the actual data, which is used in decoding to
        // make sure that the length is correct.

        // Pack the bits into field elements.
        let capacity = <<TE::BaseField as PrimeField>::Parameters as FieldParameters>::CAPACITY as usize;
        let mut res = Vec::with_capacity((bits.len() + capacity - 1) / capacity);
        for (i, chunk) in bits.chunks(capacity).enumerate() {
            res.push(Boolean::le_bits_to_fp_var(
                cs.ns(|| format!("convert a bit to a field element {}", i)),
                chunk,
            )?);
        }

        // Obtain random field elements from Poseidon.
        let sponge_field_elements =
            sponge.squeeze_field_elements(cs.ns(|| "squeeze for random elements"), res.len())?;

        // Add the random field elements to the packed bits.
        for (i, sponge_field_element) in sponge_field_elements.iter().enumerate() {
            res[i].add_in_place(
                cs.ns(|| format!("add the sponge field element {}", i)),
                sponge_field_element,
            )?;
        }

        let res_bytes = res.to_bytes(cs.ns(|| "convert the masked results into bytes"))?;

        // Put the bytes of the x coordinate of the randomness group element
        // into the beginning of the ciphertext.
        let generator_gadget = TEAffineGadget::<TE, F>::alloc_constant(cs.ns(|| "alloc generator"), || {
            Ok(self.encryption.generator.clone())
        })?;
        let randomness_elem = <TEAffineGadget<TE, F> as GroupGadget<TEAffine<TE>, F>>::mul_bits(
            &generator_gadget,
            cs.ns(|| "compute the randomness element"),
            &affine_zero,
            randomness_bits.iter().copied(),
        )?;
        let random_elem_bytes = randomness_elem
            .x
            .to_bytes(cs.ns(|| "convert the randomness element to bytes"))?;
        let public_key_commitment_bytes =
            public_key_commitment.to_bytes(cs.ns(|| "convert the public key commitment to bytes"))?;

        Ok([random_elem_bytes, public_key_commitment_bytes, res_bytes].concat())
    }
}
