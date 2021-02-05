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

use ink_lang as ink;

#[ink::contract]
mod delegator {
    use ink_storage::{
        // traits::{
        //     PackedLayout,
        //     SpreadLayout,
        // },
        Lazy,
    };

    // use flipper::Flipper;
    use tikitaka::Tikitaka;

    // #[derive(
    //     Debug,
    //     Clone,
    //     PartialEq,
    //     Eq,
    //     scale::Encode,
    //     scale::Decode,
    //     // SpreadLayout,
    //     PackedLayout,
    // )]
    // #[cfg_attr(
    //     feature = "std",
    //     derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    // )]

    /// Delegates calls to an adder or subber contract to mutate
    /// a value in an accumulator contract.
    ///
    /// In order to deploy the delegator smart contract we first
    /// have to manually put the code of the accumulator, adder
    /// and subber smart contracts, receive their code hashes from
    /// the signalled events and put their code hash into our
    /// delegator smart contract.
    
    #[derive(Clone)]
    #[ink(storage)]
    pub struct Delegator {
        // flipper: Lazy<Flipper>,
        tikitaka: Option<Lazy<Tikitaka>>,
        init_value: bool,
    }

    impl Delegator {
        #[ink(constructor)]
        pub fn new(
            init_value: bool,
            // version: u32,
        ) -> Self {
            Self {
                tikitaka: None::<Lazy<Tikitaka>>,
                init_value
            }
        }

        #[ink(message)]
        pub fn set_tikitaka(&mut self, tikitaka_code_hash: Hash) {
            let total_balance = Self::env().balance();
            // let salt = version.to_le_bytes();
            let tikitaka = Tikitaka::new(self.flip)
                .endowment(total_balance / 4)
                .code_hash(tikitaka_code_hash)
                .instantiate()
                .expect("failed at instantiating the `Tikitaka` contract");
            self.tikitaka = Some(tikitaka);
        }

        /// Returns the accumulator's value.
        #[ink(message)]
        pub fn get(&self) -> bool{
            self.init_value
        }

        /// Delegates the call to either `Adder` or `Subber`.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.init_value = !self.init_value
        }
    }
}
