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

use super::*;
use crate::GroupType;
// This macro can be more efficient once concat_idents is merged in.
macro_rules! from_bytes_impl {
    ($le_function_name:ident, $le_constant_name:ident, $le_constant_value:literal, $le_call:expr, $be_function_name:ident, $be_constant_name:ident, $be_constant_value:literal, $be_call:expr, $num_of_expected_bytes:literal) => {
        pub const $le_constant_name: &str = $le_constant_value;

        impl<'a, F: PrimeField, G: GroupType<F>> EvaluatorState<'a, F, G> {
            pub fn $le_function_name<CS: ConstraintSystem<F>>(
                &mut self,
                arguments: &[ConstrainedValue<F, G>],
                cs: &mut CS,
            ) -> Result<ConstrainedValue<F, G>> {
                let arg = match arguments.get(0) {
                    None => Err(anyhow!("illegal `from_bytes_le` call, expected 1 argument")),
                    Some(value) => Ok(value),
                }?;

                let bytes = unwrap_u8_array_argument(arg, $num_of_expected_bytes, "from_bytes_le")?;

                let cs = self.cs(cs);

                $le_call(&bytes, cs)
            }
        }

        pub const $be_constant_name: &str = $be_constant_value;

        impl<'a, F: PrimeField, G: GroupType<F>> EvaluatorState<'a, F, G> {
            pub fn $be_function_name<CS: ConstraintSystem<F>>(
                &mut self,
                arguments: &[ConstrainedValue<F, G>],
                cs: &mut CS,
            ) -> Result<ConstrainedValue<F, G>> {
                let arg = match arguments.get(0) {
                    None => Err(anyhow!("illegal `from_bytes_be` call, expected 1 argument")),
                    Some(value) => Ok(value),
                }?;

                let bytes = unwrap_u8_array_argument(arg, $num_of_expected_bytes, "from_bytes_be")?;

                let cs = self.cs(cs);

                $be_call(&bytes, cs)
            }
        }
    };
}

