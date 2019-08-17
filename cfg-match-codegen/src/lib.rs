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

extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{Error, Result};

use crate::generic::GenericArm;

mod arm;
mod generic;

type TokenIter = dyn Iterator<Item = TokenTree>;

#[proc_macro]
pub fn cfg_match_codegen(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match inner(ts.into()) {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn inner(ts: TokenStream) -> Result<TokenStream> {
    println!("input: {}", ts.to_string());
    let mut iter = ts.into_iter().peekable();

    let mut arms = Vec::new();
    let mut el = None;

    while let Some(_) = iter.peek() {
        match GenericArm::read(&mut iter)? {
            GenericArm::Arm(arm) => {
                arms.push(arm.group());
            }
            GenericArm::Else(tt) => {
                el = Some(tt);
            }
        }
    }

    let arm_ts = (0..arms.len()).map(|arm_id| {
        let cond = &arms[arm_id].cond;
        let then = &arms[arm_id].then;
        let other = (0..arms.len())
            .filter(|&i| i != arm_id)
            .map(|arm| &arms[arm].cond);
        quote! {
            #[cfg(all(#cond, not(any(#(#other),*))))] {#then}
        }
    });
    let el_ts = el.iter().map(|body| {
        let conds = arms.iter().map(|arm| &arm.cond);
        quote! {
            #[cfg(not(any(#(#conds),*)))] {#body}
        }
    });

    let ret = quote! {{
        #(#arm_ts)*
        #(#el_ts)*
    }};
    println!("ret: {}", ret.to_string());
    Ok(ret)
}

fn expect_at(ts: &mut TokenIter) -> Result<Ident> {
    let tt = expect_token(ts)?;
    if let TokenTree::Punct(punct) = &tt {
        if punct.as_char() == '@' {
            let tt = expect_token(ts)?;
            if let TokenTree::Ident(ident) = &tt {
                Ok(ident.clone())
            } else {
                Err(Error::new(tt.span(), format!("Expected ident")))
            }
        } else {
            Err(Error::new(punct.span(), ""))
        }
    } else {
        Err(Error::new(tt.span(), "Expected @"))
    }
}

fn expect_token(ts: &mut TokenIter) -> Result<TokenTree> {
    ts.next()
        .ok_or_else(|| Error::new(Span::call_site(), "Unexpected end of stream"))
}
