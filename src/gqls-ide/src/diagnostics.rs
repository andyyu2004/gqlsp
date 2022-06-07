use gqls_db::{DefDatabase, Project, SourceDatabase, TyDatabase};
use gqls_ir::{
    Arg, Directive, Implementations, InProject, ItemKind, ItemRes, Ty, TypeDefinitionKind, Value
};
use gqls_syntax::{query, Query, QueryCursor};
use gqls_ty::TyKind;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::ops::Deref;
use std::str::FromStr;
use vfs::FileId;

use crate::{Location, Range, Snapshot};

impl Snapshot {
    pub fn diagnostics(&self, project: Project) -> Diagnostics {
        self.project_files(project)
            .into_iter()
            .map(|file| (file, self.file_diagnostics(file)))
            .collect()
    }

    // TODO this can probably be a database query so we get benefit of caching
    pub fn file_diagnostics(&self, file: FileId) -> FileDiagnostics {
        DiagnosticsCtxt::new(self, file).diagnostics()
    }
}

struct DiagnosticsCtxt<'a> {
    snapshot: &'a Snapshot,
    file: FileId,
    diagnostics: HashSet<Diagnostic>,
}

impl<'a> Deref for DiagnosticsCtxt<'a> {
    type Target = Snapshot;

    fn deref(&self) -> &'a Self::Target {
        &self.snapshot
    }
}

#[macro_export]
macro_rules! error_msg {
    (E0001) => {
        "syntax error"
    };
    (E0002) => {
        "unresolved directive `{name}`"
    };
    (E0003) => {
        "unresolved type `{typename}`"
    };
    (E0004) => {
        "duplicate directive definition `{name}`"
    };
    (E0005) => {
        "duplicate type definition `{name}`"
    };
    (E0006) => {
        "{typedef_kind} `{name}` must define at least one field"
    };
    (E0007) => {
        "expected an interface, found {typedef_kind} `{typename}`"
    };
    (E0008) => {
        "expected an input type, found {ty_desc} `{ty}`"
    };
    (E0009) => {
        "expected an output type, found {ty_desc} `{ty}`"
    };
    ($ident:ident) => {
        compile_error!("unknown error code")
    };
}

impl ErrorCode {
    pub fn severity(self) -> Severity {
        Severity::Error
    }
}

#[macro_export]
macro_rules! diagnostic {
    ($code:ident @ $range:expr $(, $name:ident = $arg:expr)* $( ; [ $( $location:expr => $msg:expr ),+ ] )? ) => {{
        let message = format!($crate::error_msg!($code),  $($name = $arg),*  );
        let labels = vec![ $( $( $crate::DiagnosticLabel {
            location: $location,
            message: $msg.to_string(),
        }, )+ )? ];
        $crate::Diagnostic::new_with_labels($range.into(), stringify!($code).parse().unwrap(), message, labels)
    }};
}

impl<'a> DiagnosticsCtxt<'a> {
    fn new(snapshot: &'a Snapshot, file: FileId) -> Self {
        Self { snapshot, file, diagnostics: Default::default() }
    }

    fn diagnostics(mut self) -> HashSet<Diagnostic> {
        self.ir_diagnostics();
        self.syntax();
        self.empty_fields();
        self.duplicate_definitions();
        self.check_items();
        self.diagnostics
    }

    fn diagnose(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.insert(diagnostic);
    }

    fn ir_diagnostics(&mut self) {
        let items = self.items(self.file);
        for (idx, _) in items.iter() {
            if let Some(body) = self.item_body(ItemRes::new(self.file, idx)) {
                self.diagnostics.extend(body.diagnostics.iter().map(Into::into))
            }
        }
    }

    fn empty_fields(&mut self) {
        for (file, items) in self.project_items(InProject::unit(self.file)).iter() {
            for (idx, item) in items.iter() {
                if let Some(fields) = self
                    .snapshot
                    .item_body(ItemRes::new(file, idx))
                    .as_ref()
                    .and_then(|body| body.fields())
                {
                    if fields.is_empty() {
                        let kind = items[item.kind.into_type_definition()].kind.desc();
                        self.diagnose(
                            diagnostic!(E0006 @ item.range, typedef_kind = kind, name = item.name),
                        );
                    }
                }
            }
        }
    }

    fn duplicate_definitions(&mut self) {
        let project_items = self.project_items(InProject::unit(self.file));
        let mut directives = HashMap::new();
        let mut typedefs = HashMap::new();

        for (file, items) in project_items.iter() {
            for (_, item) in items.iter() {
                let location = Location::new(file, item.name.range);
                match item.kind {
                    ItemKind::TypeDefinition(typedef) => {
                        if items[typedef].is_ext {
                            // type extensions are not duplicates
                            continue;
                        }
                        if let Some(existing) = typedefs.insert(&item.name, location) {
                            let diagnostic = diagnostic!(E0005 @ item.range, name = item.name; [
                                existing => format!("previous definition of type `{}` here", item.name)
                            ]);
                            self.diagnose(diagnostic);
                        }
                    }
                    ItemKind::DirectiveDefinition(_) =>
                        if let Some(existing) = directives.insert(&item.name, location) {
                            let diagnostic = diagnostic!(E0004 @ item.range, name = item.name; [
                                existing => format!("previous definition of directive `{}` here", item.name)
                            ]);
                            self.diagnose(diagnostic);
                        },
                };
            }
        }
    }

