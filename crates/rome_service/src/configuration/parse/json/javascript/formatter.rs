use crate::configuration::javascript::JavascriptFormatter;
use crate::configuration::parse::json::formatter::deserialize_base_formatter_options;
use biome_deserialize::json::VisitJsonNode;
use rome_js_formatter::context::trailing_comma::TrailingComma;
use rome_js_formatter::context::{ArrowParentheses, QuoteProperties, QuoteStyle, Semicolons};

impl biome_deserialize::json::VisitJsonNode for JavascriptFormatter {}
impl biome_deserialize::VisitNode<biome_json_syntax::JsonLanguage> for JavascriptFormatter {
    fn visit_member_name(
        &mut self,
        node: &biome_json_syntax::JsonSyntaxNode,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<()> {
        biome_deserialize::json::has_only_known_keys(
            node,
            JavascriptFormatter::KNOWN_KEYS,
            diagnostics,
        )
    }

    fn visit_map(
        &mut self,
        key: &biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        value: &biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        *self = deserialize_javascript_options(name_text, &value, diagnostics)?;
        self.base_options = deserialize_base_formatter_options(name_text, &value, diagnostics)?;

        Some(())
    }
}

fn deserialize_javascript_options(
    name_text: &str,
    value: &biome_json_syntax::AnyJsonValue,
    diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
) -> Option<JavascriptFormatter> {
    let mut options = JavascriptFormatter::default();
    match name_text {
        "jsxQuoteStyle" => {
            let mut jsx_quote_style = QuoteStyle::default();
            options.map_to_known_string(value, name_text, &mut jsx_quote_style, diagnostics)?;
            options.jsx_quote_style = Some(jsx_quote_style);
        }
        "quoteStyle" => {
            let mut quote_style = QuoteStyle::default();
            options.map_to_known_string(value, name_text, &mut quote_style, diagnostics)?;
            options.quote_style = Some(quote_style);
        }
        "trailingComma" => {
            let mut trailing_comma = TrailingComma::default();
            options.map_to_known_string(value, name_text, &mut trailing_comma, diagnostics)?;
            options.trailing_comma = Some(trailing_comma);
        }
        "quoteProperties" => {
            let mut quote_properties = QuoteProperties::default();
            options.map_to_known_string(value, name_text, &mut quote_properties, diagnostics)?;
            options.quote_properties = Some(quote_properties);
        }
        "semicolons" => {
            let mut semicolons = Semicolons::default();
            options.map_to_known_string(value, name_text, &mut semicolons, diagnostics)?;
            options.semicolons = Some(semicolons);
        }
        "arrowParentheses" => {
            let mut arrow_parentheses = ArrowParentheses::default();
            options.map_to_known_string(value, name_text, &mut arrow_parentheses, diagnostics)?;
            options.arrow_parentheses = Some(arrow_parentheses);
        }
        "overrides" => {}
        _ => {}
    }

    Some(options)
}
