// cfg-match
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro2::TokenTree;
use syn::{Error, Result};

use crate::arm::Arm;
use crate::{expect_at, expect_token, TokenIter};

pub enum GenericArm {
    Arm(Arm),
    Else(TokenTree),
}

impl GenericArm {
    pub fn read(ts: &mut TokenIter) -> Result<Self> {
        let token = expect_at(ts)?;
        Ok(match token.to_string().as_str() {
            "ARM" => GenericArm::Arm(Arm::read(ts)?),
            "ELSE" => GenericArm::Else(expect_token(ts)?),
            _ => {
                return Err(Error::new(token.span(), "Expected @ARM or @ELSE"));
            }
        })
    }
}
