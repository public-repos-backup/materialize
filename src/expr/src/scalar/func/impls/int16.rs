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
    #[sqlname = "-"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(NegInt16)]
    #[is_monotone = true]
    fn neg_int16(a: i16) -> Result<i16, EvalError> {
        a.checked_neg()
            .ok_or_else(|| EvalError::Int16OutOfRange(a.to_string().into()))
    }
);

sqlfunc!(
    #[sqlname = "~"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(BitNotInt16)]
    fn bit_not_int16(a: i16) -> i16 {
        !a
    }
);

sqlfunc!(
    #[sqlname = "abs"]
    fn abs_int16(a: i16) -> Result<i16, EvalError> {
        a.checked_abs()
            .ok_or_else(|| EvalError::Int16OutOfRange(a.to_string().into()))
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_real"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastFloat32ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_float32(a: i16) -> f32 {
        f32::from(a)
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_double"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastFloat64ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_float64(a: i16) -> f64 {
        f64::from(a)
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_integer"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastInt32ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_int32(a: i16) -> i32 {
        i32::from(a)
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_bigint"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastInt64ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_int64(a: i16) -> i64 {
        i64::from(a)
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_text"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastStringToInt16)]
    fn cast_int16_to_string(a: i16) -> String {
        let mut buf = String::new();
        strconv::format_int16(&mut buf, a);
        buf
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_uint2"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastUint16ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_uint16(a: i16) -> Result<u16, EvalError> {
        u16::try_from(a).or_else(|_| Err(EvalError::UInt16OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_uint4"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastUint32ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_uint32(a: i16) -> Result<u32, EvalError> {
        u32::try_from(a).or_else(|_| Err(EvalError::UInt32OutOfRange(a.to_string().into())))
    }
);

sqlfunc!(
    #[sqlname = "smallint_to_uint8"]
    #[preserves_uniqueness = true]
    #[inverse = to_unary!(super::CastUint64ToInt16)]
    #[is_monotone = true]
    fn cast_int16_to_uint64(a: i16) -> Result<u64, EvalError> {
        u64::try_from(a).or_else(|_| Err(EvalError::UInt64OutOfRange(a.to_string().into())))
    }
);

#[derive(Ord, PartialOrd, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash, MzReflect)]
pub struct CastInt16ToNumeric(pub Option<NumericMaxScale>);

impl<'a> EagerUnaryFunc<'a> for CastInt16ToNumeric {
    type Input = i16;
    type Output = Result<Numeric, EvalError>;

    fn call(&self, a: i16) -> Result<Numeric, EvalError> {
        let mut a = Numeric::from(i32::from(a));
        if let Some(scale) = self.0 {
            if numeric::rescale(&mut a, scale.into_u8()).is_err() {
                return Err(EvalError::NumericFieldOverflow);
            }
        }
        Ok(a)
    }

    fn output_type(&self, input: ColumnType) -> ColumnType {
        ScalarType::Numeric { max_scale: self.0 }.nullable(input.nullable)
    }

    fn could_error(&self) -> bool {
        self.0.is_some()
    }

    fn inverse(&self) -> Option<crate::UnaryFunc> {
        to_unary!(super::CastNumericToInt16)
    }

    fn is_monotone(&self) -> bool {
        true
    }
}

impl fmt::Display for CastInt16ToNumeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("smallint_to_numeric")
    }
}
