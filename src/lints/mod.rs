mod fn_name_is_foo;

pub(super) fn register_early_pass_lints(lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_early_pass(|| Box::new(fn_name_is_foo::FnNameIsFoo));
}
