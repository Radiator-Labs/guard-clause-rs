#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
// By Group: Cargo, Pedantic
// Probably by Clippy defaults: Complexity, Correctness, Performance, Suspicious
// Added manually: Restriction, Style
// Not included: Nursery
// Lints up to 1.83.0 (in process)
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(clippy::absolute_paths)]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::allow_attributes)]
#![deny(clippy::allow_attributes_without_reason)]
// #![deny(clippy::arbitrary_source_item_ordering)] // Unlock when 1.84.0 released. Originally reported as 1.82.0
#![allow(
    clippy::arithmetic_side_effects,
    reason = "Kelvin Embedded style guide: Arithmetic side effects commonly used in embedded systems programming."
)]
#![allow(
    clippy::as_conversions,
    reason = "clippy::as_conversions explanation lost in history. TODO remove allow and find reason."
)]
// #![deny(clippy::as_pointer_underscore)] // Unlock when 1.84.0 released. Originally reported as 1.81.0
#![deny(clippy::as_underscore)]
#![deny(clippy::assertions_on_constants)]
#![deny(clippy::big_endian_bytes)]
#![deny(clippy::byte_char_slices)]
#![deny(clippy::cfg_not_test)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::create_dir)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::default_numeric_fallback)]
#![deny(clippy::default_union_representation)]
#![deny(clippy::deref_by_slicing)]
#![deny(clippy::disallowed_script_idents)]
// #![deny(clippy::doc_include_without_cfg)]  // Unlock when 1.84.0 released.
#![allow(
    clippy::duplicated_attributes,
    reason = "https://github.com/rust-lang/rust-clippy/issues/13500"
)]
#![deny(clippy::else_if_without_else)]
#![deny(clippy::empty_drop)]
#![deny(clippy::empty_enum_variants_with_brackets)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::error_impl_error)]
#![allow(
    clippy::exhaustive_enums,
    reason = "Kelvin Embedded style guide: allowing public enumerants is consistent with a functional style use of enumerations as pure data."
)]
#![allow(
    clippy::exhaustive_structs,
    reason = "Kelvin Embedded style guide: allowing public field access is consistent with a functional style use of data structures as pure data."
)]
#![deny(clippy::exit)]
#![deny(clippy::expect_used)]
#![deny(clippy::field_scoped_visibility_modifiers)]
#![deny(clippy::filetype_is_file)]
#![deny(clippy::filter_map_bool_then)]
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::fn_to_numeric_cast_any)]
#![deny(clippy::format_push_string)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::host_endian_bytes)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::impl_trait_in_params)]
#![allow(
    clippy::implicit_return,
    reason = "Kelvin Embedded Style Guide: Implicit returns are an idiomatic approach that improves code readability."
)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::infinite_loop)]
#![deny(clippy::inline_asm_x86_att_syntax)]
#![allow(
    clippy::inline_asm_x86_intel_syntax,
    reason = "clippy::inline_asm_x86_intel_syntax explanation lost in history. TODO remove allow and find reason."
)]
#![allow(
    clippy::integer_division,
    reason = "Kelvin Embedded Style Guide: Integer division is a normally used operation in embedded systems programming."
)]
#![allow(
    clippy::integer_division_remainder_used,
    reason = "Kelvin Embedded Style Guide: Integer division is a normally used operation in embedded systems programming."
)]
#![deny(clippy::items_after_test_module)]
#![deny(clippy::iter_over_hash_type)]
#![deny(clippy::large_include_file)]
#![deny(clippy::legacy_numeric_constants)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::let_underscore_untyped)]
#![allow(
    clippy::little_endian_bytes,
    reason = "Little Endian is both our target and host endianness. Preference is specifying explixit over local."
)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::manual_is_finite)]
#![deny(clippy::manual_is_infinite)]
#![deny(clippy::manual_next_back)]
#![deny(clippy::manual_pattern_char_comparison)]
#![deny(clippy::manual_rotate)]
#![deny(clippy::manual_while_let_some)]
// #![deny(clippy::map_with_unused_argument_over_ranges)]  // Unlock when 1.84.0 released.
#![allow(
    clippy::min_ident_chars,
    reason = "Single characters are useful in small namespaces and should not be mechanically prohibited."
)]
#![deny(clippy::map_err_ignore)]
#![deny(clippy::mem_forget)]
#![deny(clippy::missing_assert_message)]
#![allow(
    clippy::missing_asserts_for_indexing,
    reason = "Asserts panic when false, which is prohibited in the embedded system."
)]
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "clippy::missing_docs_in_private_items explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::missing_enforced_import_renames)]
#![allow(
    clippy::missing_inline_in_public_items,
    reason = "clippy::missing_inline_in_public_items explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::missing_trait_methods)]
