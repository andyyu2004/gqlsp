use lsp_types::NumberOrString;
use std::path::{Path, PathBuf};
use tower_lsp::jsonrpc;

use crate::lsp::VfsExt;
use crate::tokens;

pub trait UrlExt {
    fn to_path(&self) -> jsonrpc::Result<PathBuf>;
}

impl UrlExt for lsp_types::Url {
    fn to_path(&self) -> jsonrpc::Result<PathBuf> {
        if self.scheme() != "file" {
            return Err(jsonrpc::Error::invalid_params(
                "Only file URIs are supported for workspace folders: `{uri}`",
            ));
        }
        self.to_file_path()
            .map_err(|()| jsonrpc::Error::invalid_params(format!("Invalid file path: `{self}`")))
    }
}

pub trait PathExt {
    fn to_url(&self) -> lsp_types::Url;
}

impl PathExt for Path {
    fn to_url(&self) -> lsp_types::Url {
        lsp_types::Url::from_file_path(self).unwrap()
    }
}

pub(crate) fn locations_to_goto_definition_response(
    locations: &[gqls_ide::Location],
) -> Option<lsp_types::GotoDefinitionResponse> {
    match locations {
        [] => None,
        [location] => Some(lsp_types::GotoDefinitionResponse::Scalar(location.convert())),
        locations => Some(lsp_types::GotoDefinitionResponse::Array(locations.convert())),
    }
}

// Conversions to and from lsp types
pub trait Convert {
    type Converted;
    fn convert(&self) -> Self::Converted;
}

impl<T> Convert for [T]
where
    T: Convert,
{
    type Converted = Vec<T::Converted>;

    fn convert(&self) -> Self::Converted {
        self.iter().map(|x| x.convert()).collect()
    }
}

impl<const N: usize, T> Convert for [T; N]
where
    T: Convert + Copy,
{
    type Converted = [T::Converted; N];

    fn convert(&self) -> Self::Converted {
        self.map(|x| x.convert())
    }
}

impl Convert for gqls_ide::Diagnostic {
    type Converted = lsp_types::Diagnostic;

    fn convert(&self) -> Self::Converted {
        lsp_types::Diagnostic {
            range: self.range.convert(),
            severity: Some(self.severity.convert()),
            message: self.message.clone(),
            code: Some(NumberOrString::Number(self.code.code() as i32)),
            source: Some("gqls".to_owned()),
            related_information: Some(self.labels.convert()),
            ..Default::default()
        }
    }
}

impl Convert for gqls_ide::Severity {
    type Converted = lsp_types::DiagnosticSeverity;

    fn convert(&self) -> Self::Converted {
        match self {
            gqls_ide::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        }
    }
}

impl Convert for gqls_ide::DiagnosticLabel {
    type Converted = lsp_types::DiagnosticRelatedInformation;

    fn convert(&self) -> Self::Converted {
        lsp_types::DiagnosticRelatedInformation {
            location: self.location.convert(),
            message: self.message.to_string(),
        }
    }
}

impl Convert for lsp_types::Range {
    type Converted = gqls_ide::Range;

    fn convert(&self) -> Self::Converted {
        gqls_ide::Range { start: self.start.convert(), end: self.end.convert() }
    }
}

impl Convert for lsp_types::TextDocumentPositionParams {
    type Converted = Result<gqls_ide::Position, jsonrpc::Error>;

    fn convert(&self) -> Self::Converted {
        Ok(gqls_ide::Position {
            file: gqls_ide::VFS.read().path(&self.text_document.uri)?,
            point: self.position.convert(),
        })
    }
}

impl Convert for gqls_ide::FilePatches {
    type Converted = lsp_types::TextDocumentEdit;

    fn convert(&self) -> Self::Converted {
        lsp_types::TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier {
                uri: self.file.to_url(),
                version: None,
            },
            edits: self.patches.convert().into_iter().map(lsp_types::OneOf::Left).collect(),
        }
    }
}

impl Convert for gqls_ide::Patch {
    type Converted = lsp_types::TextEdit;

    fn convert(&self) -> Self::Converted {
        lsp_types::TextEdit { range: self.range.convert(), new_text: self.with.clone() }
    }
}

impl Convert for gqls_ide::Location {
    type Converted = lsp_types::Location;

    fn convert(&self) -> Self::Converted {
        lsp_types::Location { range: self.range.convert(), uri: self.file.to_url() }
    }
}

impl Convert for gqls_ide::Range {
    type Converted = lsp_types::Range;

    fn convert(&self) -> lsp_types::Range {
        lsp_types::Range { start: self.start.convert(), end: self.end.convert() }
    }
}

impl Convert for lsp_types::Position {
    type Converted = gqls_ide::Point;

    fn convert(&self) -> Self::Converted {
        gqls_ide::Point::new(self.line as usize, self.character as usize)
    }
}

impl Convert for gqls_ide::Point {
    type Converted = lsp_types::Position;

    fn convert(&self) -> Self::Converted {
        lsp_types::Position::new(self.row as u32, self.column as u32)
    }
}

