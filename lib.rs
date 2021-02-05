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

    use flipper::Flipper;
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
    
    #[ink(storage)]
    pub struct Delegator {
        flipper: Lazy<Flipper>,
        tikitaka: Option<Lazy<Tikitaka>>,
    }

    impl Delegator {
        #[ink(constructor)]
        pub fn new(
            init_value: bool,
            flipper_code_hash: Hash
        ) -> Self {
            let total_balance = Self::env().balance();
            let flipper = Flipper::new(init_value)
                .endowment(total_balance / 4)
                .code_hash(flipper_code_hash)
                .instantiate()
                .expect("failed at instantiating the `Flipper` contract");
            Self {
                flipper: Lazy::new(flipper),
                tikitaka: None::<Lazy<Tikitaka>>,
            }
        }

        #[ink(message)]
        pub fn set_tikitaka(&mut self, tikitaka_code_hash: Hash) {
            let total_balance = Self::env().balance();
            let tikitaka = Tikitaka::new(self.flipper.clone())
                .endowment(total_balance / 4)
                .code_hash(tikitaka_code_hash)
                .instantiate()
                .expect("failed at instantiating the `Tikitaka` contract");
            self.tikitaka = Some(Lazy::new(tikitaka));
        }

        /// Returns the accumulator's value.
        #[ink(message)]
        pub fn get(&self) -> bool{
            self.flipper.get()
        }

        /// Delegates the call to either `Adder` or `Subber`.
        #[ink(message)]
        pub fn flipper_flip(&mut self) {
            self.flipper.flip()
        }

        #[ink(message)]
        pub fn tikitaka_flip(&mut self) {
            if let Some(t) = &mut self.tikitaka {
                t.execute()
            }
            // self.tikitaka.execute()
        }
    }
}
