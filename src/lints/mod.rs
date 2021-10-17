mod fn_name_is_foo;
mod check_enum_size;

pub(super) fn register_early_pass_lints(lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_early_pass(|| Box::new(fn_name_is_foo::FnNameIsFoo));
}

// pub(super) fn register_late_pass_lints(lint_store: &mut rustc_lint::LintStore) {
//     lint_store.register_late_pass(|| Box::new(check_enum_size::CheckEnumSize));
// }
