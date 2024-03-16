use crate::kind_src::KindsSrc;

pub const CSS_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("~", "TILDE"),
        ("#", "HASH"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("||", "PIPE2"),
        ("+", "PLUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("^", "CARET"),
        ("%", "PERCENT"),
        (".", "DOT"),
        (":", "COLON"),
        ("::", "COLON2"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("-", "MINUS"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("|=", "PIPEEQ"),
        ("&=", "AMPEQ"),
        ("^=", "CARETEQ"),
        ("/=", "SLASHEQ"),
        ("*=", "STAREQ"),
        ("%=", "PERCENTEQ"),
        ("@", "AT"),
        ("$=", "DOLLAR_EQ"),
        ("~=", "TILDE_EQ"),
        ("-->", "CDC"),
        ("<!--", "CDO"),
    ],
    keywords: &[
        "media",
        "keyframes",
        "not",
        "and",
        "only",
        "or",
        "i",
        "important",
        "highlight",
        "part",
        "dir",
        "local",
        "global",
        "any",
        "current",
        "past",
        "future",
        "host",
        "host-context",
        "matches",
        "is",
        "where",
        "has",
        "lang",
        "nth-child",
        "nth-last-child",
        "nth-of-type",
        "nth-last-of-type",
        "nth-col",
        "nth-last-col",
        "charset",
        "color-profile",
        "counter-style",
        "property",
        "container",
        "style",
        "ltr",
        "rtl",
        "n",
        "even",
        "odd",
        "of",
        "from",
        "to",
        "var",
        "url",
        "src",
        "font-palette-values",
        "font-feature-values",
        "stylistic",
        "historical-forms",
        "styleset",
        "character-variant",
        "swash",
        "ornaments",
        "annotation",
        "auto",
        "thin",
        "medium",
        "thick",
        "none",
        "hidden",
        "dotted",
        "dashed",
        "solid",
        "double",
        "groove",
        "ridge",
        "inset",
        "outset",
        // HERE: Add new regular keywords _above_ here. Be sure to also add them
        // to `consume_identifier` in `biome_css_parser/src/lexer/mod.rs` as well.
        // CSS-wide keywords
        "initial",
        "inherit",
        "unset",
        "revert",
        "revert-layer",
        "default",
        // START: Only add dimension units after `em` and before `fr` below.
        // length units
        "em",
        "rem",
        "ex",
        "rex",
        "cap",
        "rcap",
        "ch",
        "rch",
        "ic",
        "ric",
        "lh",
        "rlh",
        // Viewport-percentage Lengths
        "vw",
        "svw",
        "lvw",
        "dvw",
        "vh",
        "svh",
        "lvh",
        "dvh",
        "vi",
        "svi",
        "lvi",
        "dvi",
        "vb",
        "svb",
        "lvb",
        "dvb",
        "vmin",
        "svmin",
        "lvmin",
        "dvmin",
        "vmax",
        "svmax",
        "lvmax",
        "dvmax",
        // Absolute lengths
        "cm",
        "mm",
        "q",
        "in",
        "pc",
        "pt",
        "px",
        "mozmm",
        // mini app
        "rpx",
        // container lengths
        "cqw",
        "cqh",
        "cqi",
        "cqb",
        "cqmin",
        "cqmax",
        // angle units
        "deg",
        "grad",
        "rad",
        "turn",
        // time units
        "s",
        "ms",
        // frequency units
        "hz",
        "khz",
        // resolution units
        "dpi",
        "dpcm",
        "dppx",
        "x",
        // flex units
        "fr",
        // END: Add new units _above_ `fr` to preserve range checks.
        // page at rule
        "page",
        "left",
        "right",
        "first",
        "blank",
        "top-left-corner",
        "top-left",
        "top-center",
        "top-right",
        "top-right-corner",
        "bottom-left-corner",
        "bottom-left",
        "bottom-center",
        "bottom-right",
        "bottom-right-corner",
        "left-top",
        "left-middle",
        "left-bottom",
        "right-top",
        "right-middle",
        "right-bottom",
        // layer at rule
        "layer",
        // scope at rule
        "scope",
        //
        "supports",
        "selector",
        "import",
        "namespace",
        "starting-style",
        "document",
        "url-prefix",
        "domain",
        "media-document",
        "regexp",
        //
        "font-face",
        // Don't add to the end of this list, add new keywords above the "HERE"
        // marker above, because we have a range check in is_contextual_keyword function.
    ],
    literals: &[
        "CSS_STRING_LITERAL",
        "CSS_NUMBER_LITERAL",
        "CSS_DASHED_IDENTIFIER",
        "CSS_CUSTOM_IDENTIFIER",
        "CSS_SPACE_LITERAL",
        "CSS_URL_VALUE_RAW_LITERAL",
        "CSS_COLOR_LITERAL",
        // Special literal token to represent a number that is _immediately_
        // followed by an identifier, which means it is a `<dimension>` token
        // according to the spec: https://www.w3.org/TR/css-values-4/#dimensions.
        "CSS_DIMENSION_VALUE",
        // Similarly, `<percentage>` also disallows spaces, so this token
        // represents a number immediately preceding a `%`.
        "CSS_PERCENTAGE_VALUE",
    ],
    tokens: &[
        "ERROR_TOKEN",
        "IDENT",
        "NEWLINE",
        "WHITESPACE",
        "COMMENT",
        "MULTILINE_COMMENT",
    ],
    nodes: &[
        "CSS_ROOT",
        "CSS_RULE_LIST",
        "CSS_QUALIFIED_RULE",
        "CSS_NESTED_QUALIFIED_RULE",
        "CSS_SELECTOR_LIST",
        "CSS_ANY_FUNCTION",
        "CSS_DECLARATION_BLOCK",
        "CSS_RULE_BLOCK",
        "CSS_DECLARATION_OR_AT_RULE_BLOCK",
        "CSS_DECLARATION_OR_RULE_BLOCK",
        "CSS_DECLARATION_OR_RULE_LIST",
        "CSS_DECLARATION_OR_AT_RULE_LIST",
        "CSS_DECLARATION_WITH_SEMICOLON",
        "CSS_DECLARATION",
        "CSS_IDENTIFIER",
        "CSS_NUMBER",
        "CSS_PARAMETER",
        "CSS_PERCENTAGE",
        "CSS_RATIO",
        "CSS_FUNCTION",
        "CSS_STRING",
        "CSS_VAR_FUNCTION",
        "CSS_VAR_FUNCTION_VALUE",
        "CSS_ATTRIBUTE_LIST",
        "CSS_DECLARATION_LIST",
        "CSS_COMPONENT_VALUE_LIST",
        "CSS_GENERIC_COMPONENT_VALUE_LIST",
        "CSS_GENERIC_DELIMITER",
        "CSS_GENERIC_PROPERTY",
        "CSS_UNKNOWN_PROPERTY_VALUE",
        // Properties
        "CSS_PARAMETER_LIST",
        "CSS_DECLARATION_IMPORTANT",
        "CSS_REGULAR_DIMENSION",
        "CSS_UNKNOWN_DIMENSION",
        // Selectors nodes
        "CSS_NAMESPACE",
        "CSS_NAMED_NAMESPACE_PREFIX",
        "CSS_UNIVERSAL_NAMESPACE_PREFIX",
        "CSS_ANY_SELECTOR_LIST",
        "CSS_COMPLEX_SELECTOR",
        "CSS_COMPOUND_SELECTOR",
        "CSS_SUB_SELECTOR_LIST",
        "CSS_ID_SELECTOR",
        "CSS_CLASS_SELECTOR",
        "CSS_TYPE_SELECTOR",
        "CSS_UNIVERSAL_SELECTOR",
        "CSS_PSEUDO_CLASS_SELECTOR",
        "CSS_PSEUDO_CLASS_SELECTOR_PARAMETERS",
        "CSS_PSEUDO_ELEMENT_SELECTOR",
        "CSS_PSEUDO_ELEMENT_IDENTIFIER",
        "CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR",
        "CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER",
        "CSS_PSEUDO_CLASS_IDENTIFIER",
        "CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER",
        "CSS_PSEUDO_CLASS_FUNCTION_SELECTOR",
        "CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR",
        "CSS_COMPOUND_SELECTOR_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST",
        "CSS_RELATIVE_SELECTOR_LIST",
        "CSS_RELATIVE_SELECTOR",
        "CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST",
        "CSS_PSEUDO_VALUE_LIST",
        "CSS_PSEUDO_CLASS_FUNCTION_NTH",
        "CSS_PSEUDO_CLASS_NTH_SELECTOR",
        "CSS_PSEUDO_CLASS_NTH",
        "CSS_PSEUDO_CLASS_NTH_NUMBER",
        "CSS_PSEUDO_CLASS_NTH_IDENTIFIER",
        "CSS_NTH_OFFSET",
        "CSS_PSEUDO_CLASS_OF_NTH_SELECTOR",
        "CSS_ATTRIBUTE_SELECTOR",
        "CSS_ATTRIBUTE",
        "CSS_ATTRIBUTE_NAME",
        "CSS_ATTRIBUTE_MATCHER",
        "CSS_ATTRIBUTE_MATCHER_VALUE",
        // Values
        "CSS_PARENTHESIZED_EXPRESSION",
        "CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION",
        "CSS_BINARY_EXPRESSION",
        "CSS_URL_VALUE_RAW",
        "CSS_URL_FUNCTION",
        "CSS_URL_MODIFIER_LIST",
        "CSS_COLOR",
        "CSS_BORDER",
        // At rule nodes
        "CSS_AT_RULE",
        "CSS_CHARSET_AT_RULE",
        "CSS_COLOR_PROFILE_AT_RULE",
        "CSS_COUNTER_STYLE_AT_RULE",
        "CSS_PROPERTY_AT_RULE",
        "CSS_CONTAINER_AT_RULE",
        "CSS_CONTAINER_NOT_QUERY",
        "CSS_CONTAINER_AND_QUERY",
        "CSS_CONTAINER_OR_QUERY",
        "CSS_CONTAINER_QUERY_IN_PARENS",
        "CSS_CONTAINER_STYLE_QUERY_IN_PARENS",
        "CSS_CONTAINER_SIZE_FEATURE_IN_PARENS",
        "CSS_CONTAINER_STYLE_NOT_QUERY",
        "CSS_CONTAINER_STYLE_AND_QUERY",
        "CSS_CONTAINER_STYLE_OR_QUERY",
        "CSS_CONTAINER_STYLE_IN_PARENS",
        "CSS_FONT_FACE_AT_RULE",
        "CSS_FONT_FEATURE_VALUES_AT_RULE",
        "CSS_FONT_FEATURE_VALUES_BLOCK",
        "CSS_FONT_FEATURE_VALUES_ITEM",
        "CSS_FONT_FEATURE_VALUES_ITEM_LIST",
        "CSS_FONT_FEATURE_VALUES_STYLISTIC",
        "CSS_FONT_FEATURE_VALUES_HISTORICAL_FORMS",
        "CSS_FONT_FEATURE_VALUES_STYLESET",
        "CSS_FONT_FEATURE_VALUES_CHARACTER_VARIANT",
        "CSS_FONT_FEATURE_VALUES_SWASH",
        "CSS_FONT_FEATURE_VALUES_ORNAMENTS",
        "CSS_FONT_FEATURE_VALUES_ANNOTATION",
        "CSS_FONT_PALETTE_VALUES_AT_RULE",
        "CSS_KEYFRAMES_AT_RULE",
        "CSS_KEYFRAMES_BODY",
        "CSS_MEDIA_AT_RULE",
        "CSS_MEDIA_QUERY_LIST",
        "CSS_MEDIA_QUERY",
        "CSS_MEDIA_CONDITION_QUERY",
        "CSS_MEDIA_TYPE_QUERY",
        "CSS_MEDIA_AND_TYPE_QUERY",
        "CSS_MEDIA_TYPE",
        "CSS_MEDIA_NOT_CONDITION",
        "CSS_MEDIA_AND_CONDITION",
        "CSS_MEDIA_OR_CONDITION",
        "CSS_MEDIA_CONDITION_IN_PARENS",
        "CSS_MEDIA_FEATURE_IN_PARENS",
        "CSS_QUERY_FEATURE_PLAIN",
        "CSS_QUERY_FEATURE_BOOLEAN",
        "CSS_QUERY_FEATURE_RANGE",
        "CSS_QUERY_FEATURE_REVERSE_RANGE",
        "CSS_QUERY_FEATURE_RANGE_INTERVAL",
        "CSS_QUERY_FEATURE_RANGE_COMPARISON",
        "CSS_KEYFRAMES_BLOCK",
        "CSS_KEYFRAMES_ITEM_LIST",
        "CSS_KEYFRAMES_ITEM",
        "CSS_KEYFRAMES_IDENT_SELECTOR",
        "CSS_KEYFRAMES_PERCENTAGE_SELECTOR",
        "CSS_KEYFRAMES_SELECTOR_LIST",
        "CSS_PAGE_AT_RULE",
        "CSS_PAGE_SELECTOR_LIST",
        "CSS_PAGE_SELECTOR",
        "CSS_PAGE_SELECTOR_PSEUDO_LIST",
        "CSS_PAGE_SELECTOR_PSEUDO",
        "CSS_PAGE_AT_RULE_BLOCK",
        "CSS_PAGE_AT_RULE_ITEM_LIST",
        "CSS_MARGIN_AT_RULE",
        "CSS_LAYER_AT_RULE",
        "CSS_LAYER_REFERENCE",
        "CSS_LAYER_REFERENCE_LIST",
        "CSS_LAYER_NAME_LIST",
        "CSS_LAYER_DECLARATION",
        "CSS_SUPPORTS_AT_RULE",
        "CSS_SUPPORTS_NOT_CONDITION",
        "CSS_SUPPORTS_AND_CONDITION",
        "CSS_SUPPORTS_OR_CONDITION",
        "CSS_SUPPORTS_CONDITION_IN_PARENS",
        "CSS_SUPPORTS_FEATURE_DECLARATION",
        "CSS_SUPPORTS_FEATURE_SELECTOR",
        "CSS_SCOPE_AT_RULE",
        "CSS_SCOPE_RANGE_START",
        "CSS_SCOPE_RANGE_END",
        "CSS_SCOPE_RANGE_INTERVAL",
        "CSS_SCOPE_EDGE",
        "CSS_IMPORT_AT_RULE",
        "CSS_IMPORT_ANONYMOUS_LAYER",
        "CSS_IMPORT_NAMED_LAYER",
        "CSS_IMPORT_SUPPORTS",
        "CSS_NAMESPACE_AT_RULE",
        "CSS_STARTING_STYLE_AT_RULE",
        "CSS_DOCUMENT_AT_RULE",
        "CSS_DOCUMENT_MATCHER_LIST",
        "CSS_DOCUMENT_CUSTOM_MATCHER",
        // Bogus nodes
        "CSS_BOGUS",
        "CSS_BOGUS_BLOCK",
        "CSS_BOGUS_KEYFRAMES_ITEM",
        "CSS_BOGUS_RULE",
        "CSS_BOGUS_SELECTOR",
        "CSS_BOGUS_SUB_SELECTOR",
        "CSS_BOGUS_PSEUDO_CLASS",
        "CSS_BOGUS_PSEUDO_ELEMENT",
        "CSS_BOGUS_AT_RULE",
        "CSS_BOGUS_LAYER",
        "CSS_BOGUS_PAGE_SELECTOR_PSEUDO",
        "CSS_BOGUS_DECLARATION_ITEM",
        "CSS_BOGUS_COMPONENT_VALUE",
        "CSS_BOGUS_PARAMETER",
        "CSS_BOGUS_PROPERTY",
        "CSS_BOGUS_PROPERTY_VALUE",
        "CSS_BOGUS_MEDIA_QUERY",
        "CSS_BOGUS_SCOPE_RANGE",
        "CSS_BOGUS_URL_MODIFIER",
        "CSS_BOGUS_DOCUMENT_MATCHER",
        "CSS_BOGUS_FONT_FEATURE_VALUES_ITEM",
    ],
};
