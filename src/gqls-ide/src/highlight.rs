// FIXME entire file is a messy hack that barely works
// There is definitely a cleaner way to implement this
use std::fmt::{self, Debug};

use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{ItemKind, TypeDefinitionKind};
use gqls_syntax::{Node, NodeKind, Point, Position, Range, RangeExt, Traverse, TraverseEvent};
use vfs::FileId;

use crate::Snapshot;

#[derive(Clone, PartialEq, Eq)]
pub struct SemanticToken {
    pub range: Range,
    pub kind: SemanticTokenKind,
}

impl Debug for SemanticToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} :: {:?}", self.range.debug(), self.kind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticTokenKind {
    Argument,
    Comment,
    Directive,
    Enum,
    EnumValue,
    Field,
    InputObject,
    Interface,
    Keyword,
    Number,
    Object,
    Scalar,
    String,
    Type,
    Union,
}

impl Snapshot {
    pub fn semantic_tokens(&self, file: FileId) -> Vec<SemanticToken> {
        let tree = self.file_tree(file);
        return Highlighter::new(self, file, gqls_syntax::traverse(&tree)).highlight();
    }
}

struct Highlighter<'a, 'tree> {
    snapshot: &'a Snapshot,
    file: FileId,
    nodes: Traverse<'tree>,
    tokens: Vec<SemanticToken>,
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Scope {
    Argument,
    Directive,
    Document,
    Enum,
    Field,
    InputObject,
    Interface,
    Object,
    Scalar,
    Type,
    Union,
    UnionMember,
}

impl Scope {
    fn from_node_kind(kind: &'static str) -> Option<Self> {
        match kind {
            NodeKind::ARGUMENTS_DEFINITION => Some(Scope::Argument),
            NodeKind::DIRECTIVE_DEFINITION => Some(Scope::Directive),
            NodeKind::FIELD_DEFINITION | NodeKind::INPUT_FIELDS_DEFINITION => Some(Scope::Field),
            NodeKind::OBJECT_TYPE_DEFINITION | NodeKind::OBJECT_TYPE_EXTENSION =>
                Some(Scope::Object),
            NodeKind::INTERFACE_TYPE_DEFINITION | NodeKind::INTERFACE_TYPE_EXTENSION =>
                Some(Scope::Interface),
            NodeKind::ENUM_TYPE_DEFINITION | NodeKind::ENUM_TYPE_EXTENSION => Some(Scope::Enum),
            NodeKind::INPUT_OBJECT_TYPE_DEFINITION | NodeKind::INPUT_OBJECT_TYPE_EXTENSION =>
                Some(Scope::InputObject),
            NodeKind::SCALAR_TYPE_DEFINITION | NodeKind::SCALAR_TYPE_EXTENSION =>
                Some(Scope::Scalar),
            NodeKind::UNION_TYPE_DEFINITION | NodeKind::UNION_TYPE_EXTENSION => Some(Scope::Union),
            NodeKind::UNION_MEMBER_TYPES => Some(Scope::UnionMember),
            NodeKind::TYPE => Some(Scope::Type),
            _ => None,
        }
    }
}

macro_rules! next {
    ($self:ident) => {{
        match $self.nodes.next() {
            Some(node) => node,
            None => return,
        }
    }};
}

impl<'a, 'tree> Highlighter<'a, 'tree> {
    fn new(snapshot: &'a Snapshot, file: FileId, nodes: Traverse<'tree>) -> Self {
        Self { snapshot, file, nodes, tokens: Default::default(), scopes: vec![Scope::Document] }
    }

    fn highlight(mut self) -> Vec<SemanticToken> {
        self.highlight_document();
        self.tokens
    }

    fn scope(&self) -> Scope {
        *self.scopes.last().unwrap()
    }

    fn highlight_document(&mut self) {
        let mut skip_until: Option<Node<'_>> = None;
        loop {
            // while let Some() if we can keep this structure
            let event = next!(self);
            let node = event.node();

            let scope = Scope::from_node_kind(node.kind());
            if let Some(scope) = scope {
                match event {
                    TraverseEvent::Enter(_) => self.scopes.push(scope),
                    TraverseEvent::Exit(_) => {
                        assert_eq!(self.scopes.pop().unwrap(), scope);
                    }
                }
            }

            // HACK to avoid overlapping tokens
            if let Some(s) = skip_until {
                if event.is_exit() && node.kind() == s.kind() {
                    skip_until = None;
                } else {
                    continue;
                }
            }

            if event.is_exit() {
                continue;
            }

            let at = node.range().start_point;
            let kind = match node.kind() {
                //TODO missing anonymous symbols
                "type" | "enum" | "scalar" | "interface" | "union" | "directive" | "on"
                    if !node.is_named() =>
                    SemanticTokenKind::Keyword,
                // TODO builtin types (ID, String, Int should be defaultLibrary types)
                NodeKind::TYPE if matches!(self.scope(), Scope::Type) => self.highlight_type(at),
                NodeKind::NAMED_TYPE if matches!(self.scope(), Scope::UnionMember) =>
                    self.highlight_type(at),
                NodeKind::DIRECTIVE => SemanticTokenKind::Directive,
                NodeKind::ENUM_VALUE => SemanticTokenKind::EnumValue,
                NodeKind::NAME | NodeKind::DIRECTIVE_NAME => match self.scope() {
                    Scope::Argument => SemanticTokenKind::Argument,
                    Scope::Directive => SemanticTokenKind::Directive,
                    Scope::Enum => SemanticTokenKind::Enum,
                    Scope::Field => SemanticTokenKind::Field,
                    Scope::InputObject => SemanticTokenKind::InputObject,
                    Scope::Interface => SemanticTokenKind::Interface,
                    Scope::Object => SemanticTokenKind::Object,
                    Scope::Scalar => SemanticTokenKind::Scalar,
                    Scope::Union => SemanticTokenKind::Union,
                    Scope::Type | Scope::UnionMember => unreachable!(),
                    Scope::Document => continue,
                },
                _ => continue,
            };

            let range = node.range();
            if range.is_empty() {
                continue;
            }
            let token = SemanticToken { range, kind };
            #[cfg(debug_assertions)]
            if let Some(prev) = self.tokens.last() {
                assert!(
                    prev.range.end_byte <= range.start_byte,
                    "token range overlap {prev:?}, {token:?}",
                );
                assert!(!prev.range.intersects(range));
            }
            skip_until = Some(node);
            self.tokens.push(token);
        }
    }

    fn highlight_type(&self, at: Point) -> SemanticTokenKind {
        match self
            .snapshot
            .type_at(Position::new(self.file, at))
            .expect("expected a type at this position")
            .item_resolutions()
        {
            [] => SemanticTokenKind::Type,
            [res, ..] => match self.snapshot.item(*res).kind {
                ItemKind::TypeDefinition(typedef) => {
                    match self.snapshot.typedef(res.file, typedef).kind {
                        TypeDefinitionKind::Object => SemanticTokenKind::Object,
                        TypeDefinitionKind::Interface => SemanticTokenKind::Interface,
                        TypeDefinitionKind::Input => SemanticTokenKind::InputObject,
                        TypeDefinitionKind::Scalar => SemanticTokenKind::Scalar,
                        TypeDefinitionKind::Enum => SemanticTokenKind::Enum,
                        TypeDefinitionKind::Union => SemanticTokenKind::Union,
                    }
                }
                ItemKind::DirectiveDefinition(_) => SemanticTokenKind::Directive,
            },
        }
    }
}

#[cfg(test)]
mod tests;
