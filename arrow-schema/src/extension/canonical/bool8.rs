// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! 8-bit Boolean
//!
//! <https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>

use crate::{extension::ExtensionType, ArrowError, DataType};

/// The extension type for `8-bit Boolean`.
///
/// Extension name: `arrow.bool8`.
///
/// The storage type of the extension is `Int8` where:
/// - false is denoted by the value 0.
/// - true can be specified using any non-zero value. Preferably 1.
///
/// <https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Bool8;

impl ExtensionType for Bool8 {
    const NAME: &str = "arrow.bool8";

    type Metadata = &'static str;

    fn metadata(&self) -> &Self::Metadata {
        &""
    }

    fn serialize_metadata(&self) -> Option<String> {
        Some(String::default())
    }

    fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError> {
        const ERR: &str = "Bool8 extension type expects an empty string as metadata";
        metadata.map_or_else(
            || Err(ArrowError::InvalidArgumentError(ERR.to_owned())),
            |value| match value {
                "" => Ok(""),
                _ => Err(ArrowError::InvalidArgumentError(ERR.to_owned())),
            },
        )
    }

    fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError> {
        match data_type {
            DataType::Int8 => Ok(()),
            data_type => Err(ArrowError::InvalidArgumentError(format!(
                "Bool8 data type mismatch, expected Int8, found {data_type}"
            ))),
        }
    }

    fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self, ArrowError> {
        Self.supports_data_type(data_type).map(|_| Self)
    }
}