#![deny(clippy::mixed_attributes_style)]
#![deny(clippy::mixed_read_write_in_expression)]
#![deny(clippy::mod_module_files)]
#![allow(
    clippy::modulo_arithmetic,
    reason = "clippy::modulo_arithmetic explanation lost in history. TODO remove allow and find reason."
)]
#![allow(
    clippy::multiple_crate_versions,
    reason = "clippy::multiple_crate_versions explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::multiple_inherent_impl)]
#![deny(clippy::multiple_unsafe_ops_per_block)]
#![deny(clippy::mutex_atomic)]
#![deny(clippy::needless_borrows_for_generic_args)]
#![deny(clippy::needless_else)]
#![deny(clippy::needless_pub_self)]
#![deny(clippy::needless_raw_strings)]
#![deny(clippy::needless_return_with_question_mark)]
#![deny(clippy::non_ascii_literal)]
#![deny(clippy::non_minimal_cfg)]
#![deny(clippy::non_zero_suggestions)]
#![deny(clippy::option_map_or_err_ok)]
#![deny(clippy::panic)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::partial_pub_fields)]
#![deny(clippy::pathbuf_init_then_push)]
#![deny(clippy::pattern_type_mismatch)]
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
#![deny(clippy::pub_use)]
#![allow(
    clippy::pub_with_shorthand,
    reason = "Denying the reciprocal pub_without_shorthand."
)]
#![deny(clippy::pub_without_shorthand)]
#![allow(
    clippy::question_mark_used,
    reason = "Allowed by Kelvin Style Guide. This is idiomatic Rust."
)]
#![deny(clippy::rc_buffer)]
#![deny(clippy::rc_mutex)]
#![deny(clippy::redundant_feature_names)]
#![deny(clippy::redundant_type_annotations)]
#![deny(clippy::ref_patterns)]
#![deny(clippy::renamed_function_params)]
#![deny(clippy::rest_pat_in_fully_bound_structs)]
#![allow(
    clippy::same_name_method,
    reason = "TODO reconsider. Due to bitflags! macro"
)]
#![deny(clippy::self_named_module_files)]
#![deny(clippy::semicolon_outside_block)]
#![allow(
    clippy::separated_literal_suffix,
    reason = "clippy::separated_literal_suffix explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_unrelated)]
#![allow(
    clippy::single_call_fn,
    reason = "Allowed by Kelvin style guide. This is best practice when creating well refactored code."
)]
#![allow(
    clippy::single_char_lifetime_names,
    reason = "Allowed by Kelvin style guide. Single character lifetime names are commonly used in idiomatic Rust."
)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::str_to_string)]
#![deny(clippy::string_add)]
#![deny(clippy::string_lit_chars_any)]
#![deny(clippy::string_slice)]
#![deny(clippy::string_to_string)]
#![deny(clippy::suspicious_xor_used_as_pow)]
#![deny(clippy::tests_outside_test_module)]
#![deny(clippy::to_string_trait_impl)]
#![deny(clippy::todo)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unnecessary_fallible_conversions)]
// #![deny(clippy::unnecessary_map_or)] // Unlock when 1.84.0 released.
#![deny(clippy::unnecessary_safety_comment)]
#![deny(clippy::unnecessary_safety_doc)]
#![deny(clippy::unnecessary_self_imports)]
#![deny(clippy::unneeded_field_pattern)]
#![deny(clippy::unreachable)]
#![deny(clippy::unseparated_literal_suffix)]
#![deny(clippy::unused_enumerate_index)]
#![deny(clippy::unused_result_ok)]
#![deny(clippy::unused_trait_names)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::use_debug)]
#![deny(clippy::verbose_file_reads)]
#![deny(clippy::wildcard_enum_match_arm)]

