#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 121
#define LARGE_STATE_COUNT 16
#define SYMBOL_COUNT 36
#define ALIAS_COUNT 0
#define TOKEN_COUNT 22
#define EXTERNAL_TOKEN_COUNT 1
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum {
  sym_identifier = 1,
  sym_nothing = 2,
  sym_preproc_continuation_line = 3,
  sym_preproc_line = 4,
  aux_sym_preproc_include_token1 = 5,
  anon_sym_LT = 6,
  anon_sym_GT = 7,
  sym_path = 8,
  aux_sym_define_token1 = 9,
  anon_sym_LF = 10,
  aux_sym_preproc_if_token1 = 11,
  aux_sym_preproc_if_token2 = 12,
  aux_sym_preproc_elif_token1 = 13,
  aux_sym_preproc_else_token1 = 14,
  aux_sym_undef_token1 = 15,
  aux_sym_preproc_nothing_token1 = 16,
  aux_sym_string_literal_token1 = 17,
  aux_sym_char_literal_token1 = 18,
  sym_integer_literal = 19,
  sym_comment = 20,
  sym_raw_string_literal = 21,
  sym_translation_unit = 22,
  sym__top_level_item = 23,
  sym_preproc_include = 24,
  sym_define = 25,
  sym_preproc_if = 26,
  sym_preproc_elif = 27,
  sym_preproc_else = 28,
  sym_undef = 29,
  sym_preproc_nothing = 30,
  sym_string_literal = 31,
  sym_char_literal = 32,
  aux_sym_translation_unit_repeat1 = 33,
  aux_sym_define_repeat1 = 34,
  aux_sym_preproc_if_repeat1 = 35,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [sym_nothing] = "nothing",
  [sym_preproc_continuation_line] = "preproc_continuation_line",
  [sym_preproc_line] = "preproc_line",
  [aux_sym_preproc_include_token1] = "preproc_include_token1",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [sym_path] = "path",
  [aux_sym_define_token1] = "define_token1",
  [anon_sym_LF] = "\n",
  [aux_sym_preproc_if_token1] = "preproc_if_token1",
  [aux_sym_preproc_if_token2] = "preproc_if_token2",
  [aux_sym_preproc_elif_token1] = "preproc_elif_token1",
  [aux_sym_preproc_else_token1] = "preproc_else_token1",
  [aux_sym_undef_token1] = "undef_token1",
  [aux_sym_preproc_nothing_token1] = "preproc_nothing_token1",
  [aux_sym_string_literal_token1] = "string_literal_token1",
  [aux_sym_char_literal_token1] = "char_literal_token1",
  [sym_integer_literal] = "integer_literal",
  [sym_comment] = "comment",
  [sym_raw_string_literal] = "raw_string_literal",
  [sym_translation_unit] = "translation_unit",
  [sym__top_level_item] = "_top_level_item",
  [sym_preproc_include] = "preproc_include",
  [sym_define] = "define",
  [sym_preproc_if] = "preproc_if",
  [sym_preproc_elif] = "preproc_elif",
  [sym_preproc_else] = "preproc_else",
  [sym_undef] = "undef",
  [sym_preproc_nothing] = "preproc_nothing",
  [sym_string_literal] = "string_literal",
  [sym_char_literal] = "char_literal",
  [aux_sym_translation_unit_repeat1] = "translation_unit_repeat1",
  [aux_sym_define_repeat1] = "define_repeat1",
  [aux_sym_preproc_if_repeat1] = "preproc_if_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [sym_nothing] = sym_nothing,
  [sym_preproc_continuation_line] = sym_preproc_continuation_line,
  [sym_preproc_line] = sym_preproc_line,
  [aux_sym_preproc_include_token1] = aux_sym_preproc_include_token1,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [sym_path] = sym_path,
  [aux_sym_define_token1] = aux_sym_define_token1,
  [anon_sym_LF] = anon_sym_LF,
  [aux_sym_preproc_if_token1] = aux_sym_preproc_if_token1,
  [aux_sym_preproc_if_token2] = aux_sym_preproc_if_token2,
  [aux_sym_preproc_elif_token1] = aux_sym_preproc_elif_token1,
  [aux_sym_preproc_else_token1] = aux_sym_preproc_else_token1,
  [aux_sym_undef_token1] = aux_sym_undef_token1,
  [aux_sym_preproc_nothing_token1] = aux_sym_preproc_nothing_token1,
  [aux_sym_string_literal_token1] = aux_sym_string_literal_token1,
  [aux_sym_char_literal_token1] = aux_sym_char_literal_token1,
  [sym_integer_literal] = sym_integer_literal,
  [sym_comment] = sym_comment,
  [sym_raw_string_literal] = sym_raw_string_literal,
  [sym_translation_unit] = sym_translation_unit,
  [sym__top_level_item] = sym__top_level_item,
  [sym_preproc_include] = sym_preproc_include,
  [sym_define] = sym_define,
  [sym_preproc_if] = sym_preproc_if,
  [sym_preproc_elif] = sym_preproc_elif,
  [sym_preproc_else] = sym_preproc_else,
  [sym_undef] = sym_undef,
  [sym_preproc_nothing] = sym_preproc_nothing,
  [sym_string_literal] = sym_string_literal,
  [sym_char_literal] = sym_char_literal,
  [aux_sym_translation_unit_repeat1] = aux_sym_translation_unit_repeat1,
  [aux_sym_define_repeat1] = aux_sym_define_repeat1,
  [aux_sym_preproc_if_repeat1] = aux_sym_preproc_if_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_nothing] = {
    .visible = true,
    .named = true,
  },
  [sym_preproc_continuation_line] = {
    .visible = true,
    .named = true,
  },
  [sym_preproc_line] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_preproc_include_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_path] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_define_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LF] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_preproc_if_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_preproc_if_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_preproc_elif_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_preproc_else_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_undef_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_preproc_nothing_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_literal_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_char_literal_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_integer_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_raw_string_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_translation_unit] = {
    .visible = true,
    .named = true,
  },
  [sym__top_level_item] = {
    .visible = false,
    .named = true,
  },
  [sym_preproc_include] = {
    .visible = true,
    .named = true,
  },
  [sym_define] = {
    .visible = true,
    .named = true,
  },
  [sym_preproc_if] = {
    .visible = true,
    .named = true,
  },
  [sym_preproc_elif] = {
    .visible = true,
    .named = true,
  },
  [sym_preproc_else] = {
    .visible = true,
    .named = true,
  },
  [sym_undef] = {
    .visible = true,
    .named = true,
  },
  [sym_preproc_nothing] = {
    .visible = true,
    .named = true,
  },
  [sym_string_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_char_literal] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_translation_unit_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_define_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_preproc_if_repeat1] = {
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

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 3,
  [5] = 2,
  [6] = 3,
  [7] = 2,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 8,
  [13] = 13,
  [14] = 8,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 21,
  [31] = 22,
  [32] = 24,
  [33] = 25,
  [34] = 26,
  [35] = 21,
  [36] = 27,
  [37] = 20,
  [38] = 19,
  [39] = 25,
  [40] = 18,
  [41] = 23,
  [42] = 28,
  [43] = 27,
  [44] = 16,
  [45] = 29,
  [46] = 17,
  [47] = 22,
  [48] = 17,
  [49] = 20,
  [50] = 23,
  [51] = 19,
  [52] = 26,
  [53] = 18,
  [54] = 28,
  [55] = 24,
  [56] = 16,
  [57] = 29,
  [58] = 58,
  [59] = 59,
  [60] = 59,
  [61] = 58,
  [62] = 58,
  [63] = 59,
  [64] = 64,
  [65] = 65,
  [66] = 65,
  [67] = 65,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 70,
  [73] = 73,
  [74] = 74,
  [75] = 68,
  [76] = 70,
  [77] = 68,
  [78] = 71,
  [79] = 79,
  [80] = 69,
  [81] = 79,
  [82] = 74,
  [83] = 71,
  [84] = 79,
  [85] = 69,
  [86] = 74,
  [87] = 87,
  [88] = 87,
  [89] = 89,
  [90] = 90,
  [91] = 89,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 95,
  [99] = 97,
  [100] = 94,
  [101] = 92,
  [102] = 102,
  [103] = 103,
  [104] = 103,
  [105] = 90,
  [106] = 89,
  [107] = 107,
  [108] = 107,
  [109] = 95,
  [110] = 97,
  [111] = 94,
  [112] = 107,
  [113] = 92,
  [114] = 103,
  [115] = 87,
  [116] = 90,
  [117] = 96,
  [118] = 93,
  [119] = 96,
  [120] = 93,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(73);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(96);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(12);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(72)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(102);
      if (lookahead == 'd') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(2);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(102);
      if (lookahead == 'd') ADVANCE(3);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(102);
      if (lookahead == 'e') ADVANCE(4);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(102);
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(102);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '\n') ADVANCE(104);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == '\n') ADVANCE(105);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 8:
      if (lookahead == '\n') ADVANCE(103);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 9:
      if (lookahead == '\n') ADVANCE(101);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      END_STATE();
    case 10:
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(78);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(83);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(79);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      if (lookahead != 0) ADVANCE(89);
      END_STATE();
    case 11:
      if (lookahead == '\n') ADVANCE(80);
      if (lookahead == '\r') ADVANCE(80);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(85);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(81);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      if (lookahead != 0) ADVANCE(89);
      END_STATE();
    case 12:
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(96);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(12);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 13:
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(96);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 14:
      if (lookahead == '"') ADVANCE(108);
      if (lookahead == '\\') ADVANCE(69);
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 15:
      if (lookahead == '\'') ADVANCE(109);
      if (lookahead == '\\') ADVANCE(70);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 16:
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(113);
      END_STATE();
    case 17:
      if (lookahead == '*') ADVANCE(17);
      if (lookahead == '/') ADVANCE(112);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 18:
      if (lookahead == '*') ADVANCE(17);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(43);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(107);
      END_STATE();
    case 21:
      if (lookahead == 'c') ADVANCE(50);
      END_STATE();
    case 22:
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(49);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(22);
      END_STATE();
    case 23:
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(48);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(23);
      END_STATE();
    case 24:
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(53);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(24);
      END_STATE();
    case 25:
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(62);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(25);
      END_STATE();
    case 26:
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(52);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(26);
      END_STATE();
    case 27:
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(27);
      END_STATE();
    case 28:
      if (lookahead == 'd') ADVANCE(47);
      END_STATE();
    case 29:
      if (lookahead == 'd') ADVANCE(33);
      END_STATE();
    case 30:
      if (lookahead == 'd') ADVANCE(36);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(42);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(107);
      END_STATE();
    case 33:
      if (lookahead == 'e') ADVANCE(94);
      END_STATE();
    case 34:
      if (lookahead == 'e') ADVANCE(65);
      END_STATE();
    case 35:
      if (lookahead == 'e') ADVANCE(67);
      END_STATE();
    case 36:
      if (lookahead == 'e') ADVANCE(39);
      END_STATE();
    case 37:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 38:
      if (lookahead == 'f') ADVANCE(1);
      if (lookahead == 'n') ADVANCE(21);
      END_STATE();
    case 39:
      if (lookahead == 'f') ADVANCE(66);
      END_STATE();
    case 40:
      if (lookahead == 'f') ADVANCE(6);
      END_STATE();
    case 41:
      if (lookahead == 'f') ADVANCE(8);
      END_STATE();
    case 42:
      if (lookahead == 'f') ADVANCE(46);
      END_STATE();
    case 43:
      if (lookahead == 'g') ADVANCE(51);
      END_STATE();
    case 44:
      if (lookahead == 'i') ADVANCE(55);
      END_STATE();
    case 45:
      if (lookahead == 'i') ADVANCE(40);
      if (lookahead == 's') ADVANCE(37);
      END_STATE();
    case 46:
      if (lookahead == 'i') ADVANCE(56);
      END_STATE();
    case 47:
      if (lookahead == 'i') ADVANCE(41);
      END_STATE();
    case 48:
      if (lookahead == 'l') ADVANCE(45);
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 49:
      if (lookahead == 'l') ADVANCE(45);
      if (lookahead == 'n') ADVANCE(28);
      if (lookahead == 'r') ADVANCE(60);
      END_STATE();
    case 50:
      if (lookahead == 'l') ADVANCE(64);
      END_STATE();
    case 51:
      if (lookahead == 'm') ADVANCE(20);
      END_STATE();
    case 52:
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 53:
      if (lookahead == 'n') ADVANCE(28);
      if (lookahead == 'r') ADVANCE(60);
      END_STATE();
    case 54:
      if (lookahead == 'n') ADVANCE(30);
      END_STATE();
    case 55:
      if (lookahead == 'n') ADVANCE(32);
      END_STATE();
    case 56:
      if (lookahead == 'n') ADVANCE(35);
      END_STATE();
    case 57:
      if (lookahead == 'n') ADVANCE(34);
      END_STATE();
    case 58:
      if (lookahead == 'o') ADVANCE(61);
      END_STATE();
    case 59:
      if (lookahead == 'r') ADVANCE(19);
      END_STATE();
    case 60:
      if (lookahead == 'r') ADVANCE(58);
      END_STATE();
    case 61:
      if (lookahead == 'r') ADVANCE(107);
      END_STATE();
    case 62:
      if (lookahead == 'r') ADVANCE(60);
      END_STATE();
    case 63:
      if (lookahead == 't') ADVANCE(95);
      END_STATE();
    case 64:
      if (lookahead == 'u') ADVANCE(29);
      END_STATE();
    case 65:
      if (lookahead == 'x') ADVANCE(63);
      END_STATE();
    case 66:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(106);
      END_STATE();
    case 67:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(100);
      END_STATE();
    case 68:
      if (lookahead != 0 &&
          lookahead != '\r') ADVANCE(113);
      if (lookahead == '\r') ADVANCE(114);
      END_STATE();
    case 69:
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 70:
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 71:
      if (eof) ADVANCE(73);
      if (lookahead == '\n') ADVANCE(77);
      if (lookahead == '\r') ADVANCE(77);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(86);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(76);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      if (lookahead != 0) ADVANCE(89);
      END_STATE();
    case 72:
      if (eof) ADVANCE(73);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(96);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(72)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_nothing);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(77);
      if (lookahead == '\r') ADVANCE(77);
      if (lookahead == '#') ADVANCE(86);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(76);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(89);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(77);
      if (lookahead == '\r') ADVANCE(77);
      if (lookahead == '#') ADVANCE(88);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(89);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(78);
      if (lookahead == '#') ADVANCE(84);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(78);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(89);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(78);
      if (lookahead == '#') ADVANCE(83);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(79);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(89);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(80);
      if (lookahead == '\r') ADVANCE(80);
      if (lookahead == '#') ADVANCE(87);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(80);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(89);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(80);
      if (lookahead == '\r') ADVANCE(80);
      if (lookahead == '#') ADVANCE(85);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == 'R') ADVANCE(75);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(81);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(89);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(113);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(49);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(22);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(48);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(23);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(53);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(24);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(62);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == 'p') ADVANCE(59);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(25);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'e') ADVANCE(52);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(26);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(31);
      if (lookahead == 'i') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(54);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(27);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '#' &&
          lookahead != '\'' &&
          (lookahead < '/' || '9' < lookahead) &&
          lookahead != 'R') ADVANCE(89);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(sym_preproc_continuation_line);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\r') ADVANCE(92);
      if (lookahead == '\\') ADVANCE(91);
      if (lookahead != 0) ADVANCE(93);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\\') ADVANCE(91);
      if (lookahead != 0) ADVANCE(93);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\\') ADVANCE(91);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(93);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(aux_sym_preproc_include_token1);
      if (lookahead == '_') ADVANCE(57);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(95);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(aux_sym_preproc_include_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(95);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(sym_path);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(98);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(99);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(sym_path);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(99);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(aux_sym_define_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(100);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(101);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(aux_sym_preproc_if_token1);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(aux_sym_preproc_if_token2);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(aux_sym_preproc_elif_token1);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(aux_sym_preproc_else_token1);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(aux_sym_undef_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(106);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(aux_sym_preproc_nothing_token1);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(aux_sym_char_literal_token1);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(sym_integer_literal);
      if (lookahead == '\'') ADVANCE(111);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(sym_integer_literal);
      if (lookahead == '\'' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(111);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\\') ADVANCE(68);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(113);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(113);
      if (lookahead == '\\') ADVANCE(68);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 71, .external_lex_state = 1},
  [2] = {.lex_state = 10, .external_lex_state = 1},
  [3] = {.lex_state = 10, .external_lex_state = 1},
  [4] = {.lex_state = 10, .external_lex_state = 1},
  [5] = {.lex_state = 10, .external_lex_state = 1},
  [6] = {.lex_state = 10, .external_lex_state = 1},
  [7] = {.lex_state = 10, .external_lex_state = 1},
  [8] = {.lex_state = 10, .external_lex_state = 1},
  [9] = {.lex_state = 10, .external_lex_state = 1},
  [10] = {.lex_state = 10, .external_lex_state = 1},
  [11] = {.lex_state = 11, .external_lex_state = 1},
  [12] = {.lex_state = 11, .external_lex_state = 1},
  [13] = {.lex_state = 71, .external_lex_state = 1},
  [14] = {.lex_state = 71, .external_lex_state = 1},
  [15] = {.lex_state = 11, .external_lex_state = 1},
  [16] = {.lex_state = 10, .external_lex_state = 1},
  [17] = {.lex_state = 10, .external_lex_state = 1},
  [18] = {.lex_state = 10, .external_lex_state = 1},
  [19] = {.lex_state = 10, .external_lex_state = 1},
  [20] = {.lex_state = 10, .external_lex_state = 1},
  [21] = {.lex_state = 10, .external_lex_state = 1},
  [22] = {.lex_state = 10, .external_lex_state = 1},
  [23] = {.lex_state = 10, .external_lex_state = 1},
  [24] = {.lex_state = 10, .external_lex_state = 1},
  [25] = {.lex_state = 10, .external_lex_state = 1},
  [26] = {.lex_state = 10, .external_lex_state = 1},
  [27] = {.lex_state = 10, .external_lex_state = 1},
  [28] = {.lex_state = 10, .external_lex_state = 1},
  [29] = {.lex_state = 10, .external_lex_state = 1},
  [30] = {.lex_state = 11, .external_lex_state = 1},
  [31] = {.lex_state = 71, .external_lex_state = 1},
  [32] = {.lex_state = 11, .external_lex_state = 1},
  [33] = {.lex_state = 11, .external_lex_state = 1},
  [34] = {.lex_state = 11, .external_lex_state = 1},
  [35] = {.lex_state = 71, .external_lex_state = 1},
  [36] = {.lex_state = 11, .external_lex_state = 1},
  [37] = {.lex_state = 71, .external_lex_state = 1},
  [38] = {.lex_state = 71, .external_lex_state = 1},
  [39] = {.lex_state = 71, .external_lex_state = 1},
  [40] = {.lex_state = 71, .external_lex_state = 1},
  [41] = {.lex_state = 11, .external_lex_state = 1},
  [42] = {.lex_state = 71, .external_lex_state = 1},
  [43] = {.lex_state = 71, .external_lex_state = 1},
  [44] = {.lex_state = 71, .external_lex_state = 1},
  [45] = {.lex_state = 71, .external_lex_state = 1},
  [46] = {.lex_state = 71, .external_lex_state = 1},
  [47] = {.lex_state = 11, .external_lex_state = 1},
  [48] = {.lex_state = 11, .external_lex_state = 1},
  [49] = {.lex_state = 11, .external_lex_state = 1},
  [50] = {.lex_state = 71, .external_lex_state = 1},
  [51] = {.lex_state = 11, .external_lex_state = 1},
  [52] = {.lex_state = 71, .external_lex_state = 1},
  [53] = {.lex_state = 11, .external_lex_state = 1},
  [54] = {.lex_state = 11, .external_lex_state = 1},
  [55] = {.lex_state = 71, .external_lex_state = 1},
  [56] = {.lex_state = 11, .external_lex_state = 1},
  [57] = {.lex_state = 11, .external_lex_state = 1},
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
  [68] = {.lex_state = 93},
  [69] = {.lex_state = 93},
  [70] = {.lex_state = 93},
  [71] = {.lex_state = 93},
  [72] = {.lex_state = 93},
  [73] = {.lex_state = 93},
  [74] = {.lex_state = 93},
  [75] = {.lex_state = 93},
  [76] = {.lex_state = 93},
  [77] = {.lex_state = 93},
  [78] = {.lex_state = 93},
  [79] = {.lex_state = 93},
  [80] = {.lex_state = 93},
  [81] = {.lex_state = 93},
  [82] = {.lex_state = 93},
  [83] = {.lex_state = 93},
  [84] = {.lex_state = 93},
  [85] = {.lex_state = 93},
  [86] = {.lex_state = 93},
  [87] = {.lex_state = 9},
  [88] = {.lex_state = 9},
  [89] = {.lex_state = 9},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 9},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 9},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 9},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 9},
  [104] = {.lex_state = 9},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 9},
  [107] = {.lex_state = 98},
  [108] = {.lex_state = 98},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 9},
  [112] = {.lex_state = 98},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 9},
  [115] = {.lex_state = 9},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
};

