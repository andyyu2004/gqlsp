use gqls_ide::{Diagnostic, Point, Range};
use lsp_types::*;
use std::path::PathBuf;
use tower_lsp::jsonrpc;

// Conversions to and from lsp types
pub trait Convert {
    type Converted;
    fn convert(self) -> Self::Converted;
}

impl<T> Convert for Vec<T>
where
    T: Convert,
{
    type Converted = Vec<T::Converted>;

    fn convert(self) -> Self::Converted {
        self.into_iter().map(|x| x.convert()).collect()
    }
}

impl<const N: usize, T> Convert for [T; N]
where
    T: Convert,
{
    type Converted = [T::Converted; N];

    fn convert(self) -> Self::Converted {
        self.map(Convert::convert)
    }
}

impl Convert for Diagnostic {
    type Converted = lsp_types::Diagnostic;

    fn convert(self) -> Self::Converted {
        lsp_types::Diagnostic {
            range: self.range.convert(),
            message: self.kind.to_string(),
            source: Some("gqls".to_owned()),
            ..Default::default()
        }
    }
}

impl Convert for lsp_types::Range {
    type Converted = Range;

    fn convert(self) -> Range {
        Range { start: self.start.convert(), end: self.end.convert() }
    }
}

impl Convert for Range {
    type Converted = lsp_types::Range;

    fn convert(self) -> lsp_types::Range {
        lsp_types::Range { start: self.start.convert(), end: self.end.convert() }
    }
}

impl Convert for Position {
    type Converted = Point;

    fn convert(self) -> Self::Converted {
        Point::new(self.line as usize, self.character as usize)
    }
}

impl Convert for Point {
    type Converted = Position;

    fn convert(self) -> Self::Converted {
        Position::new(self.row as u32, self.column as u32)
    }
}

pub trait UrlExt {
    fn to_path(&self) -> jsonrpc::Result<PathBuf>;
}

impl UrlExt for Url {
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
