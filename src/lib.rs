#![doc = include_str!("../README.md")]
#![feature(rustc_private, const_type_name)]
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_span;

mod lints;

pub fn rustc_main() {
    std::env::set_var("RUSTC_LOG", "warn");
    // RUSTC_LOG=rustc_infer::infer::error_reporting=info rustc +my_rustc file.rs
    let args = std::env::args().collect::<Vec<_>>();
    rustc_driver::init_rustc_env_logger();
    rustc_driver::RunCompiler::new(&args, &mut CompilerCallback)
        .run()
        .unwrap();
}

/// emit for rustc compiler plugin
#[no_mangle]
fn __rustc_plugin_registrar(reg: &mut rustc_driver::plugin::Registry) {
    lints::register_early_pass_lints(reg.lint_store);
    // lints::register_late_pass_lints(reg.lint_store);
}

struct CompilerCallback;

impl rustc_driver::Callbacks for CompilerCallback {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        // cargo capture rustc stdout by pipe like rust-analyzer vscode capture lsp server stdout
        // <https://github.com/rust-lang/rust/blob/b27661eb33c74cb514dba059b47d86b6582ac1c2/compiler/rustc_driver/src/lib.rs#L1283>
        // if here use `println!` would `error: output of --print=file-names has changed in the compiler, cannot parse`
        if std::env::var("RUSTC_LOG").is_ok() {
            eprintln!("enter rustc_driver::Callbacks::config() callback");
        }
        config.register_lints = Some(Box::new(move |_session, lint_store| {
            lints::register_early_pass_lints(lint_store);
            // lints::register_late_pass_lints(lint_store);
        }));
    }

    fn after_parsing<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        if std::env::var("RUSTC_LOG").is_ok() {
            eprintln!("enter rustc_driver::Callbacks::after_parsing() callback");
        }
        rustc_driver::Compilation::Continue
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        if std::env::var("RUSTC_LOG").is_ok() {
            eprintln!("enter rustc_driver::Callbacks::after_expansion() callback");
        }
        rustc_driver::Compilation::Continue
    }

    /// stop compiler because our executable file doesn't libstd.so and librustc_driver.so correctly
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        if std::env::var("RUSTC_LOG").is_ok() {
            eprintln!("enter rustc_driver::Callbacks::after_analysis() callback");
        }
        // stop compiler to codegen because we doesn't link compiler correctly
        rustc_driver::Compilation::Stop
    }
}
