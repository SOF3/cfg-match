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

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::{Error, Result};

use crate::{expect_at, expect_token, TokenIter};

#[derive(Clone)]
pub struct Arm {
    pub cond: Vec<TokenStream>,
    pub then: TokenTree,
}

impl Arm {
    pub fn read(ts: &mut TokenIter) -> Result<Self> {
        let mut cond = Vec::new();
        loop {
            let token = expect_at(ts)?;
            match token.to_string().as_str() {
                "COND" => {
                    let tt = expect_token(ts)?;
                    let group = match &tt {
                        TokenTree::Group(group) => group,
                        _ => {
                            return Err(Error::new(tt.span(), "Expected @COND {}"));
                        }
                    };

                    cond.push(group.stream());
                }
                "THEN" => {
                    let then = expect_token(ts)?;
                    return Ok(Arm { cond, then });
                }
                _ => {
                    return Err(Error::new(token.span(), "Expected @COND or @THEN"));
                }
            };
        }
    }

    pub fn group(self) -> ArmGrouped {
        let cond = self.cond.into_iter();
        let cond = quote!(any(#(#cond),*));
        ArmGrouped {
            cond,
            then: self.then,
        }
    }
}

#[derive(Clone)]
pub struct ArmGrouped {
    pub cond: TokenStream,
    pub then: TokenTree,
}
