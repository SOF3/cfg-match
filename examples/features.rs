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

use cfg_match::cfg_match;

use foo::*;

#[allow(unused)]
mod foo {
    struct One;
    struct Two;
    struct ThreeFour;
    struct NoImpl;
}

cfg_match! {
    #[cfg(feature = "one")] => { pub type Impl = One; },
    #[cfg(feature = "two")] => { pub type Impl = Two; },
    #[cfg(feature = "three")] | #[cfg(feature = "four")] => { pub type Impl = ThreeFour; },
    _ => { pub type Impl = NoImpl; },
}
