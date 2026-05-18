#![allow(
    ambiguous_glob_reexports,
    dead_code,
    unused_imports,
    clippy::wildcard_imports,
    clippy::pub_underscore_fields,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_safety_doc,
    clippy::missing_errors_doc,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::redundant_field_names,
    clippy::struct_excessive_bools,
    clippy::trivially_copy_pass_by_ref,
    clippy::unused_self,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::similar_names,
    clippy::upper_case_acronyms,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
//! Raw FFI declarations generated from the macOS Accelerate framework
//! headers. Lints are disabled wholesale here because this surface is
//! 100% machine-generated; style nits cannot be addressed without
//! regenerating the bindings.

pub mod blas;
/// Raw FFI declarations for BNNS routines in Accelerate.
pub mod bnns;
/// Raw FFI declarations for LAPACK routines in Accelerate.
pub mod lapack;
/// Raw FFI declarations for Accelerate quadrature routines.
pub mod quadrature;
/// Raw FFI declarations for Apple SIMD helper types.
pub mod simd;
/// Raw FFI declarations for sparse routines in Accelerate.
pub mod sparse;
/// Raw FFI declarations for vDSP routines in Accelerate.
pub mod vdsp;
/// Raw FFI declarations for vecLib helper routines in Accelerate.
pub mod veclib_extras;
/// Raw FFI declarations for vForce routines in Accelerate.
pub mod vforce;
/// Raw FFI declarations for vImage routines in Accelerate.
pub mod vimage;

pub use self::blas::*;
pub use self::bnns::*;
pub use self::lapack::*;
pub use self::quadrature::*;
pub use self::simd::*;
pub use self::sparse::*;
pub use self::vdsp::*;
pub use self::veclib_extras::*;
pub use self::vforce::*;
pub use self::vimage::*;
