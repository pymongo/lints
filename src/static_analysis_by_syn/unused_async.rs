/// similar to clippy::unused_async lint but use syn to static analysis
fn unused_async_lint(use_rayon_parallelism: bool) {
    // TODO path read from env
    let dir_entries = walkdir::WalkDir::new("/home/w/repos/clone_repos/tikv")
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let start = std::time::Instant::now();
    if use_rayon_parallelism {
        rayon::iter::ParallelIterator::for_each(
            rayon::iter::IntoParallelRefIterator::par_iter(&dir_entries),
            check_file,
        );
    } else {
        dir_entries.iter().for_each(check_file);
    }
    dbg!(start.elapsed());
    dbg!(use_rayon_parallelism);
}

struct FnVisitor {
    unused_async_ident_list: Vec<String>,
}

impl FnVisitor {
    fn visit_async_fn(&mut self, fn_sig: &syn::Signature, fn_block: &syn::Block) {
        if fn_sig.asyncness.is_none() {
            return;
        }
        for stmt in &fn_block.stmts {
            let mut expr_visitor = ExprVisitor { has_await: false };
            syn::visit::visit_stmt(&mut expr_visitor, stmt);
            if expr_visitor.has_await {
                return;
            }
        }
        self.unused_async_ident_list.push(fn_sig.ident.to_string());
    }
}

impl<'ast> syn::visit::Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, fn_: &'ast syn::ItemFn) {
        self.visit_async_fn(&fn_.sig, &fn_.block);
    }

    fn visit_impl_item_method(&mut self, fn_: &'ast syn::ImplItemMethod) {
        self.visit_async_fn(&fn_.sig, &fn_.block);
    }
}

struct ExprVisitor {
    has_await: bool,
}

impl<'ast> syn::visit::Visit<'ast> for ExprVisitor {
    fn visit_expr_await(&mut self, _i: &'ast syn::ExprAwait) {
        self.has_await = true;
    }
}

fn check_file(dir_entry: &walkdir::DirEntry) {
    let path = dir_entry.path();
    if let Some(ext) = path.extension() {
        if ext != "rs" {
            return;
        }
        if path
            .components()
            .any(|x| x == std::path::Component::Normal(std::ffi::OsStr::new("target")) || x == std::path::Component::Normal(std::ffi::OsStr::new(".docker"))  )
        {
            return;
        }
        let file = match syn::parse_file(&std::fs::read_to_string(path).unwrap()) {
            Ok(f) => f,
            Err(_) => {
                panic!("{:?}", path);
            }
        };
        let mut fn_visitor = FnVisitor {
            unused_async_ident_list: Vec::new(),
        };
        syn::visit::visit_file(&mut fn_visitor, &file);
        if !fn_visitor.unused_async_ident_list.is_empty() {
            dbg!(path);
            eprintln!("{:?}", fn_visitor.unused_async_ident_list);
        }
    }
}

#[test]
fn main() {
    unused_async_lint(false);
    unused_async_lint(true);
}
