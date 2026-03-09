#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;

mod builder;
pub use builder::AmmoniaBuilder;

/// Sanitize HTML using ammonia's default configuration.
///
/// Strips dangerous tags (script, style, iframe, etc.) while keeping safe
/// semantic HTML. Uses ammonia's conservative defaults.
///
/// # Arguments
/// * `html` - The HTML string to sanitize
///
/// # Returns
/// Sanitized HTML string
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