from_bytes_impl!(
    call_core_address_from_bytes_le,
    ADDRESS_FROM_BYTES_LE_CORE,
    "address_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!(
            "the type `address` does not implement the from_bytes_le method"
        ))
    },
    call_core_address_from_bytes_be,
    ADDRESS_FROM_BYTES_BE_CORE,
    "address_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!(
            "the type `address` does not implement the from_bytes_be method"
        ))
    },
    32
);
from_bytes_impl!(
    call_core_bool_from_bytes_le,
    BOOL_FROM_BYTES_LE_CORE,
    "bool_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `bool` does not implement the from_bytes_le method"))
    },
    call_core_bool_from_bytes_be,
    BOOL_FROM_BYTES_BE_CORE,
    "bool_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `bool` does not implement the from_bytes_be method"))
    },
    1
);
from_bytes_impl!(
    call_core_char_from_bytes_le,
    CHAR_FROM_BYTES_LE_CORE,
    "char_from_bytes_le",
    |_bytes: &[UInt8], _| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `char` does not implement the from_bytes_le method"))
    },
    call_core_char_from_bytes_be,
    CHAR_FROM_BYTES_BE_CORE,
    "char_from_bytes_be",
    |_bytes: &[UInt8], _| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `char` does not implement the from_bytes_be method"))
    },
    32
);
from_bytes_impl!(
    call_core_field_from_bytes_le,
    FIELD_FROM_BYTES_LE_CORE,
    "field_from_bytes_le",
    |_bytes: &[UInt8], _| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `field` does not implement the from_bytes_le method"))
    },
    call_core_field_from_bytes_be,
    FIELD_FROM_BYTES_BE_CORE,
    "field_from_bytes_be",
    |_bytes: &[UInt8], _| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `field` does not implement the from_bytes_be method"))
    },
    32
);
from_bytes_impl!(
    call_core_group_from_bytes_le,
    GROUP_FROM_BYTES_LE_CORE,
    "group_from_bytes_le",
    |_bytes: &[UInt8], _| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `group` does not implement the from_bytes_le method"))
    },
    call_core_group_from_bytes_be,
    GROUP_FROM_BYTES_BE_CORE,
    "group_from_bytes_be",
    |_bytes: &[UInt8], _| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `group` does not implement the from_bytes_be method"))
    },
    64
);
from_bytes_impl!(
    call_core_i8_from_bytes_le,
    I8_FROM_BYTES_LE_CORE,
    "i8_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i8` does not implement the from_bytes_le method"))
    },
    call_core_i8_from_bytes_be,
    I8_FROM_BYTES_BE_CORE,
    "i8_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i8` does not implement the from_bytes_be method"))
    },
    1
);
from_bytes_impl!(
    call_core_i16_from_bytes_le,
    I16_FROM_BYTES_LE_CORE,
    "i16_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i16` does not implement the from_bytes_le method"))
    },
    call_core_i16_from_bytes_be,
    I16_FROM_BYTES_BE_CORE,
    "i16_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i16` does not implement the from_bytes_be method"))
    },
    2
);
from_bytes_impl!(
    call_core_i32_from_bytes_le,
    I32_FROM_BYTES_LE_CORE,
    "i32_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i32` does not implement the from_bytes_le method"))
    },
    call_core_i32_from_bytes_be,
    I32_FROM_BYTES_BE_CORE,
    "i32_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i32` does not implement the from_bytes_be method"))
    },
    4
);
from_bytes_impl!(
    call_core_i64_from_bytes_le,
    I64_FROM_BYTES_LE_CORE,
    "i64_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i64` does not implement the from_bytes_le method"))
    },
    call_core_i64_from_bytes_be,
    I64_FROM_BYTES_BE_CORE,
    "i64_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i64` does not implement the from_bytes_be method"))
    },
    8
);
from_bytes_impl!(
    call_core_i128_from_bytes_le,
    I128_FROM_BYTES_LE_CORE,
    "i128_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i128` does not implement the from_bytes_le method"))
    },
    call_core_i128_from_bytes_be,
    I128_FROM_BYTES_BE_CORE,
    "i128_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `i128` does not implement the from_bytes_be method"))
    },
    16
);
from_bytes_impl!(
    call_core_u8_from_bytes_le,
    U8_FROM_BYTES_LE_CORE,
    "u8_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u8` does not implement the from_bytes_le method"))
    },
    call_core_u8_from_bytes_be,
    U8_FROM_BYTES_BE_CORE,
    "u8_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u8` does not implement the from_bytes_be method"))
    },
    1
);
from_bytes_impl!(
    call_core_u16_from_bytes_le,
    U16_FROM_BYTES_LE_CORE,
    "u16_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u16` does not implement the from_bytes_le method"))
    },
    call_core_u16_from_bytes_be,
    U16_FROM_BYTES_BE_CORE,
    "u16_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u16` does not implement the from_bytes_be method"))
    },
    2
);
from_bytes_impl!(
    call_core_u32_from_bytes_le,
    U32_FROM_BYTES_LE_CORE,
    "u32_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u32` does not implement the from_bytes_le method"))
    },
    call_core_u32_from_bytes_be,
    U32_FROM_BYTES_BE_CORE,
    "u32_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u32` does not implement the from_bytes_be method"))
    },
    4
);
from_bytes_impl!(
    call_core_u64_from_bytes_le,
    U64_FROM_BYTES_LE_CORE,
    "u64_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u64` does not implement the from_bytes_le method"))
    },
    call_core_u64_from_bytes_be,
    U64_FROM_BYTES_BE_CORE,
    "u64_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u64` does not implement the from_bytes_be method"))
    },
    8
);
from_bytes_impl!(
    call_core_u128_from_bytes_le,
    U128_FROM_BYTES_LE_CORE,
    "u128_from_bytes_le",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u128` does not implement the from_bytes_le method"))
    },
    call_core_u128_from_bytes_be,
    U128_FROM_BYTES_BE_CORE,
    "u128_from_bytes_be",
    |_bytes: &[UInt8], _cs| -> Result<ConstrainedValue<F, G>> {
        Err(anyhow!("the type `u128` does not implement the from_bytes_be method"))
    },
    16
);
