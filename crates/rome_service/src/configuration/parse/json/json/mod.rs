use crate::configuration::json::{JsonConfiguration, JsonFormatter, JsonParser};

use crate::configuration::parse::json::formatter::deserialize_base_formatter_options;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use biome_rowan::SyntaxNode;

impl VisitJsonNode for JsonConfiguration {}

impl VisitNode<JsonLanguage> for JsonConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JsonConfiguration::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();

        match name_text {
            "parser" => {
                let mut parser = JsonParser::default();
                self.map_to_object(&value, name_text, &mut parser, diagnostics)?;
                self.parser = Some(parser);
            }
            "formatter" => {
                let mut formatter = JsonFormatter::default();
                self.map_to_object(&value, name_text, &mut formatter, diagnostics)?;
                self.formatter = Some(formatter);
            }

            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for JsonParser {}
impl VisitNode<JsonLanguage> for JsonParser {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JsonParser::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "allowComments" {
            self.allow_comments = self.map_to_boolean(&value, name_text, diagnostics);
        }

        Some(())
    }
}

impl VisitJsonNode for JsonFormatter {}
impl VisitNode<JsonLanguage> for JsonFormatter {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JsonFormatter::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();

        let base_options = deserialize_base_formatter_options(name_text, &value, diagnostics)?;
        self.base_options = base_options;

        Some(())
    }
}
