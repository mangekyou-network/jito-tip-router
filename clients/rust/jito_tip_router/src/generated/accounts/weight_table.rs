//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use solana_program::pubkey::Pubkey;
use crate::generated::types::VaultEntry;
use crate::generated::types::WeightEntry;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightTable {
pub discriminator: u64,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub ncn: Pubkey,
pub epoch: u64,
pub slot_created: u64,
pub vault_count: u64,
pub bump: u8,
#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
pub reserved: [u8; 128],
#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
pub vault_registry: [VaultEntry; 64],
#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
pub table: [WeightEntry; 64],
}


impl WeightTable {
  
  
  
  #[inline(always)]
  pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
    let mut data = data;
    Self::deserialize(&mut data)
  }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for WeightTable {
  type Error = std::io::Error;

  fn try_from(account_info: &solana_program::account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
      let mut data: &[u8] = &(*account_info.data).borrow();
      Self::deserialize(&mut data)
  }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for WeightTable {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
      Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for WeightTable {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for WeightTable {
    fn owner() -> Pubkey {
      crate::JITO_TIP_ROUTER_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for WeightTable {}


#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for WeightTable {
<<<<<<< HEAD
  const DISCRIMINATOR: [u8; 8] = [0; 8];
=======
    const DISCRIMINATOR: &'static [u8] = &[0; 8];
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
}

