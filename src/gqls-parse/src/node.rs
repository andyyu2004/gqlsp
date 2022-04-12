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
    ITEM => "item"
    TYPE_DEFINITION => "type_definition"
    OBJECT_TYPE_DEFINITION => "object_type_definition"
    INTERFACE_TYPE_DEFINITION => "interface_type_definition"
    UNION_TYPE_DEFINITION => "union_type_definition"
    ENUM_TYPE_DEFINITION => "enum_type_definition"
    SCALAR_TYPE_DEFINITION => "scalar_type_definition"
    INPUT_OBJECT_TYPE_DEFINITION => "input_object_type_definition"

    TYPE_EXTENSION => "type_extension"
    OBJECT_TYPE_EXTENSION => "object_type_extension"
}
