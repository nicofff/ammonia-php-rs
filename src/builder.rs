use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendClassObject;
use std::collections::{HashMap, HashSet};

#[php_class]
#[php(name = "Ammonia\\Builder")]
#[derive(Default)]
pub struct AmmoniaBuilder {
    // "set" overrides (replace defaults entirely)
    tags_override: Option<HashSet<String>>,
    clean_content_tags_override: Option<HashSet<String>>,
    generic_attributes_override: Option<HashSet<String>>,
    url_schemes_override: Option<HashSet<String>>,

    // "add/rm" deltas
    tags_add: HashSet<String>,
    tags_rm: HashSet<String>,
    clean_content_tags_add: HashSet<String>,
    clean_content_tags_rm: HashSet<String>,
    tag_attributes_add: HashMap<String, HashSet<String>>,
    tag_attributes_rm: HashMap<String, HashSet<String>>,
    generic_attributes_add: HashSet<String>,
    generic_attributes_rm: HashSet<String>,
    url_schemes_add: HashSet<String>,
    url_schemes_rm: HashSet<String>,
    allowed_classes_add: HashMap<String, HashSet<String>>,
    allowed_classes_rm: HashMap<String, HashSet<String>>,

    // Simple value fields
    strip_comments: Option<bool>,
    link_rel: Option<Option<String>>,
    id_prefix: Option<Option<String>>,
}

impl AmmoniaBuilder {
    fn build(&self) -> ammonia::Builder<'_> {
        let mut builder = ammonia::Builder::default();

        // Tags
        if let Some(ref tags) = self.tags_override {
            builder.tags(tags.iter().map(String::as_str).collect());
        }
        if !self.tags_add.is_empty() {
            builder.add_tags(self.tags_add.iter().map(String::as_str));
        }
        if !self.tags_rm.is_empty() {
            builder.rm_tags(self.tags_rm.iter().map(String::as_str));
        }

        // Clean content tags
        if let Some(ref tags) = self.clean_content_tags_override {
            builder.clean_content_tags(tags.iter().map(String::as_str).collect());
        }
        if !self.clean_content_tags_add.is_empty() {
            builder.add_clean_content_tags(self.clean_content_tags_add.iter().map(String::as_str));
        }
        if !self.clean_content_tags_rm.is_empty() {
            builder.rm_clean_content_tags(self.clean_content_tags_rm.iter().map(String::as_str));
        }

        // Generic attributes
        if let Some(ref attrs) = self.generic_attributes_override {
            builder.generic_attributes(attrs.iter().map(String::as_str).collect());
        }
        if !self.generic_attributes_add.is_empty() {
            builder.add_generic_attributes(self.generic_attributes_add.iter().map(String::as_str));
        }
        if !self.generic_attributes_rm.is_empty() {
            builder.rm_generic_attributes(self.generic_attributes_rm.iter().map(String::as_str));
        }

        // Tag-specific attributes
        for (tag, attrs) in &self.tag_attributes_add {
            builder.add_tag_attributes(tag.as_str(), attrs.iter().map(String::as_str));
        }
        for (tag, attrs) in &self.tag_attributes_rm {
            builder.rm_tag_attributes(tag.as_str(), attrs.iter().map(String::as_str));
        }

        // URL schemes
        if let Some(ref schemes) = self.url_schemes_override {
            builder.url_schemes(schemes.iter().map(String::as_str).collect());
        }
        if !self.url_schemes_add.is_empty() {
            builder.add_url_schemes(self.url_schemes_add.iter().map(String::as_str));
        }
        if !self.url_schemes_rm.is_empty() {
            builder.rm_url_schemes(self.url_schemes_rm.iter().map(String::as_str));
        }

        // Allowed classes — ammonia requires "class" NOT be in generic_attributes
        // when using the class whitelist, so we remove it automatically.
        let has_class_whitelist = !self.allowed_classes_add.is_empty();
        if has_class_whitelist {
            builder.rm_generic_attributes(&["class"]);
        }
        for (tag, classes) in &self.allowed_classes_add {
            builder.add_allowed_classes(tag.as_str(), classes.iter().map(String::as_str));
        }
        for (tag, classes) in &self.allowed_classes_rm {
            builder.rm_allowed_classes(tag.as_str(), classes.iter().map(String::as_str));
        }

        // Simple values
        if let Some(strip) = self.strip_comments {
            builder.strip_comments(strip);
        }
        if let Some(ref rel) = self.link_rel {
            builder.link_rel(rel.as_deref());
        }
        if let Some(ref prefix) = self.id_prefix {
            builder.id_prefix(prefix.as_deref());
        }

        builder
    }
}

#[php_impl]
impl AmmoniaBuilder {
    pub fn __construct() -> Self {
        Self::default()
    }

