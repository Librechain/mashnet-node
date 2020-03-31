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

use crate::AccountSignature;
use codec::Encode;
use sp_core::{ed25519 as x25519, H256, *};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Verify},
	Perbill,
};
use support::{assert_err, assert_ok, impl_outer_origin, parameter_types, weights::Weight};

impl_outer_origin! {
	pub enum Origin for Test {}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1_000_000_000;
	pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = <AccountSignature as Verify>::Signer;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

impl ctype::Trait for Test {
	type Event = ();
}

impl error::Trait for Test {
	type ErrorCode = u16;
	type Event = ();
}

impl delegation::Trait for Test {
	type Event = ();
	type Signature = x25519::Signature;
	type Signer = <x25519::Signature as Verify>::Signer;
	type DelegationNodeId = H256;
}

impl Trait for Test {
	type Event = ();
}

type Attestation = Module<Test>;
type CType = ctype::Module<Test>;
type Delegation = delegation::Module<Test>;

fn new_test_ext() -> runtime_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

fn hash_to_u8<T: Encode>(hash: T) -> Vec<u8> {
	hash.encode()
}

#[test]
fn check_add_attestation() {
	new_test_ext().execute_with(|| {
		let pair = x25519::Pair::from_seed(&*b"Alice                           ");
		let hash = H256::from_low_u64_be(1);
		let account_hash = pair.public();
		assert_ok!(CType::add(
			Origin::signed(account_hash.clone()),
			hash.clone()
		));
		assert_ok!(Attestation::add(
			Origin::signed(account_hash.clone()),
			hash.clone(),
			hash.clone(),
			None
		));
		let existing_attestation_for_claim = {
			let opt = Attestation::attestations(hash.clone());
			assert!(opt.is_some());
			opt.unwrap()
		};
		assert_eq!(existing_attestation_for_claim.0, hash.clone());
		assert_eq!(existing_attestation_for_claim.1, account_hash);
		assert_eq!(existing_attestation_for_claim.3, false);
	});
}

#[test]
fn check_revoke_attestation() {
	new_test_ext().execute_with(|| {
		let pair = x25519::Pair::from_seed(&*b"Alice                           ");
		let hash = H256::from_low_u64_be(1);
		let account_hash = pair.public();
		assert_ok!(CType::add(
			Origin::signed(account_hash.clone()),
			hash.clone()
		));
		assert_ok!(Attestation::add(
			Origin::signed(account_hash.clone()),
			hash.clone(),
			hash.clone(),
			None
		));
		assert_ok!(Attestation::revoke(
			Origin::signed(account_hash.clone()),
			hash.clone()
		));
		let existing_attestation_for_claim = {
			let opt = Attestation::attestations(hash.clone());
			assert!(opt.is_some());
			opt.unwrap()
		};
		assert_eq!(existing_attestation_for_claim.0, hash.clone());
		assert_eq!(existing_attestation_for_claim.1, account_hash);
		assert_eq!(existing_attestation_for_claim.3, true);
	});
}

#[test]
fn check_double_attestation() {
	new_test_ext().execute_with(|| {
		let pair = x25519::Pair::from_seed(&*b"Alice                           ");
		let hash = H256::from_low_u64_be(1);
		let account_hash = pair.public();
		assert_ok!(CType::add(
			Origin::signed(account_hash.clone()),
			hash.clone()
		));
		assert_ok!(Attestation::add(
			Origin::signed(account_hash.clone()),
			hash.clone(),
			hash.clone(),
			None
		));
		assert_err!(
			Attestation::add(
				Origin::signed(account_hash),
				hash.clone(),
				hash.clone(),
				None
			),
			Attestation::ERROR_ALREADY_ATTESTED.1
		);
	});
}

#[test]
fn check_double_revoke_attestation() {
	new_test_ext().execute_with(|| {
		let pair = x25519::Pair::from_seed(&*b"Alice                           ");
		let hash = H256::from_low_u64_be(1);
		let account_hash = pair.public();
		assert_ok!(CType::add(
			Origin::signed(account_hash.clone()),
			hash.clone()
		));
		assert_ok!(Attestation::add(
			Origin::signed(account_hash.clone()),
			hash.clone(),
			hash.clone(),
			None
		));
		assert_ok!(Attestation::revoke(
			Origin::signed(account_hash.clone()),
			hash.clone()
		));
		assert_err!(
			Attestation::revoke(Origin::signed(account_hash), hash.clone()),
			Attestation::ERROR_ALREADY_REVOKED.1
		);
	});
}

#[test]
fn check_revoke_unknown() {
	new_test_ext().execute_with(|| {
		let pair = x25519::Pair::from_seed(&*b"Alice                           ");
		let hash = H256::from_low_u64_be(1);
		let account_hash = pair.public();
		assert_err!(
			Attestation::revoke(Origin::signed(account_hash), hash.clone()),
			Attestation::ERROR_ATTESTATION_NOT_FOUND.1
		);
	});
}

#[test]
fn check_revoke_not_permitted() {
	new_test_ext().execute_with(|| {
		let pair_alice = x25519::Pair::from_seed(&*b"Alice                           ");
		let account_hash_alice = pair_alice.public();
		let pair_bob = x25519::Pair::from_seed(&*b"Bob                             ");
		let account_hash_bob = pair_bob.public();
		let hash = H256::from_low_u64_be(1);
		assert_ok!(CType::add(
			Origin::signed(account_hash_alice.clone()),
			hash.clone()
		));
		assert_ok!(Attestation::add(
			Origin::signed(account_hash_alice),
			hash.clone(),
			hash.clone(),
			None
		));
		assert_err!(
			Attestation::revoke(Origin::signed(account_hash_bob), hash.clone()),
			Attestation::ERROR_NOT_PERMITTED_TO_REVOKE_ATTESTATION.1
		);
	});
}

#[test]
fn check_add_attestation_with_delegation() {
	new_test_ext().execute_with(|| {
		let pair_alice = x25519::Pair::from_seed(&*b"Alice                           ");
		let account_hash_alice = pair_alice.public();
		let pair_bob = x25519::Pair::from_seed(&*b"Bob                             ");
		let account_hash_bob = pair_bob.public();
		let pair_charlie = x25519::Pair::from_seed(&*b"Charlie                         ");
		let account_hash_charlie = pair_charlie.public();

		let ctype_hash = H256::from_low_u64_be(1);
		let other_ctype_hash = H256::from_low_u64_be(2);
		let claim_hash = H256::from_low_u64_be(1);

		let delegation_root = H256::from_low_u64_be(0);
		let delegation_1 = H256::from_low_u64_be(1);
		let delegation_2 = H256::from_low_u64_be(2);

		assert_ok!(CType::add(
			Origin::signed(account_hash_alice.clone()),
			ctype_hash.clone()
		));

		assert_err!(
			Attestation::add(
				Origin::signed(account_hash_alice.clone()),
				claim_hash.clone(),
				ctype_hash.clone(),
				Some(delegation_1)
			),
			Delegation::ERROR_DELEGATION_NOT_FOUND.1
		);

		assert_ok!(Delegation::create_root(
			Origin::signed(account_hash_alice.clone()),
			delegation_root.clone(),
			ctype_hash.clone()
		));
		assert_ok!(Delegation::add_delegation(
			Origin::signed(account_hash_alice.clone()),
			delegation_1.clone(),
			delegation_root.clone(),
			None,
			account_hash_bob.clone(),
			delegation::Permissions::DELEGATE,
			pair_bob.sign(&hash_to_u8(Delegation::calculate_hash(
				delegation_1.clone(),
				delegation_root.clone(),
				None,
				delegation::Permissions::DELEGATE
			)))
		));
		assert_ok!(Delegation::add_delegation(
			Origin::signed(account_hash_alice.clone()),
			delegation_2.clone(),
			delegation_root.clone(),
			None,
			account_hash_bob.clone(),
			delegation::Permissions::ATTEST,
			pair_bob.sign(&hash_to_u8(Delegation::calculate_hash(
				delegation_2.clone(),
				delegation_root.clone(),
				None,
				delegation::Permissions::ATTEST
			)))
		));

		assert_err!(
			Attestation::add(
				Origin::signed(account_hash_bob.clone()),
				claim_hash.clone(),
				other_ctype_hash.clone(),
				Some(delegation_2)
			),
			CType::ERROR_CTYPE_NOT_FOUND.1
		);
		assert_ok!(CType::add(
			Origin::signed(account_hash_alice.clone()),
			other_ctype_hash.clone()
		));
		assert_err!(
			Attestation::add(
				Origin::signed(account_hash_bob.clone()),
				claim_hash.clone(),
				other_ctype_hash.clone(),
				Some(delegation_2)
			),
			Attestation::ERROR_CTYPE_OF_DELEGATION_NOT_MATCHING.1
		);
		assert_err!(
			Attestation::add(
				Origin::signed(account_hash_alice.clone()),
				claim_hash.clone(),
				ctype_hash.clone(),
				Some(delegation_2)
			),
			Attestation::ERROR_NOT_DELEGATED_TO_ATTESTER.1
		);
		assert_err!(
			Attestation::add(
				Origin::signed(account_hash_bob.clone()),
				claim_hash.clone(),
				ctype_hash.clone(),
				Some(delegation_1)
			),
			Attestation::ERROR_DELEGATION_NOT_AUTHORIZED_TO_ATTEST.1
		);
		assert_ok!(Attestation::add(
			Origin::signed(account_hash_bob.clone()),
			claim_hash.clone(),
			ctype_hash.clone(),
			Some(delegation_2)
		));

		let existing_attestations_for_delegation =
			Attestation::delegated_attestations(delegation_2.clone());
		assert_eq!(existing_attestations_for_delegation.len(), 1);
		assert_eq!(existing_attestations_for_delegation[0], claim_hash.clone());

		assert_ok!(Delegation::revoke_root(
			Origin::signed(account_hash_alice.clone()),
			delegation_root.clone()
		));
		assert_err!(
			Attestation::add(
				Origin::signed(account_hash_bob),
				claim_hash.clone(),
				ctype_hash.clone(),
				Some(delegation_2)
			),
			Attestation::ERROR_DELEGATION_REVOKED.1
		);

		assert_err!(
			Attestation::revoke(Origin::signed(account_hash_charlie), claim_hash.clone()),
			Attestation::ERROR_NOT_PERMITTED_TO_REVOKE_ATTESTATION.1
		);
		assert_ok!(Attestation::revoke(
			Origin::signed(account_hash_alice),
			claim_hash.clone()
		));
	});
}
