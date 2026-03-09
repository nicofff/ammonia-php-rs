#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;

mod builder;
pub use builder::AmmoniaBuilder;

#[php_function]
#[php(name = "Ammonia\\sanitize_html")]
pub fn sanitize_html(html: &str) -> String {
    ammonia::clean(html)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(sanitize_html))
        .class::<builder::AmmoniaBuilder>()
}
