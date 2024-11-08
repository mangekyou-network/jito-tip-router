/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU128Decoder,
  getU128Encoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
} from '@solana/web3.js';

export type WeightEntry = {
  mint: Address;
  weight: bigint;
  slotSet: bigint;
  slotUpdated: bigint;
  reserved: Array<number>;
};

export type WeightEntryArgs = {
  mint: Address;
  weight: number | bigint;
  slotSet: number | bigint;
  slotUpdated: number | bigint;
  reserved: Array<number>;
};

export function getWeightEntryEncoder(): Encoder<WeightEntryArgs> {
  return getStructEncoder([
    ['mint', getAddressEncoder()],
    ['weight', getU128Encoder()],
    ['slotSet', getU64Encoder()],
    ['slotUpdated', getU64Encoder()],
    ['reserved', getArrayEncoder(getU8Encoder(), { size: 128 })],
  ]);
}

export function getWeightEntryDecoder(): Decoder<WeightEntry> {
  return getStructDecoder([
    ['mint', getAddressDecoder()],
    ['weight', getU128Decoder()],
    ['slotSet', getU64Decoder()],
    ['slotUpdated', getU64Decoder()],
    ['reserved', getArrayDecoder(getU8Decoder(), { size: 128 })],
  ]);
}

export function getWeightEntryCodec(): Codec<WeightEntryArgs, WeightEntry> {
  return combineCodec(getWeightEntryEncoder(), getWeightEntryDecoder());
}
