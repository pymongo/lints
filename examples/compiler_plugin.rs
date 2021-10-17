#![feature(plugin)]
// plugin feature is servo browser RFC, maybe remove in the feature
#![allow(deprecated)]
#![plugin(lints)]

fn main() {
    foo();
}

fn foo() {}
