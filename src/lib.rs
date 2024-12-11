#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

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
/// ```
#[macro_export]
macro_rules! guard {
    ($check:expr, $fail_return:expr) => {
        if !$check {
            return $fail_return;
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
    fn given_a_false_condition_guard_guard_with_result_return_type_then_err_with_enum() {
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
    fn given_a_false_condition_guard_guard_with_option_return_type_then_err_with_enum() {
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
    fn given_a_false_condition_guard_guard_with_string_slice_return_type_then_err_with_enum() {
        assert_eq!(
            use_guard_with_string_slice(false, BAD_STRING_SLICE),
            BAD_STRING_SLICE
        );
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
