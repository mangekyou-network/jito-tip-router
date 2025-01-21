/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  type Codec,
  type Decoder,
  type Encoder,
  type ReadonlyUint8Array,
} from '@solana/web3.js';

export type Progress = {
  tally: bigint;
  total: bigint;
  reserved: ReadonlyUint8Array;
};

export type ProgressArgs = {
  tally: number | bigint;
  total: number | bigint;
  reserved: ReadonlyUint8Array;
};

export function getProgressEncoder(): Encoder<ProgressArgs> {
  return getStructEncoder([
    ['tally', getU64Encoder()],
    ['total', getU64Encoder()],
    ['reserved', fixEncoderSize(getBytesEncoder(), 32)],
  ]);
}

export function getProgressDecoder(): Decoder<Progress> {
  return getStructDecoder([
    ['tally', getU64Decoder()],
    ['total', getU64Decoder()],
    ['reserved', fixDecoderSize(getBytesDecoder(), 32)],
  ]);
}

export function getProgressCodec(): Codec<ProgressArgs, Progress> {
  return combineCodec(getProgressEncoder(), getProgressDecoder());
}
