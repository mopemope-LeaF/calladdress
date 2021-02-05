// Copyright 2018-2021 Parity Technologies (UK) Ltd.
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

#![cfg_attr(not(feature = "std"), no_std)]

pub use self::tikitaka::Tikitaka;
use ink_lang as ink;

#[ink::contract]
pub mod tikitaka {

    // use delegator::Delegator;
    use flipper::Flipper;

    #[ink(storage)]
    pub struct Tikitaka {
        flipper: flipper::Flipper,
    }

    impl Tikitaka {
        #[ink(constructor)]
        pub fn new(flipper: Flipper) -> Self {
            Self { 
               flipper,
            }
        }

        #[ink(message)]
        pub fn execute(&mut self) {
            self.flipper.flip()
        }
    }
}
