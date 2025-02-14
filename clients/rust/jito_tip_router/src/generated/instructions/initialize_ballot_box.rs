//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeBallotBox {
<<<<<<< HEAD
      
              
          pub epoch_state: solana_program::pubkey::Pubkey,
          
              
          pub config: solana_program::pubkey::Pubkey,
          
              
          pub ballot_box: solana_program::pubkey::Pubkey,
          
              
          pub ncn: solana_program::pubkey::Pubkey,
          
              
          pub account_payer: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl InitializeBallotBox {
  pub fn instruction(&self, args: InitializeBallotBoxInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: InitializeBallotBoxInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
=======
    pub epoch_marker: solana_program::pubkey::Pubkey,

    pub epoch_state: solana_program::pubkey::Pubkey,

    pub config: solana_program::pubkey::Pubkey,

    pub ballot_box: solana_program::pubkey::Pubkey,

    pub ncn: solana_program::pubkey::Pubkey,

    pub account_payer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeBallotBox {
    pub fn instruction(
        &self,
        args: InitializeBallotBoxInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeBallotBoxInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.epoch_marker,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
            self.epoch_state,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.ballot_box,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ncn,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.account_payer,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = InitializeBallotBoxInstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::JITO_TIP_ROUTER_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeBallotBoxInstructionData {
            discriminator: u8,
            }

impl InitializeBallotBoxInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: 13,
                                }
  }
}

impl Default for InitializeBallotBoxInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeBallotBoxInstructionArgs {
                  pub epoch: u64,
      }


/// Instruction builder for `InitializeBallotBox`.
///
/// ### Accounts:
///
<<<<<<< HEAD
          ///   0. `[]` epoch_state
          ///   1. `[]` config
                ///   2. `[writable]` ballot_box
          ///   3. `[]` ncn
                ///   4. `[writable]` account_payer
                ///   5. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeBallotBoxBuilder {
            epoch_state: Option<solana_program::pubkey::Pubkey>,
                config: Option<solana_program::pubkey::Pubkey>,
                ballot_box: Option<solana_program::pubkey::Pubkey>,
                ncn: Option<solana_program::pubkey::Pubkey>,
                account_payer: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                        epoch: Option<u64>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeBallotBoxBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
=======
///   0. `[]` epoch_marker
///   1. `[]` epoch_state
///   2. `[]` config
///   3. `[writable]` ballot_box
///   4. `[]` ncn
///   5. `[writable]` account_payer
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeBallotBoxBuilder {
    epoch_marker: Option<solana_program::pubkey::Pubkey>,
    epoch_state: Option<solana_program::pubkey::Pubkey>,
    config: Option<solana_program::pubkey::Pubkey>,
    ballot_box: Option<solana_program::pubkey::Pubkey>,
    ncn: Option<solana_program::pubkey::Pubkey>,
    account_payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    epoch: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeBallotBoxBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn epoch_marker(&mut self, epoch_marker: solana_program::pubkey::Pubkey) -> &mut Self {
        self.epoch_marker = Some(epoch_marker);
        self
    }
    #[inline(always)]
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
    pub fn epoch_state(&mut self, epoch_state: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.epoch_state = Some(epoch_state);
                    self
    }
            #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.config = Some(config);
                    self
    }
            #[inline(always)]
    pub fn ballot_box(&mut self, ballot_box: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.ballot_box = Some(ballot_box);
                    self
    }
            #[inline(always)]
    pub fn ncn(&mut self, ncn: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.ncn = Some(ncn);
                    self
    }
            #[inline(always)]
    pub fn account_payer(&mut self, account_payer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.account_payer = Some(account_payer);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = Some(epoch);
        self
<<<<<<< HEAD
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = InitializeBallotBox {
                              epoch_state: self.epoch_state.expect("epoch_state is not set"),
                                        config: self.config.expect("config is not set"),
                                        ballot_box: self.ballot_box.expect("ballot_box is not set"),
                                        ncn: self.ncn.expect("ncn is not set"),
                                        account_payer: self.account_payer.expect("account_payer is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
          let args = InitializeBallotBoxInstructionArgs {
                                                              epoch: self.epoch.clone().expect("epoch is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `initialize_ballot_box` CPI accounts.
  pub struct InitializeBallotBoxCpiAccounts<'a, 'b> {
          
                    
              pub epoch_state: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub config: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub ballot_box: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub ncn: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub account_payer: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `initialize_ballot_box` CPI instruction.
pub struct InitializeBallotBoxCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub epoch_state: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub config: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub ballot_box: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub ncn: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub account_payer: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
=======
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitializeBallotBox {
            epoch_marker: self.epoch_marker.expect("epoch_marker is not set"),
            epoch_state: self.epoch_state.expect("epoch_state is not set"),
            config: self.config.expect("config is not set"),
            ballot_box: self.ballot_box.expect("ballot_box is not set"),
            ncn: self.ncn.expect("ncn is not set"),
            account_payer: self.account_payer.expect("account_payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = InitializeBallotBoxInstructionArgs {
            epoch: self.epoch.clone().expect("epoch is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_ballot_box` CPI accounts.
pub struct InitializeBallotBoxCpiAccounts<'a, 'b> {
    pub epoch_marker: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ballot_box: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub account_payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_ballot_box` CPI instruction.
pub struct InitializeBallotBoxCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_marker: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub ballot_box: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub account_payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
    pub __args: InitializeBallotBoxInstructionArgs,
  }

impl<'a, 'b> InitializeBallotBoxCpi<'a, 'b> {
<<<<<<< HEAD
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: InitializeBallotBoxCpiAccounts<'a, 'b>,
              args: InitializeBallotBoxInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              epoch_state: accounts.epoch_state,
              config: accounts.config,
              ballot_box: accounts.ballot_box,
              ncn: accounts.ncn,
              account_payer: accounts.account_payer,
              system_program: accounts.system_program,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
=======
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeBallotBoxCpiAccounts<'a, 'b>,
        args: InitializeBallotBoxInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            epoch_marker: accounts.epoch_marker,
            epoch_state: accounts.epoch_state,
            config: accounts.config,
            ballot_box: accounts.ballot_box,
            ncn: accounts.ncn,
            account_payer: accounts.account_payer,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.epoch_marker.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
            *self.epoch_state.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.ballot_box.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ncn.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.account_payer.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = InitializeBallotBoxInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::JITO_TIP_ROUTER_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.epoch_state.clone());
                        account_infos.push(self.config.clone());
                        account_infos.push(self.ballot_box.clone());
                        account_infos.push(self.ncn.clone());
                        account_infos.push(self.account_payer.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

<<<<<<< HEAD
    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
=======
        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JITO_TIP_ROUTER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.epoch_marker.clone());
        account_infos.push(self.epoch_state.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.ballot_box.clone());
        account_infos.push(self.ncn.clone());
        account_infos.push(self.account_payer.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
    }
  }
}

/// Instruction builder for `InitializeBallotBox` via CPI.
///
/// ### Accounts:
///
<<<<<<< HEAD
          ///   0. `[]` epoch_state
          ///   1. `[]` config
                ///   2. `[writable]` ballot_box
          ///   3. `[]` ncn
                ///   4. `[writable]` account_payer
          ///   5. `[]` system_program
=======
///   0. `[]` epoch_marker
///   1. `[]` epoch_state
///   2. `[]` config
///   3. `[writable]` ballot_box
///   4. `[]` ncn
///   5. `[writable]` account_payer
///   6. `[]` system_program
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
#[derive(Clone, Debug)]
pub struct InitializeBallotBoxCpiBuilder<'a, 'b> {
  instruction: Box<InitializeBallotBoxCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeBallotBoxCpiBuilder<'a, 'b> {
<<<<<<< HEAD
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(InitializeBallotBoxCpiBuilderInstruction {
      __program: program,
              epoch_state: None,
              config: None,
              ballot_box: None,
              ncn: None,
              account_payer: None,
              system_program: None,
                                            epoch: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn epoch_state(&mut self, epoch_state: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.epoch_state = Some(epoch_state);
                    self
    }
      #[inline(always)]
    pub fn config(&mut self, config: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.config = Some(config);
                    self
=======
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeBallotBoxCpiBuilderInstruction {
            __program: program,
            epoch_marker: None,
            epoch_state: None,
            config: None,
            ballot_box: None,
            ncn: None,
            account_payer: None,
            system_program: None,
            epoch: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn epoch_marker(
        &mut self,
        epoch_marker: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.epoch_marker = Some(epoch_marker);
        self
    }
    #[inline(always)]
    pub fn epoch_state(
        &mut self,
        epoch_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.epoch_state = Some(epoch_state);
        self
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
    }
      #[inline(always)]
    pub fn ballot_box(&mut self, ballot_box: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.ballot_box = Some(ballot_box);
                    self
    }
      #[inline(always)]
    pub fn ncn(&mut self, ncn: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.ncn = Some(ncn);
                    self
    }
      #[inline(always)]
    pub fn account_payer(&mut self, account_payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.account_payer = Some(account_payer);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.instruction.epoch = Some(epoch);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = InitializeBallotBoxInstructionArgs {
                                                              epoch: self.instruction.epoch.clone().expect("epoch is not set"),
                                    };
        let instruction = InitializeBallotBoxCpi {
<<<<<<< HEAD
        __program: self.instruction.__program,
                  
          epoch_state: self.instruction.epoch_state.expect("epoch_state is not set"),
                  
          config: self.instruction.config.expect("config is not set"),
                  
          ballot_box: self.instruction.ballot_box.expect("ballot_box is not set"),
                  
          ncn: self.instruction.ncn.expect("ncn is not set"),
                  
          account_payer: self.instruction.account_payer.expect("account_payer is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
=======
            __program: self.instruction.__program,

            epoch_marker: self
                .instruction
                .epoch_marker
                .expect("epoch_marker is not set"),

            epoch_state: self
                .instruction
                .epoch_state
                .expect("epoch_state is not set"),

            config: self.instruction.config.expect("config is not set"),

            ballot_box: self.instruction.ballot_box.expect("ballot_box is not set"),

            ncn: self.instruction.ncn.expect("ncn is not set"),

            account_payer: self
                .instruction
                .account_payer
                .expect("account_payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
}

#[derive(Clone, Debug)]
struct InitializeBallotBoxCpiBuilderInstruction<'a, 'b> {
<<<<<<< HEAD
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            epoch_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                ballot_box: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                ncn: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                account_payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        epoch: Option<u64>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
=======
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    epoch_marker: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ballot_box: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    account_payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
}

