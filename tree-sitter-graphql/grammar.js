module.exports = grammar({
  name: "graphql",

  extras: ($) => [/[\s\uFEFF\u0009\u0020\u000A\u000D]/, $.comma, $.comment],

  rules: {
    document: ($) => repeat($.item),
    item: ($) =>
      choice(
        $.schema_definition,
        $.type_definition,
        $.directive_definition,
        $.schema_extension,
        $.type_extension
      ),
    schema_definition: ($) =>
      seq(
        optional($.description),
        "schema",
        optional($.directives),
        "{",
        repeat1($.root_operation_type_definition),
        "}"
      ),
    schema_extension: ($) =>
      seq(
        "extend",
        "schema",
        optional($.directives),
        "{",
        $.root_operation_type_definition,
        "}"
      ),
    type_extension: ($) =>
      choice(
        $.scalar_type_extension,
        $.object_type_extension,
        $.interface_type_extension,
        $.union_type_extension,
        $.enum_type_extension,
        $.input_object_type_extension
      ),
    // TODO diagnostics (directives are not optional for scalar extension)
    scalar_type_extension: ($) =>
      seq("extend", "scalar", $.name, optional($.directives)),
    object_type_extension: ($) =>
      prec.right(
        choice(
          seq(
            "extend",
            "type",
            $.name,
            optional($.implements_interfaces),
            optional($.directives),
            $.fields_definition
          ),
          seq(
            "extend",
            "type",
            $.name,
            optional($.implements_interfaces),
            optional($.directives)
          )
        )
      ),
    interface_type_extension: ($) =>
      prec.right(
        choice(
          seq(
            "extend",
            "interface",
            $.name,
            optional($.implements_interfaces),
            optional($.directives),
            $.fields_definition
          ),
          seq(
            "extend",
            "interface",
            $.name,
            optional($.implements_interfaces),
            optional($.directives)
          )
        )
      ),
    union_type_extension: ($) =>
      prec.right(
        choice(
          seq(
            "extend",
            "union",
            $.name,
            optional($.directives),
            $.union_member_types
          ),
          seq("extend", "union", $.name, optional($.directives))
        )
      ),
    enum_type_extension: ($) =>
      prec.right(
        choice(
          seq(
            "extend",
            "enum",
            $.name,
            optional($.directives),
            $.enum_values_definition
          ),
          seq("extend", "enum", $.name, optional($.directives))
        )
      ),
    input_object_type_extension: ($) =>
      prec.right(
        choice(
          seq(
            "extend",
            "input",
            $.name,
            optional($.directives),
            repeat1($.input_fields_definition)
          ),
          seq("extend", "input", $.name, optional($.directives))
        )
      ),
    input_fields_definition: ($) =>
      seq("{", repeat($.input_value_definition), "}"),
    enum_values_definition: ($) =>
      seq("{", repeat1($.enum_value_definition), "}"),
    enum_value_definition: ($) =>
      seq(optional($.description), $.enum_value, optional($.directives)),
    // TODO implements diagnostics for when there are no interfaces provided
    implements_interfaces: ($) =>
      seq("implements", optional("&"), sepBy("&", $.named_type)),
    // allow empty fields to be syntactically valid to avoid bad error messages
    // TODO implements diagnostics
    fields_definition: ($) => seq("{", repeat($.field_definition), "}"),
    field_definition: ($) =>
      seq(
        optional($.description),
        $.name,
        optional($.arguments_definition),
        ":",
        $.type,
        optional($.directives)
      ),
    arguments_definition: ($) =>
      seq("(", repeat1($.input_value_definition), ")"),
    input_value_definition: ($) =>
      seq(
        optional($.description),
        $.name,
        ":",
        $.type,
        optional($.default_value),
        optional($.directives)
      ),
    default_value: ($) => seq("=", $.value),
    union_member_types: ($) =>
      seq("=", optional("|"), sepBy("|", $.named_type)),
    root_operation_type_definition: ($) =>
      seq($.operation_type, ":", $.named_type),
    operation_type: (_) => choice("query", "mutation", "subscription"),
    type_definition: ($) =>
      choice(
        $.scalar_type_definition,
        $.object_type_definition,
        $.interface_type_definition,
        $.union_type_definition,
        $.enum_type_definition,
        $.input_object_type_definition
      ),
    scalar_type_definition: ($) =>
      prec.right(
        seq(optional($.description), "scalar", $.name, optional($.directives))
      ),
    object_type_definition: ($) =>
      seq(
        optional($.description),
        "type",
        $.name,
        optional($.implements_interfaces),
        optional($.directives),
        optional($.fields_definition)
      ),
    interface_type_definition: ($) =>
      prec.right(
        seq(
          optional($.description),
          "interface",
          $.name,
          optional($.implements_interfaces),
          optional($.directives),
          optional($.fields_definition)
        )
      ),
    union_type_definition: ($) =>
      prec.right(
        seq(
          optional($.description),
          "union",
          $.name,
          optional($.directives),
          optional($.union_member_types)
        )
      ),
    enum_type_definition: ($) =>
      prec.right(
        seq(
          optional($.description),
          "enum",
          $.name,
          optional($.directives),
          optional($.enum_values_definition)
        )
      ),
    input_object_type_definition: ($) =>
      prec.right(
        seq(
          optional($.description),
          "input",
          $.name,
          optional($.directives),
          optional($.input_fields_definition)
        )
      ),
    variable_definitions: ($) => seq("(", repeat1($.variable_definition), ")"),
    variable_definition: ($) =>
      seq(
        $.variable,
        ":",
        $.type,
        optional($.default_value),
        optional($.directives),
        optional($.comma)
      ),
    alias: ($) => seq($.name, ":"),
    arguments: ($) => seq("(", repeat1($.argument), ")"),
    argument: ($) => seq($.name, ":", $.value),
    value: ($) =>
      choice(
        $.variable,
        $.string_value,
        $.int_value,
        $.float_value,
        $.boolean_value,
        $.null_value,
        $.enum_value,
        $.list_value,
        $.object_value
      ),
    variable: ($) => seq("$", $.name),
    string_value: ($) =>
      choice(
        seq('"""', /([^"]|\n|""?[^"])*/, '"""'),
        seq('"', /[^"\\\n]*/, '"')
      ),
    int_value: ($) => /-?(0|[1-9][0-9]*)/,
    float_value: ($) =>
      token(
        seq(
          /-?(0|[1-9][0-9]*)/,
          choice(
            /\.[0-9]+/,
            /(e|E)(\+|-)?[0-9]+/,
            seq(/\.[0-9]+/, /(e|E)(\+|-)?[0-9]+/)
          )
        )
      ),
    boolean_value: (_) => choice("true", "false"),
    null_value: ($) => "null",
    enum_value: ($) => $.name,
    list_value: ($) => seq("[", repeat($.value), "]"),
    object_value: ($) => seq("{", repeat($.object_field), "}"),
    object_field: ($) => seq($.name, ":", $.value, optional($.comma)),
    type_condition: ($) => seq("on", $.named_type),
    directives: ($) => repeat1($.directive),
    directive: ($) => seq($.directive_name, optional($.arguments)),
    directive_definition: ($) =>
      seq(
        optional($.description),
        "directive",
        $.directive_name,
        optional($.arguments_definition),
        optional("repeatable"),
        $.directive_locations
      ),
    directive_locations: ($) =>
      seq("on", optional("|"), sepBy1("|", $.directive_location)),
    directive_location: () =>
      field(
        "location",
        choice(
          "SCHEMA",
          "SCALAR",
          "OBJECT",
          "FIELD_DEFINITION",
          "ARGUMENT_DEFINITION",
          "INTERFACE",
          "UNION",
          "ENUM",
          "ENUM_VALUE",
          "INPUT_OBJECT",
          "INPUT_FIELD_DEFINITION"
        )
      ),
    type: ($) => choice($.named_type, $.list_type, $.non_null_type),
    named_type: ($) => $._name,
    list_type: ($) => seq("[", $.type, "]"),
    non_null_type: ($) => choice(seq($.named_type, "!"), seq($.list_type, "!")),
    directive_name: ($) => seq("@", $._name),
    name: ($) => $._name,
    _name: ($) => /[_A-Za-z][_0-9A-Za-z]*/,
    comment: ($) => token(seq("#", /.*/)),
    comma: ($) => ",",
    description: ($) => $.string_value,
  },
});

function sepBy1(sep, rule) {
  return seq(rule, repeat(seq(sep, rule)));
}

function sepBy(sep, rule) {
  return optional(sepBy1(sep, rule));
}
