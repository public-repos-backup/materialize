// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use std::fmt;

use mz_lowertest::MzReflect;
use mz_repr::adt::numeric::{self, Numeric, NumericMaxScale};
use mz_repr::{ColumnType, ScalarType, strconv};
use serde::{Deserialize, Serialize};

use crate::EvalError;
use crate::scalar::func::EagerUnaryFunc;

sqlfunc!(
    #[sqlname = "~"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::BitNotUint64)]
    fn bit_not_uint64(a: u64) -> u64 {
        !a
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_real"]
    #[preserves_uniqueness = false]
    #[inverse = to_unary!(super::CastFloat32ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_float32(a: u64) -> f32 {
        // TODO(benesch): remove potentially dangerous usage of `as`.
        #[allow(clippy::as_conversions)]
        {
            a as f32
        }
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_double"]
    #[preserves_uniqueness = false]
    #[inverse = to_unary!(super::CastFloat64ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_float64(a: u64) -> f64 {
        // TODO(benesch): remove potentially dangerous usage of `as`.
        #[allow(clippy::as_conversions)]
        {
            a as f64
        }
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_uint2"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastUint16ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_uint16(a: u64) -> Result<u16, EvalError> {
        u16::try_from(a).or_else(|_| Err(EvalError::UInt16OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_uint4"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastUint32ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_uint32(a: u64) -> Result<u32, EvalError> {
        u32::try_from(a).or_else(|_| Err(EvalError::UInt32OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_smallint"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastInt16ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_int16(a: u64) -> Result<i16, EvalError> {
        i16::try_from(a).or_else(|_| Err(EvalError::Int16OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_integer"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastInt32ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_int32(a: u64) -> Result<i32, EvalError> {
        i32::try_from(a).or_else(|_| Err(EvalError::Int32OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_bigint"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastInt64ToUint64)]
    #[is_monotone = true]
    fn cast_uint64_to_int64(a: u64) -> Result<i64, EvalError> {
        i64::try_from(a).or_else(|_| Err(EvalError::Int64OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "uint8_to_text"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastStringToUint64)]
    fn cast_uint64_to_string(a: u64) -> String {
        let mut buf = String::new();
        strconv::format_uint64(&mut buf, a);
        buf
    }
);

#[derive(Ord, PartialOrd, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash, MzReflect)]
pub struct CastUint64ToNumeric(pub Option<NumericMaxScale>);

impl<'a> EagerUnaryFunc<'a> for CastUint64ToNumeric {
    type Input = u64;
    type Output = Result<Numeric, EvalError>;

    fn call(&self, a: u64) -> Result<Numeric, EvalError> {
        let mut a = Numeric::from(a);
        if let Some(scale) = self.0 {
            if numeric::rescale(&mut a, scale.into_u8()).is_err() {
                return Err(EvalError::NumericFieldOverflow);
            }
        }
        // Besides `rescale`, cast is infallible.
        Ok(a)
    }

    fn output_type(&self, input: ColumnType) -> ColumnType {
        ScalarType::Numeric { max_scale: self.0 }.nullable(input.nullable)
    }

    fn could_error(&self) -> bool {
        self.0.is_some()
    }

    fn inverse(&self) -> Option<crate::UnaryFunc> {
        to_unary!(super::CastNumericToUint64)
    }

    fn is_monotone(&self) -> bool {
        true
    }
}

impl fmt::Display for CastUint64ToNumeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("uint8_to_numeric")
    }
}
