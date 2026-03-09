<?php

// Stubs for ammonia-php-rs

namespace Ammonia {
    /**
     * Configurable HTML sanitizer wrapping the ammonia crate.
     *
     * All setter methods return $this for fluent method chaining.
     * Call clean() to sanitize HTML with the configured rules.
     *
     * Defaults match ammonia's conservative settings: ~68 safe HTML tags,
     * no script/style/iframe, safe URL schemes only.
     */
    class Builder {
        /**
         * Create a new Builder with ammonia's default configuration.
         */
        public function __construct() {}

        /**
         * Add allowed CSS classes for a specific tag.
         *
         * When class whitelisting is used, only the specified classes are kept.
         * The "class" attribute is automatically handled.
         *
         * @param string $tag The tag name
         * @param array $classes Array of allowed CSS class names
         * @return \Ammonia\Builder $this for method chaining
         */
        public function addAllowedClasses(string $tag, array $classes): \Ammonia\Builder {}

        /**
         * Add tags to the content-stripping list.
         *
         * @param array $tags Array of tag names to add
         * @return \Ammonia\Builder $this for method chaining
         */
        public function addCleanContentTags(array $tags): \Ammonia\Builder {}

        /**
         * Add attributes allowed on all tags.
         *
         * @param array $attrs Array of attribute names to add
         * @return \Ammonia\Builder $this for method chaining
         */
        public function addGenericAttributes(array $attrs): \Ammonia\Builder {}

        /**
         * Add allowed attributes for a specific tag.
         *
         * @param string $tag The tag name
         * @param array $attrs Array of attribute names to allow on this tag
         * @return \Ammonia\Builder $this for method chaining
         */
        public function addTagAttributes(string $tag, array $attrs): \Ammonia\Builder {}

        /**
         * Add tags to the allowed tag whitelist.
         *
         * @param array $tags Array of tag names to add
         * @return \Ammonia\Builder $this for method chaining
         */
        public function addTags(array $tags): \Ammonia\Builder {}

        /**
         * Add URL schemes to the allowed list.
         *
         * @param array $schemes Array of URL scheme names to add
         * @return \Ammonia\Builder $this for method chaining
         */
        public function addUrlSchemes(array $schemes): \Ammonia\Builder {}

        /**
         * Sanitize HTML using this builder's configuration.
         *
         * @param string $html The HTML string to sanitize
         * @return string Sanitized HTML string
         * @throws \Exception Throws on contradictory configuration.
         */
        public function clean(string $html): string {}

        /**
         * Replace the set of tags whose content is completely stripped.
         *
         * Default: script, style. Content inside these tags is removed entirely
         * (not just the tags themselves).
         *
         * @param array $tags Array of tag names to strip content from
         * @return \Ammonia\Builder $this for method chaining
         */
        public function cleanContentTags(array $tags): \Ammonia\Builder {}

        /**
         * Replace the set of attributes allowed on all tags.
         *
         * Default: lang, title.
         *
         * @param array $attrs Array of attribute names
         * @return \Ammonia\Builder $this for method chaining
         */
        public function genericAttributes(array $attrs): \Ammonia\Builder {}

        /**
         * Set a prefix for all id attribute values.
         *
         * Useful to prevent ID collisions when embedding user content.
         * Pass null to disable prefixing.
         *
         * @param string|null $prefix The prefix string, or null to disable
         * @return \Ammonia\Builder $this for method chaining
         */
        public function idPrefix(?string $prefix = null): \Ammonia\Builder {}

        /**
         * Set the rel attribute added to links.
         *
         * Default: "noopener noreferrer". Pass null to disable.
         *
         * @param string|null $rel The rel attribute value, or null to disable
         * @return \Ammonia\Builder $this for method chaining
         */
        public function linkRel(?string $rel = null): \Ammonia\Builder {}

        /**
         * Remove allowed CSS classes for a specific tag.
         *
         * @param string $tag The tag name
         * @param array $classes Array of CSS class names to remove
         * @return \Ammonia\Builder $this for method chaining
         */
        public function rmAllowedClasses(string $tag, array $classes): \Ammonia\Builder {}

        /**
         * Remove tags from the content-stripping list.
         *
         * @param array $tags Array of tag names to remove
         * @return \Ammonia\Builder $this for method chaining
         */
        public function rmCleanContentTags(array $tags): \Ammonia\Builder {}

        /**
         * Remove attributes from the generic allowed list.
         *
         * @param array $attrs Array of attribute names to remove
         * @return \Ammonia\Builder $this for method chaining
         */
        public function rmGenericAttributes(array $attrs): \Ammonia\Builder {}

        /**
         * Remove allowed attributes from a specific tag.
         *
         * @param string $tag The tag name
         * @param array $attrs Array of attribute names to remove from this tag
         * @return \Ammonia\Builder $this for method chaining
         */
        public function rmTagAttributes(string $tag, array $attrs): \Ammonia\Builder {}

        /**
         * Remove tags from the allowed tag whitelist.
         *
         * @param array $tags Array of tag names to remove
         * @return \Ammonia\Builder $this for method chaining
         */
        public function rmTags(array $tags): \Ammonia\Builder {}

        /**
         * Remove URL schemes from the allowed list.
         *
         * @param array $schemes Array of URL scheme names to remove
         * @return \Ammonia\Builder $this for method chaining
         */
        public function rmUrlSchemes(array $schemes): \Ammonia\Builder {}

        /**
         * Set whether HTML comments are stripped.
         *
         * Default: true (comments are stripped).
         *
         * @param bool $strip true to strip comments, false to keep them
         * @return \Ammonia\Builder $this for method chaining
         */
        public function stripComments(bool $strip): \Ammonia\Builder {}

        /**
         * Replace the entire set of allowed HTML tags.
         *
         * Overrides the default whitelist entirely.
         *
         * @param array $tags Array of allowed tag names
         * @return \Ammonia\Builder $this for method chaining
         */
        public function tags(array $tags): \Ammonia\Builder {}

        /**
         * Replace the set of allowed URL schemes.
         *
         * Default: http, https, mailto, and other safe schemes.
         *
         * @param array $schemes Array of allowed URL scheme names
         * @return \Ammonia\Builder $this for method chaining
         */
        public function urlSchemes(array $schemes): \Ammonia\Builder {}
    }

    /**
     * Sanitize HTML using ammonia's default configuration.
     *
     * Strips dangerous tags (script, style, iframe, etc.) while keeping safe
     * semantic HTML. Uses ammonia's conservative defaults.
     *
     * @param string $html The HTML string to sanitize
     * @return string Sanitized HTML string
     */
    function sanitize_html(string $html): string {}
}