    // ── Tag control ─────────────────────────────────────────────

    pub fn tags(self_: &mut ZendClassObject<AmmoniaBuilder>, tags: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.tags_override = Some(tags.into_iter().collect());
        self_
    }

    pub fn add_tags(self_: &mut ZendClassObject<AmmoniaBuilder>, tags: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.tags_add.extend(tags);
        self_
    }

    pub fn rm_tags(self_: &mut ZendClassObject<AmmoniaBuilder>, tags: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.tags_rm.extend(tags);
        self_
    }

    pub fn clean_content_tags(self_: &mut ZendClassObject<AmmoniaBuilder>, tags: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.clean_content_tags_override = Some(tags.into_iter().collect());
        self_
    }

    pub fn add_clean_content_tags(self_: &mut ZendClassObject<AmmoniaBuilder>, tags: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.clean_content_tags_add.extend(tags);
        self_
    }

    pub fn rm_clean_content_tags(self_: &mut ZendClassObject<AmmoniaBuilder>, tags: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.clean_content_tags_rm.extend(tags);
        self_
    }

    // ── Generic attributes ──────────────────────────────────────

    pub fn generic_attributes(self_: &mut ZendClassObject<AmmoniaBuilder>, attrs: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.generic_attributes_override = Some(attrs.into_iter().collect());
        self_
    }

    pub fn add_generic_attributes(self_: &mut ZendClassObject<AmmoniaBuilder>, attrs: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.generic_attributes_add.extend(attrs);
        self_
    }

    pub fn rm_generic_attributes(self_: &mut ZendClassObject<AmmoniaBuilder>, attrs: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.generic_attributes_rm.extend(attrs);
        self_
    }

    // ── Tag-specific attributes ─────────────────────────────────

    pub fn add_tag_attributes(self_: &mut ZendClassObject<AmmoniaBuilder>, tag: String, attrs: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.tag_attributes_add
            .entry(tag)
            .or_default()
            .extend(attrs);
        self_
    }

    pub fn rm_tag_attributes(self_: &mut ZendClassObject<AmmoniaBuilder>, tag: String, attrs: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.tag_attributes_rm
            .entry(tag)
            .or_default()
            .extend(attrs);
        self_
    }

    // ── URL schemes ─────────────────────────────────────────────

    pub fn url_schemes(self_: &mut ZendClassObject<AmmoniaBuilder>, schemes: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.url_schemes_override = Some(schemes.into_iter().collect());
        self_
    }

    pub fn add_url_schemes(self_: &mut ZendClassObject<AmmoniaBuilder>, schemes: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.url_schemes_add.extend(schemes);
        self_
    }

    pub fn rm_url_schemes(self_: &mut ZendClassObject<AmmoniaBuilder>, schemes: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.url_schemes_rm.extend(schemes);
        self_
    }

    // ── Allowed classes ─────────────────────────────────────────

    pub fn add_allowed_classes(self_: &mut ZendClassObject<AmmoniaBuilder>, tag: String, classes: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.allowed_classes_add
            .entry(tag)
            .or_default()
            .extend(classes);
        self_
    }

    pub fn rm_allowed_classes(self_: &mut ZendClassObject<AmmoniaBuilder>, tag: String, classes: Vec<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.allowed_classes_rm
            .entry(tag)
            .or_default()
            .extend(classes);
        self_
    }

    // ── Simple values ───────────────────────────────────────────

    pub fn strip_comments(self_: &mut ZendClassObject<AmmoniaBuilder>, strip: bool) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.strip_comments = Some(strip);
        self_
    }

    pub fn link_rel(self_: &mut ZendClassObject<AmmoniaBuilder>, rel: Option<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.link_rel = Some(rel);
        self_
    }

    pub fn id_prefix(self_: &mut ZendClassObject<AmmoniaBuilder>, prefix: Option<String>) -> &mut ZendClassObject<AmmoniaBuilder> {
        self_.id_prefix = Some(prefix);
        self_
    }

    // ── Sanitize ────────────────────────────────────────────────

    pub fn clean(&self, html: &str) -> PhpResult<String> {
        let builder = self.build();
        // ammonia panics on contradictory config (e.g. "rel" in generic_attributes
        // while link_rel is set, or "class" in generic_attributes while
        // allowed_classes is used, or a tag in both tags and clean_content_tags).
        // Catch these panics and convert to PHP exceptions.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            builder.clean(html).to_string()
        }));
        match result {
            Ok(cleaned) => Ok(cleaned),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| payload.downcast_ref::<&str>().copied())
                    .unwrap_or("unknown panic in ammonia");
                Err(PhpException::default(format!(
                    "Ammonia configuration error: {msg}"
                )))
            }
        }
    }
}
