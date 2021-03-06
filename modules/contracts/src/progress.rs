// Copyright 2020 by Trinkler Software AG (Switzerland).
// This file is part of Katal Chain.
//
// Katal Chain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal Chain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use super::*;

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn progress(event: Event, contract_id: H256) -> Result {
        // Getting the contract.
        let mut contract = <Self as Store>::Contracts::get(contract_id);

        // Calculating the resulting contract contract.
        let mut payoff = Real::from(0);
        match contract.terms.contract_type {
            Some(ContractType::PAM) => {
                let result = Self::progress_pam(event, contract)?;
                payoff = result.0;
                contract = result.1;
            }
            _ => {
                Err("Contract type not supported")?;
            }
        }

        // Executing the payoff.
        // Note: not sure if those unwrap() will not panic.
        // TODO: Real is Option<i64> but use generic_asset T::Balance
        if payoff >= Real::from(0) {
            <assets::Module<T>>::transfer(
                contract.terms.counterparty_id.unwrap(),
                contract.terms.creator_id.unwrap(),
                contract.terms.settlement_currency.unwrap(),
                payoff.abs(),
            )?;
        } else {
            <assets::Module<T>>::transfer(
                contract.terms.creator_id.unwrap(),
                contract.terms.counterparty_id.unwrap(),
                contract.terms.settlement_currency.unwrap(),
                payoff.abs(),
            )?;
        }

        // TODO: Set contract performance variable to something other than `Performant`

        // Storing the contract contract.
        <Self as Store>::Contracts::insert(contract_id, contract);

        // Return Ok if successful.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::H256;
    // The testing primitives are very useful for avoiding having to work with signatures
    // or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
    use sr_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use support::{assert_ok, impl_outer_origin, parameter_types};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: u32 = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::one();
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type Call = ();
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type AvailableBlockRatio = AvailableBlockRatio;
        type MaximumBlockLength = MaximumBlockLength;
        type Version = ();
    }

    pub const MILLISECS_PER_BLOCK: u64 = 6000;
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
    parameter_types! {
        pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
    }
    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = MinimumPeriod;
    }
    impl oracle::Trait for Test {}
    impl assets::Trait for Test {}
    impl Trait for Test {}
    type Assets = assets::Module<Test>;
    type Contracts = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn progress_works() {
        new_test_ext().execute_with(|| {
            let t0 = Time::from_values(2015, 01, 01, 00, 00, 00);
            let id = H256::random();
            let creator_id = H256::random();
            let counterparty_id = H256::random();
            let currency = 1;
            let mut terms = Terms::new(id);
            terms.contract_deal_date = Time::from_values(2015, 01, 01, 00, 00, 00);
            terms.contract_id = id;
            terms.contract_role = Some(ContractRole::RPA);
            terms.contract_type = Some(ContractType::PAM);
            terms.counterparty_id = Some(counterparty_id);
            terms.creator_id = Some(creator_id);
            terms.settlement_currency = Some(currency);
            terms.currency = Some(currency);
            terms.day_count_convention = Some(DayCountConvention::_30E360);
            terms.initial_exchange_date = Time::from_values(2015, 01, 02, 00, 00, 00);
            terms.maturity_date = Time::from_values(2015, 04, 02, 00, 00, 00);
            terms.nominal_interest_rate = Real::from(0);
            terms.notional_principal = Real::from(1000);
            terms.premium_discount_at_ied = Real::from(-5);
            terms.rate_spread = Real::from(0);
            terms.scaling_effect = None;
            Assets::mint(creator_id, currency, Real::from(1000));
            Assets::mint(counterparty_id, currency, Real::from(1000));
            let mut contract = Contracts::deploy_pam(t0, terms).unwrap();
            <Contracts as Store>::Contracts::insert(id, contract.clone());
            assert_eq!(
                contract.schedule[0],
                Event::new(Time::from_values(2015, 01, 02, 00, 00, 00), EventType::IED)
            );
            Contracts::progress(contract.schedule[0], id);
            contract = <Contracts as Store>::Contracts::get(id);
            assert_eq!(contract.states.notional_principal, Real::from(1000));
            assert_eq!(contract.states.nominal_interest_rate, Real::from(0));
            assert_eq!(contract.states.accrued_interest, Real::from(0));
            assert_eq!(Assets::balances((currency, creator_id)), Real::from(5));
            assert_eq!(
                Assets::balances((currency, counterparty_id)),
                Real::from(1995)
            );
            // Event 3 is being used, instead of the next in the sequence 1, because the
            // given test vectors don't mention event 1 (probably because it has no effect
            // on the contract).
            assert_eq!(
                contract.schedule[3],
                Event::new(Time::from_values(2015, 04, 02, 00, 00, 00), EventType::MD)
            );
            Contracts::progress(contract.schedule[3], id);
            contract = <Contracts as Store>::Contracts::get(id);
            assert_eq!(contract.states.notional_principal, Real::from(0));
            assert_eq!(contract.states.nominal_interest_rate, Real::from(0));
            assert_eq!(contract.states.accrued_interest, Real::from(0));
            assert_eq!(Assets::balances((currency, creator_id)), Real::from(1005));
            assert_eq!(
                Assets::balances((currency, counterparty_id)),
                Real::from(995)
            );
        });
    }
}
