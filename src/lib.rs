//! `intl_c` — Handl stdlib port (`src/index.hnl`).

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl` — the `intl_c` package entry.
pub const INTL_C_HNL: &str = include_str!("index.hnl");

/// Checkout root of the vendored sdkgen-generated ICU4C bindings
/// (`handl-packages/generated/`); the stdlib module table embeds the
/// adapter sources below via `include_str!`.

/// `../handl-packages/generated/icu-c-linux-x86_64/ucol.hnl` (sdkgen adapter `ucol`).
pub const ICU_UCOL_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/ucol.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/udat.hnl` (sdkgen adapter `udat`).
pub const ICU_UDAT_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/udat.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/ufieldpositer.hnl` (sdkgen adapter `ufieldpositer`).
pub const ICU_UFIELDPOSITER_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/ufieldpositer.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/uformattednumber.hnl` (sdkgen adapter `uformattednumber`).
pub const ICU_UFORMATTEDNUMBER_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/uformattednumber.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/unum.hnl` (sdkgen adapter `unum`).
pub const ICU_UNUM_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/unum.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/unumberformatter.hnl` (sdkgen adapter `unumberformatter`).
pub const ICU_UNUMBERFORMATTER_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/unumberformatter.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/upluralrules.hnl` (sdkgen adapter `upluralrules`).
pub const ICU_UPLURALRULES_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/upluralrules.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/ureldatefmt.hnl` (sdkgen adapter `ureldatefmt`).
pub const ICU_URELDATEFMT_HNL: &str = include_str!("../handl-packages/generated/icu-c-linux-x86_64/ureldatefmt.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/ubrk.hnl` (sdkgen adapter `ubrk`).
pub const ICU_UC_UBRK_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/ubrk.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/uchar.hnl` (sdkgen adapter `uchar`).
pub const ICU_UC_UCHAR_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/uchar.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/uldnames.hnl` (sdkgen adapter `uldnames`).
pub const ICU_UC_ULDNAMES_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/uldnames.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/uloc.hnl` (sdkgen adapter `uloc`).
pub const ICU_UC_ULOC_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/uloc.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/unorm2.hnl` (sdkgen adapter `unorm2`).
pub const ICU_UC_UNORM2_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/unorm2.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/ures.hnl` (sdkgen adapter `ures`).
pub const ICU_UC_URES_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/ures.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/ustring.hnl` (sdkgen adapter `ustring`).
pub const ICU_UC_USTRING_HNL: &str = include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/ustring.hnl");
