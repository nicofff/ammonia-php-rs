# ammonia-php-rs

Rust-based HTML sanitizer for PHP — wraps the [ammonia](https://github.com/rust-ammonia/ammonia) crate.

10-30x faster than HTMLPurifier with comparable security defaults.

## Install via PIE

Requires [PIE](https://github.com/php/pie) 1.4.0+:

```bash
pie install nicofff/ammonia
```

## Usage

```php
<?php

// Strips dangerous HTML, keeps safe tags
$clean = sanitize_html('<p>Hello <script>alert("xss")</script> <b>world</b></p>');
// Output: <p>Hello  <b>world</b></p>
```

## Building from source

```bash
cargo build --release
php -d extension=./target/release/libammonia_php_rs.dylib -r 'echo sanitize_html("<b>hi</b>");'
```

## Running benchmarks

```bash
cd benchmark
composer install
cd ..
php -d extension=./target/release/libammonia_php_rs.dylib benchmark/benchmark.php
```
