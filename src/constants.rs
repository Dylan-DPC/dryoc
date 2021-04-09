#![allow(missing_docs)]

const fn min(a: usize, b: usize) -> usize {
    [a, b][(a > b) as usize]
}

const SODIUM_SIZE_MAX: usize = min(usize::MAX, u64::MAX as usize);

pub const CRYPTO_SCALARMULT_CURVE25519_BYTES: usize = 32;
pub const CRYPTO_SCALARMULT_CURVE25519_SCALARBYTES: usize = 32;

pub const CRYPTO_SCALARMULT_BYTES: usize = CRYPTO_SCALARMULT_CURVE25519_BYTES;
pub const CRYPTO_SCALARMULT_SCALARBYTES: usize = CRYPTO_SCALARMULT_CURVE25519_SCALARBYTES;

const CRYPTO_BOX_CURVE25519XSALSA20POLY1305_PUBLICKEYBYTES: usize = 32;
const CRYPTO_BOX_CURVE25519XSALSA20POLY1305_SECRETKEYBYTES: usize = 32;
const CRYPTO_BOX_CURVE25519XSALSA20POLY1305_MACBYTES: usize = 16;
const CRYPTO_BOX_CURVE25519XSALSA20POLY1305_NONCEBYTES: usize = 24;
const CRYPTO_BOX_CURVE25519XSALSA20POLY1305_SEEDBYTES: usize = 32;
const CRYPTO_BOX_CURVE25519XSALSA20POLY1305_BEFORENMBYTES: usize = 32;

const CRYPTO_STREAM_XSALSA20_MESSAGEBYTES_MAX: usize = SODIUM_SIZE_MAX;

pub const CRYPTO_BOX_PUBLICKEYBYTES: usize = CRYPTO_BOX_CURVE25519XSALSA20POLY1305_PUBLICKEYBYTES;
pub const CRYPTO_BOX_SECRETKEYBYTES: usize = CRYPTO_BOX_CURVE25519XSALSA20POLY1305_SECRETKEYBYTES;
pub const CRYPTO_BOX_MACBYTES: usize = CRYPTO_BOX_CURVE25519XSALSA20POLY1305_MACBYTES;
pub const CRYPTO_BOX_NONCEBYTES: usize = CRYPTO_BOX_CURVE25519XSALSA20POLY1305_NONCEBYTES;
pub const CRYPTO_BOX_SEEDBYTES: usize = CRYPTO_BOX_CURVE25519XSALSA20POLY1305_SEEDBYTES;
pub const CRYPTO_BOX_BEFORENMBYTES: usize = CRYPTO_BOX_CURVE25519XSALSA20POLY1305_BEFORENMBYTES;

pub const CRYPTO_SECRETBOX_XSALSA20POLY1305_KEYBYTES: usize = 32;
pub const CRYPTO_SECRETBOX_XSALSA20POLY1305_NONCEBYTES: usize = 24;
pub const CRYPTO_SECRETBOX_XSALSA20POLY1305_MACBYTES: usize = 16;
pub const CRYPTO_SECRETBOX_XSALSA20POLY1305_MESSAGEBYTES_MAX: usize = SODIUM_SIZE_MAX;

pub const CRYPTO_SECRETBOX_KEYBYTES: usize = CRYPTO_SECRETBOX_XSALSA20POLY1305_KEYBYTES;
pub const CRYPTO_SECRETBOX_NONCEBYTES: usize = CRYPTO_SECRETBOX_XSALSA20POLY1305_NONCEBYTES;
pub const CRYPTO_SECRETBOX_MACBYTES: usize = CRYPTO_SECRETBOX_XSALSA20POLY1305_MACBYTES;
pub const CRYPTO_SECRETBOX_PRIMITIVE: &str = "xsalsa20poly1305";
pub const CRYPTO_SECRETBOX_MESSAGEBYTES_MAX: usize =
    CRYPTO_SECRETBOX_XSALSA20POLY1305_MESSAGEBYTES_MAX;

pub const CRYPTO_BOX_MESSAGEBYTES_MAX: usize =
    CRYPTO_STREAM_XSALSA20_MESSAGEBYTES_MAX - CRYPTO_BOX_CURVE25519XSALSA20POLY1305_MACBYTES;

pub const CRYPTO_AEAD_XCHACHA20POLY1305_IETF_KEYBYTES: usize = 32;
pub const CRYPTO_AEAD_XCHACHA20POLY1305_IETF_NPUBBYTES: usize = 24;
pub const CRYPTO_AEAD_XCHACHA20POLY1305_IETF_ABYTES: usize = 16;
pub const CRYPTO_AEAD_CHACHA20POLY1305_IETF_MESSAGEBYTES_MAX: usize =
    (64u64 * ((1u64 << 32) - 1u64)) as usize;

pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_KEYBYTES: usize =
    CRYPTO_AEAD_XCHACHA20POLY1305_IETF_KEYBYTES;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_HEADERBYTES: usize =
    CRYPTO_AEAD_XCHACHA20POLY1305_IETF_NPUBBYTES;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_INONCEBYTES: usize = 8;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_COUNTERBYTES: usize = 4;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_ABYTES: usize =
    1 + CRYPTO_AEAD_XCHACHA20POLY1305_IETF_ABYTES;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_MESSAGEBYTES_MAX: usize = min(
    SODIUM_SIZE_MAX - CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_ABYTES,
    (64u64 * ((1u64 << 32) - 2u64)) as usize,
);

pub const CRYPTO_STREAM_CHACHA20_IETF_KEYBYTES: usize = 32;
pub const CRYPTO_STREAM_CHACHA20_IETF_NONCEBYTES: usize = 12;

pub const CRYPTO_CORE_HCHACHA20_INPUTBYTES: usize = 16;
pub const CRYPTO_CORE_HCHACHA20_OUTPUTBYTES: usize = 32;
pub const CRYPTO_CORE_HCHACHA20_KEYBYTES: usize = 32;

pub const CRYPTO_CORE_HSALSA20_OUTPUTBYTES: usize = 32;
pub const CRYPTO_CORE_HSALSA20_INPUTBYTES: usize = 16;
pub const CRYPTO_CORE_HSALSA20_KEYBYTES: usize = 32;
pub const CRYPTO_CORE_HSALSA20_CONSTBYTES: usize = 16;

pub const CRYPTO_SECRETSTREAM_PADBYTES: usize = 8;

pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_TAG_MESSAGE: u8 = 0x00;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_TAG_PUSH: u8 = 0x01;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_TAG_REKEY: u8 = 0x02;
pub const CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_TAG_FINAL: u8 =
    CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_TAG_PUSH
        | CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_TAG_REKEY;

pub const CRYPTO_GENERICHASH_BLAKE2B_BYTES_MIN: usize = 16;
pub const CRYPTO_GENERICHASH_BLAKE2B_BYTES_MAX: usize = 64;
pub const CRYPTO_GENERICHASH_BLAKE2B_BYTES: usize = 32;
pub const CRYPTO_GENERICHASH_BLAKE2B_KEYBYTES_MIN: usize = 16;
pub const CRYPTO_GENERICHASH_BLAKE2B_KEYBYTES_MAX: usize = 64;
pub const CRYPTO_GENERICHASH_BLAKE2B_KEYBYTES: usize = 32;
pub const CRYPTO_GENERICHASH_BLAKE2B_SALTBYTES: usize = 16;
pub const CRYPTO_GENERICHASH_BLAKE2B_PERSONALBYTES: usize = 16;

pub const CRYPTO_GENERICHASH_BYTES: usize = CRYPTO_GENERICHASH_BLAKE2B_BYTES;
pub const CRYPTO_GENERICHASH_KEYBYTES: usize = CRYPTO_GENERICHASH_BLAKE2B_KEYBYTES;
pub const CRYPTO_GENERICHASH_BYTES_MIN: usize = CRYPTO_GENERICHASH_BLAKE2B_BYTES_MIN;
pub const CRYPTO_GENERICHASH_BYTES_MAX: usize = CRYPTO_GENERICHASH_BLAKE2B_BYTES_MAX;
pub const CRYPTO_GENERICHASH_KEYBYTES_MIN: usize = CRYPTO_GENERICHASH_BLAKE2B_KEYBYTES_MIN;
pub const CRYPTO_GENERICHASH_KEYBYTES_MAX: usize = CRYPTO_GENERICHASH_BLAKE2B_KEYBYTES_MAX;

pub const CRYPTO_ONETIMEAUTH_POLY1305_BYTES: usize = 16;
pub const CRYPTO_ONETIMEAUTH_POLY1305_KEYBYTES: usize = 32;

pub const CRYPTO_ONETIMEAUTH_BYTES: usize = CRYPTO_ONETIMEAUTH_POLY1305_BYTES;
pub const CRYPTO_ONETIMEAUTH_KEYBYTES: usize = CRYPTO_ONETIMEAUTH_POLY1305_KEYBYTES;

pub const CRYPTO_AUTH_HMACSHA512256_BYTES: usize = 32;
pub const CRYPTO_AUTH_HMACSHA512256_KEYBYTES: usize = 32;

pub const CRYPTO_AUTH_BYTES: usize = CRYPTO_AUTH_HMACSHA512256_BYTES;
pub const CRYPTO_AUTH_KEYBYTES: usize = CRYPTO_AUTH_HMACSHA512256_KEYBYTES;

pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;
