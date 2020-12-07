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
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    string::UAString,
    date_time::DateTime,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub message: UAString,
    pub user_name: UAString,
    pub annotation_time: DateTime,
}

impl MessageInfo for Annotation {
    fn object_id(&self) -> ObjectId {
        ObjectId::Annotation_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<Annotation> for Annotation {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.message.byte_len();
        size += self.user_name.byte_len();
        size += self.annotation_time.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.message.encode(stream)?;
        size += self.user_name.encode(stream)?;
        size += self.annotation_time.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let message = UAString::decode(stream, decoding_limits)?;
        let user_name = UAString::decode(stream, decoding_limits)?;
        let annotation_time = DateTime::decode(stream, decoding_limits)?;
        Ok(Annotation {
            message,
            user_name,
            annotation_time,
        })
    }
}
