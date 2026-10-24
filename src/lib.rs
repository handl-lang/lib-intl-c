//! `intl_c` — Handl stdlib port (`src/index.hnl`).

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl` — the `intl_c` package entry.
pub const INTL_C_HNL: &str = include_str!("index.hnl");

/// Checkout root of the vendored sdkgen-generated ICU4C bindings
/// (`handl-packages/generated/`); the stdlib module table embeds the
/// port and adapter sources below via `include_str!`.
///
/// Naming: `ICU_<MOD>_HNL` is the *port* spec (`<lib>/<mod>.hnl` — the
/// contract, FFI signatures); `ICU_<MOD>_C_HNL` is the C *adapter*
/// (`<lib>-c-linux-x86_64/<mod>_c.hnl` — the impl). The `_c` split keeps
/// the adapter's own `from "../<lib>/<mod>" import` resolving the port
/// instead of basename-colliding back onto itself.

// ── icu (libicui18n) ports ────────────────────────────────────────────

/// `../handl-packages/generated/icu/ucol.hnl` (sdkgen port `ucol`).
pub const ICU_UCOL_HNL: &str = include_str!("../handl-packages/generated/icu/ucol.hnl");

/// `../handl-packages/generated/icu/udat.hnl` (sdkgen port `udat`).
pub const ICU_UDAT_HNL: &str = include_str!("../handl-packages/generated/icu/udat.hnl");

/// `../handl-packages/generated/icu/ufieldpositer.hnl` (sdkgen port `ufieldpositer`).
pub const ICU_UFIELDPOSITER_HNL: &str =
    include_str!("../handl-packages/generated/icu/ufieldpositer.hnl");

/// `../handl-packages/generated/icu/uformattednumber.hnl` (sdkgen port `uformattednumber`).
pub const ICU_UFORMATTEDNUMBER_HNL: &str =
    include_str!("../handl-packages/generated/icu/uformattednumber.hnl");

/// `../handl-packages/generated/icu/unum.hnl` (sdkgen port `unum`).
pub const ICU_UNUM_HNL: &str = include_str!("../handl-packages/generated/icu/unum.hnl");

/// `../handl-packages/generated/icu/unumberformatter.hnl` (sdkgen port `unumberformatter`).
pub const ICU_UNUMBERFORMATTER_HNL: &str =
    include_str!("../handl-packages/generated/icu/unumberformatter.hnl");

/// `../handl-packages/generated/icu/upluralrules.hnl` (sdkgen port `upluralrules`).
pub const ICU_UPLURALRULES_HNL: &str =
    include_str!("../handl-packages/generated/icu/upluralrules.hnl");

/// `../handl-packages/generated/icu/ureldatefmt.hnl` (sdkgen port `ureldatefmt`).
pub const ICU_URELDATEFMT_HNL: &str =
    include_str!("../handl-packages/generated/icu/ureldatefmt.hnl");

// ── icu-uc (libicuuc) ports ───────────────────────────────────────────

/// `../handl-packages/generated/icu-uc/ubrk.hnl` (sdkgen port `ubrk`).
pub const ICU_UC_UBRK_HNL: &str = include_str!("../handl-packages/generated/icu-uc/ubrk.hnl");

/// `../handl-packages/generated/icu-uc/uchar.hnl` (sdkgen port `uchar`).
pub const ICU_UC_UCHAR_HNL: &str = include_str!("../handl-packages/generated/icu-uc/uchar.hnl");

/// `../handl-packages/generated/icu-uc/uldnames.hnl` (sdkgen port `uldnames`).
pub const ICU_UC_ULDNAMES_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc/uldnames.hnl");

/// `../handl-packages/generated/icu-uc/uloc.hnl` (sdkgen port `uloc`).
pub const ICU_UC_ULOC_HNL: &str = include_str!("../handl-packages/generated/icu-uc/uloc.hnl");

/// `../handl-packages/generated/icu-uc/unorm2.hnl` (sdkgen port `unorm2`).
pub const ICU_UC_UNORM2_HNL: &str = include_str!("../handl-packages/generated/icu-uc/unorm2.hnl");

/// `../handl-packages/generated/icu-uc/ures.hnl` (sdkgen port `ures`).
pub const ICU_UC_URES_HNL: &str = include_str!("../handl-packages/generated/icu-uc/ures.hnl");

/// `../handl-packages/generated/icu-uc/ustring.hnl` (sdkgen port `ustring`).
pub const ICU_UC_USTRING_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc/ustring.hnl");

// ── icu (libicui18n) C adapters ───────────────────────────────────────

/// `../handl-packages/generated/icu-c-linux-x86_64/ucol_c.hnl` (sdkgen adapter `ucol_c`).
pub const ICU_UCOL_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/ucol_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/udat_c.hnl` (sdkgen adapter `udat_c`).
pub const ICU_UDAT_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/udat_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/ufieldpositer_c.hnl` (sdkgen adapter `ufieldpositer_c`).
pub const ICU_UFIELDPOSITER_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/ufieldpositer_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/uformattednumber_c.hnl` (sdkgen adapter `uformattednumber_c`).
pub const ICU_UFORMATTEDNUMBER_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/uformattednumber_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/unum_c.hnl` (sdkgen adapter `unum_c`).
pub const ICU_UNUM_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/unum_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/unumberformatter_c.hnl` (sdkgen adapter `unumberformatter_c`).
pub const ICU_UNUMBERFORMATTER_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/unumberformatter_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/upluralrules_c.hnl` (sdkgen adapter `upluralrules_c`).
pub const ICU_UPLURALRULES_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/upluralrules_c.hnl");

/// `../handl-packages/generated/icu-c-linux-x86_64/ureldatefmt_c.hnl` (sdkgen adapter `ureldatefmt_c`).
pub const ICU_URELDATEFMT_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-c-linux-x86_64/ureldatefmt_c.hnl");

// ── icu-uc (libicuuc) C adapters ──────────────────────────────────────

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/ubrk_c.hnl` (sdkgen adapter `ubrk_c`).
pub const ICU_UC_UBRK_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/ubrk_c.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/uchar_c.hnl` (sdkgen adapter `uchar_c`).
pub const ICU_UC_UCHAR_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/uchar_c.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/uldnames_c.hnl` (sdkgen adapter `uldnames_c`).
pub const ICU_UC_ULDNAMES_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/uldnames_c.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/uloc_c.hnl` (sdkgen adapter `uloc_c`).
pub const ICU_UC_ULOC_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/uloc_c.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/unorm2_c.hnl` (sdkgen adapter `unorm2_c`).
pub const ICU_UC_UNORM2_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/unorm2_c.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/ures_c.hnl` (sdkgen adapter `ures_c`).
pub const ICU_UC_URES_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/ures_c.hnl");

/// `../handl-packages/generated/icu-uc-c-linux-x86_64/ustring_c.hnl` (sdkgen adapter `ustring_c`).
pub const ICU_UC_USTRING_C_HNL: &str =
    include_str!("../handl-packages/generated/icu-uc-c-linux-x86_64/ustring_c.hnl");
