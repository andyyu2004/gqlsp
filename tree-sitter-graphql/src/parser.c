#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 314
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 123
#define ALIAS_COUNT 0
#define TOKEN_COUNT 60
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_schema = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  anon_sym_extend = 4,
  anon_sym_scalar = 5,
  anon_sym_type = 6,
  anon_sym_interface = 7,
  anon_sym_union = 8,
  anon_sym_enum = 9,
  anon_sym_input = 10,
  anon_sym_AMP = 11,
  anon_sym_implements = 12,
  anon_sym_COLON = 13,
  anon_sym_LPAREN = 14,
  anon_sym_RPAREN = 15,
  anon_sym_EQ = 16,
  anon_sym_PIPE = 17,
  anon_sym_query = 18,
  anon_sym_mutation = 19,
  anon_sym_subscription = 20,
  anon_sym_DOLLAR = 21,
  anon_sym_DQUOTE_DQUOTE_DQUOTE = 22,
  aux_sym_string_value_token1 = 23,
  anon_sym_DQUOTE = 24,
  aux_sym_string_value_token2 = 25,
  sym_int_value = 26,
  sym_float_value = 27,
  anon_sym_true = 28,
  anon_sym_false = 29,
  sym_null_value = 30,
  anon_sym_LBRACK = 31,
  anon_sym_RBRACK = 32,
  anon_sym_on = 33,
  anon_sym_AT = 34,
  anon_sym_directive = 35,
  anon_sym_repeatable = 36,
  anon_sym_QUERY = 37,
  anon_sym_MUTATION = 38,
  anon_sym_SUBSCRIPTION = 39,
  anon_sym_FIELD = 40,
  anon_sym_FRAGMENT_DEFINITION = 41,
  anon_sym_FRAGMENT_SPREAD = 42,
  anon_sym_INLINE_FRAGMENT = 43,
  anon_sym_VARIABLE_DEFINITION = 44,
  anon_sym_SCHEMA = 45,
  anon_sym_SCALAR = 46,
  anon_sym_OBJECT = 47,
  anon_sym_FIELD_DEFINITION = 48,
  anon_sym_ARGUMENT_DEFINITION = 49,
  anon_sym_INTERFACE = 50,
  anon_sym_UNION = 51,
  anon_sym_ENUM = 52,
  anon_sym_ENUM_VALUE = 53,
  anon_sym_INPUT_OBJECT = 54,
  anon_sym_INPUT_FIELD_DEFINITION = 55,
  anon_sym_BANG = 56,
  sym_name = 57,
  sym_comment = 58,
  sym_comma = 59,
  sym_document = 60,
  sym_item = 61,
  sym_schema_definition = 62,
  sym_schema_extension = 63,
  sym_type_extension = 64,
  sym_scalar_type_extension = 65,
  sym_object_type_extension = 66,
  sym_interface_type_extension = 67,
  sym_union_type_extension = 68,
  sym_enum_type_extension = 69,
  sym_input_object_type_extension = 70,
  sym_input_fields_definition = 71,
  sym_enum_values_definition = 72,
  sym_enum_value_definition = 73,
  sym_implements_interfaces = 74,
  sym_fields_definition = 75,
  sym_field_definition = 76,
  sym_arguments_definition = 77,
  sym_input_value_definition = 78,
  sym_default_value = 79,
  sym_union_member_types = 80,
  sym_root_operation_type_definition = 81,
  sym_operation_type = 82,
  sym_type_definition = 83,
  sym_scalar_type_definition = 84,
  sym_object_type_definition = 85,
  sym_interface_type_definition = 86,
  sym_union_type_definition = 87,
  sym_enum_type_definition = 88,
  sym_input_object_type_definition = 89,
  sym_arguments = 90,
  sym_argument = 91,
  sym_value = 92,
  sym_variable = 93,
  sym_string_value = 94,
  sym_boolean_value = 95,
  sym_enum_value = 96,
  sym_list_value = 97,
  sym_object_value = 98,
  sym_object_field = 99,
  sym_directives = 100,
  sym_directive = 101,
  sym_directive_definition = 102,
  sym_directive_locations = 103,
  sym_directive_location = 104,
  sym_executable_directive_location = 105,
  sym_type_system_directive_location = 106,
  sym_type = 107,
  sym_named_type = 108,
  sym_list_type = 109,
  sym_non_null_type = 110,
  sym_description = 111,
  aux_sym_document_repeat1 = 112,
  aux_sym_schema_definition_repeat1 = 113,
  aux_sym_input_object_type_extension_repeat1 = 114,
  aux_sym_input_fields_definition_repeat1 = 115,
  aux_sym_enum_values_definition_repeat1 = 116,
  aux_sym_fields_definition_repeat1 = 117,
  aux_sym_union_member_types_repeat1 = 118,
  aux_sym_arguments_repeat1 = 119,
  aux_sym_list_value_repeat1 = 120,
  aux_sym_object_value_repeat1 = 121,
  aux_sym_directives_repeat1 = 122,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_schema] = "schema",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_extend] = "extend",
  [anon_sym_scalar] = "scalar",
  [anon_sym_type] = "type",
  [anon_sym_interface] = "interface",
  [anon_sym_union] = "union",
  [anon_sym_enum] = "enum",
  [anon_sym_input] = "input",
  [anon_sym_AMP] = "&",
  [anon_sym_implements] = "implements",
  [anon_sym_COLON] = ":",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_EQ] = "=",
  [anon_sym_PIPE] = "|",
  [anon_sym_query] = "query",
  [anon_sym_mutation] = "mutation",
  [anon_sym_subscription] = "subscription",
  [anon_sym_DOLLAR] = "$",
  [anon_sym_DQUOTE_DQUOTE_DQUOTE] = "\"\"\"",
  [aux_sym_string_value_token1] = "string_value_token1",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_value_token2] = "string_value_token2",
  [sym_int_value] = "int_value",
  [sym_float_value] = "float_value",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_null_value] = "null_value",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_on] = "on",
  [anon_sym_AT] = "@",
  [anon_sym_directive] = "directive",
  [anon_sym_repeatable] = "repeatable",
  [anon_sym_QUERY] = "QUERY",
  [anon_sym_MUTATION] = "MUTATION",
  [anon_sym_SUBSCRIPTION] = "SUBSCRIPTION",
  [anon_sym_FIELD] = "FIELD",
  [anon_sym_FRAGMENT_DEFINITION] = "FRAGMENT_DEFINITION",
  [anon_sym_FRAGMENT_SPREAD] = "FRAGMENT_SPREAD",
  [anon_sym_INLINE_FRAGMENT] = "INLINE_FRAGMENT",
  [anon_sym_VARIABLE_DEFINITION] = "VARIABLE_DEFINITION",
  [anon_sym_SCHEMA] = "SCHEMA",
  [anon_sym_SCALAR] = "SCALAR",
  [anon_sym_OBJECT] = "OBJECT",
  [anon_sym_FIELD_DEFINITION] = "FIELD_DEFINITION",
  [anon_sym_ARGUMENT_DEFINITION] = "ARGUMENT_DEFINITION",
  [anon_sym_INTERFACE] = "INTERFACE",
  [anon_sym_UNION] = "UNION",
  [anon_sym_ENUM] = "ENUM",
  [anon_sym_ENUM_VALUE] = "ENUM_VALUE",
  [anon_sym_INPUT_OBJECT] = "INPUT_OBJECT",
  [anon_sym_INPUT_FIELD_DEFINITION] = "INPUT_FIELD_DEFINITION",
  [anon_sym_BANG] = "!",
  [sym_name] = "name",
  [sym_comment] = "comment",
  [sym_comma] = "comma",
  [sym_document] = "document",
  [sym_item] = "item",
  [sym_schema_definition] = "schema_definition",
  [sym_schema_extension] = "schema_extension",
  [sym_type_extension] = "type_extension",
  [sym_scalar_type_extension] = "scalar_type_extension",
  [sym_object_type_extension] = "object_type_extension",
  [sym_interface_type_extension] = "interface_type_extension",
  [sym_union_type_extension] = "union_type_extension",
  [sym_enum_type_extension] = "enum_type_extension",
  [sym_input_object_type_extension] = "input_object_type_extension",
  [sym_input_fields_definition] = "input_fields_definition",
  [sym_enum_values_definition] = "enum_values_definition",
  [sym_enum_value_definition] = "enum_value_definition",
  [sym_implements_interfaces] = "implements_interfaces",
  [sym_fields_definition] = "fields_definition",
  [sym_field_definition] = "field_definition",
  [sym_arguments_definition] = "arguments_definition",
  [sym_input_value_definition] = "input_value_definition",
  [sym_default_value] = "default_value",
  [sym_union_member_types] = "union_member_types",
  [sym_root_operation_type_definition] = "root_operation_type_definition",
  [sym_operation_type] = "operation_type",
  [sym_type_definition] = "type_definition",
  [sym_scalar_type_definition] = "scalar_type_definition",
  [sym_object_type_definition] = "object_type_definition",
  [sym_interface_type_definition] = "interface_type_definition",
  [sym_union_type_definition] = "union_type_definition",
  [sym_enum_type_definition] = "enum_type_definition",
  [sym_input_object_type_definition] = "input_object_type_definition",
  [sym_arguments] = "arguments",
  [sym_argument] = "argument",
  [sym_value] = "value",
  [sym_variable] = "variable",
  [sym_string_value] = "string_value",
  [sym_boolean_value] = "boolean_value",
  [sym_enum_value] = "enum_value",
  [sym_list_value] = "list_value",
  [sym_object_value] = "object_value",
  [sym_object_field] = "object_field",
  [sym_directives] = "directives",
  [sym_directive] = "directive",
  [sym_directive_definition] = "directive_definition",
  [sym_directive_locations] = "directive_locations",
  [sym_directive_location] = "directive_location",
  [sym_executable_directive_location] = "executable_directive_location",
  [sym_type_system_directive_location] = "type_system_directive_location",
  [sym_type] = "type",
  [sym_named_type] = "named_type",
  [sym_list_type] = "list_type",
  [sym_non_null_type] = "non_null_type",
  [sym_description] = "description",
  [aux_sym_document_repeat1] = "document_repeat1",
  [aux_sym_schema_definition_repeat1] = "schema_definition_repeat1",
  [aux_sym_input_object_type_extension_repeat1] = "input_object_type_extension_repeat1",
  [aux_sym_input_fields_definition_repeat1] = "input_fields_definition_repeat1",
  [aux_sym_enum_values_definition_repeat1] = "enum_values_definition_repeat1",
  [aux_sym_fields_definition_repeat1] = "fields_definition_repeat1",
  [aux_sym_union_member_types_repeat1] = "union_member_types_repeat1",
  [aux_sym_arguments_repeat1] = "arguments_repeat1",
  [aux_sym_list_value_repeat1] = "list_value_repeat1",
  [aux_sym_object_value_repeat1] = "object_value_repeat1",
  [aux_sym_directives_repeat1] = "directives_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_schema] = anon_sym_schema,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_extend] = anon_sym_extend,
  [anon_sym_scalar] = anon_sym_scalar,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_interface] = anon_sym_interface,
  [anon_sym_union] = anon_sym_union,
  [anon_sym_enum] = anon_sym_enum,
  [anon_sym_input] = anon_sym_input,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_implements] = anon_sym_implements,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_query] = anon_sym_query,
  [anon_sym_mutation] = anon_sym_mutation,
  [anon_sym_subscription] = anon_sym_subscription,
  [anon_sym_DOLLAR] = anon_sym_DOLLAR,
  [anon_sym_DQUOTE_DQUOTE_DQUOTE] = anon_sym_DQUOTE_DQUOTE_DQUOTE,
  [aux_sym_string_value_token1] = aux_sym_string_value_token1,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_value_token2] = aux_sym_string_value_token2,
  [sym_int_value] = sym_int_value,
  [sym_float_value] = sym_float_value,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_null_value] = sym_null_value,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_on] = anon_sym_on,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_directive] = anon_sym_directive,
  [anon_sym_repeatable] = anon_sym_repeatable,
  [anon_sym_QUERY] = anon_sym_QUERY,
  [anon_sym_MUTATION] = anon_sym_MUTATION,
  [anon_sym_SUBSCRIPTION] = anon_sym_SUBSCRIPTION,
  [anon_sym_FIELD] = anon_sym_FIELD,
  [anon_sym_FRAGMENT_DEFINITION] = anon_sym_FRAGMENT_DEFINITION,
  [anon_sym_FRAGMENT_SPREAD] = anon_sym_FRAGMENT_SPREAD,
  [anon_sym_INLINE_FRAGMENT] = anon_sym_INLINE_FRAGMENT,
  [anon_sym_VARIABLE_DEFINITION] = anon_sym_VARIABLE_DEFINITION,
  [anon_sym_SCHEMA] = anon_sym_SCHEMA,
  [anon_sym_SCALAR] = anon_sym_SCALAR,
  [anon_sym_OBJECT] = anon_sym_OBJECT,
  [anon_sym_FIELD_DEFINITION] = anon_sym_FIELD_DEFINITION,
  [anon_sym_ARGUMENT_DEFINITION] = anon_sym_ARGUMENT_DEFINITION,
  [anon_sym_INTERFACE] = anon_sym_INTERFACE,
  [anon_sym_UNION] = anon_sym_UNION,
  [anon_sym_ENUM] = anon_sym_ENUM,
  [anon_sym_ENUM_VALUE] = anon_sym_ENUM_VALUE,
  [anon_sym_INPUT_OBJECT] = anon_sym_INPUT_OBJECT,
  [anon_sym_INPUT_FIELD_DEFINITION] = anon_sym_INPUT_FIELD_DEFINITION,
  [anon_sym_BANG] = anon_sym_BANG,
  [sym_name] = sym_name,
  [sym_comment] = sym_comment,
  [sym_comma] = sym_comma,
  [sym_document] = sym_document,
  [sym_item] = sym_item,
  [sym_schema_definition] = sym_schema_definition,
  [sym_schema_extension] = sym_schema_extension,
  [sym_type_extension] = sym_type_extension,
  [sym_scalar_type_extension] = sym_scalar_type_extension,
  [sym_object_type_extension] = sym_object_type_extension,
  [sym_interface_type_extension] = sym_interface_type_extension,
  [sym_union_type_extension] = sym_union_type_extension,
  [sym_enum_type_extension] = sym_enum_type_extension,
  [sym_input_object_type_extension] = sym_input_object_type_extension,
  [sym_input_fields_definition] = sym_input_fields_definition,
  [sym_enum_values_definition] = sym_enum_values_definition,
  [sym_enum_value_definition] = sym_enum_value_definition,
  [sym_implements_interfaces] = sym_implements_interfaces,
  [sym_fields_definition] = sym_fields_definition,
  [sym_field_definition] = sym_field_definition,
  [sym_arguments_definition] = sym_arguments_definition,
  [sym_input_value_definition] = sym_input_value_definition,
  [sym_default_value] = sym_default_value,
  [sym_union_member_types] = sym_union_member_types,
  [sym_root_operation_type_definition] = sym_root_operation_type_definition,
  [sym_operation_type] = sym_operation_type,
  [sym_type_definition] = sym_type_definition,
  [sym_scalar_type_definition] = sym_scalar_type_definition,
  [sym_object_type_definition] = sym_object_type_definition,
  [sym_interface_type_definition] = sym_interface_type_definition,
  [sym_union_type_definition] = sym_union_type_definition,
  [sym_enum_type_definition] = sym_enum_type_definition,
  [sym_input_object_type_definition] = sym_input_object_type_definition,
  [sym_arguments] = sym_arguments,
  [sym_argument] = sym_argument,
  [sym_value] = sym_value,
  [sym_variable] = sym_variable,
  [sym_string_value] = sym_string_value,
  [sym_boolean_value] = sym_boolean_value,
  [sym_enum_value] = sym_enum_value,
  [sym_list_value] = sym_list_value,
  [sym_object_value] = sym_object_value,
  [sym_object_field] = sym_object_field,
  [sym_directives] = sym_directives,
  [sym_directive] = sym_directive,
  [sym_directive_definition] = sym_directive_definition,
  [sym_directive_locations] = sym_directive_locations,
  [sym_directive_location] = sym_directive_location,
  [sym_executable_directive_location] = sym_executable_directive_location,
  [sym_type_system_directive_location] = sym_type_system_directive_location,
  [sym_type] = sym_type,
  [sym_named_type] = sym_named_type,
  [sym_list_type] = sym_list_type,
  [sym_non_null_type] = sym_non_null_type,
  [sym_description] = sym_description,
  [aux_sym_document_repeat1] = aux_sym_document_repeat1,
  [aux_sym_schema_definition_repeat1] = aux_sym_schema_definition_repeat1,
  [aux_sym_input_object_type_extension_repeat1] = aux_sym_input_object_type_extension_repeat1,
  [aux_sym_input_fields_definition_repeat1] = aux_sym_input_fields_definition_repeat1,
  [aux_sym_enum_values_definition_repeat1] = aux_sym_enum_values_definition_repeat1,
  [aux_sym_fields_definition_repeat1] = aux_sym_fields_definition_repeat1,
  [aux_sym_union_member_types_repeat1] = aux_sym_union_member_types_repeat1,
  [aux_sym_arguments_repeat1] = aux_sym_arguments_repeat1,
  [aux_sym_list_value_repeat1] = aux_sym_list_value_repeat1,
  [aux_sym_object_value_repeat1] = aux_sym_object_value_repeat1,
  [aux_sym_directives_repeat1] = aux_sym_directives_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_schema] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_extend] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_scalar] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interface] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_union] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enum] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_input] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_implements] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_query] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mutation] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subscription] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOLLAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE_DQUOTE_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_value_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_value_token2] = {
    .visible = false,
    .named = false,
  },
  [sym_int_value] = {
    .visible = true,
    .named = true,
  },
  [sym_float_value] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_null_value] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_on] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_directive] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_repeatable] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_QUERY] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_MUTATION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SUBSCRIPTION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FIELD] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FRAGMENT_DEFINITION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FRAGMENT_SPREAD] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_INLINE_FRAGMENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_VARIABLE_DEFINITION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SCHEMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SCALAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_OBJECT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_FIELD_DEFINITION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ARGUMENT_DEFINITION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_INTERFACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_UNION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ENUM] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ENUM_VALUE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_INPUT_OBJECT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_INPUT_FIELD_DEFINITION] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [sym_name] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_comma] = {
    .visible = true,
    .named = true,
  },
  [sym_document] = {
    .visible = true,
    .named = true,
  },
  [sym_item] = {
    .visible = true,
    .named = true,
  },
  [sym_schema_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_schema_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_scalar_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_object_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_interface_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_union_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_input_object_type_extension] = {
    .visible = true,
    .named = true,
  },
  [sym_input_fields_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_values_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_value_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_implements_interfaces] = {
    .visible = true,
    .named = true,
  },
  [sym_fields_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_field_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_arguments_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_input_value_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_default_value] = {
    .visible = true,
    .named = true,
  },
  [sym_union_member_types] = {
    .visible = true,
    .named = true,
  },
  [sym_root_operation_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_operation_type] = {
    .visible = true,
    .named = true,
  },
  [sym_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_scalar_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_object_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_interface_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_union_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_input_object_type_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_argument] = {
    .visible = true,
    .named = true,
  },
  [sym_value] = {
    .visible = true,
    .named = true,
  },
  [sym_variable] = {
    .visible = true,
    .named = true,
  },
  [sym_string_value] = {
    .visible = true,
    .named = true,
  },
  [sym_boolean_value] = {
    .visible = true,
    .named = true,
  },
  [sym_enum_value] = {
    .visible = true,
    .named = true,
  },
  [sym_list_value] = {
    .visible = true,
    .named = true,
  },
  [sym_object_value] = {
    .visible = true,
    .named = true,
  },
  [sym_object_field] = {
    .visible = true,
    .named = true,
  },
  [sym_directives] = {
    .visible = true,
    .named = true,
  },
  [sym_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_directive_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_directive_locations] = {
    .visible = true,
    .named = true,
  },
  [sym_directive_location] = {
    .visible = true,
    .named = true,
  },
  [sym_executable_directive_location] = {
    .visible = true,
    .named = true,
  },
  [sym_type_system_directive_location] = {
    .visible = true,
    .named = true,
  },
  [sym_type] = {
    .visible = true,
    .named = true,
  },
  [sym_named_type] = {
    .visible = true,
    .named = true,
  },
  [sym_list_type] = {
    .visible = true,
    .named = true,
  },
  [sym_non_null_type] = {
    .visible = true,
    .named = true,
  },
  [sym_description] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_document_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_schema_definition_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_input_object_type_extension_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_input_fields_definition_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_enum_values_definition_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_fields_definition_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_union_member_types_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_list_value_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_object_value_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_directives_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(262);
      if (lookahead == '!') ADVANCE(337);
      if (lookahead == '"') ADVANCE(297);
      if (lookahead == '#') ADVANCE(391);
      if (lookahead == '$') ADVANCE(291);
      if (lookahead == '&') ADVANCE(281);
      if (lookahead == '(') ADVANCE(284);
      if (lookahead == ')') ADVANCE(285);
      if (lookahead == ',') ADVANCE(392);
      if (lookahead == '-') ADVANCE(6);
      if (lookahead == '0') ADVANCE(301);
      if (lookahead == ':') ADVANCE(283);
      if (lookahead == '=') ADVANCE(286);
      if (lookahead == '@') ADVANCE(314);
      if (lookahead == 'A') ADVANCE(132);
      if (lookahead == 'E') ADVANCE(112);
      if (lookahead == 'F') ADVANCE(66);
      if (lookahead == 'I') ADVANCE(102);
      if (lookahead == 'M') ADVANCE(155);
      if (lookahead == 'O') ADVANCE(18);
      if (lookahead == 'Q') ADVANCE(157);
      if (lookahead == 'S') ADVANCE(22);
      if (lookahead == 'U') ADVANCE(103);
      if (lookahead == 'V') ADVANCE(8);
      if (lookahead == '[') ADVANCE(311);
      if (lookahead == ']') ADVANCE(312);
      if (lookahead == 'd') ADVANCE(200);
      if (lookahead == 'e') ADVANCE(222);
      if (lookahead == 'f') ADVANCE(169);
      if (lookahead == 'i') ADVANCE(212);
      if (lookahead == 'm') ADVANCE(251);
      if (lookahead == 'n') ADVANCE(252);
      if (lookahead == 'o') ADVANCE(216);
      if (lookahead == 'q') ADVANCE(250);
      if (lookahead == 'r') ADVANCE(192);
      if (lookahead == 's') ADVANCE(179);
      if (lookahead == 't') ADVANCE(237);
      if (lookahead == 'u') ADVANCE(221);
      if (lookahead == '{') ADVANCE(265);
      if (lookahead == '|') ADVANCE(287);
      if (lookahead == '}') ADVANCE(266);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(302);
      END_STATE();
    case 1:
      if (lookahead == '!') ADVANCE(337);
      if (lookahead == '"') ADVANCE(297);
      if (lookahead == '#') ADVANCE(391);
      if (lookahead == '&') ADVANCE(281);
      if (lookahead == '(') ADVANCE(284);
      if (lookahead == ')') ADVANCE(285);
      if (lookahead == ',') ADVANCE(392);
      if (lookahead == '=') ADVANCE(286);
      if (lookahead == '@') ADVANCE(314);
      if (lookahead == '[') ADVANCE(311);
      if (lookahead == ']') ADVANCE(312);
      if (lookahead == '}') ADVANCE(266);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) SKIP(1)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(297);
      if (lookahead == '#') ADVANCE(391);
      if (lookahead == '$') ADVANCE(291);
      if (lookahead == ',') ADVANCE(392);
      if (lookahead == '-') ADVANCE(6);
      if (lookahead == '0') ADVANCE(301);
      if (lookahead == '[') ADVANCE(311);
      if (lookahead == ']') ADVANCE(312);
      if (lookahead == 'f') ADVANCE(338);
      if (lookahead == 'n') ADVANCE(384);
      if (lookahead == 't') ADVANCE(376);
      if (lookahead == '{') ADVANCE(265);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) SKIP(2)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(302);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(292);
      END_STATE();
    case 4:
      if (lookahead == '"') ADVANCE(260);
      if (lookahead != 0) ADVANCE(295);
      END_STATE();
    case 5:
      if (lookahead == '"') ADVANCE(296);
      if (lookahead == '#') ADVANCE(391);
      if (lookahead == ',') ADVANCE(392);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) SKIP(5)
      END_STATE();
    case 6:
      if (lookahead == '0') ADVANCE(301);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(302);
      END_STATE();
    case 7:
      if (lookahead == 'A') ADVANCE(326);
      END_STATE();
    case 8:
      if (lookahead == 'A') ADVANCE(136);
      END_STATE();
    case 9:
      if (lookahead == 'A') ADVANCE(64);
      END_STATE();
    case 10:
      if (lookahead == 'A') ADVANCE(95);
      if (lookahead == 'H') ADVANCE(41);
      END_STATE();
    case 11:
      if (lookahead == 'A') ADVANCE(20);
      END_STATE();
    case 12:
      if (lookahead == 'A') ADVANCE(148);
      END_STATE();
    case 13:
      if (lookahead == 'A') ADVANCE(25);
      END_STATE();
    case 14:
      if (lookahead == 'A') ADVANCE(96);
      END_STATE();
    case 15:
      if (lookahead == 'A') ADVANCE(28);
      END_STATE();
    case 16:
      if (lookahead == 'A') ADVANCE(135);
      END_STATE();
    case 17:
      if (lookahead == 'A') ADVANCE(65);
      END_STATE();
    case 18:
      if (lookahead == 'B') ADVANCE(89);
      END_STATE();
    case 19:
      if (lookahead == 'B') ADVANCE(140);
      END_STATE();
    case 20:
      if (lookahead == 'B') ADVANCE(94);
      END_STATE();
    case 21:
      if (lookahead == 'B') ADVANCE(90);
      END_STATE();
    case 22:
      if (lookahead == 'C') ADVANCE(10);
      if (lookahead == 'U') ADVANCE(19);
      END_STATE();
    case 23:
      if (lookahead == 'C') ADVANCE(142);
      END_STATE();
    case 24:
      if (lookahead == 'C') ADVANCE(143);
      END_STATE();
    case 25:
      if (lookahead == 'C') ADVANCE(37);
      END_STATE();
    case 26:
      if (lookahead == 'C') ADVANCE(137);
      END_STATE();
    case 27:
      if (lookahead == 'D') ADVANCE(321);
      END_STATE();
    case 28:
      if (lookahead == 'D') ADVANCE(323);
      END_STATE();
    case 29:
      if (lookahead == 'D') ADVANCE(42);
      END_STATE();
    case 30:
      if (lookahead == 'D') ADVANCE(50);
      END_STATE();
    case 31:
      if (lookahead == 'D') ADVANCE(51);
      if (lookahead == 'S') ADVANCE(130);
      END_STATE();
    case 32:
      if (lookahead == 'D') ADVANCE(52);
      END_STATE();
    case 33:
      if (lookahead == 'D') ADVANCE(53);
      END_STATE();
    case 34:
      if (lookahead == 'D') ADVANCE(168);
      END_STATE();
    case 35:
      if (lookahead == 'E') ADVANCE(91);
      END_STATE();
    case 36:
      if (lookahead == 'E') ADVANCE(23);
      END_STATE();
    case 37:
      if (lookahead == 'E') ADVANCE(331);
      END_STATE();
    case 38:
      if (lookahead == 'E') ADVANCE(334);
      END_STATE();
    case 39:
      if (lookahead == 'E') ADVANCE(165);
      END_STATE();
    case 40:
      if (lookahead == 'E') ADVANCE(133);
      END_STATE();
    case 41:
      if (lookahead == 'E') ADVANCE(98);
      END_STATE();
    case 42:
      if (lookahead == 'E') ADVANCE(58);
      END_STATE();
    case 43:
      if (lookahead == 'E') ADVANCE(134);
      END_STATE();
    case 44:
      if (lookahead == 'E') ADVANCE(113);
      END_STATE();
    case 45:
      if (lookahead == 'E') ADVANCE(115);
      END_STATE();
    case 46:
      if (lookahead == 'E') ADVANCE(15);
      END_STATE();
    case 47:
      if (lookahead == 'E') ADVANCE(117);
      END_STATE();
    case 48:
      if (lookahead == 'E') ADVANCE(93);
      END_STATE();
    case 49:
      if (lookahead == 'E') ADVANCE(24);
      END_STATE();
    case 50:
      if (lookahead == 'E') ADVANCE(59);
      END_STATE();
    case 51:
      if (lookahead == 'E') ADVANCE(60);
      END_STATE();
    case 52:
      if (lookahead == 'E') ADVANCE(61);
      END_STATE();
    case 53:
      if (lookahead == 'E') ADVANCE(62);
      END_STATE();
    case 54:
      if (lookahead == 'E') ADVANCE(167);
      END_STATE();
    case 55:
      if (lookahead == 'F') ADVANCE(79);
      if (lookahead == 'O') ADVANCE(21);
      END_STATE();
    case 56:
      if (lookahead == 'F') ADVANCE(139);
      END_STATE();
    case 57:
      if (lookahead == 'F') ADVANCE(13);
      END_STATE();
    case 58:
      if (lookahead == 'F') ADVANCE(71);
      END_STATE();
    case 59:
      if (lookahead == 'F') ADVANCE(85);
      END_STATE();
    case 60:
      if (lookahead == 'F') ADVANCE(86);
      END_STATE();
    case 61:
      if (lookahead == 'F') ADVANCE(87);
      END_STATE();
    case 62:
      if (lookahead == 'F') ADVANCE(88);
      END_STATE();
    case 63:
      if (lookahead == 'G') ADVANCE(159);
      END_STATE();
    case 64:
      if (lookahead == 'G') ADVANCE(100);
      END_STATE();
    case 65:
      if (lookahead == 'G') ADVANCE(101);
      END_STATE();
    case 66:
      if (lookahead == 'I') ADVANCE(35);
      if (lookahead == 'R') ADVANCE(9);
      END_STATE();
    case 67:
      if (lookahead == 'I') ADVANCE(122);
      END_STATE();
    case 68:
      if (lookahead == 'I') ADVANCE(131);
      END_STATE();
    case 69:
      if (lookahead == 'I') ADVANCE(114);
      END_STATE();
    case 70:
      if (lookahead == 'I') ADVANCE(11);
      END_STATE();
    case 71:
      if (lookahead == 'I') ADVANCE(116);
      END_STATE();
    case 72:
      if (lookahead == 'I') ADVANCE(123);
      END_STATE();
    case 73:
      if (lookahead == 'I') ADVANCE(124);
      END_STATE();
    case 74:
      if (lookahead == 'I') ADVANCE(125);
      END_STATE();
    case 75:
      if (lookahead == 'I') ADVANCE(126);
      END_STATE();
    case 76:
      if (lookahead == 'I') ADVANCE(127);
      END_STATE();
    case 77:
      if (lookahead == 'I') ADVANCE(128);
      END_STATE();
    case 78:
      if (lookahead == 'I') ADVANCE(129);
      END_STATE();
    case 79:
      if (lookahead == 'I') ADVANCE(48);
      END_STATE();
    case 80:
      if (lookahead == 'I') ADVANCE(150);
      END_STATE();
    case 81:
      if (lookahead == 'I') ADVANCE(151);
      END_STATE();
    case 82:
      if (lookahead == 'I') ADVANCE(152);
      END_STATE();
    case 83:
      if (lookahead == 'I') ADVANCE(153);
      END_STATE();
    case 84:
      if (lookahead == 'I') ADVANCE(154);
      END_STATE();
    case 85:
      if (lookahead == 'I') ADVANCE(118);
      END_STATE();
    case 86:
      if (lookahead == 'I') ADVANCE(119);
      END_STATE();
    case 87:
      if (lookahead == 'I') ADVANCE(120);
      END_STATE();
    case 88:
      if (lookahead == 'I') ADVANCE(121);
      END_STATE();
    case 89:
      if (lookahead == 'J') ADVANCE(36);
      END_STATE();
    case 90:
      if (lookahead == 'J') ADVANCE(49);
      END_STATE();
    case 91:
      if (lookahead == 'L') ADVANCE(27);
      END_STATE();
    case 92:
      if (lookahead == 'L') ADVANCE(69);
      if (lookahead == 'P') ADVANCE(158);
      if (lookahead == 'T') ADVANCE(43);
      END_STATE();
    case 93:
      if (lookahead == 'L') ADVANCE(34);
      END_STATE();
    case 94:
      if (lookahead == 'L') ADVANCE(54);
      END_STATE();
    case 95:
      if (lookahead == 'L') ADVANCE(16);
      END_STATE();
    case 96:
      if (lookahead == 'L') ADVANCE(160);
      END_STATE();
    case 97:
      if (lookahead == 'M') ADVANCE(333);
      END_STATE();
    case 98:
      if (lookahead == 'M') ADVANCE(7);
      END_STATE();
    case 99:
      if (lookahead == 'M') ADVANCE(44);
      END_STATE();
    case 100:
      if (lookahead == 'M') ADVANCE(45);
      END_STATE();
    case 101:
      if (lookahead == 'M') ADVANCE(47);
      END_STATE();
    case 102:
      if (lookahead == 'N') ADVANCE(92);
      END_STATE();
    case 103:
      if (lookahead == 'N') ADVANCE(67);
      END_STATE();
    case 104:
      if (lookahead == 'N') ADVANCE(332);
      END_STATE();
    case 105:
      if (lookahead == 'N') ADVANCE(319);
      END_STATE();
    case 106:
      if (lookahead == 'N') ADVANCE(320);
      END_STATE();
    case 107:
      if (lookahead == 'N') ADVANCE(329);
      END_STATE();
    case 108:
      if (lookahead == 'N') ADVANCE(330);
      END_STATE();
    case 109:
      if (lookahead == 'N') ADVANCE(322);
      END_STATE();
    case 110:
      if (lookahead == 'N') ADVANCE(325);
      END_STATE();
    case 111:
      if (lookahead == 'N') ADVANCE(336);
      END_STATE();
    case 112:
      if (lookahead == 'N') ADVANCE(156);
      END_STATE();
    case 113:
      if (lookahead == 'N') ADVANCE(146);
      END_STATE();
    case 114:
      if (lookahead == 'N') ADVANCE(39);
      END_STATE();
    case 115:
      if (lookahead == 'N') ADVANCE(147);
      END_STATE();
    case 116:
      if (lookahead == 'N') ADVANCE(80);
      END_STATE();
    case 117:
      if (lookahead == 'N') ADVANCE(144);
      END_STATE();
    case 118:
      if (lookahead == 'N') ADVANCE(81);
      END_STATE();
    case 119:
      if (lookahead == 'N') ADVANCE(82);
      END_STATE();
    case 120:
      if (lookahead == 'N') ADVANCE(83);
      END_STATE();
    case 121:
      if (lookahead == 'N') ADVANCE(84);
      END_STATE();
    case 122:
      if (lookahead == 'O') ADVANCE(104);
      END_STATE();
    case 123:
      if (lookahead == 'O') ADVANCE(105);
      END_STATE();
    case 124:
      if (lookahead == 'O') ADVANCE(106);
      END_STATE();
    case 125:
      if (lookahead == 'O') ADVANCE(107);
      END_STATE();
    case 126:
      if (lookahead == 'O') ADVANCE(108);
      END_STATE();
    case 127:
      if (lookahead == 'O') ADVANCE(109);
      END_STATE();
    case 128:
      if (lookahead == 'O') ADVANCE(110);
      END_STATE();
    case 129:
      if (lookahead == 'O') ADVANCE(111);
      END_STATE();
    case 130:
      if (lookahead == 'P') ADVANCE(138);
      END_STATE();
    case 131:
      if (lookahead == 'P') ADVANCE(149);
      END_STATE();
    case 132:
      if (lookahead == 'R') ADVANCE(63);
      END_STATE();
    case 133:
      if (lookahead == 'R') ADVANCE(162);
      END_STATE();
    case 134:
      if (lookahead == 'R') ADVANCE(57);
      END_STATE();
    case 135:
      if (lookahead == 'R') ADVANCE(327);
      END_STATE();
    case 136:
      if (lookahead == 'R') ADVANCE(70);
      END_STATE();
    case 137:
      if (lookahead == 'R') ADVANCE(68);
      END_STATE();
    case 138:
      if (lookahead == 'R') ADVANCE(46);
      END_STATE();
    case 139:
      if (lookahead == 'R') ADVANCE(17);
      END_STATE();
    case 140:
      if (lookahead == 'S') ADVANCE(26);
      END_STATE();
    case 141:
      if (lookahead == 'T') ADVANCE(163);
      END_STATE();
    case 142:
      if (lookahead == 'T') ADVANCE(328);
      END_STATE();
    case 143:
      if (lookahead == 'T') ADVANCE(335);
      END_STATE();
    case 144:
      if (lookahead == 'T') ADVANCE(324);
      END_STATE();
    case 145:
      if (lookahead == 'T') ADVANCE(12);
      END_STATE();
    case 146:
      if (lookahead == 'T') ADVANCE(166);
      END_STATE();
    case 147:
      if (lookahead == 'T') ADVANCE(164);
      END_STATE();
    case 148:
      if (lookahead == 'T') ADVANCE(72);
      END_STATE();
    case 149:
      if (lookahead == 'T') ADVANCE(73);
      END_STATE();
    case 150:
      if (lookahead == 'T') ADVANCE(74);
      END_STATE();
    case 151:
      if (lookahead == 'T') ADVANCE(75);
      END_STATE();
    case 152:
      if (lookahead == 'T') ADVANCE(76);
      END_STATE();
    case 153:
      if (lookahead == 'T') ADVANCE(77);
      END_STATE();
    case 154:
      if (lookahead == 'T') ADVANCE(78);
      END_STATE();
    case 155:
      if (lookahead == 'U') ADVANCE(145);
      END_STATE();
    case 156:
      if (lookahead == 'U') ADVANCE(97);
      END_STATE();
    case 157:
      if (lookahead == 'U') ADVANCE(40);
      END_STATE();
    case 158:
      if (lookahead == 'U') ADVANCE(141);
      END_STATE();
    case 159:
      if (lookahead == 'U') ADVANCE(99);
      END_STATE();
    case 160:
      if (lookahead == 'U') ADVANCE(38);
      END_STATE();
    case 161:
      if (lookahead == 'V') ADVANCE(14);
      END_STATE();
    case 162:
      if (lookahead == 'Y') ADVANCE(318);
      END_STATE();
    case 163:
      if (lookahead == '_') ADVANCE(55);
      END_STATE();
    case 164:
      if (lookahead == '_') ADVANCE(31);
      END_STATE();
    case 165:
      if (lookahead == '_') ADVANCE(56);
      END_STATE();
    case 166:
      if (lookahead == '_') ADVANCE(30);
      END_STATE();
    case 167:
      if (lookahead == '_') ADVANCE(32);
      END_STATE();
    case 168:
      if (lookahead == '_') ADVANCE(33);
      END_STATE();
    case 169:
      if (lookahead == 'a') ADVANCE(206);
      END_STATE();
    case 170:
      if (lookahead == 'a') ADVANCE(263);
      END_STATE();
    case 171:
      if (lookahead == 'a') ADVANCE(178);
      END_STATE();
    case 172:
      if (lookahead == 'a') ADVANCE(182);
      END_STATE();
    case 173:
      if (lookahead == 'a') ADVANCE(234);
      END_STATE();
    case 174:
      if (lookahead == 'a') ADVANCE(247);
      END_STATE();
    case 175:
      if (lookahead == 'a') ADVANCE(208);
      if (lookahead == 'h') ADVANCE(193);
      END_STATE();
    case 176:
      if (lookahead == 'a') ADVANCE(246);
      END_STATE();
    case 177:
      if (lookahead == 'b') ADVANCE(239);
      END_STATE();
    case 178:
      if (lookahead == 'b') ADVANCE(211);
      END_STATE();
    case 179:
      if (lookahead == 'c') ADVANCE(175);
      if (lookahead == 'u') ADVANCE(177);
      END_STATE();
    case 180:
      if (lookahead == 'c') ADVANCE(236);
      END_STATE();
    case 181:
      if (lookahead == 'c') ADVANCE(243);
      END_STATE();
    case 182:
      if (lookahead == 'c') ADVANCE(189);
      END_STATE();
    case 183:
      if (lookahead == 'd') ADVANCE(267);
      END_STATE();
    case 184:
      if (lookahead == 'e') ADVANCE(181);
      END_STATE();
    case 185:
      if (lookahead == 'e') ADVANCE(305);
      END_STATE();
    case 186:
      if (lookahead == 'e') ADVANCE(271);
      END_STATE();
    case 187:
      if (lookahead == 'e') ADVANCE(307);
      END_STATE();
    case 188:
      if (lookahead == 'e') ADVANCE(315);
      END_STATE();
    case 189:
      if (lookahead == 'e') ADVANCE(273);
      END_STATE();
    case 190:
      if (lookahead == 'e') ADVANCE(317);
      END_STATE();
    case 191:
      if (lookahead == 'e') ADVANCE(232);
      END_STATE();
    case 192:
      if (lookahead == 'e') ADVANCE(228);
      END_STATE();
    case 193:
      if (lookahead == 'e') ADVANCE(214);
      END_STATE();
    case 194:
      if (lookahead == 'e') ADVANCE(217);
      END_STATE();
    case 195:
      if (lookahead == 'e') ADVANCE(233);
      END_STATE();
    case 196:
      if (lookahead == 'e') ADVANCE(215);
      END_STATE();
    case 197:
      if (lookahead == 'e') ADVANCE(223);
      END_STATE();
    case 198:
      if (lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 199:
      if (lookahead == 'f') ADVANCE(172);
      END_STATE();
    case 200:
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 201:
      if (lookahead == 'i') ADVANCE(224);
      END_STATE();
    case 202:
      if (lookahead == 'i') ADVANCE(255);
      END_STATE();
    case 203:
      if (lookahead == 'i') ADVANCE(231);
      END_STATE();
    case 204:
      if (lookahead == 'i') ADVANCE(225);
      END_STATE();
    case 205:
      if (lookahead == 'i') ADVANCE(226);
      END_STATE();
    case 206:
      if (lookahead == 'l') ADVANCE(240);
      END_STATE();
    case 207:
      if (lookahead == 'l') ADVANCE(309);
      END_STATE();
    case 208:
      if (lookahead == 'l') ADVANCE(173);
      END_STATE();
    case 209:
      if (lookahead == 'l') ADVANCE(207);
      END_STATE();
    case 210:
      if (lookahead == 'l') ADVANCE(196);
      END_STATE();
    case 211:
      if (lookahead == 'l') ADVANCE(190);
      END_STATE();
    case 212:
      if (lookahead == 'm') ADVANCE(227);
      if (lookahead == 'n') ADVANCE(230);
      END_STATE();
    case 213:
      if (lookahead == 'm') ADVANCE(277);
      END_STATE();
    case 214:
      if (lookahead == 'm') ADVANCE(170);
      END_STATE();
    case 215:
      if (lookahead == 'm') ADVANCE(197);
      END_STATE();
    case 216:
      if (lookahead == 'n') ADVANCE(313);
      END_STATE();
    case 217:
      if (lookahead == 'n') ADVANCE(183);
      END_STATE();
    case 218:
      if (lookahead == 'n') ADVANCE(275);
      END_STATE();
    case 219:
      if (lookahead == 'n') ADVANCE(289);
      END_STATE();
    case 220:
      if (lookahead == 'n') ADVANCE(290);
      END_STATE();
    case 221:
      if (lookahead == 'n') ADVANCE(201);
      END_STATE();
    case 222:
      if (lookahead == 'n') ADVANCE(249);
      if (lookahead == 'x') ADVANCE(245);
      END_STATE();
    case 223:
      if (lookahead == 'n') ADVANCE(244);
      END_STATE();
    case 224:
      if (lookahead == 'o') ADVANCE(218);
      END_STATE();
    case 225:
      if (lookahead == 'o') ADVANCE(219);
      END_STATE();
    case 226:
      if (lookahead == 'o') ADVANCE(220);
      END_STATE();
    case 227:
      if (lookahead == 'p') ADVANCE(210);
      END_STATE();
    case 228:
      if (lookahead == 'p') ADVANCE(198);
      END_STATE();
    case 229:
      if (lookahead == 'p') ADVANCE(186);
      END_STATE();
    case 230:
      if (lookahead == 'p') ADVANCE(253);
      if (lookahead == 't') ADVANCE(195);
      END_STATE();
    case 231:
      if (lookahead == 'p') ADVANCE(248);
      END_STATE();
    case 232:
      if (lookahead == 'r') ADVANCE(256);
      END_STATE();
    case 233:
      if (lookahead == 'r') ADVANCE(199);
      END_STATE();
    case 234:
      if (lookahead == 'r') ADVANCE(269);
      END_STATE();
    case 235:
      if (lookahead == 'r') ADVANCE(184);
      END_STATE();
    case 236:
      if (lookahead == 'r') ADVANCE(203);
      END_STATE();
    case 237:
      if (lookahead == 'r') ADVANCE(254);
      if (lookahead == 'y') ADVANCE(229);
      END_STATE();
    case 238:
      if (lookahead == 's') ADVANCE(282);
      END_STATE();
    case 239:
      if (lookahead == 's') ADVANCE(180);
      END_STATE();
    case 240:
      if (lookahead == 's') ADVANCE(187);
      END_STATE();
    case 241:
      if (lookahead == 't') ADVANCE(279);
      END_STATE();
    case 242:
      if (lookahead == 't') ADVANCE(174);
      END_STATE();
    case 243:
      if (lookahead == 't') ADVANCE(202);
      END_STATE();
    case 244:
      if (lookahead == 't') ADVANCE(238);
      END_STATE();
    case 245:
      if (lookahead == 't') ADVANCE(194);
      END_STATE();
    case 246:
      if (lookahead == 't') ADVANCE(171);
      END_STATE();
    case 247:
      if (lookahead == 't') ADVANCE(204);
      END_STATE();
    case 248:
      if (lookahead == 't') ADVANCE(205);
      END_STATE();
    case 249:
      if (lookahead == 'u') ADVANCE(213);
      END_STATE();
    case 250:
      if (lookahead == 'u') ADVANCE(191);
      END_STATE();
    case 251:
      if (lookahead == 'u') ADVANCE(242);
      END_STATE();
    case 252:
      if (lookahead == 'u') ADVANCE(209);
      END_STATE();
    case 253:
      if (lookahead == 'u') ADVANCE(241);
      END_STATE();
    case 254:
      if (lookahead == 'u') ADVANCE(185);
      END_STATE();
    case 255:
      if (lookahead == 'v') ADVANCE(188);
      END_STATE();
    case 256:
      if (lookahead == 'y') ADVANCE(288);
      END_STATE();
    case 257:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(259);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(304);
      END_STATE();
    case 258:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(303);
      END_STATE();
    case 259:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(304);
      END_STATE();
    case 260:
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(295);
      END_STATE();
    case 261:
      if (eof) ADVANCE(262);
      if (lookahead == '"') ADVANCE(297);
      if (lookahead == '#') ADVANCE(391);
      if (lookahead == ',') ADVANCE(392);
      if (lookahead == 'd') ADVANCE(359);
      if (lookahead == 'e') ADVANCE(370);
      if (lookahead == 'i') ADVANCE(366);
      if (lookahead == 's') ADVANCE(343);
      if (lookahead == 't') ADVANCE(387);
      if (lookahead == 'u') ADVANCE(369);
      if (lookahead == '|') ADVANCE(287);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) SKIP(261)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 263:
      ACCEPT_TOKEN(anon_sym_schema);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(anon_sym_schema);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(anon_sym_extend);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(anon_sym_extend);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_scalar);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(anon_sym_scalar);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(anon_sym_type);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_interface);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(anon_sym_union);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(anon_sym_union);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(anon_sym_enum);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_enum);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_input);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_implements);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_query);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_mutation);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(anon_sym_subscription);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym_DOLLAR);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(anon_sym_DQUOTE_DQUOTE_DQUOTE);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead == '\n') ADVANCE(295);
      if (lookahead == '"') ADVANCE(390);
      if (lookahead != 0) ADVANCE(293);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) ADVANCE(294);
      if (lookahead == '"') ADVANCE(4);
      if (lookahead == '#') ADVANCE(293);
      if (lookahead == ',') ADVANCE(394);
      if (lookahead != 0) ADVANCE(295);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(295);
      if (lookahead == '"') ADVANCE(4);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      if (lookahead == '"') ADVANCE(3);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead == '#') ADVANCE(299);
      if (lookahead == ',') ADVANCE(393);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == 65279) ADVANCE(298);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(300);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead == '"' ||
          lookahead == '\\') ADVANCE(391);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(299);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(300);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(sym_int_value);
      if (lookahead == '.') ADVANCE(258);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(sym_int_value);
      if (lookahead == '.') ADVANCE(258);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(257);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(302);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(sym_float_value);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(257);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(303);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(sym_float_value);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(304);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(sym_null_value);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(sym_null_value);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(anon_sym_on);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(anon_sym_directive);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_directive);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(anon_sym_repeatable);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(anon_sym_QUERY);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(anon_sym_MUTATION);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(anon_sym_SUBSCRIPTION);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(anon_sym_FIELD);
      if (lookahead == '_') ADVANCE(29);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(anon_sym_FRAGMENT_DEFINITION);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(anon_sym_FRAGMENT_SPREAD);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(anon_sym_INLINE_FRAGMENT);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(anon_sym_VARIABLE_DEFINITION);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(anon_sym_SCHEMA);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(anon_sym_SCALAR);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(anon_sym_OBJECT);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(anon_sym_FIELD_DEFINITION);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(anon_sym_ARGUMENT_DEFINITION);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(anon_sym_INTERFACE);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(anon_sym_UNION);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_ENUM);
      if (lookahead == '_') ADVANCE(161);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(anon_sym_ENUM_VALUE);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(anon_sym_INPUT_OBJECT);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_INPUT_FIELD_DEFINITION);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'a') ADVANCE(360);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'a') ADVANCE(264);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'a') ADVANCE(345);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'a') ADVANCE(375);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'a') ADVANCE(362);
      if (lookahead == 'h') ADVANCE(353);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'c') ADVANCE(342);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'c') ADVANCE(380);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'c') ADVANCE(351);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'd') ADVANCE(268);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(306);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(308);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(272);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(316);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(274);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(344);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(365);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(374);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'e') ADVANCE(367);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'f') ADVANCE(340);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'i') ADVANCE(371);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'i') ADVANCE(386);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'i') ADVANCE(377);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'l') ADVANCE(378);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'l') ADVANCE(310);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'l') ADVANCE(341);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'l') ADVANCE(361);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'm') ADVANCE(278);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'm') ADVANCE(339);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'n') ADVANCE(372);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'n') ADVANCE(346);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'n') ADVANCE(276);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'n') ADVANCE(357);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'n') ADVANCE(383);
      if (lookahead == 'x') ADVANCE(381);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'o') ADVANCE(368);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'p') ADVANCE(385);
      if (lookahead == 't') ADVANCE(354);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'p') ADVANCE(349);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'r') ADVANCE(356);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'r') ADVANCE(270);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'r') ADVANCE(382);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'r') ADVANCE(352);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 's') ADVANCE(348);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 't') ADVANCE(280);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 't') ADVANCE(358);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 't') ADVANCE(355);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'u') ADVANCE(347);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'u') ADVANCE(364);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'u') ADVANCE(363);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'u') ADVANCE(379);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'v') ADVANCE(350);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(sym_name);
      if (lookahead == 'y') ADVANCE(373);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(sym_name);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(388);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\n') ADVANCE(295);
      if (lookahead == '"') ADVANCE(391);
      if (lookahead != 0) ADVANCE(293);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\n') ADVANCE(295);
      if (lookahead == '"') ADVANCE(389);
      if (lookahead != 0) ADVANCE(293);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(391);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(sym_comma);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(sym_comma);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(300);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(sym_comma);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(295);
      if (lookahead == '"') ADVANCE(4);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 2},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 2},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 2},
  [39] = {.lex_state = 2},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 261},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 261},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 2},
  [105] = {.lex_state = 2},
  [106] = {.lex_state = 2},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 2},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 2},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 2},
  [120] = {.lex_state = 2},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 2},
  [123] = {.lex_state = 0},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 2},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 0},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 1},
  [144] = {.lex_state = 1},
  [145] = {.lex_state = 1},
  [146] = {.lex_state = 1},
  [147] = {.lex_state = 1},
  [148] = {.lex_state = 1},
  [149] = {.lex_state = 1},
  [150] = {.lex_state = 1},
  [151] = {.lex_state = 1},
  [152] = {.lex_state = 1},
  [153] = {.lex_state = 1},
  [154] = {.lex_state = 1},
  [155] = {.lex_state = 1},
  [156] = {.lex_state = 1},
  [157] = {.lex_state = 1},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 1},
  [161] = {.lex_state = 0},
  [162] = {.lex_state = 1},
  [163] = {.lex_state = 1},
  [164] = {.lex_state = 1},
  [165] = {.lex_state = 1},
  [166] = {.lex_state = 1},
  [167] = {.lex_state = 1},
  [168] = {.lex_state = 1},
  [169] = {.lex_state = 1},
  [170] = {.lex_state = 1},
  [171] = {.lex_state = 1},
  [172] = {.lex_state = 0},
  [173] = {.lex_state = 1},
  [174] = {.lex_state = 0},
  [175] = {.lex_state = 1},
  [176] = {.lex_state = 0},
  [177] = {.lex_state = 0},
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 1},
  [180] = {.lex_state = 1},
  [181] = {.lex_state = 1},
  [182] = {.lex_state = 1},
  [183] = {.lex_state = 1},
  [184] = {.lex_state = 1},
  [185] = {.lex_state = 1},
  [186] = {.lex_state = 1},
  [187] = {.lex_state = 1},
  [188] = {.lex_state = 1},
  [189] = {.lex_state = 0},
  [190] = {.lex_state = 1},
  [191] = {.lex_state = 0},
  [192] = {.lex_state = 1},
  [193] = {.lex_state = 1},
  [194] = {.lex_state = 1},
  [195] = {.lex_state = 1},
  [196] = {.lex_state = 1},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 1},
  [199] = {.lex_state = 1},
  [200] = {.lex_state = 1},
  [201] = {.lex_state = 1},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 0},
  [204] = {.lex_state = 0},
  [205] = {.lex_state = 1},
  [206] = {.lex_state = 0},
  [207] = {.lex_state = 0},
  [208] = {.lex_state = 1},
  [209] = {.lex_state = 0},
  [210] = {.lex_state = 1},
  [211] = {.lex_state = 1},
  [212] = {.lex_state = 1},
  [213] = {.lex_state = 0},
  [214] = {.lex_state = 1},
  [215] = {.lex_state = 1},
  [216] = {.lex_state = 1},
  [217] = {.lex_state = 1},
  [218] = {.lex_state = 1},
  [219] = {.lex_state = 1},
  [220] = {.lex_state = 1},
  [221] = {.lex_state = 1},
  [222] = {.lex_state = 0},
  [223] = {.lex_state = 1},
  [224] = {.lex_state = 1},
  [225] = {.lex_state = 1},
  [226] = {.lex_state = 1},
  [227] = {.lex_state = 0},
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 1},
  [230] = {.lex_state = 0},
  [231] = {.lex_state = 1},
  [232] = {.lex_state = 1},
  [233] = {.lex_state = 1},
  [234] = {.lex_state = 0},
  [235] = {.lex_state = 1},
  [236] = {.lex_state = 1},
  [237] = {.lex_state = 1},
  [238] = {.lex_state = 1},
  [239] = {.lex_state = 1},
  [240] = {.lex_state = 1},
  [241] = {.lex_state = 1},
  [242] = {.lex_state = 1},
  [243] = {.lex_state = 1},
  [244] = {.lex_state = 1},
  [245] = {.lex_state = 1},
  [246] = {.lex_state = 1},
  [247] = {.lex_state = 1},
  [248] = {.lex_state = 0},
  [249] = {.lex_state = 1},
  [250] = {.lex_state = 0},
  [251] = {.lex_state = 0},
  [252] = {.lex_state = 294},
  [253] = {.lex_state = 0},
  [254] = {.lex_state = 1},
  [255] = {.lex_state = 1},
  [256] = {.lex_state = 0},
  [257] = {.lex_state = 0},
  [258] = {.lex_state = 1},
  [259] = {.lex_state = 1},
  [260] = {.lex_state = 0},
  [261] = {.lex_state = 1},
  [262] = {.lex_state = 0},
  [263] = {.lex_state = 1},
  [264] = {.lex_state = 1},
  [265] = {.lex_state = 1},
  [266] = {.lex_state = 0},
  [267] = {.lex_state = 1},
  [268] = {.lex_state = 0},
  [269] = {.lex_state = 1},
  [270] = {.lex_state = 1},
  [271] = {.lex_state = 0},
  [272] = {.lex_state = 0},
  [273] = {.lex_state = 1},
  [274] = {.lex_state = 1},
  [275] = {.lex_state = 0},
  [276] = {.lex_state = 0},
  [277] = {.lex_state = 0},
  [278] = {.lex_state = 298},
  [279] = {.lex_state = 1},
  [280] = {.lex_state = 0},
  [281] = {.lex_state = 1},
  [282] = {.lex_state = 1},
  [283] = {.lex_state = 0},
  [284] = {.lex_state = 1},
  [285] = {.lex_state = 1},
  [286] = {.lex_state = 5},
  [287] = {.lex_state = 0},
  [288] = {.lex_state = 0},
  [289] = {.lex_state = 1},
  [290] = {.lex_state = 0},
  [291] = {.lex_state = 5},
  [292] = {.lex_state = 1},
  [293] = {.lex_state = 1},
  [294] = {.lex_state = 1},
  [295] = {.lex_state = 1},
  [296] = {.lex_state = 0},
  [297] = {.lex_state = 1},
  [298] = {.lex_state = 0},
  [299] = {.lex_state = 5},
  [300] = {.lex_state = 1},
  [301] = {.lex_state = 1},
  [302] = {.lex_state = 1},
  [303] = {.lex_state = 1},
  [304] = {.lex_state = 0},
  [305] = {.lex_state = 0},
  [306] = {.lex_state = 5},
  [307] = {.lex_state = 294},
  [308] = {.lex_state = 298},
  [309] = {.lex_state = 0},
  [310] = {.lex_state = 294},
  [311] = {.lex_state = 298},
  [312] = {.lex_state = 294},
  [313] = {.lex_state = 298},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_schema] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_extend] = ACTIONS(1),
    [anon_sym_scalar] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_interface] = ACTIONS(1),
    [anon_sym_union] = ACTIONS(1),
    [anon_sym_enum] = ACTIONS(1),
    [anon_sym_input] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_implements] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_query] = ACTIONS(1),
    [anon_sym_mutation] = ACTIONS(1),
    [anon_sym_subscription] = ACTIONS(1),
    [anon_sym_DOLLAR] = ACTIONS(1),
    [anon_sym_DQUOTE_DQUOTE_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [sym_int_value] = ACTIONS(1),
    [sym_float_value] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_null_value] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_on] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_directive] = ACTIONS(1),
    [anon_sym_repeatable] = ACTIONS(1),
    [anon_sym_QUERY] = ACTIONS(1),
    [anon_sym_MUTATION] = ACTIONS(1),
    [anon_sym_SUBSCRIPTION] = ACTIONS(1),
    [anon_sym_FIELD] = ACTIONS(1),
    [anon_sym_FRAGMENT_DEFINITION] = ACTIONS(1),
    [anon_sym_FRAGMENT_SPREAD] = ACTIONS(1),
    [anon_sym_INLINE_FRAGMENT] = ACTIONS(1),
    [anon_sym_VARIABLE_DEFINITION] = ACTIONS(1),
    [anon_sym_SCHEMA] = ACTIONS(1),
    [anon_sym_SCALAR] = ACTIONS(1),
    [anon_sym_OBJECT] = ACTIONS(1),
    [anon_sym_FIELD_DEFINITION] = ACTIONS(1),
    [anon_sym_ARGUMENT_DEFINITION] = ACTIONS(1),
    [anon_sym_INTERFACE] = ACTIONS(1),
    [anon_sym_UNION] = ACTIONS(1),
    [anon_sym_ENUM] = ACTIONS(1),
    [anon_sym_ENUM_VALUE] = ACTIONS(1),
    [anon_sym_INPUT_OBJECT] = ACTIONS(1),
    [anon_sym_INPUT_FIELD_DEFINITION] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym_comma] = ACTIONS(3),
  },
  [1] = {
    [sym_document] = STATE(251),
    [sym_item] = STATE(3),
    [sym_schema_definition] = STATE(132),
    [sym_schema_extension] = STATE(132),
    [sym_type_extension] = STATE(132),
    [sym_scalar_type_extension] = STATE(128),
    [sym_object_type_extension] = STATE(128),
    [sym_interface_type_extension] = STATE(128),
    [sym_union_type_extension] = STATE(128),
    [sym_enum_type_extension] = STATE(128),
    [sym_input_object_type_extension] = STATE(128),
    [sym_type_definition] = STATE(132),
    [sym_scalar_type_definition] = STATE(112),
    [sym_object_type_definition] = STATE(112),
    [sym_interface_type_definition] = STATE(112),
    [sym_union_type_definition] = STATE(112),
    [sym_enum_type_definition] = STATE(112),
    [sym_input_object_type_definition] = STATE(112),
    [sym_string_value] = STATE(161),
    [sym_directive_definition] = STATE(132),
    [sym_description] = STATE(159),
    [aux_sym_document_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_schema] = ACTIONS(7),
    [anon_sym_extend] = ACTIONS(9),
    [anon_sym_scalar] = ACTIONS(11),
    [anon_sym_type] = ACTIONS(13),
    [anon_sym_interface] = ACTIONS(15),
    [anon_sym_union] = ACTIONS(17),
    [anon_sym_enum] = ACTIONS(19),
    [anon_sym_input] = ACTIONS(21),
    [anon_sym_DQUOTE_DQUOTE_DQUOTE] = ACTIONS(23),
    [anon_sym_DQUOTE] = ACTIONS(25),
    [anon_sym_directive] = ACTIONS(27),
    [sym_comment] = ACTIONS(3),
    [sym_comma] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 19,
    ACTIONS(29), 1,
      ts_builtin_sym_end,
    ACTIONS(31), 1,
      anon_sym_schema,
    ACTIONS(34), 1,
      anon_sym_extend,
    ACTIONS(37), 1,
      anon_sym_scalar,
    ACTIONS(40), 1,
      anon_sym_type,
    ACTIONS(43), 1,
      anon_sym_interface,
    ACTIONS(46), 1,
      anon_sym_union,
    ACTIONS(49), 1,
      anon_sym_enum,
    ACTIONS(52), 1,
      anon_sym_input,
    ACTIONS(55), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(58), 1,
      anon_sym_DQUOTE,
    ACTIONS(61), 1,
      anon_sym_directive,
    STATE(159), 1,
      sym_description,
    STATE(161), 1,
      sym_string_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(2), 2,
      sym_item,
      aux_sym_document_repeat1,
    STATE(132), 5,
      sym_schema_definition,
      sym_schema_extension,
      sym_type_extension,
      sym_type_definition,
      sym_directive_definition,
    STATE(112), 6,
      sym_scalar_type_definition,
      sym_object_type_definition,
      sym_interface_type_definition,
      sym_union_type_definition,
      sym_enum_type_definition,
      sym_input_object_type_definition,
    STATE(128), 6,
      sym_scalar_type_extension,
      sym_object_type_extension,
      sym_interface_type_extension,
      sym_union_type_extension,
      sym_enum_type_extension,
      sym_input_object_type_extension,
  [74] = 19,
    ACTIONS(7), 1,
      anon_sym_schema,
    ACTIONS(9), 1,
      anon_sym_extend,
    ACTIONS(11), 1,
      anon_sym_scalar,
    ACTIONS(13), 1,
      anon_sym_type,
    ACTIONS(15), 1,
      anon_sym_interface,
    ACTIONS(17), 1,
      anon_sym_union,
    ACTIONS(19), 1,
      anon_sym_enum,
    ACTIONS(21), 1,
      anon_sym_input,
    ACTIONS(23), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_directive,
    ACTIONS(64), 1,
      ts_builtin_sym_end,
    STATE(159), 1,
      sym_description,
    STATE(161), 1,
      sym_string_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(2), 2,
      sym_item,
      aux_sym_document_repeat1,
    STATE(132), 5,
      sym_schema_definition,
      sym_schema_extension,
      sym_type_extension,
      sym_type_definition,
      sym_directive_definition,
    STATE(112), 6,
      sym_scalar_type_definition,
      sym_object_type_definition,
      sym_interface_type_definition,
      sym_union_type_definition,
      sym_enum_type_definition,
      sym_input_object_type_definition,
    STATE(128), 6,
      sym_scalar_type_extension,
      sym_object_type_extension,
      sym_interface_type_extension,
      sym_union_type_extension,
      sym_enum_type_extension,
      sym_input_object_type_extension,
  [148] = 9,
    ACTIONS(66), 1,
      anon_sym_PIPE,
    ACTIONS(70), 1,
      anon_sym_FIELD,
    ACTIONS(74), 1,
      anon_sym_ENUM,
    STATE(83), 1,
      sym_directive_location,
    STATE(87), 1,
      sym_directive_locations,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(81), 2,
      sym_executable_directive_location,
      sym_type_system_directive_location,
    ACTIONS(68), 7,
      anon_sym_QUERY,
      anon_sym_MUTATION,
      anon_sym_SUBSCRIPTION,
      anon_sym_FRAGMENT_DEFINITION,
      anon_sym_FRAGMENT_SPREAD,
      anon_sym_INLINE_FRAGMENT,
      anon_sym_VARIABLE_DEFINITION,
    ACTIONS(72), 10,
      anon_sym_SCHEMA,
      anon_sym_SCALAR,
      anon_sym_OBJECT,
      anon_sym_FIELD_DEFINITION,
      anon_sym_ARGUMENT_DEFINITION,
      anon_sym_INTERFACE,
      anon_sym_UNION,
      anon_sym_ENUM_VALUE,
      anon_sym_INPUT_OBJECT,
      anon_sym_INPUT_FIELD_DEFINITION,
  [193] = 9,
    ACTIONS(66), 1,
      anon_sym_PIPE,
    ACTIONS(70), 1,
      anon_sym_FIELD,
    ACTIONS(74), 1,
      anon_sym_ENUM,
    STATE(83), 1,
      sym_directive_location,
    STATE(85), 1,
      sym_directive_locations,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(81), 2,
      sym_executable_directive_location,
      sym_type_system_directive_location,
    ACTIONS(68), 7,
      anon_sym_QUERY,
      anon_sym_MUTATION,
      anon_sym_SUBSCRIPTION,
      anon_sym_FRAGMENT_DEFINITION,
      anon_sym_FRAGMENT_SPREAD,
      anon_sym_INLINE_FRAGMENT,
      anon_sym_VARIABLE_DEFINITION,
    ACTIONS(72), 10,
      anon_sym_SCHEMA,
      anon_sym_SCALAR,
      anon_sym_OBJECT,
      anon_sym_FIELD_DEFINITION,
      anon_sym_ARGUMENT_DEFINITION,
      anon_sym_INTERFACE,
      anon_sym_UNION,
      anon_sym_ENUM_VALUE,
      anon_sym_INPUT_OBJECT,
      anon_sym_INPUT_FIELD_DEFINITION,
  [238] = 9,
    ACTIONS(66), 1,
      anon_sym_PIPE,
    ACTIONS(70), 1,
      anon_sym_FIELD,
    ACTIONS(74), 1,
      anon_sym_ENUM,
    STATE(83), 1,
      sym_directive_location,
    STATE(89), 1,
      sym_directive_locations,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(81), 2,
      sym_executable_directive_location,
      sym_type_system_directive_location,
    ACTIONS(68), 7,
      anon_sym_QUERY,
      anon_sym_MUTATION,
      anon_sym_SUBSCRIPTION,
      anon_sym_FRAGMENT_DEFINITION,
      anon_sym_FRAGMENT_SPREAD,
      anon_sym_INLINE_FRAGMENT,
      anon_sym_VARIABLE_DEFINITION,
    ACTIONS(72), 10,
      anon_sym_SCHEMA,
      anon_sym_SCALAR,
      anon_sym_OBJECT,
      anon_sym_FIELD_DEFINITION,
      anon_sym_ARGUMENT_DEFINITION,
      anon_sym_INTERFACE,
      anon_sym_UNION,
      anon_sym_ENUM_VALUE,
      anon_sym_INPUT_OBJECT,
      anon_sym_INPUT_FIELD_DEFINITION,
  [283] = 9,
    ACTIONS(66), 1,
      anon_sym_PIPE,
    ACTIONS(70), 1,
      anon_sym_FIELD,
    ACTIONS(74), 1,
      anon_sym_ENUM,
    STATE(82), 1,
      sym_directive_locations,
    STATE(83), 1,
      sym_directive_location,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(81), 2,
      sym_executable_directive_location,
      sym_type_system_directive_location,
    ACTIONS(68), 7,
      anon_sym_QUERY,
      anon_sym_MUTATION,
      anon_sym_SUBSCRIPTION,
      anon_sym_FRAGMENT_DEFINITION,
      anon_sym_FRAGMENT_SPREAD,
      anon_sym_INLINE_FRAGMENT,
      anon_sym_VARIABLE_DEFINITION,
    ACTIONS(72), 10,
      anon_sym_SCHEMA,
      anon_sym_SCALAR,
      anon_sym_OBJECT,
      anon_sym_FIELD_DEFINITION,
      anon_sym_ARGUMENT_DEFINITION,
      anon_sym_INTERFACE,
      anon_sym_UNION,
      anon_sym_ENUM_VALUE,
      anon_sym_INPUT_OBJECT,
      anon_sym_INPUT_FIELD_DEFINITION,
  [328] = 7,
    ACTIONS(70), 1,
      anon_sym_FIELD,
    ACTIONS(74), 1,
      anon_sym_ENUM,
    STATE(86), 1,
      sym_directive_location,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(81), 2,
      sym_executable_directive_location,
      sym_type_system_directive_location,
    ACTIONS(68), 7,
      anon_sym_QUERY,
      anon_sym_MUTATION,
      anon_sym_SUBSCRIPTION,
      anon_sym_FRAGMENT_DEFINITION,
      anon_sym_FRAGMENT_SPREAD,
      anon_sym_INLINE_FRAGMENT,
      anon_sym_VARIABLE_DEFINITION,
    ACTIONS(72), 10,
      anon_sym_SCHEMA,
      anon_sym_SCALAR,
      anon_sym_OBJECT,
      anon_sym_FIELD_DEFINITION,
      anon_sym_ARGUMENT_DEFINITION,
      anon_sym_INTERFACE,
      anon_sym_UNION,
      anon_sym_ENUM_VALUE,
      anon_sym_INPUT_OBJECT,
      anon_sym_INPUT_FIELD_DEFINITION,
  [367] = 7,
    ACTIONS(70), 1,
      anon_sym_FIELD,
    ACTIONS(74), 1,
      anon_sym_ENUM,
    STATE(88), 1,
      sym_directive_location,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(81), 2,
      sym_executable_directive_location,
      sym_type_system_directive_location,
    ACTIONS(68), 7,
      anon_sym_QUERY,
      anon_sym_MUTATION,
      anon_sym_SUBSCRIPTION,
      anon_sym_FRAGMENT_DEFINITION,
      anon_sym_FRAGMENT_SPREAD,
      anon_sym_INLINE_FRAGMENT,
      anon_sym_VARIABLE_DEFINITION,
    ACTIONS(72), 10,
      anon_sym_SCHEMA,
      anon_sym_SCALAR,
      anon_sym_OBJECT,
      anon_sym_FIELD_DEFINITION,
      anon_sym_ARGUMENT_DEFINITION,
      anon_sym_INTERFACE,
      anon_sym_UNION,
      anon_sym_ENUM_VALUE,
      anon_sym_INPUT_OBJECT,
      anon_sym_INPUT_FIELD_DEFINITION,
  [406] = 3,
    ACTIONS(78), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(76), 21,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_RBRACK,
      anon_sym_AT,
      anon_sym_directive,
      anon_sym_BANG,
  [437] = 13,
    ACTIONS(80), 1,
      anon_sym_LBRACE,
    ACTIONS(82), 1,
      anon_sym_DOLLAR,
    ACTIONS(84), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(86), 1,
      anon_sym_DQUOTE,
    ACTIONS(90), 1,
      sym_float_value,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(96), 1,
      anon_sym_RBRACK,
    ACTIONS(98), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(88), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(92), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(16), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [486] = 10,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(104), 1,
      anon_sym_implements,
    ACTIONS(106), 1,
      anon_sym_DQUOTE,
    ACTIONS(108), 1,
      anon_sym_AT,
    STATE(26), 1,
      sym_implements_interfaces,
    STATE(64), 1,
      sym_directives,
    STATE(114), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(100), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [529] = 10,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(104), 1,
      anon_sym_implements,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(112), 1,
      anon_sym_DQUOTE,
    STATE(28), 1,
      sym_implements_interfaces,
    STATE(68), 1,
      sym_directives,
    STATE(95), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(110), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [572] = 13,
    ACTIONS(80), 1,
      anon_sym_LBRACE,
    ACTIONS(82), 1,
      anon_sym_DOLLAR,
    ACTIONS(84), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(86), 1,
      anon_sym_DQUOTE,
    ACTIONS(90), 1,
      sym_float_value,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(98), 1,
      sym_name,
    ACTIONS(114), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(88), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(92), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [621] = 10,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(104), 1,
      anon_sym_implements,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(118), 1,
      anon_sym_DQUOTE,
    STATE(24), 1,
      sym_implements_interfaces,
    STATE(60), 1,
      sym_directives,
    STATE(124), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(116), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [664] = 13,
    ACTIONS(80), 1,
      anon_sym_LBRACE,
    ACTIONS(82), 1,
      anon_sym_DOLLAR,
    ACTIONS(84), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(86), 1,
      anon_sym_DQUOTE,
    ACTIONS(90), 1,
      sym_float_value,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(98), 1,
      sym_name,
    ACTIONS(120), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(88), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(92), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [713] = 10,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(104), 1,
      anon_sym_implements,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(124), 1,
      anon_sym_DQUOTE,
    STATE(29), 1,
      sym_implements_interfaces,
    STATE(79), 1,
      sym_directives,
    STATE(123), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(122), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [756] = 13,
    ACTIONS(126), 1,
      anon_sym_LBRACE,
    ACTIONS(129), 1,
      anon_sym_DOLLAR,
    ACTIONS(132), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(135), 1,
      anon_sym_DQUOTE,
    ACTIONS(141), 1,
      sym_float_value,
    ACTIONS(147), 1,
      anon_sym_LBRACK,
    ACTIONS(150), 1,
      anon_sym_RBRACK,
    ACTIONS(152), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(138), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(144), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [805] = 13,
    ACTIONS(80), 1,
      anon_sym_LBRACE,
    ACTIONS(82), 1,
      anon_sym_DOLLAR,
    ACTIONS(84), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(86), 1,
      anon_sym_DQUOTE,
    ACTIONS(90), 1,
      sym_float_value,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(98), 1,
      sym_name,
    ACTIONS(155), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(88), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(92), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(22), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [854] = 10,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(104), 1,
      anon_sym_implements,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(159), 1,
      anon_sym_DQUOTE,
    STATE(25), 1,
      sym_implements_interfaces,
    STATE(61), 1,
      sym_directives,
    STATE(115), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(157), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [897] = 10,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(104), 1,
      anon_sym_implements,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(163), 1,
      anon_sym_DQUOTE,
    STATE(27), 1,
      sym_implements_interfaces,
    STATE(71), 1,
      sym_directives,
    STATE(96), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(161), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [940] = 13,
    ACTIONS(80), 1,
      anon_sym_LBRACE,
    ACTIONS(82), 1,
      anon_sym_DOLLAR,
    ACTIONS(84), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(86), 1,
      anon_sym_DQUOTE,
    ACTIONS(90), 1,
      sym_float_value,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(98), 1,
      sym_name,
    ACTIONS(165), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(88), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(92), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(18), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [989] = 13,
    ACTIONS(80), 1,
      anon_sym_LBRACE,
    ACTIONS(82), 1,
      anon_sym_DOLLAR,
    ACTIONS(84), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(86), 1,
      anon_sym_DQUOTE,
    ACTIONS(90), 1,
      sym_float_value,
    ACTIONS(94), 1,
      anon_sym_LBRACK,
    ACTIONS(98), 1,
      sym_name,
    ACTIONS(167), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(88), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(92), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(14), 2,
      sym_value,
      aux_sym_list_value_repeat1,
    STATE(122), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [1038] = 9,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(171), 1,
      anon_sym_AMP,
    ACTIONS(173), 1,
      anon_sym_DQUOTE,
    STATE(69), 1,
      sym_directives,
    STATE(133), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(169), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1078] = 9,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(171), 1,
      anon_sym_AMP,
    ACTIONS(177), 1,
      anon_sym_DQUOTE,
    STATE(75), 1,
      sym_directives,
    STATE(93), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(175), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1118] = 9,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(171), 1,
      anon_sym_AMP,
    ACTIONS(181), 1,
      anon_sym_DQUOTE,
    STATE(67), 1,
      sym_directives,
    STATE(139), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(179), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1158] = 9,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(118), 1,
      anon_sym_DQUOTE,
    ACTIONS(171), 1,
      anon_sym_AMP,
    STATE(60), 1,
      sym_directives,
    STATE(124), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(116), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1198] = 9,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(124), 1,
      anon_sym_DQUOTE,
    ACTIONS(171), 1,
      anon_sym_AMP,
    STATE(79), 1,
      sym_directives,
    STATE(123), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(122), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1238] = 9,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(171), 1,
      anon_sym_AMP,
    ACTIONS(185), 1,
      anon_sym_DQUOTE,
    STATE(66), 1,
      sym_directives,
    STATE(135), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(183), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1278] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(191), 1,
      anon_sym_DQUOTE,
    STATE(55), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    STATE(52), 2,
      sym_input_fields_definition,
      aux_sym_input_object_type_extension_repeat1,
    ACTIONS(187), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1316] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(195), 1,
      anon_sym_EQ,
    ACTIONS(197), 1,
      anon_sym_DQUOTE,
    STATE(72), 1,
      sym_directives,
    STATE(125), 1,
      sym_union_member_types,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(193), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1353] = 12,
    ACTIONS(199), 1,
      anon_sym_LBRACE,
    ACTIONS(201), 1,
      anon_sym_DOLLAR,
    ACTIONS(203), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(205), 1,
      anon_sym_DQUOTE,
    ACTIONS(209), 1,
      sym_float_value,
    ACTIONS(213), 1,
      anon_sym_LBRACK,
    ACTIONS(215), 1,
      sym_name,
    STATE(249), 1,
      sym_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(207), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(211), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(242), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [1398] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(195), 1,
      anon_sym_EQ,
    ACTIONS(219), 1,
      anon_sym_DQUOTE,
    STATE(58), 1,
      sym_directives,
    STATE(97), 1,
      sym_union_member_types,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(217), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1435] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(223), 1,
      anon_sym_LBRACE,
    ACTIONS(225), 1,
      anon_sym_DQUOTE,
    STATE(77), 1,
      sym_directives,
    STATE(99), 1,
      sym_enum_values_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(221), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1472] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(229), 1,
      anon_sym_DQUOTE,
    STATE(80), 1,
      sym_directives,
    STATE(101), 1,
      sym_input_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(227), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1509] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(223), 1,
      anon_sym_LBRACE,
    ACTIONS(233), 1,
      anon_sym_DQUOTE,
    STATE(65), 1,
      sym_directives,
    STATE(117), 1,
      sym_enum_values_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(231), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1546] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(195), 1,
      anon_sym_EQ,
    ACTIONS(237), 1,
      anon_sym_DQUOTE,
    STATE(59), 1,
      sym_directives,
    STATE(116), 1,
      sym_union_member_types,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(235), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1583] = 12,
    ACTIONS(239), 1,
      anon_sym_LBRACE,
    ACTIONS(241), 1,
      anon_sym_DOLLAR,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(249), 1,
      sym_float_value,
    ACTIONS(253), 1,
      anon_sym_LBRACK,
    ACTIONS(255), 1,
      sym_name,
    STATE(236), 1,
      sym_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(247), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(251), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(186), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [1628] = 12,
    ACTIONS(239), 1,
      anon_sym_LBRACE,
    ACTIONS(241), 1,
      anon_sym_DOLLAR,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(249), 1,
      sym_float_value,
    ACTIONS(253), 1,
      anon_sym_LBRACK,
    ACTIONS(255), 1,
      sym_name,
    STATE(199), 1,
      sym_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(247), 2,
      sym_int_value,
      sym_null_value,
    ACTIONS(251), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(186), 6,
      sym_variable,
      sym_string_value,
      sym_boolean_value,
      sym_enum_value,
      sym_list_value,
      sym_object_value,
  [1673] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(223), 1,
      anon_sym_LBRACE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    STATE(73), 1,
      sym_directives,
    STATE(130), 1,
      sym_enum_values_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(257), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1710] = 8,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(263), 1,
      anon_sym_DQUOTE,
    STATE(78), 1,
      sym_directives,
    STATE(134), 1,
      sym_input_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(261), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1747] = 5,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(267), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(43), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(265), 13,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1777] = 5,
    ACTIONS(271), 1,
      anon_sym_DQUOTE,
    ACTIONS(273), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(43), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(269), 13,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1807] = 5,
    ACTIONS(278), 1,
      anon_sym_LPAREN,
    ACTIONS(280), 1,
      anon_sym_DQUOTE,
    STATE(49), 1,
      sym_arguments,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(276), 14,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      anon_sym_directive,
  [1837] = 6,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(284), 1,
      anon_sym_DQUOTE,
    STATE(142), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(282), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1868] = 6,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(288), 1,
      anon_sym_DQUOTE,
    STATE(140), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(286), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1899] = 6,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(292), 1,
      anon_sym_DQUOTE,
    STATE(113), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(290), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1930] = 5,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(296), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(54), 2,
      sym_input_fields_definition,
      aux_sym_input_object_type_extension_repeat1,
    ACTIONS(294), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [1958] = 3,
    ACTIONS(300), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(298), 14,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      anon_sym_directive,
  [1982] = 3,
    ACTIONS(304), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(302), 14,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      anon_sym_directive,
  [2006] = 3,
    ACTIONS(308), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(306), 14,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_AMP,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      anon_sym_directive,
  [2030] = 5,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(312), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(54), 2,
      sym_input_fields_definition,
      aux_sym_input_object_type_extension_repeat1,
    ACTIONS(310), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2058] = 6,
    ACTIONS(318), 1,
      anon_sym_PIPE,
    ACTIONS(320), 1,
      sym_name,
    STATE(63), 1,
      sym_named_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(314), 2,
      ts_builtin_sym_end,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(316), 10,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE,
      anon_sym_directive,
  [2088] = 5,
    ACTIONS(324), 1,
      anon_sym_LBRACE,
    ACTIONS(327), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(54), 2,
      sym_input_fields_definition,
      aux_sym_input_object_type_extension_repeat1,
    ACTIONS(322), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2116] = 5,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(312), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(48), 2,
      sym_input_fields_definition,
      aux_sym_input_object_type_extension_repeat1,
    ACTIONS(310), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2144] = 3,
    ACTIONS(331), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(329), 14,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_AMP,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      anon_sym_directive,
  [2168] = 5,
    ACTIONS(335), 1,
      anon_sym_PIPE,
    ACTIONS(337), 1,
      anon_sym_DQUOTE,
    STATE(62), 1,
      aux_sym_union_member_types_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(333), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2195] = 5,
    ACTIONS(195), 1,
      anon_sym_EQ,
    ACTIONS(197), 1,
      anon_sym_DQUOTE,
    STATE(125), 1,
      sym_union_member_types,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(193), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2222] = 5,
    ACTIONS(195), 1,
      anon_sym_EQ,
    ACTIONS(341), 1,
      anon_sym_DQUOTE,
    STATE(138), 1,
      sym_union_member_types,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(339), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2249] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(173), 1,
      anon_sym_DQUOTE,
    STATE(133), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(169), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2276] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(177), 1,
      anon_sym_DQUOTE,
    STATE(93), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(175), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2303] = 5,
    ACTIONS(345), 1,
      anon_sym_PIPE,
    ACTIONS(348), 1,
      anon_sym_DQUOTE,
    STATE(62), 1,
      aux_sym_union_member_types_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(343), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2330] = 5,
    ACTIONS(335), 1,
      anon_sym_PIPE,
    ACTIONS(352), 1,
      anon_sym_DQUOTE,
    STATE(74), 1,
      aux_sym_union_member_types_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(350), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2357] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(181), 1,
      anon_sym_DQUOTE,
    STATE(139), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(179), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2384] = 5,
    ACTIONS(223), 1,
      anon_sym_LBRACE,
    ACTIONS(356), 1,
      anon_sym_DQUOTE,
    STATE(137), 1,
      sym_enum_values_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(354), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2411] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(360), 1,
      anon_sym_DQUOTE,
    STATE(126), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(358), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2438] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(364), 1,
      anon_sym_DQUOTE,
    STATE(100), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(362), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2465] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(124), 1,
      anon_sym_DQUOTE,
    STATE(123), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(122), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2492] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(368), 1,
      anon_sym_DQUOTE,
    STATE(127), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(366), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2519] = 5,
    ACTIONS(335), 1,
      anon_sym_PIPE,
    ACTIONS(372), 1,
      anon_sym_DQUOTE,
    STATE(57), 1,
      aux_sym_union_member_types_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(370), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2546] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(118), 1,
      anon_sym_DQUOTE,
    STATE(124), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(116), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2573] = 5,
    ACTIONS(195), 1,
      anon_sym_EQ,
    ACTIONS(376), 1,
      anon_sym_DQUOTE,
    STATE(110), 1,
      sym_union_member_types,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(374), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2600] = 5,
    ACTIONS(223), 1,
      anon_sym_LBRACE,
    ACTIONS(380), 1,
      anon_sym_DQUOTE,
    STATE(108), 1,
      sym_enum_values_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(378), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2627] = 5,
    ACTIONS(335), 1,
      anon_sym_PIPE,
    ACTIONS(372), 1,
      anon_sym_DQUOTE,
    STATE(62), 1,
      aux_sym_union_member_types_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(370), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2654] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(384), 1,
      anon_sym_DQUOTE,
    STATE(98), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(382), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2681] = 5,
    ACTIONS(320), 1,
      sym_name,
    STATE(70), 1,
      sym_named_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(350), 2,
      ts_builtin_sym_end,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(352), 10,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE,
      anon_sym_directive,
  [2708] = 5,
    ACTIONS(223), 1,
      anon_sym_LBRACE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    STATE(130), 1,
      sym_enum_values_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(257), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2735] = 5,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(388), 1,
      anon_sym_DQUOTE,
    STATE(103), 1,
      sym_input_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(386), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2762] = 5,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    ACTIONS(185), 1,
      anon_sym_DQUOTE,
    STATE(135), 1,
      sym_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(183), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2789] = 5,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    ACTIONS(263), 1,
      anon_sym_DQUOTE,
    STATE(134), 1,
      sym_input_fields_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(261), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2816] = 3,
    ACTIONS(392), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(390), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2838] = 4,
    ACTIONS(396), 1,
      anon_sym_PIPE,
    ACTIONS(398), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(394), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2862] = 3,
    ACTIONS(402), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(400), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2884] = 3,
    ACTIONS(348), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(343), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2906] = 4,
    ACTIONS(396), 1,
      anon_sym_PIPE,
    ACTIONS(406), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(404), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2930] = 3,
    ACTIONS(410), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(408), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2952] = 4,
    ACTIONS(396), 1,
      anon_sym_PIPE,
    ACTIONS(414), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(412), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2976] = 3,
    ACTIONS(418), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(416), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [2998] = 4,
    ACTIONS(396), 1,
      anon_sym_PIPE,
    ACTIONS(422), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(420), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3022] = 3,
    ACTIONS(426), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(424), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3044] = 3,
    ACTIONS(430), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(428), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_PIPE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3066] = 3,
    ACTIONS(434), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(432), 12,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_LBRACE,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3088] = 3,
    ACTIONS(384), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(382), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3109] = 3,
    ACTIONS(438), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(436), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3130] = 3,
    ACTIONS(124), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(122), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3151] = 3,
    ACTIONS(118), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(116), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3172] = 3,
    ACTIONS(197), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(193), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3193] = 3,
    ACTIONS(442), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(440), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3214] = 3,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(257), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3235] = 3,
    ACTIONS(446), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(444), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3256] = 3,
    ACTIONS(263), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(261), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3277] = 3,
    ACTIONS(450), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(448), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3298] = 3,
    ACTIONS(454), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(452), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3319] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(456), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(458), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3340] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(460), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(462), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3361] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(464), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(466), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3382] = 3,
    ACTIONS(470), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(468), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3403] = 3,
    ACTIONS(474), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(472), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3424] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(476), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(478), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3445] = 3,
    ACTIONS(482), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(480), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3466] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(484), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(486), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3487] = 3,
    ACTIONS(490), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(488), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3508] = 3,
    ACTIONS(494), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(492), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3529] = 3,
    ACTIONS(181), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(179), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3550] = 3,
    ACTIONS(177), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(175), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3571] = 3,
    ACTIONS(341), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(339), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3592] = 3,
    ACTIONS(356), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(354), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3613] = 3,
    ACTIONS(498), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(496), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3634] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(500), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(502), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3655] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(504), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(506), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3676] = 3,
    ACTIONS(510), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(508), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3697] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(512), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(514), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3718] = 3,
    ACTIONS(185), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(183), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3739] = 3,
    ACTIONS(173), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(169), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3760] = 3,
    ACTIONS(376), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(374), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3781] = 3,
    ACTIONS(518), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(516), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3802] = 3,
    ACTIONS(522), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(520), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3823] = 3,
    ACTIONS(526), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(524), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3844] = 3,
    ACTIONS(530), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(528), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3865] = 3,
    ACTIONS(380), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(378), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3886] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(532), 6,
      anon_sym_LBRACE,
      anon_sym_DOLLAR,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_float_value,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
    ACTIONS(534), 6,
      anon_sym_DQUOTE,
      sym_int_value,
      anon_sym_true,
      anon_sym_false,
      sym_null_value,
      sym_name,
  [3907] = 3,
    ACTIONS(538), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(536), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3928] = 3,
    ACTIONS(368), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(366), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3949] = 3,
    ACTIONS(388), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(386), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3970] = 3,
    ACTIONS(360), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(358), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [3991] = 3,
    ACTIONS(542), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(540), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4012] = 3,
    ACTIONS(546), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(544), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4033] = 3,
    ACTIONS(550), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(548), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4054] = 3,
    ACTIONS(364), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(362), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4075] = 3,
    ACTIONS(554), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(552), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4096] = 3,
    ACTIONS(558), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(556), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4117] = 3,
    ACTIONS(288), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(286), 11,
      ts_builtin_sym_end,
      anon_sym_schema,
      anon_sym_extend,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_directive,
  [4138] = 8,
    ACTIONS(562), 1,
      anon_sym_EQ,
    ACTIONS(564), 1,
      anon_sym_DQUOTE,
    ACTIONS(566), 1,
      anon_sym_AT,
    STATE(145), 1,
      sym_default_value,
    STATE(201), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(560), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4168] = 8,
    ACTIONS(562), 1,
      anon_sym_EQ,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(570), 1,
      anon_sym_DQUOTE,
    STATE(150), 1,
      sym_default_value,
    STATE(205), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(568), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4198] = 6,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(574), 1,
      anon_sym_DQUOTE,
    STATE(200), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(572), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4222] = 4,
    ACTIONS(578), 1,
      anon_sym_DQUOTE,
    ACTIONS(580), 1,
      anon_sym_BANG,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(576), 7,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_RBRACK,
      anon_sym_AT,
      sym_name,
  [4242] = 8,
    ACTIONS(584), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(587), 1,
      anon_sym_DQUOTE,
    ACTIONS(590), 1,
      sym_name,
    STATE(263), 1,
      sym_string_value,
    STATE(270), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(582), 2,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
    STATE(147), 2,
      sym_input_value_definition,
      aux_sym_input_fields_definition_repeat1,
  [4270] = 9,
    ACTIONS(593), 1,
      anon_sym_RBRACE,
    ACTIONS(595), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(598), 1,
      anon_sym_DQUOTE,
    ACTIONS(601), 1,
      sym_name,
    STATE(164), 1,
      sym_enum_value,
    STATE(247), 1,
      sym_description,
    STATE(263), 1,
      sym_string_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(148), 2,
      sym_enum_value_definition,
      aux_sym_enum_values_definition_repeat1,
  [4300] = 9,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(604), 1,
      anon_sym_RBRACE,
    ACTIONS(606), 1,
      sym_name,
    STATE(164), 1,
      sym_enum_value,
    STATE(247), 1,
      sym_description,
    STATE(263), 1,
      sym_string_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(148), 2,
      sym_enum_value_definition,
      aux_sym_enum_values_definition_repeat1,
  [4330] = 6,
    ACTIONS(564), 1,
      anon_sym_DQUOTE,
    ACTIONS(566), 1,
      anon_sym_AT,
    STATE(201), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(560), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4354] = 3,
    ACTIONS(610), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(608), 8,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_RBRACK,
      anon_sym_AT,
      anon_sym_BANG,
      sym_name,
  [4372] = 6,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(614), 1,
      anon_sym_DQUOTE,
    STATE(210), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(612), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4395] = 3,
    ACTIONS(78), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(76), 7,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      anon_sym_BANG,
      sym_name,
  [4412] = 3,
    ACTIONS(578), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(576), 7,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_RBRACK,
      anon_sym_AT,
      sym_name,
  [4429] = 8,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(616), 1,
      anon_sym_RBRACE,
    ACTIONS(618), 1,
      sym_name,
    STATE(263), 1,
      sym_string_value,
    STATE(274), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(163), 2,
      sym_field_definition,
      aux_sym_fields_definition_repeat1,
  [4456] = 6,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(622), 1,
      anon_sym_DQUOTE,
    STATE(214), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(620), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4479] = 8,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(606), 1,
      sym_name,
    STATE(164), 1,
      sym_enum_value,
    STATE(247), 1,
      sym_description,
    STATE(263), 1,
      sym_string_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(149), 2,
      sym_enum_value_definition,
      aux_sym_enum_values_definition_repeat1,
  [4506] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(456), 8,
      anon_sym_schema,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_directive,
  [4521] = 9,
    ACTIONS(624), 1,
      anon_sym_schema,
    ACTIONS(626), 1,
      anon_sym_scalar,
    ACTIONS(628), 1,
      anon_sym_type,
    ACTIONS(630), 1,
      anon_sym_interface,
    ACTIONS(632), 1,
      anon_sym_union,
    ACTIONS(634), 1,
      anon_sym_enum,
    ACTIONS(636), 1,
      anon_sym_input,
    ACTIONS(638), 1,
      anon_sym_directive,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [4550] = 5,
    ACTIONS(271), 1,
      anon_sym_DQUOTE,
    ACTIONS(640), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(160), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(269), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4571] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(643), 8,
      anon_sym_schema,
      anon_sym_scalar,
      anon_sym_type,
      anon_sym_interface,
      anon_sym_union,
      anon_sym_enum,
      anon_sym_input,
      anon_sym_directive,
  [4586] = 5,
    ACTIONS(280), 1,
      anon_sym_DQUOTE,
    ACTIONS(645), 1,
      anon_sym_LPAREN,
    STATE(183), 1,
      sym_arguments,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(276), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [4607] = 8,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(618), 1,
      sym_name,
    ACTIONS(647), 1,
      anon_sym_RBRACE,
    STATE(263), 1,
      sym_string_value,
    STATE(274), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(167), 2,
      sym_field_definition,
      aux_sym_fields_definition_repeat1,
  [4634] = 6,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(651), 1,
      anon_sym_DQUOTE,
    STATE(224), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(649), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4657] = 3,
    ACTIONS(655), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(653), 7,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_RBRACK,
      anon_sym_AT,
      sym_name,
  [4674] = 6,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(659), 1,
      anon_sym_DQUOTE,
    STATE(221), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(657), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4697] = 8,
    ACTIONS(661), 1,
      anon_sym_RBRACE,
    ACTIONS(663), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(666), 1,
      anon_sym_DQUOTE,
    ACTIONS(669), 1,
      sym_name,
    STATE(263), 1,
      sym_string_value,
    STATE(274), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(167), 2,
      sym_field_definition,
      aux_sym_fields_definition_repeat1,
  [4724] = 8,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(672), 1,
      anon_sym_RPAREN,
    ACTIONS(674), 1,
      sym_name,
    STATE(263), 1,
      sym_string_value,
    STATE(270), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(147), 2,
      sym_input_value_definition,
      aux_sym_input_fields_definition_repeat1,
  [4751] = 6,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(678), 1,
      anon_sym_DQUOTE,
    STATE(211), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(171), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(676), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4774] = 8,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(674), 1,
      sym_name,
    ACTIONS(680), 1,
      anon_sym_RBRACE,
    STATE(263), 1,
      sym_string_value,
    STATE(270), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(147), 2,
      sym_input_value_definition,
      aux_sym_input_fields_definition_repeat1,
  [4801] = 5,
    ACTIONS(267), 1,
      anon_sym_DQUOTE,
    ACTIONS(566), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(160), 2,
      sym_directive,
      aux_sym_directives_repeat1,
    ACTIONS(265), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [4822] = 5,
    ACTIONS(682), 1,
      anon_sym_RBRACE,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(172), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(684), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [4842] = 7,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(674), 1,
      sym_name,
    STATE(263), 1,
      sym_string_value,
    STATE(270), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(168), 2,
      sym_input_value_definition,
      aux_sym_input_fields_definition_repeat1,
  [4866] = 5,
    ACTIONS(687), 1,
      anon_sym_RBRACE,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(172), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [4886] = 7,
    ACTIONS(243), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
    ACTIONS(674), 1,
      sym_name,
    STATE(263), 1,
      sym_string_value,
    STATE(270), 1,
      sym_description,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(170), 2,
      sym_input_value_definition,
      aux_sym_input_fields_definition_repeat1,
  [4910] = 8,
    ACTIONS(691), 1,
      anon_sym_schema,
    ACTIONS(693), 1,
      anon_sym_scalar,
    ACTIONS(695), 1,
      anon_sym_type,
    ACTIONS(697), 1,
      anon_sym_interface,
    ACTIONS(699), 1,
      anon_sym_union,
    ACTIONS(701), 1,
      anon_sym_enum,
    ACTIONS(703), 1,
      anon_sym_input,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [4936] = 5,
    ACTIONS(705), 1,
      anon_sym_RBRACE,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(172), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [4956] = 5,
    ACTIONS(707), 1,
      anon_sym_RBRACE,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(172), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [4976] = 3,
    ACTIONS(502), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(500), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [4991] = 6,
    ACTIONS(709), 1,
      anon_sym_LBRACK,
    ACTIONS(711), 1,
      sym_name,
    STATE(144), 1,
      sym_type,
    STATE(154), 1,
      sym_non_null_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(146), 2,
      sym_named_type,
      sym_list_type,
  [5012] = 3,
    ACTIONS(466), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(464), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5027] = 6,
    ACTIONS(709), 1,
      anon_sym_LBRACK,
    ACTIONS(711), 1,
      sym_name,
    STATE(154), 1,
      sym_non_null_type,
    STATE(166), 1,
      sym_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(146), 2,
      sym_named_type,
      sym_list_type,
  [5048] = 3,
    ACTIONS(300), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(298), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5063] = 3,
    ACTIONS(304), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(302), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5078] = 3,
    ACTIONS(506), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(504), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5093] = 3,
    ACTIONS(514), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(512), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5108] = 3,
    ACTIONS(486), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(484), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5123] = 3,
    ACTIONS(534), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(532), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5138] = 4,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(178), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [5155] = 3,
    ACTIONS(478), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(476), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5170] = 4,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(177), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [5187] = 6,
    ACTIONS(709), 1,
      anon_sym_LBRACK,
    ACTIONS(711), 1,
      sym_name,
    STATE(152), 1,
      sym_type,
    STATE(154), 1,
      sym_non_null_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(146), 2,
      sym_named_type,
      sym_list_type,
  [5208] = 3,
    ACTIONS(462), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(460), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5223] = 6,
    ACTIONS(709), 1,
      anon_sym_LBRACK,
    ACTIONS(713), 1,
      sym_name,
    STATE(154), 1,
      sym_non_null_type,
    STATE(288), 1,
      sym_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(146), 2,
      sym_named_type,
      sym_list_type,
  [5244] = 6,
    ACTIONS(709), 1,
      anon_sym_LBRACK,
    ACTIONS(711), 1,
      sym_name,
    STATE(154), 1,
      sym_non_null_type,
    STATE(156), 1,
      sym_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(146), 2,
      sym_named_type,
      sym_list_type,
  [5265] = 3,
    ACTIONS(458), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(456), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5280] = 4,
    STATE(257), 1,
      sym_operation_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(174), 2,
      sym_root_operation_type_definition,
      aux_sym_schema_definition_repeat1,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [5297] = 6,
    ACTIONS(709), 1,
      anon_sym_LBRACK,
    ACTIONS(711), 1,
      sym_name,
    STATE(143), 1,
      sym_type,
    STATE(154), 1,
      sym_non_null_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(146), 2,
      sym_named_type,
      sym_list_type,
  [5318] = 3,
    ACTIONS(717), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(715), 5,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_AT,
      sym_name,
  [5333] = 3,
    ACTIONS(721), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(719), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5347] = 3,
    ACTIONS(574), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(572), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5361] = 5,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(723), 1,
      anon_sym_LBRACE,
    STATE(287), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
  [5379] = 4,
    STATE(257), 1,
      sym_operation_type,
    STATE(276), 1,
      sym_root_operation_type_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [5395] = 4,
    STATE(257), 1,
      sym_operation_type,
    STATE(260), 1,
      sym_root_operation_type_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(689), 3,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [5411] = 3,
    ACTIONS(564), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(560), 4,
      anon_sym_RBRACE,
      anon_sym_RPAREN,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5425] = 5,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(725), 1,
      anon_sym_LBRACE,
    STATE(304), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
  [5443] = 5,
    ACTIONS(108), 1,
      anon_sym_AT,
    ACTIONS(727), 1,
      anon_sym_LBRACE,
    STATE(266), 1,
      sym_directives,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(42), 2,
      sym_directive,
      aux_sym_directives_repeat1,
  [5461] = 4,
    ACTIONS(729), 1,
      anon_sym_RPAREN,
    ACTIONS(731), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(215), 2,
      sym_argument,
      aux_sym_arguments_repeat1,
  [5476] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(733), 4,
      anon_sym_RBRACE,
      anon_sym_query,
      anon_sym_mutation,
      anon_sym_subscription,
  [5487] = 3,
    ACTIONS(737), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(735), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5500] = 3,
    ACTIONS(741), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(739), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5513] = 4,
    ACTIONS(743), 1,
      anon_sym_RBRACE,
    ACTIONS(745), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(225), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5528] = 5,
    ACTIONS(747), 1,
      anon_sym_LPAREN,
    ACTIONS(749), 1,
      anon_sym_on,
    ACTIONS(751), 1,
      anon_sym_repeatable,
    STATE(248), 1,
      sym_arguments_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5545] = 3,
    ACTIONS(659), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(657), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5558] = 4,
    ACTIONS(753), 1,
      anon_sym_RPAREN,
    ACTIONS(755), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(215), 2,
      sym_argument,
      aux_sym_arguments_repeat1,
  [5573] = 4,
    ACTIONS(745), 1,
      sym_name,
    ACTIONS(758), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(212), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5588] = 4,
    ACTIONS(745), 1,
      sym_name,
    ACTIONS(760), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(225), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5603] = 4,
    ACTIONS(745), 1,
      sym_name,
    ACTIONS(762), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(217), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5618] = 4,
    ACTIONS(731), 1,
      sym_name,
    ACTIONS(764), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(215), 2,
      sym_argument,
      aux_sym_arguments_repeat1,
  [5633] = 4,
    ACTIONS(745), 1,
      sym_name,
    ACTIONS(766), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(225), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5648] = 3,
    ACTIONS(614), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(612), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5661] = 5,
    ACTIONS(747), 1,
      anon_sym_LPAREN,
    ACTIONS(768), 1,
      anon_sym_on,
    ACTIONS(770), 1,
      anon_sym_repeatable,
    STATE(234), 1,
      sym_arguments_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5678] = 4,
    ACTIONS(745), 1,
      sym_name,
    ACTIONS(772), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(220), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5693] = 3,
    ACTIONS(678), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(676), 3,
      anon_sym_RBRACE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      sym_name,
  [5706] = 4,
    ACTIONS(774), 1,
      anon_sym_RBRACE,
    ACTIONS(776), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(225), 2,
      sym_object_field,
      aux_sym_object_value_repeat1,
  [5721] = 4,
    ACTIONS(171), 1,
      anon_sym_AMP,
    ACTIONS(713), 1,
      sym_name,
    STATE(51), 1,
      sym_named_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5735] = 4,
    ACTIONS(747), 1,
      anon_sym_LPAREN,
    ACTIONS(779), 1,
      anon_sym_COLON,
    STATE(309), 1,
      sym_arguments_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5749] = 4,
    ACTIONS(747), 1,
      anon_sym_LPAREN,
    ACTIONS(781), 1,
      anon_sym_COLON,
    STATE(253), 1,
      sym_arguments_definition,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5763] = 3,
    ACTIONS(731), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(208), 2,
      sym_argument,
      aux_sym_arguments_repeat1,
  [5775] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(783), 3,
      anon_sym_COLON,
      anon_sym_on,
      anon_sym_repeatable,
  [5785] = 3,
    ACTIONS(731), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    STATE(219), 2,
      sym_argument,
      aux_sym_arguments_repeat1,
  [5797] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(460), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5806] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(785), 2,
      anon_sym_RBRACE,
      sym_name,
  [5815] = 3,
    ACTIONS(787), 1,
      anon_sym_on,
    ACTIONS(789), 1,
      anon_sym_repeatable,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5826] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(456), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5835] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
    ACTIONS(791), 2,
      anon_sym_RPAREN,
      sym_name,
  [5844] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(464), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5853] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(476), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5862] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(484), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5871] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(500), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5880] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(504), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5889] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5898] = 3,
    ACTIONS(713), 1,
      sym_name,
    STATE(84), 1,
      sym_named_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5909] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(532), 3,
      anon_sym_RBRACE,
      sym_name,
      sym_comma,
  [5918] = 3,
    ACTIONS(713), 1,
      sym_name,
    STATE(209), 1,
      sym_named_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5929] = 3,
    ACTIONS(713), 1,
      sym_name,
    STATE(56), 1,
      sym_named_type,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5940] = 3,
    ACTIONS(606), 1,
      sym_name,
    STATE(169), 1,
      sym_enum_value,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5951] = 3,
    ACTIONS(768), 1,
      anon_sym_on,
    ACTIONS(770), 1,
      anon_sym_repeatable,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5962] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(795), 1,
      sym_comma,
    ACTIONS(793), 2,
      anon_sym_RBRACE,
      sym_name,
  [5973] = 2,
    ACTIONS(797), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5981] = 2,
    ACTIONS(799), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [5989] = 2,
    ACTIONS(801), 1,
      aux_sym_string_value_token1,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [5997] = 2,
    ACTIONS(779), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6005] = 2,
    ACTIONS(805), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6013] = 2,
    ACTIONS(807), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6021] = 2,
    ACTIONS(809), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6029] = 2,
    ACTIONS(811), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6037] = 2,
    ACTIONS(813), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6045] = 2,
    ACTIONS(815), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6053] = 2,
    ACTIONS(817), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6061] = 2,
    ACTIONS(819), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6069] = 2,
    ACTIONS(821), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6077] = 2,
    ACTIONS(643), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6085] = 2,
    ACTIONS(823), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6093] = 2,
    ACTIONS(825), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6101] = 2,
    ACTIONS(827), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6109] = 2,
    ACTIONS(829), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6117] = 2,
    ACTIONS(768), 1,
      anon_sym_on,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6125] = 2,
    ACTIONS(831), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6133] = 2,
    ACTIONS(833), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6141] = 2,
    ACTIONS(835), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6149] = 2,
    ACTIONS(837), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6157] = 2,
    ACTIONS(839), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6165] = 2,
    ACTIONS(841), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6173] = 2,
    ACTIONS(843), 1,
      anon_sym_on,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6181] = 2,
    ACTIONS(845), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6189] = 2,
    ACTIONS(787), 1,
      anon_sym_on,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6197] = 2,
    ACTIONS(847), 1,
      aux_sym_string_value_token2,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [6205] = 2,
    ACTIONS(849), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6213] = 2,
    ACTIONS(851), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6221] = 2,
    ACTIONS(853), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6229] = 2,
    ACTIONS(855), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6237] = 2,
    ACTIONS(857), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6245] = 2,
    ACTIONS(859), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6253] = 2,
    ACTIONS(861), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6261] = 2,
    ACTIONS(863), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6269] = 2,
    ACTIONS(725), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6277] = 2,
    ACTIONS(865), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6285] = 2,
    ACTIONS(867), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6293] = 2,
    ACTIONS(869), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6301] = 2,
    ACTIONS(869), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6309] = 2,
    ACTIONS(871), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6317] = 2,
    ACTIONS(873), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6325] = 2,
    ACTIONS(875), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6333] = 2,
    ACTIONS(877), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6341] = 2,
    ACTIONS(863), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6349] = 2,
    ACTIONS(879), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6357] = 2,
    ACTIONS(881), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6365] = 2,
    ACTIONS(881), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6373] = 2,
    ACTIONS(883), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6381] = 2,
    ACTIONS(885), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6389] = 2,
    ACTIONS(887), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6397] = 2,
    ACTIONS(889), 1,
      sym_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6405] = 2,
    ACTIONS(891), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6413] = 2,
    ACTIONS(893), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6421] = 2,
    ACTIONS(893), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6429] = 2,
    ACTIONS(895), 1,
      aux_sym_string_value_token1,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [6437] = 2,
    ACTIONS(897), 1,
      aux_sym_string_value_token2,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [6445] = 2,
    ACTIONS(899), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_comma,
  [6453] = 2,
    ACTIONS(901), 1,
      aux_sym_string_value_token1,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [6461] = 2,
    ACTIONS(903), 1,
      aux_sym_string_value_token2,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [6469] = 2,
    ACTIONS(905), 1,
      aux_sym_string_value_token1,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
  [6477] = 2,
    ACTIONS(907), 1,
      aux_sym_string_value_token2,
    ACTIONS(803), 2,
      sym_comment,
      sym_comma,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 74,
  [SMALL_STATE(4)] = 148,
  [SMALL_STATE(5)] = 193,
  [SMALL_STATE(6)] = 238,
  [SMALL_STATE(7)] = 283,
  [SMALL_STATE(8)] = 328,
  [SMALL_STATE(9)] = 367,
  [SMALL_STATE(10)] = 406,
  [SMALL_STATE(11)] = 437,
  [SMALL_STATE(12)] = 486,
  [SMALL_STATE(13)] = 529,
  [SMALL_STATE(14)] = 572,
  [SMALL_STATE(15)] = 621,
  [SMALL_STATE(16)] = 664,
  [SMALL_STATE(17)] = 713,
  [SMALL_STATE(18)] = 756,
  [SMALL_STATE(19)] = 805,
  [SMALL_STATE(20)] = 854,
  [SMALL_STATE(21)] = 897,
  [SMALL_STATE(22)] = 940,
  [SMALL_STATE(23)] = 989,
  [SMALL_STATE(24)] = 1038,
  [SMALL_STATE(25)] = 1078,
  [SMALL_STATE(26)] = 1118,
  [SMALL_STATE(27)] = 1158,
  [SMALL_STATE(28)] = 1198,
  [SMALL_STATE(29)] = 1238,
  [SMALL_STATE(30)] = 1278,
  [SMALL_STATE(31)] = 1316,
  [SMALL_STATE(32)] = 1353,
  [SMALL_STATE(33)] = 1398,
  [SMALL_STATE(34)] = 1435,
  [SMALL_STATE(35)] = 1472,
  [SMALL_STATE(36)] = 1509,
  [SMALL_STATE(37)] = 1546,
  [SMALL_STATE(38)] = 1583,
  [SMALL_STATE(39)] = 1628,
  [SMALL_STATE(40)] = 1673,
  [SMALL_STATE(41)] = 1710,
  [SMALL_STATE(42)] = 1747,
  [SMALL_STATE(43)] = 1777,
  [SMALL_STATE(44)] = 1807,
  [SMALL_STATE(45)] = 1837,
  [SMALL_STATE(46)] = 1868,
  [SMALL_STATE(47)] = 1899,
  [SMALL_STATE(48)] = 1930,
  [SMALL_STATE(49)] = 1958,
  [SMALL_STATE(50)] = 1982,
  [SMALL_STATE(51)] = 2006,
  [SMALL_STATE(52)] = 2030,
  [SMALL_STATE(53)] = 2058,
  [SMALL_STATE(54)] = 2088,
  [SMALL_STATE(55)] = 2116,
  [SMALL_STATE(56)] = 2144,
  [SMALL_STATE(57)] = 2168,
  [SMALL_STATE(58)] = 2195,
  [SMALL_STATE(59)] = 2222,
  [SMALL_STATE(60)] = 2249,
  [SMALL_STATE(61)] = 2276,
  [SMALL_STATE(62)] = 2303,
  [SMALL_STATE(63)] = 2330,
  [SMALL_STATE(64)] = 2357,
  [SMALL_STATE(65)] = 2384,
  [SMALL_STATE(66)] = 2411,
  [SMALL_STATE(67)] = 2438,
  [SMALL_STATE(68)] = 2465,
  [SMALL_STATE(69)] = 2492,
  [SMALL_STATE(70)] = 2519,
  [SMALL_STATE(71)] = 2546,
  [SMALL_STATE(72)] = 2573,
  [SMALL_STATE(73)] = 2600,
  [SMALL_STATE(74)] = 2627,
  [SMALL_STATE(75)] = 2654,
  [SMALL_STATE(76)] = 2681,
  [SMALL_STATE(77)] = 2708,
  [SMALL_STATE(78)] = 2735,
  [SMALL_STATE(79)] = 2762,
  [SMALL_STATE(80)] = 2789,
  [SMALL_STATE(81)] = 2816,
  [SMALL_STATE(82)] = 2838,
  [SMALL_STATE(83)] = 2862,
  [SMALL_STATE(84)] = 2884,
  [SMALL_STATE(85)] = 2906,
  [SMALL_STATE(86)] = 2930,
  [SMALL_STATE(87)] = 2952,
  [SMALL_STATE(88)] = 2976,
  [SMALL_STATE(89)] = 2998,
  [SMALL_STATE(90)] = 3022,
  [SMALL_STATE(91)] = 3044,
  [SMALL_STATE(92)] = 3066,
  [SMALL_STATE(93)] = 3088,
  [SMALL_STATE(94)] = 3109,
  [SMALL_STATE(95)] = 3130,
  [SMALL_STATE(96)] = 3151,
  [SMALL_STATE(97)] = 3172,
  [SMALL_STATE(98)] = 3193,
  [SMALL_STATE(99)] = 3214,
  [SMALL_STATE(100)] = 3235,
  [SMALL_STATE(101)] = 3256,
  [SMALL_STATE(102)] = 3277,
  [SMALL_STATE(103)] = 3298,
  [SMALL_STATE(104)] = 3319,
  [SMALL_STATE(105)] = 3340,
  [SMALL_STATE(106)] = 3361,
  [SMALL_STATE(107)] = 3382,
  [SMALL_STATE(108)] = 3403,
  [SMALL_STATE(109)] = 3424,
  [SMALL_STATE(110)] = 3445,
  [SMALL_STATE(111)] = 3466,
  [SMALL_STATE(112)] = 3487,
  [SMALL_STATE(113)] = 3508,
  [SMALL_STATE(114)] = 3529,
  [SMALL_STATE(115)] = 3550,
  [SMALL_STATE(116)] = 3571,
  [SMALL_STATE(117)] = 3592,
  [SMALL_STATE(118)] = 3613,
  [SMALL_STATE(119)] = 3634,
  [SMALL_STATE(120)] = 3655,
  [SMALL_STATE(121)] = 3676,
  [SMALL_STATE(122)] = 3697,
  [SMALL_STATE(123)] = 3718,
  [SMALL_STATE(124)] = 3739,
  [SMALL_STATE(125)] = 3760,
  [SMALL_STATE(126)] = 3781,
  [SMALL_STATE(127)] = 3802,
  [SMALL_STATE(128)] = 3823,
  [SMALL_STATE(129)] = 3844,
  [SMALL_STATE(130)] = 3865,
  [SMALL_STATE(131)] = 3886,
  [SMALL_STATE(132)] = 3907,
  [SMALL_STATE(133)] = 3928,
  [SMALL_STATE(134)] = 3949,
  [SMALL_STATE(135)] = 3970,
  [SMALL_STATE(136)] = 3991,
  [SMALL_STATE(137)] = 4012,
  [SMALL_STATE(138)] = 4033,
  [SMALL_STATE(139)] = 4054,
  [SMALL_STATE(140)] = 4075,
  [SMALL_STATE(141)] = 4096,
  [SMALL_STATE(142)] = 4117,
  [SMALL_STATE(143)] = 4138,
  [SMALL_STATE(144)] = 4168,
  [SMALL_STATE(145)] = 4198,
  [SMALL_STATE(146)] = 4222,
  [SMALL_STATE(147)] = 4242,
  [SMALL_STATE(148)] = 4270,
  [SMALL_STATE(149)] = 4300,
  [SMALL_STATE(150)] = 4330,
  [SMALL_STATE(151)] = 4354,
  [SMALL_STATE(152)] = 4372,
  [SMALL_STATE(153)] = 4395,
  [SMALL_STATE(154)] = 4412,
  [SMALL_STATE(155)] = 4429,
  [SMALL_STATE(156)] = 4456,
  [SMALL_STATE(157)] = 4479,
  [SMALL_STATE(158)] = 4506,
  [SMALL_STATE(159)] = 4521,
  [SMALL_STATE(160)] = 4550,
  [SMALL_STATE(161)] = 4571,
  [SMALL_STATE(162)] = 4586,
  [SMALL_STATE(163)] = 4607,
  [SMALL_STATE(164)] = 4634,
  [SMALL_STATE(165)] = 4657,
  [SMALL_STATE(166)] = 4674,
  [SMALL_STATE(167)] = 4697,
  [SMALL_STATE(168)] = 4724,
  [SMALL_STATE(169)] = 4751,
  [SMALL_STATE(170)] = 4774,
  [SMALL_STATE(171)] = 4801,
  [SMALL_STATE(172)] = 4822,
  [SMALL_STATE(173)] = 4842,
  [SMALL_STATE(174)] = 4866,
  [SMALL_STATE(175)] = 4886,
  [SMALL_STATE(176)] = 4910,
  [SMALL_STATE(177)] = 4936,
  [SMALL_STATE(178)] = 4956,
  [SMALL_STATE(179)] = 4976,
  [SMALL_STATE(180)] = 4991,
  [SMALL_STATE(181)] = 5012,
  [SMALL_STATE(182)] = 5027,
  [SMALL_STATE(183)] = 5048,
  [SMALL_STATE(184)] = 5063,
  [SMALL_STATE(185)] = 5078,
  [SMALL_STATE(186)] = 5093,
  [SMALL_STATE(187)] = 5108,
  [SMALL_STATE(188)] = 5123,
  [SMALL_STATE(189)] = 5138,
  [SMALL_STATE(190)] = 5155,
  [SMALL_STATE(191)] = 5170,
  [SMALL_STATE(192)] = 5187,
  [SMALL_STATE(193)] = 5208,
  [SMALL_STATE(194)] = 5223,
  [SMALL_STATE(195)] = 5244,
  [SMALL_STATE(196)] = 5265,
  [SMALL_STATE(197)] = 5280,
  [SMALL_STATE(198)] = 5297,
  [SMALL_STATE(199)] = 5318,
  [SMALL_STATE(200)] = 5333,
  [SMALL_STATE(201)] = 5347,
  [SMALL_STATE(202)] = 5361,
  [SMALL_STATE(203)] = 5379,
  [SMALL_STATE(204)] = 5395,
  [SMALL_STATE(205)] = 5411,
  [SMALL_STATE(206)] = 5425,
  [SMALL_STATE(207)] = 5443,
  [SMALL_STATE(208)] = 5461,
  [SMALL_STATE(209)] = 5476,
  [SMALL_STATE(210)] = 5487,
  [SMALL_STATE(211)] = 5500,
  [SMALL_STATE(212)] = 5513,
  [SMALL_STATE(213)] = 5528,
  [SMALL_STATE(214)] = 5545,
  [SMALL_STATE(215)] = 5558,
  [SMALL_STATE(216)] = 5573,
  [SMALL_STATE(217)] = 5588,
  [SMALL_STATE(218)] = 5603,
  [SMALL_STATE(219)] = 5618,
  [SMALL_STATE(220)] = 5633,
  [SMALL_STATE(221)] = 5648,
  [SMALL_STATE(222)] = 5661,
  [SMALL_STATE(223)] = 5678,
  [SMALL_STATE(224)] = 5693,
  [SMALL_STATE(225)] = 5706,
  [SMALL_STATE(226)] = 5721,
  [SMALL_STATE(227)] = 5735,
  [SMALL_STATE(228)] = 5749,
  [SMALL_STATE(229)] = 5763,
  [SMALL_STATE(230)] = 5775,
  [SMALL_STATE(231)] = 5785,
  [SMALL_STATE(232)] = 5797,
  [SMALL_STATE(233)] = 5806,
  [SMALL_STATE(234)] = 5815,
  [SMALL_STATE(235)] = 5826,
  [SMALL_STATE(236)] = 5835,
  [SMALL_STATE(237)] = 5844,
  [SMALL_STATE(238)] = 5853,
  [SMALL_STATE(239)] = 5862,
  [SMALL_STATE(240)] = 5871,
  [SMALL_STATE(241)] = 5880,
  [SMALL_STATE(242)] = 5889,
  [SMALL_STATE(243)] = 5898,
  [SMALL_STATE(244)] = 5909,
  [SMALL_STATE(245)] = 5918,
  [SMALL_STATE(246)] = 5929,
  [SMALL_STATE(247)] = 5940,
  [SMALL_STATE(248)] = 5951,
  [SMALL_STATE(249)] = 5962,
  [SMALL_STATE(250)] = 5973,
  [SMALL_STATE(251)] = 5981,
  [SMALL_STATE(252)] = 5989,
  [SMALL_STATE(253)] = 5997,
  [SMALL_STATE(254)] = 6005,
  [SMALL_STATE(255)] = 6013,
  [SMALL_STATE(256)] = 6021,
  [SMALL_STATE(257)] = 6029,
  [SMALL_STATE(258)] = 6037,
  [SMALL_STATE(259)] = 6045,
  [SMALL_STATE(260)] = 6053,
  [SMALL_STATE(261)] = 6061,
  [SMALL_STATE(262)] = 6069,
  [SMALL_STATE(263)] = 6077,
  [SMALL_STATE(264)] = 6085,
  [SMALL_STATE(265)] = 6093,
  [SMALL_STATE(266)] = 6101,
  [SMALL_STATE(267)] = 6109,
  [SMALL_STATE(268)] = 6117,
  [SMALL_STATE(269)] = 6125,
  [SMALL_STATE(270)] = 6133,
  [SMALL_STATE(271)] = 6141,
  [SMALL_STATE(272)] = 6149,
  [SMALL_STATE(273)] = 6157,
  [SMALL_STATE(274)] = 6165,
  [SMALL_STATE(275)] = 6173,
  [SMALL_STATE(276)] = 6181,
  [SMALL_STATE(277)] = 6189,
  [SMALL_STATE(278)] = 6197,
  [SMALL_STATE(279)] = 6205,
  [SMALL_STATE(280)] = 6213,
  [SMALL_STATE(281)] = 6221,
  [SMALL_STATE(282)] = 6229,
  [SMALL_STATE(283)] = 6237,
  [SMALL_STATE(284)] = 6245,
  [SMALL_STATE(285)] = 6253,
  [SMALL_STATE(286)] = 6261,
  [SMALL_STATE(287)] = 6269,
  [SMALL_STATE(288)] = 6277,
  [SMALL_STATE(289)] = 6285,
  [SMALL_STATE(290)] = 6293,
  [SMALL_STATE(291)] = 6301,
  [SMALL_STATE(292)] = 6309,
  [SMALL_STATE(293)] = 6317,
  [SMALL_STATE(294)] = 6325,
  [SMALL_STATE(295)] = 6333,
  [SMALL_STATE(296)] = 6341,
  [SMALL_STATE(297)] = 6349,
  [SMALL_STATE(298)] = 6357,
  [SMALL_STATE(299)] = 6365,
  [SMALL_STATE(300)] = 6373,
  [SMALL_STATE(301)] = 6381,
  [SMALL_STATE(302)] = 6389,
  [SMALL_STATE(303)] = 6397,
  [SMALL_STATE(304)] = 6405,
  [SMALL_STATE(305)] = 6413,
  [SMALL_STATE(306)] = 6421,
  [SMALL_STATE(307)] = 6429,
  [SMALL_STATE(308)] = 6437,
  [SMALL_STATE(309)] = 6445,
  [SMALL_STATE(310)] = 6453,
  [SMALL_STATE(311)] = 6461,
  [SMALL_STATE(312)] = 6469,
  [SMALL_STATE(313)] = 6477,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(302),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(297),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(295),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(261),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(259),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(278),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(280),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2),
  [31] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(202),
  [34] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(176),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(302),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(297),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(295),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(261),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(259),
  [52] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(258),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(252),
  [58] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(278),
  [61] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2), SHIFT_REPEAT(280),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [74] = {.entry = {.count = 1, .reusable = false}}, SHIFT(90),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_named_type, 1),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_named_type, 1),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(294),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(310),
  [86] = {.entry = {.count = 1, .reusable = false}}, SHIFT(311),
  [88] = {.entry = {.count = 1, .reusable = false}}, SHIFT(122),
  [90] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [92] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [98] = {.entry = {.count = 1, .reusable = false}}, SHIFT(131),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_extension, 3),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_extension, 3),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(264),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_definition, 2),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_definition, 2),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_definition, 3),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_definition, 3),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_definition, 3),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_definition, 3),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(218),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(294),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(310),
  [135] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(311),
  [138] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(122),
  [141] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(122),
  [144] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(120),
  [147] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(19),
  [150] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_list_value_repeat1, 2),
  [152] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_value_repeat1, 2), SHIFT_REPEAT(131),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_extension, 3),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_extension, 3),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_definition, 2),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_definition, 2),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_definition, 4),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [173] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_definition, 4),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_extension, 4),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_extension, 4),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_extension, 4),
  [181] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_extension, 4),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_definition, 4),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_definition, 4),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_extension, 3),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_extension, 3),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_definition, 3),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_definition, 3),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [201] = {.entry = {.count = 1, .reusable = true}}, SHIFT(301),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(312),
  [205] = {.entry = {.count = 1, .reusable = false}}, SHIFT(313),
  [207] = {.entry = {.count = 1, .reusable = false}}, SHIFT(242),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [211] = {.entry = {.count = 1, .reusable = false}}, SHIFT(241),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [215] = {.entry = {.count = 1, .reusable = false}}, SHIFT(244),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_definition, 2),
  [219] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_definition, 2),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_definition, 2),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_definition, 2),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_definition, 2),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_definition, 2),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_extension, 3),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_extension, 3),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_extension, 3),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_extension, 3),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(265),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(307),
  [245] = {.entry = {.count = 1, .reusable = false}}, SHIFT(308),
  [247] = {.entry = {.count = 1, .reusable = false}}, SHIFT(186),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [251] = {.entry = {.count = 1, .reusable = false}}, SHIFT(185),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [255] = {.entry = {.count = 1, .reusable = false}}, SHIFT(188),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_definition, 3),
  [259] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_definition, 3),
  [261] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_definition, 3),
  [263] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_definition, 3),
  [265] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directives, 1),
  [267] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directives, 1),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_directives_repeat1, 2),
  [271] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_directives_repeat1, 2),
  [273] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_directives_repeat1, 2), SHIFT_REPEAT(264),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive, 2),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [280] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive, 2),
  [282] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_scalar_type_definition, 2),
  [284] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_scalar_type_definition, 2),
  [286] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_scalar_type_definition, 3),
  [288] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_scalar_type_definition, 3),
  [290] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_scalar_type_extension, 3),
  [292] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_scalar_type_extension, 3),
  [294] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_extension, 5),
  [296] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_extension, 5),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive, 3),
  [300] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive, 3),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 3),
  [304] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 3),
  [306] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_implements_interfaces, 2),
  [308] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_implements_interfaces, 2),
  [310] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_extension, 4),
  [312] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_extension, 4),
  [314] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_member_types, 1),
  [316] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_member_types, 1),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [320] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_input_object_type_extension_repeat1, 2),
  [324] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_input_object_type_extension_repeat1, 2), SHIFT_REPEAT(175),
  [327] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_input_object_type_extension_repeat1, 2),
  [329] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_implements_interfaces, 3),
  [331] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_implements_interfaces, 3),
  [333] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_member_types, 4),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_member_types, 4),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_extension, 4),
  [341] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_extension, 4),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_union_member_types_repeat1, 2),
  [345] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_union_member_types_repeat1, 2), SHIFT_REPEAT(243),
  [348] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_union_member_types_repeat1, 2),
  [350] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_member_types, 2),
  [352] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_member_types, 2),
  [354] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_extension, 4),
  [356] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_extension, 4),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_definition, 5),
  [360] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_definition, 5),
  [362] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_extension, 5),
  [364] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_extension, 5),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_definition, 5),
  [368] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_definition, 5),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_member_types, 3),
  [372] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_member_types, 3),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_definition, 4),
  [376] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_definition, 4),
  [378] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_definition, 4),
  [380] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_definition, 4),
  [382] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_extension, 5),
  [384] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_extension, 5),
  [386] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_definition, 4),
  [388] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_definition, 4),
  [390] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_location, 1),
  [392] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_location, 1),
  [394] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_definition, 8),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [398] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_definition, 8),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_locations, 1),
  [402] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_locations, 1),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_definition, 5),
  [406] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_definition, 5),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_locations, 2),
  [410] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_locations, 2),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_definition, 6),
  [414] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_definition, 6),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_locations, 3),
  [418] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_locations, 3),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive_definition, 7),
  [422] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive_definition, 7),
  [424] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_system_directive_location, 1),
  [426] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_system_directive_location, 1),
  [428] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_executable_directive_location, 1),
  [430] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_executable_directive_location, 1),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_fields_definition, 3),
  [434] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_fields_definition, 3),
  [436] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_schema_extension, 5),
  [438] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_schema_extension, 5),
  [440] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_extension, 6),
  [442] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_extension, 6),
  [444] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_extension, 6),
  [446] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_extension, 6),
  [448] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_schema_extension, 6),
  [450] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_schema_extension, 6),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_object_type_definition, 5),
  [454] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_object_type_definition, 5),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_value, 3),
  [458] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_value, 3),
  [460] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_value, 3),
  [462] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_value, 3),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_value, 3),
  [466] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_value, 3),
  [468] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_schema_definition, 4),
  [470] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_schema_definition, 4),
  [472] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_definition, 5),
  [474] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_definition, 5),
  [476] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_value, 2),
  [478] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_value, 2),
  [480] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_definition, 5),
  [482] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_definition, 5),
  [484] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable, 2),
  [486] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 2),
  [488] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_definition, 1),
  [490] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_definition, 1),
  [492] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_scalar_type_extension, 4),
  [494] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_scalar_type_extension, 4),
  [496] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fields_definition, 2),
  [498] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fields_definition, 2),
  [500] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_value, 2),
  [502] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_value, 2),
  [504] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean_value, 1),
  [506] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean_value, 1),
  [508] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_schema_definition, 6),
  [510] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_schema_definition, 6),
  [512] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value, 1),
  [514] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value, 1),
  [516] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_type_definition, 6),
  [518] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_type_definition, 6),
  [520] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interface_type_definition, 6),
  [522] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interface_type_definition, 6),
  [524] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_extension, 1),
  [526] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_extension, 1),
  [528] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_values_definition, 3),
  [530] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_values_definition, 3),
  [532] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value, 1),
  [534] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_value, 1),
  [536] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_item, 1),
  [538] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_item, 1),
  [540] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fields_definition, 3),
  [542] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fields_definition, 3),
  [544] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_type_extension, 5),
  [546] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_type_extension, 5),
  [548] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type_extension, 5),
  [550] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type_extension, 5),
  [552] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_scalar_type_definition, 4),
  [554] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_scalar_type_definition, 4),
  [556] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_schema_definition, 5),
  [558] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_schema_definition, 5),
  [560] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_value_definition, 4),
  [562] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [564] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_value_definition, 4),
  [566] = {.entry = {.count = 1, .reusable = true}}, SHIFT(289),
  [568] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_value_definition, 3),
  [570] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_value_definition, 3),
  [572] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_value_definition, 5),
  [574] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_value_definition, 5),
  [576] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 1),
  [578] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type, 1),
  [580] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [582] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_input_fields_definition_repeat1, 2),
  [584] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_input_fields_definition_repeat1, 2), SHIFT_REPEAT(307),
  [587] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_input_fields_definition_repeat1, 2), SHIFT_REPEAT(308),
  [590] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_input_fields_definition_repeat1, 2), SHIFT_REPEAT(271),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_enum_values_definition_repeat1, 2),
  [595] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_enum_values_definition_repeat1, 2), SHIFT_REPEAT(307),
  [598] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_enum_values_definition_repeat1, 2), SHIFT_REPEAT(308),
  [601] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_enum_values_definition_repeat1, 2), SHIFT_REPEAT(188),
  [604] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [606] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [608] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_type, 3),
  [610] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_type, 3),
  [612] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_definition, 5),
  [614] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_definition, 5),
  [616] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [618] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [620] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_definition, 3),
  [622] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_definition, 3),
  [624] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [626] = {.entry = {.count = 1, .reusable = true}}, SHIFT(284),
  [628] = {.entry = {.count = 1, .reusable = true}}, SHIFT(269),
  [630] = {.entry = {.count = 1, .reusable = true}}, SHIFT(282),
  [632] = {.entry = {.count = 1, .reusable = true}}, SHIFT(273),
  [634] = {.entry = {.count = 1, .reusable = true}}, SHIFT(281),
  [636] = {.entry = {.count = 1, .reusable = true}}, SHIFT(254),
  [638] = {.entry = {.count = 1, .reusable = true}}, SHIFT(250),
  [640] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_directives_repeat1, 2), SHIFT_REPEAT(289),
  [643] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_description, 1),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [649] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition, 1),
  [651] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_value_definition, 1),
  [653] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_non_null_type, 2),
  [655] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_non_null_type, 2),
  [657] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_definition, 4),
  [659] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_definition, 4),
  [661] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fields_definition_repeat1, 2),
  [663] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fields_definition_repeat1, 2), SHIFT_REPEAT(307),
  [666] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fields_definition_repeat1, 2), SHIFT_REPEAT(308),
  [669] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fields_definition_repeat1, 2), SHIFT_REPEAT(228),
  [672] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [674] = {.entry = {.count = 1, .reusable = true}}, SHIFT(271),
  [676] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition, 2),
  [678] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_value_definition, 2),
  [680] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [682] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_schema_definition_repeat1, 2),
  [684] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_schema_definition_repeat1, 2), SHIFT_REPEAT(256),
  [687] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [689] = {.entry = {.count = 1, .reusable = true}}, SHIFT(256),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [693] = {.entry = {.count = 1, .reusable = true}}, SHIFT(303),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(300),
  [697] = {.entry = {.count = 1, .reusable = true}}, SHIFT(293),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(292),
  [701] = {.entry = {.count = 1, .reusable = true}}, SHIFT(255),
  [703] = {.entry = {.count = 1, .reusable = true}}, SHIFT(267),
  [705] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [711] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [715] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_default_value, 2),
  [717] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_default_value, 2),
  [719] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_value_definition, 6),
  [721] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_input_value_definition, 6),
  [723] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [725] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [729] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [731] = {.entry = {.count = 1, .reusable = true}}, SHIFT(262),
  [733] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root_operation_type_definition, 3),
  [735] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_definition, 6),
  [737] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_definition, 6),
  [739] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enum_value_definition, 3),
  [741] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enum_value_definition, 3),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [745] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [747] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [749] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [751] = {.entry = {.count = 1, .reusable = true}}, SHIFT(268),
  [753] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2),
  [755] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2), SHIFT_REPEAT(262),
  [758] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [760] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [762] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [764] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [766] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [768] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [770] = {.entry = {.count = 1, .reusable = true}}, SHIFT(277),
  [772] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [774] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_object_value_repeat1, 2),
  [776] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_object_value_repeat1, 2), SHIFT_REPEAT(272),
  [779] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [783] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments_definition, 3),
  [785] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_field, 4),
  [787] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [789] = {.entry = {.count = 1, .reusable = true}}, SHIFT(275),
  [791] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument, 3),
  [793] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_field, 3),
  [795] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [797] = {.entry = {.count = 1, .reusable = true}}, SHIFT(279),
  [799] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [801] = {.entry = {.count = 1, .reusable = false}}, SHIFT(296),
  [803] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [805] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [807] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [809] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_operation_type, 1),
  [811] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [813] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [815] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [817] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [819] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [821] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [823] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [825] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [827] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [829] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [831] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [833] = {.entry = {.count = 1, .reusable = true}}, SHIFT(283),
  [835] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [837] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [839] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [841] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [843] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [845] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [847] = {.entry = {.count = 1, .reusable = false}}, SHIFT(286),
  [849] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [851] = {.entry = {.count = 1, .reusable = true}}, SHIFT(285),
  [853] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [855] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [857] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [859] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [861] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [863] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [865] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [867] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [869] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [871] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [873] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [875] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [877] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [879] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [881] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [883] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [885] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [887] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [889] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [891] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [893] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [895] = {.entry = {.count = 1, .reusable = false}}, SHIFT(290),
  [897] = {.entry = {.count = 1, .reusable = false}}, SHIFT(291),
  [899] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [901] = {.entry = {.count = 1, .reusable = false}}, SHIFT(298),
  [903] = {.entry = {.count = 1, .reusable = false}}, SHIFT(299),
  [905] = {.entry = {.count = 1, .reusable = false}}, SHIFT(305),
  [907] = {.entry = {.count = 1, .reusable = false}}, SHIFT(306),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_graphql(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
