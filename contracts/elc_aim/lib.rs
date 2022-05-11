#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod elc_aim {
    // use brush::contracts::{
    //     ownable::*,
    //     // access_control::*,
    // };
    // use brush::modifiers;
    use lending_project::traits::elc_aim::*;
    use ink_storage::{
        traits::SpreadAllocate,
    };
    use ink_lang::codegen::Env;

    // const StabCaller: RoleType = ink_lang::selector_id!("StabCaller");

    #[ink(storage)]
    // #[derive(OwnableStorage)] // , AccessControlStorage)]
    #[derive(SpreadAllocate)]
    pub struct ElcAimContract {
        // #[OwnableStorageField]
        // ownable: OwnableData,
        // #[AccessControlStorageField]
        // access: AccessControlData,

        /// Adjustment cycle: 60000s
        k_cycle: u128,
        /// K value setting caller
        stabilization_cyn: AccountId,
        aim: u128,
        timestamp: u128,
        k: u128,      
    }

    // impl Ownable for ElcAimContract {}
    // impl AccessControl for ElcAimContract {}
    impl ElcAim for ElcAimContract {
        #[ink(message)]
        fn get_elcaim(&self) -> u128 {
            let mut cycle = 0;
            let (mut _elcaim, _k) = (self.aim, self.k);
            let now: u128 = self.env().block_timestamp().into();

            if now > self.timestamp {
                cycle = (now - self.timestamp) / self.k_cycle; 
            }

            for _ in 0..cycle {
                _elcaim = _elcaim * (_k + 1e6 as u128) / 1e6 as u128;
            }
            _elcaim
        }

        #[ink(message)]
        fn update_elcaim(&mut self, _cycle: u128) -> u128 {
            let mut cycle = 0;
            let (mut _elcaim, _k) = (self.aim, self.k);
            let now: u128 = self.env().block_timestamp().into();

            if now > self.timestamp {
                cycle = (now - self.timestamp) / self.k_cycle; 
            }

            if cycle == 0 { return _elcaim }
            if cycle > _cycle && _cycle > 0 { cycle = _cycle }
            for _ in 0..cycle {
                _elcaim = _elcaim * (_k + 1e6 as u128) / 1e6 as u128;
            }

            self.aim = _elcaim;
            self.timestamp += self.k_cycle * cycle;
            _elcaim
        }

        #[ink(message)]
        fn set_k(&mut self, _newk: u128) {
            assert!(self.stabilization_cyn == self.env().caller(), "Only stabilizationCYN can set K");
            self.k = _newk;
        }
    }

    impl ElcAimContract {
        #[ink(constructor)]
        pub fn new(_stabilization_cyn: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut ElcAimContract| {
                instance.k_cycle = 60000;
                instance.stabilization_cyn = _stabilization_cyn;
                instance.aim = 1e8 as u128;
                instance.timestamp = Self::env().block_timestamp().into();
                instance.k = 50;
            })
        }
    }        
}