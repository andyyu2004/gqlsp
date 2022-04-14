use std::path::{Path, PathBuf};
use tower_lsp::jsonrpc;

// Conversions to and from lsp types
pub trait Convert {
    type Converted;
    fn convert(&self) -> Self::Converted;
}

impl<'a, T> Convert for &'a [T]
where
    T: Convert,
{
    type Converted = Vec<T::Converted>;

    fn convert(&self) -> Self::Converted {
        self.into_iter().map(|x| x.convert()).collect()
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
            message: self.kind.to_string(),
            source: Some("gqls".to_owned()),
            ..Default::default()
        }
    }
}

impl Convert for lsp_types::Range {
    type Converted = gqls_ide::Range;

    fn convert(&self) -> Self::Converted {
        gqls_ide::Range { start: self.start.convert(), end: self.end.convert() }
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