    fn check_items(&mut self) {
        let items = self.items(self.file);
        for (idx, item) in items.iter() {
            let typedef = match item.kind {
                ItemKind::TypeDefinition(idx) => &items[idx],
                ItemKind::DirectiveDefinition(_) => continue,
            };

            self.check_directives(&typedef.directives);
            if let Some(impls) = &typedef.implementations {
                self.check_implementations(impls);
            };

            if let Some(fields) = self
                .snapshot
                .item_body(ItemRes::new(self.file, idx))
                .as_ref()
                .and_then(|body| body.fields())
            {
                for (_, field) in fields.iter() {
                    // TODO check default value
                    self.check_directives(&field.directives);
                    self.check_args(&field.args);
                    match typedef.kind {
                        TypeDefinitionKind::Object
                        | TypeDefinitionKind::Interface
                        | TypeDefinitionKind::Union => self.check_output_ty(field.ty.clone()),
                        TypeDefinitionKind::Input => self.check_input_ty(field.ty.clone()),
                        TypeDefinitionKind::Scalar | TypeDefinitionKind::Enum =>
                            unreachable!("don't have fields"),
                    }
                }
            }
        }
    }

    fn check_args(&mut self, args: &[Arg]) {
        for arg in args {
            self.check_directives(&arg.directives);
            self.check_input_ty(arg.ty.clone());
            if let Some(default_value) = arg.default_value.clone() {
                self.ensure_subtype(&arg.ty, default_value);
            }
        }
    }

    fn ensure_subtype(&mut self, expected: &Ty, value: Value) {
    }

    fn check_input_ty(&mut self, ty: Ty) {
        let range = ty.range;
        let ty = self.lower_type(ty);
        if !ty.is_input() {
            self.diagnose(diagnostic!(E0008 @ range, ty_desc = ty.desc(), ty = ty));
        }
    }

    fn check_output_ty(&mut self, ty: Ty) {
        let range = ty.range;
        let ty = self.lower_type(ty);
        if !ty.is_output() {
            self.diagnose(diagnostic!(E0009 @ range, ty_desc = ty.desc(), ty = ty));
        }
    }

    fn check_implementations(&mut self, impls: &Implementations) {
        for name in impls {
            let res = self.resolve_item(InProject::new(self.file, name.clone()));
            let ty = self.type_of_res(res.clone());
            match ty.kind {
                TyKind::Boolean | TyKind::Float | TyKind::ID | TyKind::Int | TyKind::String =>
                    self.diagnose(diagnostic!(E0007 @ name.range, typedef_kind = ty.kind.desc(), typename = ty.kind)),
                TyKind::Err => self.diagnose(diagnostic!(E0003 @ name.range, typename = name)),
                TyKind::Union(_)
                | TyKind::Enum(_)
                | TyKind::Scalar(_)
                | TyKind::Object(_)
                | TyKind::Input(_) =>
                    for res in res.into_item() {
                        let location = Location::new(res.file, self.item(res).name.range);
                        self.diagnose(
                            diagnostic!(E0007 @ name.range, typedef_kind = ty.desc(), typename = name; [
                                location => "not an interface"
                            ]),
                        )
                    },
                TyKind::Interface(_) => {}
                TyKind::NonNull(_) | TyKind::List(_) => unreachable!(),
            }
        }
    }

    fn check_directives<'d>(&mut self, directives: impl IntoIterator<Item = &'d Directive>) {
        for directive in directives {
            if self
                .snapshot
                .resolve_item(InProject::new(self.file, directive.name.clone()))
                .is_err()
            {
                self.diagnose(diagnostic!(E0002 @ directive.name.range, name = directive.name))
            }
        }
    }

    fn syntax(&mut self) {
        // can't query for missing nodes atm, so just traversing the entire tree to find any missing nodes
        static QUERY: Lazy<Query> = Lazy::new(|| query("(ERROR) @error"));
        let mut cursor = QueryCursor::new();
        let data = self.file_data(self.file);
        let tree = data.tree;
        cursor.set_match_limit(30);
        let diags = cursor
            .captures(&QUERY, tree.root_node(), data.text.as_bytes())
            .flat_map(|(captures, _)| captures.captures)
            .map(|capture| capture.node)
            .chain(gqls_syntax::traverse_preorder(&tree).filter(|node| node.is_missing()))
            .map(|node| diagnostic!(E0001 @ node.range()));
        self.diagnostics.extend(diags);
    }
}

pub type Diagnostics = HashMap<FileId, FileDiagnostics>;
pub type FileDiagnostics = HashSet<Diagnostic>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Diagnostic {
    pub range: Range,
    pub code: ErrorCode,
    pub message: String,
    pub severity: Severity,
    pub labels: Vec<DiagnosticLabel>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Severity {
    Error,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DiagnosticLabel {
    pub location: Location,
    pub message: String,
}

impl Diagnostic {
    pub fn new(range: Range, code: ErrorCode, message: String) -> Self {
        Self::new_with_labels(range, code, message, vec![])
    }

    pub fn new_with_labels(
        range: Range,
        code: ErrorCode,
        message: String,
        labels: Vec<DiagnosticLabel>,
    ) -> Self {
        Self { range, code, message, severity: code.severity(), labels }
    }
}

impl<'a> From<&'_ gqls_ir::Diagnostic> for Diagnostic {
    fn from(diag: &'_ gqls_ir::Diagnostic) -> Self {
        match &diag.kind {
            gqls_ir::DiagnosticKind::UnresolvedType(typename) =>
                diagnostic!(E0003 @ diag.range, typename = typename),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ErrorCode(u16);

impl ErrorCode {
    pub fn code(self) -> u16 {
        self.0
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl FromStr for ErrorCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.starts_with('E'));
        Ok(Self(u16::from_str_radix(&s[1..], 10).expect("failed to parse error code")))
    }
}

#[cfg(test)]
mod tests;
