# ammonia-php-rs

Rust-based HTML sanitizer for PHP — wraps the [ammonia](https://github.com/rust-ammonia/ammonia) crate.

10-30x faster than HTMLPurifier with comparable security defaults.

## Install via PIE

Requires [PIE](https://github.com/php/pie) 1.4.0+:

```bash
pie install nicofff/ammonia
```

## Usage

### Quick sanitize (defaults)

```php
use function Ammonia\sanitize_html;

$clean = sanitize_html('<p>Hello <script>alert("xss")</script> <b>world</b></p>');
// Output: <p>Hello  <b>world</b></p>
```

### Builder API (custom configuration)

```php
use Ammonia\Builder;

$clean = (new Builder())
    ->addTags(['custom-element'])
    ->rmTags(['img'])
    ->addGenericAttributes(['class', 'id'])
    ->addTagAttributes('a', ['href', 'target'])
    ->addAllowedClasses('div', ['container', 'wrapper'])
    ->stripComments(true)
    ->linkRel('noopener noreferrer')
    ->idPrefix('user-content-')
    ->clean($dirtyHtml);
```

### Available Builder methods

All methods (except `clean()`) return `$this` for chaining.

| Method | Description |
|--------|-------------|
| `tags(string[])` | Replace entire allowed tag list |
| `addTags(string[])` | Add tags to whitelist |
| `rmTags(string[])` | Remove tags from whitelist |
| `cleanContentTags(string[])` | Replace content-stripping tag list |
| `addCleanContentTags(string[])` | Add content-stripping tags |
| `rmCleanContentTags(string[])` | Remove content-stripping tags |
| `genericAttributes(string[])` | Replace allowed attributes (all tags) |
| `addGenericAttributes(string[])` | Add allowed attributes |
| `rmGenericAttributes(string[])` | Remove allowed attributes |
| `addTagAttributes(string, string[])` | Add attributes for a specific tag |
| `rmTagAttributes(string, string[])` | Remove attributes for a specific tag |
| `urlSchemes(string[])` | Replace allowed URL schemes |
| `addUrlSchemes(string[])` | Add URL schemes |
| `rmUrlSchemes(string[])` | Remove URL schemes |
| `addAllowedClasses(string, string[])` | Add allowed CSS classes for a tag |
| `rmAllowedClasses(string, string[])` | Remove allowed CSS classes |
| `stripComments(bool)` | Strip HTML comments (default: true) |
| `linkRel(?string)` | Set rel attribute on links (null to disable) |
| `idPrefix(?string)` | Prefix all id attribute values |
| `clean(string): string` | Sanitize HTML and return result |

## Building from source

```bash
cargo build --release
php -d extension=./target/release/libammonia_php_rs.dylib -r 'echo Ammonia\sanitize_html("<b>hi</b>");'
```

## Running benchmarks

```bash
cd benchmark
composer install
cd ..
php -d extension=./target/release/libammonia_php_rs.dylib benchmark/benchmark.php
```
