pub(super) struct CheckEnumSize;

impl CheckEnumSize {
    const NAME: &'static str = std::any::type_name::<Self>();
    const LINT: rustc_lint::Lint = {
        let mut lint = rustc_lint::Lint::default_fields_for_macro();
        lint.name = Self::NAME; // can't use LintPass::name(), because trait cannot declared const fn
        lint.default_level = rustc_lint::Level::Warn;
        lint
    };
}

impl rustc_lint::LintPass for CheckEnumSize {
    fn name(&self) -> &'static str {
        Self::NAME
    }
}

impl<'tcx> rustc_lint::LateLintPass<'tcx> for CheckEnumSize {
    fn check_item(&mut self, cx: &rustc_lint::LateContext<'_>, item: &rustc_hir::Item<'_>) {
        if let rustc_hir::ItemKind::Enum(ref _def, _) = item.kind {
            let ty = cx.tcx.type_of(item.def_id);
            let adt = ty
                .ty_adt_def()
                .expect("already checked whether this is an enum");
            dbg!(&adt.variants);
            let mut temp = vec![];
            for variant in &adt.variants {
                // dbg!(variant.ident);
                let mut variant_size = 0;
                for filed in &variant.fields {
                    let ty = cx.tcx.type_of(filed.did);
                    // dbg!(ty);
                    let type_ = rustc_middle::ty::layout::LayoutOf::layout_of(cx, ty).unwrap();
                    // dbg!(type_.size.bytes());
                    variant_size += type_.size.bytes();
                }
                temp.push((variant_size, variant.ident.to_string()));
            }
            // dbg!(temp);
            let mut max_size_index = 0;
            let mut max_size = 0;
            let mut min_size_index = 0;
            let mut min_size = u64::MAX;
            for (idx, (size, _ident)) in temp.iter().enumerate() {
                let size = *size;
                if size < min_size {
                    min_size_index = idx;
                    min_size = size;
                }
                if size > max_size {
                    max_size_index = idx;
                    max_size = size;
                }
            }
            dbg!(min_size_index, max_size_index);
            if temp[max_size_index].0 - temp[min_size_index].0 > 200 {
                rustc_lint::LintContext::struct_span_lint(
                    cx,
                    &Self::LINT,
                    item.span,
                    |diagnostic| {
                        let mut diagnostic = diagnostic.build("enum size has waste");
                        diagnostic.emit();
                    },
                );
            }
        }
    }
}
