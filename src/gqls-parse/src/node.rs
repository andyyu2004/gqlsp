macro_rules! node_kinds {
    ($($name:ident => $value:literal)*) => {
        pub enum NodeKind {}
        impl NodeKind {
            $(
                pub const $name: &'static str = $value;
            )*
        }
    };
}

node_kinds! {
    NAME => "name"
    TYPE_DEFINITION => "type_definition"
}
