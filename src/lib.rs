//! A friendly Rust interface to the BeagleBone family of devices.

// TODO: Re-enable missing_docs warning once the following PR is merged:
// https://github.com/nix-rust/nix/pull/661
#![deny(bad_style,
        const_err,
        dead_code,
        extra_requirement_in_impl,
        improper_ctypes,
        legacy_directory_ownership,
        non_shorthand_field_patterns,
        no_mangle_generic_items,
        overflowing_literals,
        path_statements,
        patterns_in_fns_without_body,
        plugin_as_library,
        private_in_public,
        private_no_mangle_fns,
        private_no_mangle_statics,
        safe_extern_statics,
        unconditional_recursion,
        unions_with_drop_fields,
        unused,
        unused_allocation,
        unused_parens,
        while_true,
        missing_debug_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        unused_results)]

// Don't recurse too deeply (with error-chain enabled)
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate nix;

pub mod gpio;
pub mod enums;
pub mod errors;
pub mod pwm;
pub mod util;
pub mod adc;
pub mod i2c;
