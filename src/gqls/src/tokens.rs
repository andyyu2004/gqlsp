use lsp_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType};

use crate::Convert;

pub const TOKEN_TYPE_UNION: SemanticTokenType = SemanticTokenType::new("union");
pub const TOKEN_TYPE_SCALAR: SemanticTokenType = SemanticTokenType::new("union");

pub const TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::COMMENT,
    SemanticTokenType::ENUM,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::INTERFACE,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::MACRO,
    SemanticTokenType::NUMBER,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::STRING,
    SemanticTokenType::STRUCT,
    SemanticTokenType::TYPE,
    TOKEN_TYPE_SCALAR,
    TOKEN_TYPE_UNION,
];

pub const MODIFIERS: &[SemanticTokenModifier] = &[];

pub(crate) fn convert(ts: &[gqls_ide::SemanticToken]) -> Vec<SemanticToken> {
    let mut semantic_tokens = Vec::with_capacity(ts.len());
    for i in 0..ts.len() {
        let token = &ts[i];
        let range = token.range;
        semantic_tokens.push(SemanticToken {
            delta_line: if i == 0 {
                range.start_point.row as u32
            } else {
                (range.start_point.row - ts[i - 1].range.end_point.row) as u32
            },
            delta_start: if i == 0 || range.start_point.row != ts[i - 1].range.end_point.row {
                range.start_point.column as u32
            } else {
                (range.start_point.column - ts[i - 1].range.end_point.column) as u32
            },
            length: (token.range.end_byte - token.range.start_byte) as u32,
            token_type: TYPES
                .iter()
                .position(|k| k == &token.kind.convert())
                .unwrap_or_else(|| panic!("missing token in legend `{:?}`", token.kind.convert()))
                as u32,
            token_modifiers_bitset: 0,
        });
    }
    semantic_tokens
}
