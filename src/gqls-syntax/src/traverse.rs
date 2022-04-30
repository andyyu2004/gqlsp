use std::fmt::Debug;

use tree_sitter::{Node, TreeCursor};

pub struct Traverse<'tree> {
    cursor: Option<TreeCursor<'tree>>,
    buffered: Option<TraverseEvent<'tree>>,
    retracing: bool,
}

impl<'tree> Traverse<'tree> {
    pub fn new(cursor: TreeCursor<'tree>) -> Self {
        Self { cursor: Some(cursor), buffered: Default::default(), retracing: false }
    }
}

#[derive(Copy, Clone)]
pub enum TraverseEvent<'tree> {
    Enter(Node<'tree>),
    Exit(Node<'tree>),
}

impl<'tree> TraverseEvent<'tree> {
    pub fn node(self) -> Node<'tree> {
        match self {
            TraverseEvent::Enter(node) | TraverseEvent::Exit(node) => node,
        }
    }

    #[must_use]
    pub fn is_exit(&self) -> bool {
        matches!(self, Self::Exit(..))
    }
}

impl Debug for TraverseEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enter(node) => write!(f, "> {}", node.kind()),
            Self::Exit(node) => write!(f, "< {}", node.kind()),
        }
    }
}

impl<'tree> Iterator for Traverse<'tree> {
    type Item = TraverseEvent<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.buffered.take() {
            return Some(node);
        }

        let c = self.cursor.as_mut()?;
        let node = c.node();

        if !self.retracing && c.goto_first_child() {
            return Some(TraverseEvent::Enter(c.node()));
        }

        if c.goto_next_sibling() {
            self.retracing = false;
            self.buffered = Some(TraverseEvent::Enter(c.node()));
            return Some(TraverseEvent::Exit(node));
        }

        if c.goto_parent() {
            self.retracing = true;
            return Some(TraverseEvent::Exit(node));
        }

        None
    }
}

#[cfg(test)]
mod tests;
