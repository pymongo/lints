pub(super) struct FnNameIsFoo;

// move these boilerplate(impl FnNameIsFoo + impl rustc_lint::LintPass) to macro?
impl FnNameIsFoo {
    const NAME: &'static str = std::any::type_name::<Self>();
    const LINT: rustc_lint::Lint = {
        let mut lint = rustc_lint::Lint::default_fields_for_macro();
        lint.name = Self::NAME; // can't use LintPass::name(), because trait cannot declared const fn
        lint.default_level = rustc_lint::Level::Warn;
        lint
    };
}

impl rustc_lint::LintPass for FnNameIsFoo {
    fn name(&self) -> &'static str {
        Self::NAME
    }
}

impl rustc_lint::EarlyLintPass for FnNameIsFoo {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::EarlyContext<'_>,
        fn_kind: rustc_ast::visit::FnKind<'_>,
        span: rustc_span::Span,
        _: rustc_ast::NodeId,
    ) {
        if let rustc_ast::visit::FnKind::Fn(_, ident, ..) = fn_kind {
            if ident.as_str() == "foo" {
                // rustc_lint::LintContext::struct_span_lint(cx, &Self::LINT, span, |diagnostic| {
                //     let mut diagnostic = diagnostic.build("foo is a bad name for function");
                //     diagnostic.emit();
                // });
                rustc_lint::LintContext::lint(cx, &Self::LINT, |lint| {
                    lint.build("foo is a bad name for function")
                        .set_span(span)
                        .emit()
                });
            }
        }
    }
}
