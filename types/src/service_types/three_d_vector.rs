// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
#![rustfmt::skip]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ThreeDVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BinaryEncoder<ThreeDVector> for ThreeDVector {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.x.byte_len();
        size += self.y.byte_len();
        size += self.z.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.x.encode(stream)?;
        size += self.y.encode(stream)?;
        size += self.z.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let x = f64::decode(stream, decoding_limits)?;
        let y = f64::decode(stream, decoding_limits)?;
        let z = f64::decode(stream, decoding_limits)?;
        Ok(ThreeDVector {
            x,
            y,
            z,
        })
    }
}
