<?php

use function Ammonia\sanitize_html;
use Ammonia\Builder;

// ── Quick sanitize with defaults ────────────────────────────────────

$dirty = '<p>Hello <script>alert("xss")</script> <b>world</b></p>';
echo sanitize_html($dirty) . "\n";
// Output: <p>Hello  <b>world</b></p>

// ── Builder: allow custom tags ──────────────────────────────────────

$html = '<custom-element>content</custom-element><script>xss</script>';
$clean = (new Builder())
    ->addTags(['custom-element'])
    ->clean($html);
echo $clean . "\n";
// Output: <custom-element>content</custom-element>

// ── Builder: restrict to minimal tags ───────────────────────────────

$html = '<h1>Title</h1><p>Text with <b>bold</b> and <i>italic</i></p><div>block</div>';
$clean = (new Builder())
    ->tags(['p', 'b', 'i'])
    ->clean($html);
echo $clean . "\n";
// Output: Title<p>Text with <b>bold</b> and <i>italic</i></p>block

// ── Builder: control attributes ─────────────────────────────────────

$html = '<a href="https://example.com" onclick="steal()" target="_blank">link</a>';
$clean = (new Builder())
    ->addTagAttributes('a', ['href', 'target'])
    ->clean($html);
echo $clean . "\n";
// Output: <a href="https://example.com" target="_blank" rel="noopener noreferrer">link</a>

// ── Builder: CSS class whitelist ────────────────────────────────────

$html = '<div class="safe malicious">content</div><span class="highlight">text</span>';
$clean = (new Builder())
    ->addAllowedClasses('div', ['safe'])
    ->addAllowedClasses('span', ['highlight'])
    ->clean($html);
echo $clean . "\n";
// Output: <div class="safe">content</div><span class="highlight">text</span>

// ── Builder: reusable sanitizer ─────────────────────────────────────

$sanitizer = (new Builder())
    ->addTags(['section', 'article'])
    ->addGenericAttributes(['id'])
    ->idPrefix('user-')
    ->stripComments(true);

echo $sanitizer->clean('<section id="intro"><!-- hidden -->Hello</section>') . "\n";
// Output: <section id="user-intro">Hello</section>

echo $sanitizer->clean('<article id="post"><!-- draft -->Content</article>') . "\n";
// Output: <article id="user-post">Content</article>