impl Convert for gqls_ide::SymbolKind {
    type Converted = lsp_types::SymbolKind;

    fn convert(&self) -> Self::Converted {
        match self {
            gqls_ide::SymbolKind::Struct => lsp_types::SymbolKind::STRUCT,
            gqls_ide::SymbolKind::Field => lsp_types::SymbolKind::FIELD,
            gqls_ide::SymbolKind::Constant => lsp_types::SymbolKind::CONSTANT,
        }
    }
}

impl Convert for gqls_ide::DocumentSymbol {
    type Converted = lsp_types::DocumentSymbol;

    fn convert(&self) -> Self::Converted {
        #[allow(deprecated)]
        lsp_types::DocumentSymbol {
            name: self.name.to_string(),
            detail: self.detail.clone(),
            kind: self.kind.convert(),
            range: self.range.convert(),
            selection_range: self.range.convert(),
            children: Some(self.children.convert()),
            tags: None,
            deprecated: None,
        }
    }
}

impl Convert for gqls_ide::WorkspaceSymbol {
    type Converted = lsp_types::SymbolInformation;

    fn convert(&self) -> Self::Converted {
        #[allow(deprecated)]
        lsp_types::SymbolInformation {
            name: self.name.to_string(),
            kind: self.kind.convert(),
            tags: None,
            deprecated: None,
            location: self.location.convert(),
            container_name: None,
        }
    }
}

impl Convert for gqls_ide::CompletionItem {
    type Converted = lsp_types::CompletionItem;

    fn convert(&self) -> Self::Converted {
        lsp_types::CompletionItem {
            label: self.label.clone(),
            kind: Some(self.kind.convert()),
            // TODO
            ..Default::default()
        }
    }
}

impl Convert for gqls_ide::CompletionItemKind {
    type Converted = lsp_types::CompletionItemKind;

    fn convert(&self) -> Self::Converted {
        match self {
            gqls_ide::CompletionItemKind::Directive(..) => lsp_types::CompletionItemKind::FUNCTION,
            gqls_ide::CompletionItemKind::Enum => lsp_types::CompletionItemKind::ENUM,
            gqls_ide::CompletionItemKind::Interface => lsp_types::CompletionItemKind::INTERFACE,
            gqls_ide::CompletionItemKind::Keyword => lsp_types::CompletionItemKind::KEYWORD,
            gqls_ide::CompletionItemKind::InputObject | gqls_ide::CompletionItemKind::Object =>
                lsp_types::CompletionItemKind::STRUCT,
            gqls_ide::CompletionItemKind::Scalar => lsp_types::CompletionItemKind::CONSTANT,
            gqls_ide::CompletionItemKind::Union => lsp_types::CompletionItemKind::CLASS,
            gqls_ide::CompletionItemKind::DirectiveLocation =>
                lsp_types::CompletionItemKind::MODULE,
        }
    }
}

impl Convert for gqls_ide::SemanticTokenKind {
    type Converted = lsp_types::SemanticTokenType;

    fn convert(&self) -> Self::Converted {
        match self {
            gqls_ide::SemanticTokenKind::Comment => lsp_types::SemanticTokenType::COMMENT,
            gqls_ide::SemanticTokenKind::Directive => lsp_types::SemanticTokenType::MACRO,
            gqls_ide::SemanticTokenKind::Enum => lsp_types::SemanticTokenType::ENUM,
            gqls_ide::SemanticTokenKind::EnumValue => lsp_types::SemanticTokenType::ENUM_MEMBER,
            gqls_ide::SemanticTokenKind::Field => lsp_types::SemanticTokenType::PROPERTY,
            gqls_ide::SemanticTokenKind::Interface => lsp_types::SemanticTokenType::INTERFACE,
            gqls_ide::SemanticTokenKind::Keyword => lsp_types::SemanticTokenType::KEYWORD,
            gqls_ide::SemanticTokenKind::Number => lsp_types::SemanticTokenType::NUMBER,
            gqls_ide::SemanticTokenKind::Argument => lsp_types::SemanticTokenType::PARAMETER,
            gqls_ide::SemanticTokenKind::String => lsp_types::SemanticTokenType::STRING,
            gqls_ide::SemanticTokenKind::Type => lsp_types::SemanticTokenType::TYPE,
            gqls_ide::SemanticTokenKind::Scalar => tokens::TOKEN_TYPE_SCALAR,
            gqls_ide::SemanticTokenKind::Union => tokens::TOKEN_TYPE_UNION,
            gqls_ide::SemanticTokenKind::InputObject | gqls_ide::SemanticTokenKind::Object =>
                lsp_types::SemanticTokenType::STRUCT,
        }
    }
}

impl Convert for gqls_ide::RenameError {
    type Converted = jsonrpc::Error;

    fn convert(&self) -> Self::Converted {
        jsonrpc::Error {
            // FIXME change to `RequestFailed` when available: https://github.com/microsoft/language-server-protocol/issues/1341
            code: jsonrpc::ErrorCode::InvalidParams,
            message: self.to_string(),
            data: None,
        }
    }
}
