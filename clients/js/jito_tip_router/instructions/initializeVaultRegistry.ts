/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type WritableAccount,
} from '@solana/web3.js';
import { JITO_TIP_ROUTER_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const INITIALIZE_VAULT_REGISTRY_DISCRIMINATOR = 1;

export function getInitializeVaultRegistryDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_VAULT_REGISTRY_DISCRIMINATOR);
}

export type InitializeVaultRegistryInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountVaultRegistry extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountAccountPayer extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountVaultRegistry extends string
        ? WritableAccount<TAccountVaultRegistry>
        : TAccountVaultRegistry,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountAccountPayer extends string
        ? WritableAccount<TAccountAccountPayer>
        : TAccountAccountPayer,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeVaultRegistryInstructionData = { discriminator: number };

export type InitializeVaultRegistryInstructionDataArgs = {};

export function getInitializeVaultRegistryInstructionDataEncoder(): Encoder<InitializeVaultRegistryInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({
      ...value,
      discriminator: INITIALIZE_VAULT_REGISTRY_DISCRIMINATOR,
    })
  );
}

export function getInitializeVaultRegistryInstructionDataDecoder(): Decoder<InitializeVaultRegistryInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getInitializeVaultRegistryInstructionDataCodec(): Codec<
  InitializeVaultRegistryInstructionDataArgs,
  InitializeVaultRegistryInstructionData
> {
  return combineCodec(
    getInitializeVaultRegistryInstructionDataEncoder(),
    getInitializeVaultRegistryInstructionDataDecoder()
  );
}

export type InitializeVaultRegistryInput<
  TAccountConfig extends string = string,
  TAccountVaultRegistry extends string = string,
  TAccountNcn extends string = string,
  TAccountAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  config: Address<TAccountConfig>;
  vaultRegistry: Address<TAccountVaultRegistry>;
  ncn: Address<TAccountNcn>;
  accountPayer: Address<TAccountAccountPayer>;
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getInitializeVaultRegistryInstruction<
  TAccountConfig extends string,
  TAccountVaultRegistry extends string,
  TAccountNcn extends string,
  TAccountAccountPayer extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
>(
  input: InitializeVaultRegistryInput<
    TAccountConfig,
    TAccountVaultRegistry,
    TAccountNcn,
    TAccountAccountPayer,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeVaultRegistryInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountVaultRegistry,
  TAccountNcn,
  TAccountAccountPayer,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_TIP_ROUTER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    vaultRegistry: { value: input.vaultRegistry ?? null, isWritable: true },
    ncn: { value: input.ncn ?? null, isWritable: false },
    accountPayer: { value: input.accountPayer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.vaultRegistry),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.accountPayer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeVaultRegistryInstructionDataEncoder().encode({}),
  } as InitializeVaultRegistryInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountVaultRegistry,
    TAccountNcn,
    TAccountAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeVaultRegistryInstruction<
  TProgram extends string = typeof JITO_TIP_ROUTER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    vaultRegistry: TAccountMetas[1];
    ncn: TAccountMetas[2];
    accountPayer: TAccountMetas[3];
    systemProgram: TAccountMetas[4];
  };
  data: InitializeVaultRegistryInstructionData;
};

export function parseInitializeVaultRegistryInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeVaultRegistryInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      vaultRegistry: getNextAccount(),
      ncn: getNextAccount(),
      accountPayer: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeVaultRegistryInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