/// Checks that a boolean expression is `true` at runtime, otherwise performs
/// a return with the second parameter, short-circuiting the operation of the
/// enclosing function.
///
/// # Uses
///
/// Guard clauses are used, generally at the start of functions, to ensure that
/// conditions are valid for the operation of the function. The usage is similar
/// to an assertion, but the guard clause returns a value that may be evaluated
/// and acted on by the calling function, while assertions end the program by
/// panicking.
///
/// The guard macro is syntactic sugar that replaces an if clause that would
/// return when true. The guard macro can be written on one line, while the
/// default Rust code formatter expands any if clause to three lines.
///
/// # Examples
///
/// ```rust
/// use guard_clause::guard;
///
/// fn safe_divide_using_option(lval: i32, rval: i32) -> Option<i32> {
///     guard!(rval != 0, None);
///     Some(lval / rval)
/// }
/// assert_eq!(Some(3), safe_divide_using_option(6, 2));
/// assert_eq!(None, safe_divide_using_option(6, 0));
///
/// fn safe_divide_using_result_str(lval: i32, rval: i32) -> Result<i32, &'static str> {
///     guard!(rval != 0, Err("Divide by zero!"));
///     Ok(lval / rval)
/// }
/// assert_eq!(Ok(3), safe_divide_using_result_str(6, 2));
/// assert_eq!(Err("Divide by zero!"), safe_divide_using_result_str(6, 0));
///
/// #[derive(Debug, PartialEq)]
/// enum DivError {
///     DivideByZero
/// }
/// fn safe_divide_using_result_enum(lval: i32, rval: i32) -> Result<i32, DivError> {
///     guard!(rval != 0, Err(DivError::DivideByZero));
///     Ok(lval / rval)
/// }
/// assert_eq!(Ok(3), safe_divide_using_result_enum(6, 2));
/// assert_eq!(Err(DivError::DivideByZero), safe_divide_using_result_enum(6, 0));
///
/// fn increments_if_zero(i: &mut i32) {
///     guard!(*i == 0);
///     *i += 1;
/// }
///
/// let mut i = 0;
/// increments_if_zero(&mut i);
/// assert_eq!(1, i);
///
/// let mut j = 2;
/// increments_if_zero(&mut j);
/// assert_eq!(2, j);
/// ```
#[macro_export]
macro_rules! guard {
    ($check:expr, $fail_return:expr) => {
        if !$check {
            return $fail_return;
        }
    };
    ($check:expr) => {
        if !$check {
            return;
        }
    };
}

#[cfg(test)]
mod tests {
    fn use_guard_with_result(test: bool, guard_fail: TestError) -> Result<(), TestError> {
        guard!(test, Err(guard_fail));
        Ok(())
    }
    #[test]
    fn given_a_true_condition_guard_with_result_return_type_then_ok_with_enum() {
        assert_eq!(use_guard_with_result(true, TestError::Reason2), Ok(()));
    }
    #[test]
    fn given_a_false_condition_guard_with_result_return_type_then_err_with_enum() {
        let expected_err = TestError::Reason2;
        assert_eq!(
            use_guard_with_result(false, expected_err),
            Err(expected_err)
        );
    }

    fn use_guard_with_option(test: bool) -> Option<()> {
        guard!(test, None);
        Some(())
    }
    #[test]
    fn given_a_true_condition_guard_with_option_return_type_then_ok_with_enum() {
        assert_eq!(use_guard_with_option(true), Some(()));
    }
    #[test]
    fn given_a_false_condition_guard_with_option_return_type_then_err_with_enum() {
        assert_eq!(use_guard_with_option(false), None);
    }

    fn use_guard_with_string_slice(test: bool, guard_fail: &'static str) -> &'static str {
        guard!(test, guard_fail);
        GOOD_STRING_SLICE
    }
    #[test]
    fn given_a_true_condition_guard_with_string_slice_return_type_then_ok_with_enum() {
        assert_eq!(
            use_guard_with_string_slice(true, BAD_STRING_SLICE),
            GOOD_STRING_SLICE
        );
    }
    #[test]
    fn given_a_false_condition_guard_with_string_slice_return_type_then_err_with_enum() {
        assert_eq!(
            use_guard_with_string_slice(false, BAD_STRING_SLICE),
            BAD_STRING_SLICE
        );
    }

    fn use_guard_with_no_return(test: bool, run_counter: &mut usize) {
        guard!(test);
        *run_counter += 1;
    }
    #[test]
    fn given_a_true_condition_guard_with_no_return_type_then_no_return_and_code_after_guard_ran() {
        let mut run_counter = 0;
        use_guard_with_no_return(true, &mut run_counter);
        assert_eq!(1, run_counter);
    }
    #[test]
    fn given_a_false_condition_guard_with_no_return_type_then_no_return_but_code_after_guard_not_run(
    ) {
        let mut run_counter = 0;
        use_guard_with_no_return(false, &mut run_counter);
        assert_eq!(0, run_counter);
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum TestError {
        _Reason1,
        Reason2,
        _Reason3(&'static str),
    }

    const GOOD_STRING_SLICE: &str = "Good!";
    const BAD_STRING_SLICE: &str = "bad...";
}
