// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019  BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use super::*;
use sp_core::{H256, *};
use sp_externalities::with_externalities;
use support::{assert_err, assert_ok, impl_outer_origin, weights::Weight};

use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Verify},
	BuildStorage,
};

impl_outer_origin! {
	pub enum Origin for Test {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = ();
	type MaximumBlockWeight = Weight;
	type MaximumBlockLength = ();
	type AvailableBlockRatio = ();
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

impl Trait for Test {
	type Event = ();
	type PublicSigningKey = H256;
	type PublicBoxKey = H256;
}

type DID = Module<Test>;

fn new_test_ext() -> runtime_io::TestExternalities {
	system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.0
		.into()
}

#[test]
fn check_add_did() {
	with_externalities(&mut new_test_ext(), || {
		let pair = ed25519::Pair::from_seed(*b"Alice                           ");
		let signing_key = H256::from_low_u64_be(1);
		let box_key = H256::from_low_u64_be(2);
		let account_hash = H256::from(pair.public().0);
		assert_ok!(DID::add(
			Origin::signed(account_hash.clone()),
			signing_key.clone(),
			box_key.clone(),
			Some(b"http://kilt.org/submit".to_vec())
		));

		assert_eq!(<DIDs<Test>>::exists(account_hash), true);
		let did = {
			let opt = DID::dids(account_hash.clone());
			assert!(opt.is_some());
			opt.unwrap()
		};
		assert_eq!(did.0, signing_key.clone());
		assert_eq!(did.1, box_key.clone());
		assert_eq!(did.2, Some(b"http://kilt.org/submit".to_vec()));

		assert_ok!(DID::remove(Origin::signed(account_hash.clone())));
		assert_eq!(<DIDs<Test>>::exists(account_hash), false);
	});
}