enum {
  ts_external_token_raw_string_literal = 0,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_raw_string_literal] = sym_raw_string_literal,
};

static const bool ts_external_scanner_states[2][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_raw_string_literal] = true,
  },
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [aux_sym_preproc_include_token1] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [aux_sym_define_token1] = ACTIONS(1),
    [aux_sym_preproc_if_token1] = ACTIONS(1),
    [aux_sym_preproc_if_token2] = ACTIONS(1),
    [aux_sym_preproc_elif_token1] = ACTIONS(1),
    [aux_sym_preproc_else_token1] = ACTIONS(1),
    [aux_sym_undef_token1] = ACTIONS(1),
    [aux_sym_preproc_nothing_token1] = ACTIONS(1),
    [aux_sym_string_literal_token1] = ACTIONS(1),
    [aux_sym_char_literal_token1] = ACTIONS(1),
    [sym_integer_literal] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
    [sym_raw_string_literal] = ACTIONS(1),
  },
  [1] = {
    [sym_translation_unit] = STATE(102),
    [sym__top_level_item] = STATE(13),
    [sym_preproc_include] = STATE(13),
    [sym_define] = STATE(13),
    [sym_preproc_if] = STATE(13),
    [sym_undef] = STATE(13),
    [sym_preproc_nothing] = STATE(13),
    [sym_string_literal] = STATE(13),
    [sym_char_literal] = STATE(13),
    [aux_sym_translation_unit_repeat1] = STATE(13),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_nothing] = ACTIONS(5),
    [aux_sym_preproc_include_token1] = ACTIONS(7),
    [aux_sym_define_token1] = ACTIONS(9),
    [aux_sym_preproc_if_token1] = ACTIONS(11),
    [aux_sym_undef_token1] = ACTIONS(13),
    [aux_sym_preproc_nothing_token1] = ACTIONS(15),
    [aux_sym_string_literal_token1] = ACTIONS(17),
    [aux_sym_char_literal_token1] = ACTIONS(19),
    [sym_integer_literal] = ACTIONS(21),
    [sym_comment] = ACTIONS(5),
    [sym_raw_string_literal] = ACTIONS(21),
  },
  [2] = {
    [sym__top_level_item] = STATE(8),
    [sym_preproc_include] = STATE(8),
    [sym_define] = STATE(8),
    [sym_preproc_if] = STATE(8),
    [sym_preproc_elif] = STATE(62),
    [sym_preproc_else] = STATE(101),
    [sym_undef] = STATE(8),
    [sym_preproc_nothing] = STATE(8),
    [sym_string_literal] = STATE(8),
    [sym_char_literal] = STATE(8),
    [aux_sym_translation_unit_repeat1] = STATE(8),
    [aux_sym_preproc_if_repeat1] = STATE(62),
    [sym_nothing] = ACTIONS(23),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(31),
    [aux_sym_preproc_elif_token1] = ACTIONS(33),
    [aux_sym_preproc_else_token1] = ACTIONS(35),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(45),
    [sym_comment] = ACTIONS(23),
    [sym_raw_string_literal] = ACTIONS(45),
  },
  [3] = {
    [sym__top_level_item] = STATE(2),
    [sym_preproc_include] = STATE(2),
    [sym_define] = STATE(2),
    [sym_preproc_if] = STATE(2),
    [sym_preproc_elif] = STATE(59),
    [sym_preproc_else] = STATE(95),
    [sym_undef] = STATE(2),
    [sym_preproc_nothing] = STATE(2),
    [sym_string_literal] = STATE(2),
    [sym_char_literal] = STATE(2),
    [aux_sym_translation_unit_repeat1] = STATE(2),
    [aux_sym_preproc_if_repeat1] = STATE(59),
    [sym_nothing] = ACTIONS(47),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(49),
    [aux_sym_preproc_elif_token1] = ACTIONS(33),
    [aux_sym_preproc_else_token1] = ACTIONS(35),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(51),
    [sym_comment] = ACTIONS(47),
    [sym_raw_string_literal] = ACTIONS(51),
  },
  [4] = {
    [sym__top_level_item] = STATE(7),
    [sym_preproc_include] = STATE(7),
    [sym_define] = STATE(7),
    [sym_preproc_if] = STATE(7),
    [sym_preproc_elif] = STATE(63),
    [sym_preproc_else] = STATE(109),
    [sym_undef] = STATE(7),
    [sym_preproc_nothing] = STATE(7),
    [sym_string_literal] = STATE(7),
    [sym_char_literal] = STATE(7),
    [aux_sym_translation_unit_repeat1] = STATE(7),
    [aux_sym_preproc_if_repeat1] = STATE(63),
    [sym_nothing] = ACTIONS(53),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(55),
    [aux_sym_preproc_elif_token1] = ACTIONS(33),
    [aux_sym_preproc_else_token1] = ACTIONS(35),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(57),
    [sym_comment] = ACTIONS(53),
    [sym_raw_string_literal] = ACTIONS(57),
  },
  [5] = {
    [sym__top_level_item] = STATE(8),
    [sym_preproc_include] = STATE(8),
    [sym_define] = STATE(8),
    [sym_preproc_if] = STATE(8),
    [sym_preproc_elif] = STATE(61),
    [sym_preproc_else] = STATE(92),
    [sym_undef] = STATE(8),
    [sym_preproc_nothing] = STATE(8),
    [sym_string_literal] = STATE(8),
    [sym_char_literal] = STATE(8),
    [aux_sym_translation_unit_repeat1] = STATE(8),
    [aux_sym_preproc_if_repeat1] = STATE(61),
    [sym_nothing] = ACTIONS(23),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(59),
    [aux_sym_preproc_elif_token1] = ACTIONS(33),
    [aux_sym_preproc_else_token1] = ACTIONS(35),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(45),
    [sym_comment] = ACTIONS(23),
    [sym_raw_string_literal] = ACTIONS(45),
  },
  [6] = {
    [sym__top_level_item] = STATE(5),
    [sym_preproc_include] = STATE(5),
    [sym_define] = STATE(5),
    [sym_preproc_if] = STATE(5),
    [sym_preproc_elif] = STATE(60),
    [sym_preproc_else] = STATE(98),
    [sym_undef] = STATE(5),
    [sym_preproc_nothing] = STATE(5),
    [sym_string_literal] = STATE(5),
    [sym_char_literal] = STATE(5),
    [aux_sym_translation_unit_repeat1] = STATE(5),
    [aux_sym_preproc_if_repeat1] = STATE(60),
    [sym_nothing] = ACTIONS(61),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(63),
    [aux_sym_preproc_elif_token1] = ACTIONS(33),
    [aux_sym_preproc_else_token1] = ACTIONS(35),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(65),
    [sym_comment] = ACTIONS(61),
    [sym_raw_string_literal] = ACTIONS(65),
  },
  [7] = {
    [sym__top_level_item] = STATE(8),
    [sym_preproc_include] = STATE(8),
    [sym_define] = STATE(8),
    [sym_preproc_if] = STATE(8),
    [sym_preproc_elif] = STATE(58),
    [sym_preproc_else] = STATE(113),
    [sym_undef] = STATE(8),
    [sym_preproc_nothing] = STATE(8),
    [sym_string_literal] = STATE(8),
    [sym_char_literal] = STATE(8),
    [aux_sym_translation_unit_repeat1] = STATE(8),
    [aux_sym_preproc_if_repeat1] = STATE(58),
    [sym_nothing] = ACTIONS(23),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(67),
    [aux_sym_preproc_elif_token1] = ACTIONS(33),
    [aux_sym_preproc_else_token1] = ACTIONS(35),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(45),
    [sym_comment] = ACTIONS(23),
    [sym_raw_string_literal] = ACTIONS(45),
  },
  [8] = {
    [sym__top_level_item] = STATE(8),
    [sym_preproc_include] = STATE(8),
    [sym_define] = STATE(8),
    [sym_preproc_if] = STATE(8),
    [sym_undef] = STATE(8),
    [sym_preproc_nothing] = STATE(8),
    [sym_string_literal] = STATE(8),
    [sym_char_literal] = STATE(8),
    [aux_sym_translation_unit_repeat1] = STATE(8),
    [sym_nothing] = ACTIONS(69),
    [aux_sym_preproc_include_token1] = ACTIONS(72),
    [aux_sym_define_token1] = ACTIONS(75),
    [aux_sym_preproc_if_token1] = ACTIONS(78),
    [aux_sym_preproc_if_token2] = ACTIONS(81),
    [aux_sym_preproc_elif_token1] = ACTIONS(81),
    [aux_sym_preproc_else_token1] = ACTIONS(81),
    [aux_sym_undef_token1] = ACTIONS(83),
    [aux_sym_preproc_nothing_token1] = ACTIONS(86),
    [aux_sym_string_literal_token1] = ACTIONS(89),
    [aux_sym_char_literal_token1] = ACTIONS(92),
    [sym_integer_literal] = ACTIONS(95),
    [sym_comment] = ACTIONS(69),
    [sym_raw_string_literal] = ACTIONS(95),
  },
  [9] = {
    [sym__top_level_item] = STATE(8),
    [sym_preproc_include] = STATE(8),
    [sym_define] = STATE(8),
    [sym_preproc_if] = STATE(8),
    [sym_undef] = STATE(8),
    [sym_preproc_nothing] = STATE(8),
    [sym_string_literal] = STATE(8),
    [sym_char_literal] = STATE(8),
    [aux_sym_translation_unit_repeat1] = STATE(8),
    [sym_nothing] = ACTIONS(23),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(98),
    [aux_sym_preproc_elif_token1] = ACTIONS(98),
    [aux_sym_preproc_else_token1] = ACTIONS(98),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(45),
    [sym_comment] = ACTIONS(23),
    [sym_raw_string_literal] = ACTIONS(45),
  },
  [10] = {
    [sym__top_level_item] = STATE(9),
    [sym_preproc_include] = STATE(9),
    [sym_define] = STATE(9),
    [sym_preproc_if] = STATE(9),
    [sym_undef] = STATE(9),
    [sym_preproc_nothing] = STATE(9),
    [sym_string_literal] = STATE(9),
    [sym_char_literal] = STATE(9),
    [aux_sym_translation_unit_repeat1] = STATE(9),
    [sym_nothing] = ACTIONS(100),
    [aux_sym_preproc_include_token1] = ACTIONS(25),
    [aux_sym_define_token1] = ACTIONS(27),
    [aux_sym_preproc_if_token1] = ACTIONS(29),
    [aux_sym_preproc_if_token2] = ACTIONS(102),
    [aux_sym_preproc_elif_token1] = ACTIONS(102),
    [aux_sym_preproc_else_token1] = ACTIONS(102),
    [aux_sym_undef_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_string_literal_token1] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(43),
    [sym_integer_literal] = ACTIONS(104),
    [sym_comment] = ACTIONS(100),
    [sym_raw_string_literal] = ACTIONS(104),
  },
  [11] = {
    [sym__top_level_item] = STATE(15),
    [sym_preproc_include] = STATE(15),
    [sym_define] = STATE(15),
    [sym_preproc_if] = STATE(15),
    [sym_undef] = STATE(15),
    [sym_preproc_nothing] = STATE(15),
    [sym_string_literal] = STATE(15),
    [sym_char_literal] = STATE(15),
    [aux_sym_translation_unit_repeat1] = STATE(15),
    [sym_nothing] = ACTIONS(106),
    [aux_sym_preproc_include_token1] = ACTIONS(108),
    [aux_sym_define_token1] = ACTIONS(110),
    [aux_sym_preproc_if_token1] = ACTIONS(112),
    [aux_sym_preproc_if_token2] = ACTIONS(114),
    [aux_sym_undef_token1] = ACTIONS(116),
    [aux_sym_preproc_nothing_token1] = ACTIONS(118),
    [aux_sym_string_literal_token1] = ACTIONS(120),
    [aux_sym_char_literal_token1] = ACTIONS(122),
    [sym_integer_literal] = ACTIONS(124),
    [sym_comment] = ACTIONS(106),
    [sym_raw_string_literal] = ACTIONS(124),
  },
  [12] = {
    [sym__top_level_item] = STATE(12),
    [sym_preproc_include] = STATE(12),
    [sym_define] = STATE(12),
    [sym_preproc_if] = STATE(12),
    [sym_undef] = STATE(12),
    [sym_preproc_nothing] = STATE(12),
    [sym_string_literal] = STATE(12),
    [sym_char_literal] = STATE(12),
    [aux_sym_translation_unit_repeat1] = STATE(12),
    [sym_nothing] = ACTIONS(126),
    [aux_sym_preproc_include_token1] = ACTIONS(129),
    [aux_sym_define_token1] = ACTIONS(132),
    [aux_sym_preproc_if_token1] = ACTIONS(135),
    [aux_sym_preproc_if_token2] = ACTIONS(81),
    [aux_sym_undef_token1] = ACTIONS(138),
    [aux_sym_preproc_nothing_token1] = ACTIONS(141),
    [aux_sym_string_literal_token1] = ACTIONS(144),
    [aux_sym_char_literal_token1] = ACTIONS(147),
    [sym_integer_literal] = ACTIONS(150),
    [sym_comment] = ACTIONS(126),
    [sym_raw_string_literal] = ACTIONS(150),
  },
  [13] = {
    [sym__top_level_item] = STATE(14),
    [sym_preproc_include] = STATE(14),
    [sym_define] = STATE(14),
    [sym_preproc_if] = STATE(14),
    [sym_undef] = STATE(14),
    [sym_preproc_nothing] = STATE(14),
    [sym_string_literal] = STATE(14),
    [sym_char_literal] = STATE(14),
    [aux_sym_translation_unit_repeat1] = STATE(14),
    [ts_builtin_sym_end] = ACTIONS(153),
    [sym_nothing] = ACTIONS(155),
    [aux_sym_preproc_include_token1] = ACTIONS(7),
    [aux_sym_define_token1] = ACTIONS(9),
    [aux_sym_preproc_if_token1] = ACTIONS(11),
    [aux_sym_undef_token1] = ACTIONS(13),
    [aux_sym_preproc_nothing_token1] = ACTIONS(15),
    [aux_sym_string_literal_token1] = ACTIONS(17),
    [aux_sym_char_literal_token1] = ACTIONS(19),
    [sym_integer_literal] = ACTIONS(157),
    [sym_comment] = ACTIONS(155),
    [sym_raw_string_literal] = ACTIONS(157),
  },
  [14] = {
    [sym__top_level_item] = STATE(14),
    [sym_preproc_include] = STATE(14),
    [sym_define] = STATE(14),
    [sym_preproc_if] = STATE(14),
    [sym_undef] = STATE(14),
    [sym_preproc_nothing] = STATE(14),
    [sym_string_literal] = STATE(14),
    [sym_char_literal] = STATE(14),
    [aux_sym_translation_unit_repeat1] = STATE(14),
    [ts_builtin_sym_end] = ACTIONS(159),
    [sym_nothing] = ACTIONS(161),
    [aux_sym_preproc_include_token1] = ACTIONS(164),
    [aux_sym_define_token1] = ACTIONS(167),
    [aux_sym_preproc_if_token1] = ACTIONS(170),
    [aux_sym_undef_token1] = ACTIONS(173),
    [aux_sym_preproc_nothing_token1] = ACTIONS(176),
    [aux_sym_string_literal_token1] = ACTIONS(179),
    [aux_sym_char_literal_token1] = ACTIONS(182),
    [sym_integer_literal] = ACTIONS(185),
    [sym_comment] = ACTIONS(161),
    [sym_raw_string_literal] = ACTIONS(185),
  },
  [15] = {
    [sym__top_level_item] = STATE(12),
    [sym_preproc_include] = STATE(12),
    [sym_define] = STATE(12),
    [sym_preproc_if] = STATE(12),
    [sym_undef] = STATE(12),
    [sym_preproc_nothing] = STATE(12),
    [sym_string_literal] = STATE(12),
    [sym_char_literal] = STATE(12),
    [aux_sym_translation_unit_repeat1] = STATE(12),
    [sym_nothing] = ACTIONS(188),
    [aux_sym_preproc_include_token1] = ACTIONS(108),
    [aux_sym_define_token1] = ACTIONS(110),
    [aux_sym_preproc_if_token1] = ACTIONS(112),
    [aux_sym_preproc_if_token2] = ACTIONS(190),
    [aux_sym_undef_token1] = ACTIONS(116),
    [aux_sym_preproc_nothing_token1] = ACTIONS(118),
    [aux_sym_string_literal_token1] = ACTIONS(120),
    [aux_sym_char_literal_token1] = ACTIONS(122),
    [sym_integer_literal] = ACTIONS(192),
    [sym_comment] = ACTIONS(188),
    [sym_raw_string_literal] = ACTIONS(192),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(196), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(194), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [19] = 2,
    ACTIONS(200), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(198), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [38] = 2,
    ACTIONS(204), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(202), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [57] = 2,
    ACTIONS(208), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(206), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [76] = 2,
    ACTIONS(212), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(210), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [95] = 2,
    ACTIONS(216), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(214), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [114] = 2,
    ACTIONS(220), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(218), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [133] = 2,
    ACTIONS(224), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(222), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [152] = 2,
    ACTIONS(228), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(226), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [171] = 2,
    ACTIONS(232), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(230), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [190] = 2,
    ACTIONS(236), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(234), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [209] = 2,
    ACTIONS(240), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(238), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [228] = 2,
    ACTIONS(244), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(242), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [247] = 2,
    ACTIONS(248), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(246), 12,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_elif_token1,
      aux_sym_preproc_else_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [266] = 2,
    ACTIONS(216), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(214), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [283] = 2,
    ACTIONS(220), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(218), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [300] = 2,
    ACTIONS(228), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(226), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [317] = 2,
    ACTIONS(232), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(230), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [334] = 2,
    ACTIONS(236), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(234), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [351] = 2,
    ACTIONS(216), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(214), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [368] = 2,
    ACTIONS(240), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(238), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [385] = 2,
    ACTIONS(212), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(210), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [402] = 2,
    ACTIONS(208), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(206), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [419] = 2,
    ACTIONS(232), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(230), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [436] = 2,
    ACTIONS(204), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(202), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [453] = 2,
    ACTIONS(224), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(222), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [470] = 2,
    ACTIONS(244), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(242), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [487] = 2,
    ACTIONS(240), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(238), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [504] = 2,
    ACTIONS(196), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(194), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [521] = 2,
    ACTIONS(248), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(246), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [538] = 2,
    ACTIONS(200), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(198), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [555] = 2,
    ACTIONS(220), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(218), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [572] = 2,
    ACTIONS(200), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(198), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [589] = 2,
    ACTIONS(212), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(210), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [606] = 2,
    ACTIONS(224), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(222), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [623] = 2,
    ACTIONS(208), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(206), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [640] = 2,
    ACTIONS(236), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(234), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [657] = 2,
    ACTIONS(204), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(202), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [674] = 2,
    ACTIONS(244), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(242), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [691] = 2,
    ACTIONS(228), 3,
      sym_raw_string_literal,
      ts_builtin_sym_end,
      sym_integer_literal,
    ACTIONS(226), 9,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [708] = 2,
    ACTIONS(196), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(194), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [725] = 2,
    ACTIONS(248), 2,
      sym_raw_string_literal,
      sym_integer_literal,
    ACTIONS(246), 10,
      sym_nothing,
      aux_sym_preproc_include_token1,
      aux_sym_define_token1,
      aux_sym_preproc_if_token1,
      aux_sym_preproc_if_token2,
      aux_sym_undef_token1,
      aux_sym_preproc_nothing_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [742] = 5,
    ACTIONS(250), 1,
      aux_sym_preproc_if_token2,
    ACTIONS(252), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(254), 1,
      aux_sym_preproc_else_token1,
    STATE(116), 1,
      sym_preproc_else,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [759] = 5,
    ACTIONS(252), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(254), 1,
      aux_sym_preproc_else_token1,
    ACTIONS(256), 1,
      aux_sym_preproc_if_token2,
    STATE(101), 1,
      sym_preproc_else,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [776] = 5,
    ACTIONS(252), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(254), 1,
      aux_sym_preproc_else_token1,
    ACTIONS(258), 1,
      aux_sym_preproc_if_token2,
    STATE(92), 1,
      sym_preproc_else,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [793] = 5,
    ACTIONS(252), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(254), 1,
      aux_sym_preproc_else_token1,
    ACTIONS(260), 1,
      aux_sym_preproc_if_token2,
    STATE(90), 1,
      sym_preproc_else,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [810] = 5,
    ACTIONS(252), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(254), 1,
      aux_sym_preproc_else_token1,
    ACTIONS(262), 1,
      aux_sym_preproc_if_token2,
    STATE(105), 1,
      sym_preproc_else,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [827] = 5,
    ACTIONS(252), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(254), 1,
      aux_sym_preproc_else_token1,
    ACTIONS(264), 1,
      aux_sym_preproc_if_token2,
    STATE(113), 1,
      sym_preproc_else,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [844] = 3,
    ACTIONS(268), 1,
      aux_sym_preproc_elif_token1,
    ACTIONS(266), 2,
      aux_sym_preproc_if_token2,
      aux_sym_preproc_else_token1,
    STATE(64), 2,
      sym_preproc_elif,
      aux_sym_preproc_if_repeat1,
  [856] = 4,
    ACTIONS(271), 1,
      sym_identifier,
    ACTIONS(273), 1,
      anon_sym_LT,
    ACTIONS(275), 1,
      aux_sym_string_literal_token1,
    STATE(25), 1,
      sym_string_literal,
  [869] = 4,
    ACTIONS(277), 1,
      sym_identifier,
    ACTIONS(279), 1,
      anon_sym_LT,
    ACTIONS(281), 1,
      aux_sym_string_literal_token1,
    STATE(39), 1,
      sym_string_literal,
  [882] = 4,
    ACTIONS(283), 1,
      sym_identifier,
    ACTIONS(285), 1,
      anon_sym_LT,
    ACTIONS(287), 1,
      aux_sym_string_literal_token1,
    STATE(33), 1,
      sym_string_literal,
  [895] = 3,
    ACTIONS(289), 1,
      sym_preproc_continuation_line,
    ACTIONS(291), 1,
      sym_preproc_line,
    STATE(69), 1,
      aux_sym_define_repeat1,
  [905] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(295), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [915] = 3,
    ACTIONS(297), 1,
      sym_preproc_continuation_line,
    ACTIONS(299), 1,
      sym_preproc_line,
    STATE(86), 1,
      aux_sym_define_repeat1,
  [925] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(301), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [935] = 3,
    ACTIONS(303), 1,
      sym_preproc_continuation_line,
    ACTIONS(305), 1,
      sym_preproc_line,
    STATE(74), 1,
      aux_sym_define_repeat1,
  [945] = 3,
    ACTIONS(307), 1,
      sym_preproc_continuation_line,
    ACTIONS(310), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [955] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(312), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [965] = 3,
    ACTIONS(314), 1,
      sym_preproc_continuation_line,
    ACTIONS(316), 1,
      sym_preproc_line,
    STATE(85), 1,
      aux_sym_define_repeat1,
  [975] = 3,
    ACTIONS(318), 1,
      sym_preproc_continuation_line,
    ACTIONS(320), 1,
      sym_preproc_line,
    STATE(82), 1,
      aux_sym_define_repeat1,
  [985] = 3,
    ACTIONS(322), 1,
      sym_preproc_continuation_line,
    ACTIONS(324), 1,
      sym_preproc_line,
    STATE(80), 1,
      aux_sym_define_repeat1,
  [995] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(326), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [1005] = 3,
    ACTIONS(328), 1,
      sym_preproc_continuation_line,
    ACTIONS(330), 1,
      sym_preproc_line,
    STATE(83), 1,
      aux_sym_define_repeat1,
  [1015] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(332), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [1025] = 3,
    ACTIONS(334), 1,
      sym_preproc_continuation_line,
    ACTIONS(336), 1,
      sym_preproc_line,
    STATE(71), 1,
      aux_sym_define_repeat1,
  [1035] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(338), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [1045] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(340), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [1055] = 3,
    ACTIONS(342), 1,
      sym_preproc_continuation_line,
    ACTIONS(344), 1,
      sym_preproc_line,
    STATE(78), 1,
      aux_sym_define_repeat1,
  [1065] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(346), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [1075] = 3,
    ACTIONS(293), 1,
      sym_preproc_continuation_line,
    ACTIONS(348), 1,
      sym_preproc_line,
    STATE(73), 1,
      aux_sym_define_repeat1,
  [1085] = 1,
    ACTIONS(350), 1,
      anon_sym_LF,
  [1089] = 1,
    ACTIONS(352), 1,
      anon_sym_LF,
  [1093] = 1,
    ACTIONS(354), 1,
      anon_sym_LF,
  [1097] = 1,
    ACTIONS(356), 1,
      aux_sym_preproc_if_token2,
  [1101] = 1,
    ACTIONS(358), 1,
      anon_sym_LF,
  [1105] = 1,
    ACTIONS(260), 1,
      aux_sym_preproc_if_token2,
  [1109] = 1,
    ACTIONS(360), 1,
      sym_identifier,
  [1113] = 1,
    ACTIONS(362), 1,
      anon_sym_LF,
  [1117] = 1,
    ACTIONS(256), 1,
      aux_sym_preproc_if_token2,
  [1121] = 1,
    ACTIONS(364), 1,
      sym_identifier,
  [1125] = 1,
    ACTIONS(366), 1,
      anon_sym_GT,
  [1129] = 1,
    ACTIONS(258), 1,
      aux_sym_preproc_if_token2,
  [1133] = 1,
    ACTIONS(368), 1,
      anon_sym_GT,
  [1137] = 1,
    ACTIONS(370), 1,
      anon_sym_LF,
  [1141] = 1,
    ACTIONS(262), 1,
      aux_sym_preproc_if_token2,
  [1145] = 1,
    ACTIONS(372), 1,
      ts_builtin_sym_end,
  [1149] = 1,
    ACTIONS(374), 1,
      anon_sym_LF,
  [1153] = 1,
    ACTIONS(376), 1,
      anon_sym_LF,
  [1157] = 1,
    ACTIONS(378), 1,
      aux_sym_preproc_if_token2,
  [1161] = 1,
    ACTIONS(380), 1,
      anon_sym_LF,
  [1165] = 1,
    ACTIONS(382), 1,
      sym_path,
  [1169] = 1,
    ACTIONS(384), 1,
      sym_path,
  [1173] = 1,
    ACTIONS(264), 1,
      aux_sym_preproc_if_token2,
  [1177] = 1,
    ACTIONS(386), 1,
      anon_sym_GT,
  [1181] = 1,
    ACTIONS(388), 1,
      anon_sym_LF,
  [1185] = 1,
    ACTIONS(390), 1,
      sym_path,
  [1189] = 1,
    ACTIONS(250), 1,
      aux_sym_preproc_if_token2,
  [1193] = 1,
    ACTIONS(392), 1,
      anon_sym_LF,
  [1197] = 1,
    ACTIONS(394), 1,
      anon_sym_LF,
  [1201] = 1,
    ACTIONS(396), 1,
      aux_sym_preproc_if_token2,
  [1205] = 1,
    ACTIONS(398), 1,
      sym_identifier,
  [1209] = 1,
    ACTIONS(400), 1,
      sym_identifier,
  [1213] = 1,
    ACTIONS(402), 1,
      sym_identifier,
  [1217] = 1,
    ACTIONS(404), 1,
      sym_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(16)] = 0,
  [SMALL_STATE(17)] = 19,
  [SMALL_STATE(18)] = 38,
  [SMALL_STATE(19)] = 57,
  [SMALL_STATE(20)] = 76,
  [SMALL_STATE(21)] = 95,
  [SMALL_STATE(22)] = 114,
  [SMALL_STATE(23)] = 133,
  [SMALL_STATE(24)] = 152,
  [SMALL_STATE(25)] = 171,
  [SMALL_STATE(26)] = 190,
  [SMALL_STATE(27)] = 209,
  [SMALL_STATE(28)] = 228,
  [SMALL_STATE(29)] = 247,
  [SMALL_STATE(30)] = 266,
  [SMALL_STATE(31)] = 283,
  [SMALL_STATE(32)] = 300,
  [SMALL_STATE(33)] = 317,
  [SMALL_STATE(34)] = 334,
  [SMALL_STATE(35)] = 351,
  [SMALL_STATE(36)] = 368,
  [SMALL_STATE(37)] = 385,
  [SMALL_STATE(38)] = 402,
  [SMALL_STATE(39)] = 419,
  [SMALL_STATE(40)] = 436,
  [SMALL_STATE(41)] = 453,
  [SMALL_STATE(42)] = 470,
  [SMALL_STATE(43)] = 487,
  [SMALL_STATE(44)] = 504,
  [SMALL_STATE(45)] = 521,
  [SMALL_STATE(46)] = 538,
  [SMALL_STATE(47)] = 555,
  [SMALL_STATE(48)] = 572,
  [SMALL_STATE(49)] = 589,
  [SMALL_STATE(50)] = 606,
  [SMALL_STATE(51)] = 623,
  [SMALL_STATE(52)] = 640,
  [SMALL_STATE(53)] = 657,
  [SMALL_STATE(54)] = 674,
  [SMALL_STATE(55)] = 691,
  [SMALL_STATE(56)] = 708,
  [SMALL_STATE(57)] = 725,
  [SMALL_STATE(58)] = 742,
  [SMALL_STATE(59)] = 759,
  [SMALL_STATE(60)] = 776,
  [SMALL_STATE(61)] = 793,
  [SMALL_STATE(62)] = 810,
  [SMALL_STATE(63)] = 827,
  [SMALL_STATE(64)] = 844,
  [SMALL_STATE(65)] = 856,
  [SMALL_STATE(66)] = 869,
  [SMALL_STATE(67)] = 882,
  [SMALL_STATE(68)] = 895,
  [SMALL_STATE(69)] = 905,
  [SMALL_STATE(70)] = 915,
  [SMALL_STATE(71)] = 925,
  [SMALL_STATE(72)] = 935,
  [SMALL_STATE(73)] = 945,
  [SMALL_STATE(74)] = 955,
  [SMALL_STATE(75)] = 965,
  [SMALL_STATE(76)] = 975,
  [SMALL_STATE(77)] = 985,
  [SMALL_STATE(78)] = 995,
  [SMALL_STATE(79)] = 1005,
  [SMALL_STATE(80)] = 1015,
  [SMALL_STATE(81)] = 1025,
  [SMALL_STATE(82)] = 1035,
  [SMALL_STATE(83)] = 1045,
  [SMALL_STATE(84)] = 1055,
  [SMALL_STATE(85)] = 1065,
  [SMALL_STATE(86)] = 1075,
  [SMALL_STATE(87)] = 1085,
  [SMALL_STATE(88)] = 1089,
  [SMALL_STATE(89)] = 1093,
  [SMALL_STATE(90)] = 1097,
  [SMALL_STATE(91)] = 1101,
  [SMALL_STATE(92)] = 1105,
  [SMALL_STATE(93)] = 1109,
  [SMALL_STATE(94)] = 1113,
  [SMALL_STATE(95)] = 1117,
  [SMALL_STATE(96)] = 1121,
  [SMALL_STATE(97)] = 1125,
  [SMALL_STATE(98)] = 1129,
  [SMALL_STATE(99)] = 1133,
  [SMALL_STATE(100)] = 1137,
  [SMALL_STATE(101)] = 1141,
  [SMALL_STATE(102)] = 1145,
  [SMALL_STATE(103)] = 1149,
  [SMALL_STATE(104)] = 1153,
  [SMALL_STATE(105)] = 1157,
  [SMALL_STATE(106)] = 1161,
  [SMALL_STATE(107)] = 1165,
  [SMALL_STATE(108)] = 1169,
  [SMALL_STATE(109)] = 1173,
  [SMALL_STATE(110)] = 1177,
  [SMALL_STATE(111)] = 1181,
  [SMALL_STATE(112)] = 1185,
  [SMALL_STATE(113)] = 1189,
  [SMALL_STATE(114)] = 1193,
  [SMALL_STATE(115)] = 1197,
  [SMALL_STATE(116)] = 1201,
  [SMALL_STATE(117)] = 1205,
  [SMALL_STATE(118)] = 1209,
  [SMALL_STATE(119)] = 1213,
  [SMALL_STATE(120)] = 1217,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(96),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(93),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(65),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(117),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(47),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(118),
  [39] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [43] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [47] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [49] = {.entry = {.count = 1, .reusable = false}}, SHIFT(32),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [61] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(8),
  [72] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(65),
  [75] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(117),
  [78] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(6),
  [81] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [83] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(118),
  [86] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(79),
  [89] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(27),
  [92] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(26),
  [95] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(8),
  [98] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_elif, 2),
  [100] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [102] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_elif, 1),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [106] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [108] = {.entry = {.count = 1, .reusable = false}}, SHIFT(67),
  [110] = {.entry = {.count = 1, .reusable = false}}, SHIFT(119),
  [112] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [114] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_else, 1),
  [116] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [118] = {.entry = {.count = 1, .reusable = false}}, SHIFT(84),
  [120] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [122] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [124] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [126] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(12),
  [129] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(67),
  [132] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(119),
  [135] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(3),
  [138] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(120),
  [141] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(84),
  [144] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(36),
  [147] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(34),
  [150] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(12),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 1),
  [155] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [161] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(14),
  [164] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(66),
  [167] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(96),
  [170] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(4),
  [173] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(93),
  [176] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(81),
  [179] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(43),
  [182] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(52),
  [185] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(14),
  [188] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [190] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_else, 2),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [194] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_define, 5),
  [196] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_define, 5),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_undef, 5),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_undef, 5),
  [202] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_if, 4),
  [204] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_if, 4),
  [206] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_define, 4),
  [208] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_define, 4),
  [210] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_include, 4),
  [212] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_include, 4),
  [214] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_nothing, 3),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_nothing, 3),
  [218] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_if, 3),
  [220] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_if, 3),
  [222] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_nothing, 2),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_nothing, 2),
  [226] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_if, 2),
  [228] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_if, 2),
  [230] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_include, 2),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_include, 2),
  [234] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_char_literal, 1),
  [236] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_char_literal, 1),
  [238] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_literal, 1),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_literal, 1),
  [242] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_undef, 4),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_undef, 4),
  [246] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_preproc_if, 5),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_preproc_if, 5),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_preproc_if_repeat1, 2),
  [268] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_preproc_if_repeat1, 2), SHIFT_REPEAT(10),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [275] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [279] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [285] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [287] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [289] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [291] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [293] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [295] = {.entry = {.count = 1, .reusable = false}}, SHIFT(87),
  [297] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [299] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [301] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [303] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [305] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [307] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_define_repeat1, 2), SHIFT_REPEAT(73),
  [310] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_define_repeat1, 2),
  [312] = {.entry = {.count = 1, .reusable = false}}, SHIFT(89),
  [314] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [316] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [320] = {.entry = {.count = 1, .reusable = false}}, SHIFT(103),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [324] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [326] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [330] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [332] = {.entry = {.count = 1, .reusable = false}}, SHIFT(88),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [336] = {.entry = {.count = 1, .reusable = false}}, SHIFT(50),
  [338] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [340] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [344] = {.entry = {.count = 1, .reusable = false}}, SHIFT(41),
  [346] = {.entry = {.count = 1, .reusable = false}}, SHIFT(115),
  [348] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [356] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [358] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [360] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [362] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [372] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [378] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [380] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [398] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [404] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_preproc_external_scanner_create(void);
void tree_sitter_preproc_external_scanner_destroy(void *);
bool tree_sitter_preproc_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_preproc_external_scanner_serialize(void *, char *);
void tree_sitter_preproc_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_preproc(void) {
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
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_preproc_external_scanner_create,
      tree_sitter_preproc_external_scanner_destroy,
      tree_sitter_preproc_external_scanner_scan,
      tree_sitter_preproc_external_scanner_serialize,
      tree_sitter_preproc_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
