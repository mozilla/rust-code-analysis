#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 10
#define STATE_COUNT 84
#define SYMBOL_COUNT 36
#define ALIAS_COUNT 0
#define TOKEN_COUNT 22
#define EXTERNAL_TOKEN_COUNT 1
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5

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

static const char *ts_symbol_names[] = {
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

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  switch (state) {
    case 0:
      if (lookahead == 0) ADVANCE(64);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(79);
      if (lookahead == '>') ADVANCE(80);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(12);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(65);
      END_STATE();
    case 1:
      if (lookahead == 0) ADVANCE(64);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(68);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(70);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(69);
      if (lookahead == 'R') ADVANCE(66);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(67);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      if (lookahead != 0) ADVANCE(72);
      END_STATE();
    case 2:
      if (lookahead == 0) ADVANCE(64);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(79);
      if (lookahead == '>') ADVANCE(80);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(2)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(65);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(85);
      if (lookahead == 'd') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(4);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(85);
      if (lookahead == 'd') ADVANCE(5);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(85);
      if (lookahead == 'e') ADVANCE(6);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 6:
      if (lookahead == '\n') ADVANCE(85);
      if (lookahead == 'f') ADVANCE(7);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 7:
      if (lookahead == '\n') ADVANCE(85);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 8:
      if (lookahead == '\n') ADVANCE(87);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 9:
      if (lookahead == '\n') ADVANCE(88);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 10:
      if (lookahead == '\n') ADVANCE(86);
      if (lookahead != 0) ADVANCE(10);
      END_STATE();
    case 11:
      if (lookahead == '\n') ADVANCE(84);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(11)
      END_STATE();
    case 12:
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(79);
      if (lookahead == '>') ADVANCE(80);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(12);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(65);
      END_STATE();
    case 13:
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(79);
      if (lookahead == '>') ADVANCE(80);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(65);
      END_STATE();
    case 14:
      if (lookahead == '"') ADVANCE(91);
      if (lookahead == '\\') ADVANCE(61);
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 15:
      if (lookahead == '\'') ADVANCE(92);
      if (lookahead == '\\') ADVANCE(62);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 16:
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(96);
      END_STATE();
    case 17:
      if (lookahead == '*') ADVANCE(17);
      if (lookahead == '/') ADVANCE(95);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 18:
      if (lookahead == '*') ADVANCE(17);
      if (lookahead != 0) ADVANCE(18);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(39);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(90);
      END_STATE();
    case 21:
      if (lookahead == 'c') ADVANCE(46);
      END_STATE();
    case 22:
      if (lookahead == 'd') ADVANCE(27);
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'p') ADVANCE(53);
      if (lookahead == 'u') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(22);
      END_STATE();
    case 23:
      if (lookahead == 'd') ADVANCE(27);
      if (lookahead == 'e') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'u') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(23);
      END_STATE();
    case 24:
      if (lookahead == 'd') ADVANCE(43);
      END_STATE();
    case 25:
      if (lookahead == 'd') ADVANCE(29);
      END_STATE();
    case 26:
      if (lookahead == 'd') ADVANCE(32);
      END_STATE();
    case 27:
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(90);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(77);
      END_STATE();
    case 30:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(60);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(35);
      END_STATE();
    case 33:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 34:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(21);
      END_STATE();
    case 35:
      if (lookahead == 'f') ADVANCE(59);
      END_STATE();
    case 36:
      if (lookahead == 'f') ADVANCE(8);
      END_STATE();
    case 37:
      if (lookahead == 'f') ADVANCE(10);
      END_STATE();
    case 38:
      if (lookahead == 'f') ADVANCE(42);
      END_STATE();
    case 39:
      if (lookahead == 'g') ADVANCE(47);
      END_STATE();
    case 40:
      if (lookahead == 'i') ADVANCE(49);
      END_STATE();
    case 41:
      if (lookahead == 'i') ADVANCE(36);
      if (lookahead == 's') ADVANCE(33);
      END_STATE();
    case 42:
      if (lookahead == 'i') ADVANCE(50);
      END_STATE();
    case 43:
      if (lookahead == 'i') ADVANCE(37);
      END_STATE();
    case 44:
      if (lookahead == 'l') ADVANCE(41);
      if (lookahead == 'n') ADVANCE(24);
      END_STATE();
    case 45:
      if (lookahead == 'l') ADVANCE(41);
      if (lookahead == 'n') ADVANCE(24);
      if (lookahead == 'r') ADVANCE(54);
      END_STATE();
    case 46:
      if (lookahead == 'l') ADVANCE(57);
      END_STATE();
    case 47:
      if (lookahead == 'm') ADVANCE(20);
      END_STATE();
    case 48:
      if (lookahead == 'n') ADVANCE(26);
      END_STATE();
    case 49:
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 50:
      if (lookahead == 'n') ADVANCE(31);
      END_STATE();
    case 51:
      if (lookahead == 'n') ADVANCE(30);
      END_STATE();
    case 52:
      if (lookahead == 'o') ADVANCE(55);
      END_STATE();
    case 53:
      if (lookahead == 'r') ADVANCE(19);
      END_STATE();
    case 54:
      if (lookahead == 'r') ADVANCE(52);
      END_STATE();
    case 55:
      if (lookahead == 'r') ADVANCE(90);
      END_STATE();
    case 56:
      if (lookahead == 't') ADVANCE(78);
      END_STATE();
    case 57:
      if (lookahead == 'u') ADVANCE(25);
      END_STATE();
    case 58:
      if (lookahead == 'x') ADVANCE(56);
      END_STATE();
    case 59:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      END_STATE();
    case 60:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(83);
      END_STATE();
    case 61:
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 62:
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 63:
      if (lookahead != 0 &&
          lookahead != '\r') ADVANCE(96);
      if (lookahead == '\r') ADVANCE(97);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(65);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_nothing);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(68);
      if (lookahead == '#') ADVANCE(70);
      if (lookahead == '/') ADVANCE(69);
      if (lookahead == 'R') ADVANCE(66);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(67);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(72);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(68);
      if (lookahead == '\r') ADVANCE(68);
      if (lookahead == '#') ADVANCE(71);
      if (lookahead == '/') ADVANCE(69);
      if (lookahead == 'R') ADVANCE(66);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(68);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(72);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(96);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(27);
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'p') ADVANCE(53);
      if (lookahead == 'u') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(22);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(27);
      if (lookahead == 'e') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'u') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(23);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '#' &&
          lookahead != '\'' &&
          (lookahead < '/' || '9' < lookahead) &&
          lookahead != 'R') ADVANCE(72);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_preproc_continuation_line);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(73);
      if (lookahead == '\r') ADVANCE(75);
      if (lookahead == '\\') ADVANCE(74);
      if (lookahead != 0) ADVANCE(76);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(73);
      if (lookahead == '\\') ADVANCE(74);
      if (lookahead != 0) ADVANCE(76);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\\') ADVANCE(74);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(76);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(aux_sym_preproc_include_token1);
      if (lookahead == '_') ADVANCE(51);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(78);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(aux_sym_preproc_include_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(78);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_path);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(81);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(82);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_path);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(82);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(aux_sym_define_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(83);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(84);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(aux_sym_preproc_if_token1);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(aux_sym_preproc_if_token2);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(aux_sym_preproc_elif_token1);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(aux_sym_preproc_else_token1);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(aux_sym_undef_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(aux_sym_preproc_nothing_token1);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(aux_sym_char_literal_token1);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(sym_integer_literal);
      if (lookahead == '\'') ADVANCE(94);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(93);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(sym_integer_literal);
      if (lookahead == '\'' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(94);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\\') ADVANCE(63);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(96);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(96);
      if (lookahead == '\\') ADVANCE(63);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 1, .external_lex_state = 1},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 1, .external_lex_state = 1},
  [5] = {.lex_state = 1, .external_lex_state = 1},
  [6] = {.lex_state = 76},
  [7] = {.lex_state = 1, .external_lex_state = 1},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 1, .external_lex_state = 1},
  [11] = {.lex_state = 1, .external_lex_state = 1},
  [12] = {.lex_state = 81},
  [13] = {.lex_state = 76},
  [14] = {.lex_state = 1, .external_lex_state = 1},
  [15] = {.lex_state = 1, .external_lex_state = 1},
  [16] = {.lex_state = 1, .external_lex_state = 1},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 1, .external_lex_state = 1},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 1, .external_lex_state = 1},
  [21] = {.lex_state = 76},
  [22] = {.lex_state = 76},
  [23] = {.lex_state = 1, .external_lex_state = 1},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 11},
  [26] = {.lex_state = 76},
  [27] = {.lex_state = 1, .external_lex_state = 1},
  [28] = {.lex_state = 1, .external_lex_state = 1},
  [29] = {.lex_state = 1, .external_lex_state = 1},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 1, .external_lex_state = 1},
  [34] = {.lex_state = 76},
  [35] = {.lex_state = 11},
  [36] = {.lex_state = 76},
  [37] = {.lex_state = 1, .external_lex_state = 1},
  [38] = {.lex_state = 1, .external_lex_state = 1},
  [39] = {.lex_state = 11},
  [40] = {.lex_state = 1, .external_lex_state = 1},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 1, .external_lex_state = 1},
  [43] = {.lex_state = 11},
  [44] = {.lex_state = 1, .external_lex_state = 1},
  [45] = {.lex_state = 1, .external_lex_state = 1},
  [46] = {.lex_state = 1, .external_lex_state = 1},
  [47] = {.lex_state = 1, .external_lex_state = 1},
  [48] = {.lex_state = 1, .external_lex_state = 1},
  [49] = {.lex_state = 1, .external_lex_state = 1},
  [50] = {.lex_state = 1, .external_lex_state = 1},
  [51] = {.lex_state = 1, .external_lex_state = 1},
  [52] = {.lex_state = 1, .external_lex_state = 1},
  [53] = {.lex_state = 1, .external_lex_state = 1},
  [54] = {.lex_state = 1, .external_lex_state = 1},
  [55] = {.lex_state = 1, .external_lex_state = 1},
  [56] = {.lex_state = 1, .external_lex_state = 1},
  [57] = {.lex_state = 1, .external_lex_state = 1},
  [58] = {.lex_state = 1, .external_lex_state = 1},
  [59] = {.lex_state = 1, .external_lex_state = 1},
  [60] = {.lex_state = 1, .external_lex_state = 1},
  [61] = {.lex_state = 1, .external_lex_state = 1},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 1, .external_lex_state = 1},
  [64] = {.lex_state = 76},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 1, .external_lex_state = 1},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 76},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 11},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 11},
  [74] = {.lex_state = 11},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 11},
  [77] = {.lex_state = 81},
  [78] = {.lex_state = 76},
  [79] = {.lex_state = 76},
  [80] = {.lex_state = 76},
  [81] = {.lex_state = 76},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
};

enum {
  ts_external_token_raw_string_literal = 0,
};

static TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_raw_string_literal] = sym_raw_string_literal,
};

static bool ts_external_scanner_states[2][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_raw_string_literal] = true,
  },
};

static uint16_t ts_parse_table[STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_raw_string_literal] = ACTIONS(1),
    [aux_sym_undef_token1] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [aux_sym_char_literal_token1] = ACTIONS(1),
    [aux_sym_preproc_if_token2] = ACTIONS(1),
    [aux_sym_define_token1] = ACTIONS(1),
    [aux_sym_preproc_else_token1] = ACTIONS(1),
    [aux_sym_preproc_include_token1] = ACTIONS(1),
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
    [aux_sym_string_literal_token1] = ACTIONS(1),
    [aux_sym_preproc_if_token1] = ACTIONS(1),
    [aux_sym_preproc_nothing_token1] = ACTIONS(1),
    [aux_sym_preproc_elif_token1] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [sym_integer_literal] = ACTIONS(1),
  },
  [1] = {
    [sym_define] = STATE(10),
    [sym_preproc_if] = STATE(10),
    [sym_string_literal] = STATE(10),
    [sym_char_literal] = STATE(10),
    [sym_translation_unit] = STATE(9),
    [aux_sym_translation_unit_repeat1] = STATE(10),
    [sym__top_level_item] = STATE(10),
    [sym_preproc_include] = STATE(10),
    [sym_undef] = STATE(10),
    [sym_preproc_nothing] = STATE(10),
    [sym_raw_string_literal] = ACTIONS(3),
    [aux_sym_preproc_include_token1] = ACTIONS(5),
    [ts_builtin_sym_end] = ACTIONS(7),
    [aux_sym_undef_token1] = ACTIONS(9),
    [sym_comment] = ACTIONS(11),
    [aux_sym_string_literal_token1] = ACTIONS(13),
    [aux_sym_preproc_if_token1] = ACTIONS(15),
    [aux_sym_preproc_nothing_token1] = ACTIONS(17),
    [sym_nothing] = ACTIONS(11),
    [aux_sym_char_literal_token1] = ACTIONS(19),
    [aux_sym_define_token1] = ACTIONS(21),
    [sym_integer_literal] = ACTIONS(3),
  },
  [2] = {
    [sym_string_literal] = STATE(11),
    [aux_sym_string_literal_token1] = ACTIONS(23),
    [sym_identifier] = ACTIONS(25),
    [anon_sym_LT] = ACTIONS(27),
  },
  [3] = {
    [sym_identifier] = ACTIONS(29),
  },
  [4] = {
    [aux_sym_preproc_include_token1] = ACTIONS(31),
    [sym_raw_string_literal] = ACTIONS(33),
    [aux_sym_undef_token1] = ACTIONS(31),
    [sym_comment] = ACTIONS(31),
    [aux_sym_string_literal_token1] = ACTIONS(31),
    [ts_builtin_sym_end] = ACTIONS(33),
    [aux_sym_preproc_if_token1] = ACTIONS(31),
    [aux_sym_preproc_nothing_token1] = ACTIONS(31),
    [sym_nothing] = ACTIONS(31),
    [aux_sym_char_literal_token1] = ACTIONS(31),
    [aux_sym_define_token1] = ACTIONS(31),
    [sym_integer_literal] = ACTIONS(33),
  },
  [5] = {
    [sym_define] = STATE(18),
    [sym_preproc_if] = STATE(18),
    [sym_string_literal] = STATE(18),
    [sym_char_literal] = STATE(18),
    [sym_preproc_elif] = STATE(19),
    [sym_preproc_else] = STATE(17),
    [aux_sym_translation_unit_repeat1] = STATE(18),
    [sym__top_level_item] = STATE(18),
    [sym_preproc_include] = STATE(18),
    [sym_undef] = STATE(18),
    [sym_preproc_nothing] = STATE(18),
    [aux_sym_preproc_if_repeat1] = STATE(19),
    [sym_raw_string_literal] = ACTIONS(35),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_undef_token1] = ACTIONS(39),
    [sym_comment] = ACTIONS(41),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [aux_sym_preproc_elif_token1] = ACTIONS(49),
    [sym_nothing] = ACTIONS(41),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(53),
    [aux_sym_define_token1] = ACTIONS(55),
    [aux_sym_preproc_else_token1] = ACTIONS(57),
    [sym_integer_literal] = ACTIONS(35),
  },
  [6] = {
    [aux_sym_define_repeat1] = STATE(21),
    [sym_preproc_continuation_line] = ACTIONS(59),
    [sym_preproc_line] = ACTIONS(61),
  },
  [7] = {
    [aux_sym_preproc_include_token1] = ACTIONS(63),
    [sym_raw_string_literal] = ACTIONS(65),
    [aux_sym_undef_token1] = ACTIONS(63),
    [sym_comment] = ACTIONS(63),
    [aux_sym_string_literal_token1] = ACTIONS(63),
    [ts_builtin_sym_end] = ACTIONS(65),
    [aux_sym_preproc_if_token1] = ACTIONS(63),
    [aux_sym_preproc_nothing_token1] = ACTIONS(63),
    [sym_nothing] = ACTIONS(63),
    [aux_sym_char_literal_token1] = ACTIONS(63),
    [aux_sym_define_token1] = ACTIONS(63),
    [sym_integer_literal] = ACTIONS(65),
  },
  [8] = {
    [sym_identifier] = ACTIONS(67),
  },
  [9] = {
    [ts_builtin_sym_end] = ACTIONS(69),
  },
  [10] = {
    [sym_define] = STATE(23),
    [sym_preproc_if] = STATE(23),
    [sym_string_literal] = STATE(23),
    [sym_char_literal] = STATE(23),
    [aux_sym_translation_unit_repeat1] = STATE(23),
    [sym__top_level_item] = STATE(23),
    [sym_preproc_include] = STATE(23),
    [sym_undef] = STATE(23),
    [sym_preproc_nothing] = STATE(23),
    [sym_raw_string_literal] = ACTIONS(71),
    [aux_sym_preproc_include_token1] = ACTIONS(5),
    [ts_builtin_sym_end] = ACTIONS(73),
    [aux_sym_undef_token1] = ACTIONS(9),
    [sym_comment] = ACTIONS(75),
    [aux_sym_string_literal_token1] = ACTIONS(13),
    [aux_sym_preproc_if_token1] = ACTIONS(15),
    [aux_sym_preproc_nothing_token1] = ACTIONS(17),
    [sym_nothing] = ACTIONS(75),
    [aux_sym_char_literal_token1] = ACTIONS(19),
    [aux_sym_define_token1] = ACTIONS(21),
    [sym_integer_literal] = ACTIONS(71),
  },
  [11] = {
    [aux_sym_preproc_include_token1] = ACTIONS(77),
    [sym_raw_string_literal] = ACTIONS(79),
    [aux_sym_undef_token1] = ACTIONS(77),
    [sym_comment] = ACTIONS(77),
    [aux_sym_string_literal_token1] = ACTIONS(77),
    [ts_builtin_sym_end] = ACTIONS(79),
    [aux_sym_preproc_if_token1] = ACTIONS(77),
    [aux_sym_preproc_nothing_token1] = ACTIONS(77),
    [sym_nothing] = ACTIONS(77),
    [aux_sym_char_literal_token1] = ACTIONS(77),
    [aux_sym_define_token1] = ACTIONS(77),
    [sym_integer_literal] = ACTIONS(79),
  },
  [12] = {
    [sym_path] = ACTIONS(81),
  },
  [13] = {
    [aux_sym_define_repeat1] = STATE(26),
    [sym_preproc_continuation_line] = ACTIONS(83),
    [sym_preproc_line] = ACTIONS(85),
  },
  [14] = {
    [sym_define] = STATE(27),
    [sym_preproc_if] = STATE(27),
    [sym_string_literal] = STATE(27),
    [sym_char_literal] = STATE(27),
    [aux_sym_translation_unit_repeat1] = STATE(27),
    [sym__top_level_item] = STATE(27),
    [sym_preproc_include] = STATE(27),
    [sym_undef] = STATE(27),
    [sym_preproc_nothing] = STATE(27),
    [sym_raw_string_literal] = ACTIONS(87),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_undef_token1] = ACTIONS(39),
    [sym_comment] = ACTIONS(89),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [aux_sym_preproc_elif_token1] = ACTIONS(91),
    [sym_nothing] = ACTIONS(89),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(91),
    [aux_sym_define_token1] = ACTIONS(55),
    [aux_sym_preproc_else_token1] = ACTIONS(91),
    [sym_integer_literal] = ACTIONS(87),
  },
  [15] = {
    [aux_sym_preproc_include_token1] = ACTIONS(93),
    [sym_raw_string_literal] = ACTIONS(95),
    [aux_sym_undef_token1] = ACTIONS(93),
    [sym_comment] = ACTIONS(93),
    [aux_sym_string_literal_token1] = ACTIONS(93),
    [ts_builtin_sym_end] = ACTIONS(95),
    [aux_sym_preproc_if_token1] = ACTIONS(93),
    [aux_sym_preproc_nothing_token1] = ACTIONS(93),
    [sym_nothing] = ACTIONS(93),
    [aux_sym_char_literal_token1] = ACTIONS(93),
    [aux_sym_define_token1] = ACTIONS(93),
    [sym_integer_literal] = ACTIONS(95),
  },
  [16] = {
    [sym_define] = STATE(28),
    [sym_preproc_if] = STATE(28),
    [sym_string_literal] = STATE(28),
    [sym_char_literal] = STATE(28),
    [aux_sym_translation_unit_repeat1] = STATE(28),
    [sym__top_level_item] = STATE(28),
    [sym_preproc_include] = STATE(28),
    [sym_undef] = STATE(28),
    [sym_preproc_nothing] = STATE(28),
    [sym_raw_string_literal] = ACTIONS(97),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_undef_token1] = ACTIONS(39),
    [sym_comment] = ACTIONS(99),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [sym_nothing] = ACTIONS(99),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(101),
    [aux_sym_define_token1] = ACTIONS(55),
    [sym_integer_literal] = ACTIONS(97),
  },
  [17] = {
    [aux_sym_preproc_if_token2] = ACTIONS(103),
  },
  [18] = {
    [sym_define] = STATE(52),
    [sym_preproc_if] = STATE(52),
    [sym_string_literal] = STATE(52),
    [sym_char_literal] = STATE(52),
    [sym_preproc_elif] = STATE(31),
    [sym_preproc_else] = STATE(30),
    [aux_sym_translation_unit_repeat1] = STATE(52),
    [sym__top_level_item] = STATE(52),
    [sym_preproc_include] = STATE(52),
    [sym_undef] = STATE(52),
    [sym_preproc_nothing] = STATE(52),
    [aux_sym_preproc_if_repeat1] = STATE(31),
    [sym_raw_string_literal] = ACTIONS(105),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_undef_token1] = ACTIONS(39),
    [sym_comment] = ACTIONS(107),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [aux_sym_preproc_elif_token1] = ACTIONS(49),
    [sym_nothing] = ACTIONS(107),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(109),
    [aux_sym_define_token1] = ACTIONS(55),
    [aux_sym_preproc_else_token1] = ACTIONS(57),
    [sym_integer_literal] = ACTIONS(105),
  },
  [19] = {
    [sym_preproc_else] = STATE(30),
    [sym_preproc_elif] = STATE(32),
    [aux_sym_preproc_if_repeat1] = STATE(32),
    [aux_sym_preproc_elif_token1] = ACTIONS(111),
    [aux_sym_preproc_else_token1] = ACTIONS(113),
    [aux_sym_preproc_if_token2] = ACTIONS(103),
  },
  [20] = {
    [aux_sym_preproc_include_token1] = ACTIONS(115),
    [sym_raw_string_literal] = ACTIONS(117),
    [aux_sym_undef_token1] = ACTIONS(115),
    [sym_comment] = ACTIONS(115),
    [aux_sym_string_literal_token1] = ACTIONS(115),
    [ts_builtin_sym_end] = ACTIONS(117),
    [aux_sym_preproc_if_token1] = ACTIONS(115),
    [aux_sym_preproc_nothing_token1] = ACTIONS(115),
    [sym_nothing] = ACTIONS(115),
    [aux_sym_char_literal_token1] = ACTIONS(115),
    [aux_sym_define_token1] = ACTIONS(115),
    [sym_integer_literal] = ACTIONS(117),
  },
  [21] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(119),
    [sym_preproc_line] = ACTIONS(121),
  },
  [22] = {
    [aux_sym_define_repeat1] = STATE(36),
    [sym_preproc_continuation_line] = ACTIONS(123),
    [sym_preproc_line] = ACTIONS(125),
  },
  [23] = {
    [sym_define] = STATE(23),
    [sym_preproc_if] = STATE(23),
    [sym_string_literal] = STATE(23),
    [sym_char_literal] = STATE(23),
    [aux_sym_translation_unit_repeat1] = STATE(23),
    [sym__top_level_item] = STATE(23),
    [sym_preproc_include] = STATE(23),
    [sym_undef] = STATE(23),
    [sym_preproc_nothing] = STATE(23),
    [aux_sym_preproc_include_token1] = ACTIONS(127),
    [sym_raw_string_literal] = ACTIONS(130),
    [aux_sym_undef_token1] = ACTIONS(133),
    [sym_comment] = ACTIONS(136),
    [aux_sym_string_literal_token1] = ACTIONS(139),
    [ts_builtin_sym_end] = ACTIONS(142),
    [aux_sym_preproc_if_token1] = ACTIONS(144),
    [aux_sym_preproc_nothing_token1] = ACTIONS(147),
    [sym_nothing] = ACTIONS(136),
    [aux_sym_char_literal_token1] = ACTIONS(150),
    [aux_sym_define_token1] = ACTIONS(153),
    [sym_integer_literal] = ACTIONS(130),
  },
  [24] = {
    [anon_sym_GT] = ACTIONS(156),
  },
  [25] = {
    [anon_sym_LF] = ACTIONS(158),
  },
  [26] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(119),
    [sym_preproc_line] = ACTIONS(160),
  },
  [27] = {
    [sym_define] = STATE(52),
    [sym_preproc_if] = STATE(52),
    [sym_string_literal] = STATE(52),
    [sym_char_literal] = STATE(52),
    [aux_sym_translation_unit_repeat1] = STATE(52),
    [sym__top_level_item] = STATE(52),
    [sym_preproc_include] = STATE(52),
    [sym_undef] = STATE(52),
    [sym_preproc_nothing] = STATE(52),
    [sym_raw_string_literal] = ACTIONS(105),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_undef_token1] = ACTIONS(39),
    [sym_comment] = ACTIONS(107),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [aux_sym_preproc_elif_token1] = ACTIONS(162),
    [sym_nothing] = ACTIONS(107),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(162),
    [aux_sym_define_token1] = ACTIONS(55),
    [aux_sym_preproc_else_token1] = ACTIONS(162),
    [sym_integer_literal] = ACTIONS(105),
  },
  [28] = {
    [sym_define] = STATE(52),
    [sym_preproc_if] = STATE(52),
    [sym_string_literal] = STATE(52),
    [sym_char_literal] = STATE(52),
    [aux_sym_translation_unit_repeat1] = STATE(52),
    [sym__top_level_item] = STATE(52),
    [sym_preproc_include] = STATE(52),
    [sym_undef] = STATE(52),
    [sym_preproc_nothing] = STATE(52),
    [sym_raw_string_literal] = ACTIONS(105),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_undef_token1] = ACTIONS(39),
    [sym_comment] = ACTIONS(107),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [sym_nothing] = ACTIONS(107),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(164),
    [aux_sym_define_token1] = ACTIONS(55),
    [sym_integer_literal] = ACTIONS(105),
  },
  [29] = {
    [aux_sym_preproc_include_token1] = ACTIONS(166),
    [sym_raw_string_literal] = ACTIONS(168),
    [aux_sym_undef_token1] = ACTIONS(166),
    [sym_comment] = ACTIONS(166),
    [aux_sym_string_literal_token1] = ACTIONS(166),
    [ts_builtin_sym_end] = ACTIONS(168),
    [aux_sym_preproc_if_token1] = ACTIONS(166),
    [aux_sym_preproc_nothing_token1] = ACTIONS(166),
    [sym_nothing] = ACTIONS(166),
    [aux_sym_char_literal_token1] = ACTIONS(166),
    [aux_sym_define_token1] = ACTIONS(166),
    [sym_integer_literal] = ACTIONS(168),
  },
  [30] = {
    [aux_sym_preproc_if_token2] = ACTIONS(170),
  },
  [31] = {
    [sym_preproc_else] = STATE(41),
    [sym_preproc_elif] = STATE(32),
    [aux_sym_preproc_if_repeat1] = STATE(32),
    [aux_sym_preproc_elif_token1] = ACTIONS(111),
    [aux_sym_preproc_else_token1] = ACTIONS(113),
    [aux_sym_preproc_if_token2] = ACTIONS(170),
  },
  [32] = {
    [sym_preproc_elif] = STATE(32),
    [aux_sym_preproc_if_repeat1] = STATE(32),
    [aux_sym_preproc_elif_token1] = ACTIONS(172),
    [aux_sym_preproc_else_token1] = ACTIONS(175),
    [aux_sym_preproc_if_token2] = ACTIONS(175),
  },
  [33] = {
    [aux_sym_preproc_include_token1] = ACTIONS(177),
    [sym_raw_string_literal] = ACTIONS(179),
    [aux_sym_undef_token1] = ACTIONS(177),
    [sym_comment] = ACTIONS(177),
    [aux_sym_string_literal_token1] = ACTIONS(177),
    [ts_builtin_sym_end] = ACTIONS(179),
    [aux_sym_preproc_if_token1] = ACTIONS(177),
    [aux_sym_preproc_nothing_token1] = ACTIONS(177),
    [sym_nothing] = ACTIONS(177),
    [aux_sym_char_literal_token1] = ACTIONS(177),
    [aux_sym_define_token1] = ACTIONS(177),
    [sym_integer_literal] = ACTIONS(179),
  },
  [34] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(181),
    [sym_preproc_line] = ACTIONS(184),
  },
  [35] = {
    [anon_sym_LF] = ACTIONS(186),
  },
  [36] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(119),
    [sym_preproc_line] = ACTIONS(188),
  },
  [37] = {
    [aux_sym_preproc_include_token1] = ACTIONS(190),
    [sym_raw_string_literal] = ACTIONS(192),
    [aux_sym_undef_token1] = ACTIONS(190),
    [sym_comment] = ACTIONS(190),
    [aux_sym_string_literal_token1] = ACTIONS(190),
    [ts_builtin_sym_end] = ACTIONS(192),
    [aux_sym_preproc_if_token1] = ACTIONS(190),
    [aux_sym_preproc_nothing_token1] = ACTIONS(190),
    [sym_nothing] = ACTIONS(190),
    [aux_sym_char_literal_token1] = ACTIONS(190),
    [aux_sym_define_token1] = ACTIONS(190),
    [sym_integer_literal] = ACTIONS(192),
  },
  [38] = {
    [aux_sym_preproc_include_token1] = ACTIONS(194),
    [sym_raw_string_literal] = ACTIONS(196),
    [aux_sym_undef_token1] = ACTIONS(194),
    [sym_comment] = ACTIONS(194),
    [aux_sym_string_literal_token1] = ACTIONS(194),
    [ts_builtin_sym_end] = ACTIONS(196),
    [aux_sym_preproc_if_token1] = ACTIONS(194),
    [aux_sym_preproc_nothing_token1] = ACTIONS(194),
    [sym_nothing] = ACTIONS(194),
    [aux_sym_char_literal_token1] = ACTIONS(194),
    [aux_sym_define_token1] = ACTIONS(194),
    [sym_integer_literal] = ACTIONS(196),
  },
  [39] = {
    [anon_sym_LF] = ACTIONS(198),
  },
  [40] = {
    [aux_sym_preproc_include_token1] = ACTIONS(200),
    [sym_raw_string_literal] = ACTIONS(202),
    [aux_sym_undef_token1] = ACTIONS(200),
    [sym_comment] = ACTIONS(200),
    [aux_sym_string_literal_token1] = ACTIONS(200),
    [ts_builtin_sym_end] = ACTIONS(202),
    [aux_sym_preproc_if_token1] = ACTIONS(200),
    [aux_sym_preproc_nothing_token1] = ACTIONS(200),
    [sym_nothing] = ACTIONS(200),
    [aux_sym_char_literal_token1] = ACTIONS(200),
    [aux_sym_define_token1] = ACTIONS(200),
    [sym_integer_literal] = ACTIONS(202),
  },
  [41] = {
    [aux_sym_preproc_if_token2] = ACTIONS(204),
  },
  [42] = {
    [aux_sym_preproc_include_token1] = ACTIONS(206),
    [sym_raw_string_literal] = ACTIONS(208),
    [aux_sym_undef_token1] = ACTIONS(206),
    [sym_comment] = ACTIONS(206),
    [aux_sym_string_literal_token1] = ACTIONS(206),
    [ts_builtin_sym_end] = ACTIONS(208),
    [aux_sym_preproc_if_token1] = ACTIONS(206),
    [aux_sym_preproc_nothing_token1] = ACTIONS(206),
    [sym_nothing] = ACTIONS(206),
    [aux_sym_char_literal_token1] = ACTIONS(206),
    [aux_sym_define_token1] = ACTIONS(206),
    [sym_integer_literal] = ACTIONS(208),
  },
  [43] = {
    [anon_sym_LF] = ACTIONS(210),
  },
  [44] = {
    [aux_sym_preproc_include_token1] = ACTIONS(212),
    [sym_raw_string_literal] = ACTIONS(214),
    [aux_sym_undef_token1] = ACTIONS(212),
    [sym_comment] = ACTIONS(212),
    [aux_sym_string_literal_token1] = ACTIONS(212),
    [ts_builtin_sym_end] = ACTIONS(214),
    [aux_sym_preproc_if_token1] = ACTIONS(212),
    [aux_sym_preproc_nothing_token1] = ACTIONS(212),
    [sym_nothing] = ACTIONS(212),
    [aux_sym_char_literal_token1] = ACTIONS(212),
    [aux_sym_define_token1] = ACTIONS(212),
    [sym_integer_literal] = ACTIONS(214),
  },
  [45] = {
    [aux_sym_preproc_include_token1] = ACTIONS(216),
    [sym_raw_string_literal] = ACTIONS(218),
    [aux_sym_undef_token1] = ACTIONS(216),
    [sym_comment] = ACTIONS(216),
    [aux_sym_string_literal_token1] = ACTIONS(216),
    [ts_builtin_sym_end] = ACTIONS(218),
    [aux_sym_preproc_if_token1] = ACTIONS(216),
    [aux_sym_preproc_nothing_token1] = ACTIONS(216),
    [sym_nothing] = ACTIONS(216),
    [aux_sym_char_literal_token1] = ACTIONS(216),
    [aux_sym_define_token1] = ACTIONS(216),
    [sym_integer_literal] = ACTIONS(218),
  },
  [46] = {
    [aux_sym_preproc_include_token1] = ACTIONS(220),
    [sym_raw_string_literal] = ACTIONS(222),
    [aux_sym_undef_token1] = ACTIONS(220),
    [sym_comment] = ACTIONS(220),
    [aux_sym_string_literal_token1] = ACTIONS(220),
    [ts_builtin_sym_end] = ACTIONS(222),
    [aux_sym_preproc_if_token1] = ACTIONS(220),
    [aux_sym_preproc_nothing_token1] = ACTIONS(220),
    [sym_nothing] = ACTIONS(220),
    [aux_sym_char_literal_token1] = ACTIONS(220),
    [aux_sym_define_token1] = ACTIONS(220),
    [sym_integer_literal] = ACTIONS(222),
  },
  [47] = {
    [sym_raw_string_literal] = ACTIONS(33),
    [aux_sym_undef_token1] = ACTIONS(31),
    [aux_sym_char_literal_token1] = ACTIONS(31),
    [aux_sym_preproc_if_token2] = ACTIONS(31),
    [aux_sym_define_token1] = ACTIONS(31),
    [aux_sym_preproc_else_token1] = ACTIONS(31),
    [aux_sym_preproc_include_token1] = ACTIONS(31),
    [sym_comment] = ACTIONS(31),
    [aux_sym_string_literal_token1] = ACTIONS(31),
    [aux_sym_preproc_if_token1] = ACTIONS(31),
    [aux_sym_preproc_nothing_token1] = ACTIONS(31),
    [aux_sym_preproc_elif_token1] = ACTIONS(31),
    [sym_nothing] = ACTIONS(31),
    [sym_integer_literal] = ACTIONS(33),
  },
  [48] = {
    [sym_raw_string_literal] = ACTIONS(65),
    [aux_sym_undef_token1] = ACTIONS(63),
    [aux_sym_char_literal_token1] = ACTIONS(63),
    [aux_sym_preproc_if_token2] = ACTIONS(63),
    [aux_sym_define_token1] = ACTIONS(63),
    [aux_sym_preproc_else_token1] = ACTIONS(63),
    [aux_sym_preproc_include_token1] = ACTIONS(63),
    [sym_comment] = ACTIONS(63),
    [aux_sym_string_literal_token1] = ACTIONS(63),
    [aux_sym_preproc_if_token1] = ACTIONS(63),
    [aux_sym_preproc_nothing_token1] = ACTIONS(63),
    [aux_sym_preproc_elif_token1] = ACTIONS(63),
    [sym_nothing] = ACTIONS(63),
    [sym_integer_literal] = ACTIONS(65),
  },
  [49] = {
    [sym_raw_string_literal] = ACTIONS(79),
    [aux_sym_undef_token1] = ACTIONS(77),
    [aux_sym_char_literal_token1] = ACTIONS(77),
    [aux_sym_preproc_if_token2] = ACTIONS(77),
    [aux_sym_define_token1] = ACTIONS(77),
    [aux_sym_preproc_else_token1] = ACTIONS(77),
    [aux_sym_preproc_include_token1] = ACTIONS(77),
    [sym_comment] = ACTIONS(77),
    [aux_sym_string_literal_token1] = ACTIONS(77),
    [aux_sym_preproc_if_token1] = ACTIONS(77),
    [aux_sym_preproc_nothing_token1] = ACTIONS(77),
    [aux_sym_preproc_elif_token1] = ACTIONS(77),
    [sym_nothing] = ACTIONS(77),
    [sym_integer_literal] = ACTIONS(79),
  },
  [50] = {
    [sym_raw_string_literal] = ACTIONS(95),
    [aux_sym_undef_token1] = ACTIONS(93),
    [aux_sym_char_literal_token1] = ACTIONS(93),
    [aux_sym_preproc_if_token2] = ACTIONS(93),
    [aux_sym_define_token1] = ACTIONS(93),
    [aux_sym_preproc_else_token1] = ACTIONS(93),
    [aux_sym_preproc_include_token1] = ACTIONS(93),
    [sym_comment] = ACTIONS(93),
    [aux_sym_string_literal_token1] = ACTIONS(93),
    [aux_sym_preproc_if_token1] = ACTIONS(93),
    [aux_sym_preproc_nothing_token1] = ACTIONS(93),
    [aux_sym_preproc_elif_token1] = ACTIONS(93),
    [sym_nothing] = ACTIONS(93),
    [sym_integer_literal] = ACTIONS(95),
  },
  [51] = {
    [sym_raw_string_literal] = ACTIONS(117),
    [aux_sym_undef_token1] = ACTIONS(115),
    [aux_sym_char_literal_token1] = ACTIONS(115),
    [aux_sym_preproc_if_token2] = ACTIONS(115),
    [aux_sym_define_token1] = ACTIONS(115),
    [aux_sym_preproc_else_token1] = ACTIONS(115),
    [aux_sym_preproc_include_token1] = ACTIONS(115),
    [sym_comment] = ACTIONS(115),
    [aux_sym_string_literal_token1] = ACTIONS(115),
    [aux_sym_preproc_if_token1] = ACTIONS(115),
    [aux_sym_preproc_nothing_token1] = ACTIONS(115),
    [aux_sym_preproc_elif_token1] = ACTIONS(115),
    [sym_nothing] = ACTIONS(115),
    [sym_integer_literal] = ACTIONS(117),
  },
  [52] = {
    [sym_define] = STATE(52),
    [sym_preproc_if] = STATE(52),
    [sym_string_literal] = STATE(52),
    [sym_char_literal] = STATE(52),
    [aux_sym_translation_unit_repeat1] = STATE(52),
    [sym__top_level_item] = STATE(52),
    [sym_preproc_include] = STATE(52),
    [sym_undef] = STATE(52),
    [sym_preproc_nothing] = STATE(52),
    [sym_raw_string_literal] = ACTIONS(224),
    [aux_sym_undef_token1] = ACTIONS(227),
    [aux_sym_char_literal_token1] = ACTIONS(230),
    [aux_sym_preproc_if_token2] = ACTIONS(233),
    [aux_sym_define_token1] = ACTIONS(235),
    [aux_sym_preproc_else_token1] = ACTIONS(233),
    [aux_sym_preproc_include_token1] = ACTIONS(238),
    [sym_comment] = ACTIONS(241),
    [aux_sym_string_literal_token1] = ACTIONS(244),
    [aux_sym_preproc_if_token1] = ACTIONS(247),
    [aux_sym_preproc_nothing_token1] = ACTIONS(250),
    [aux_sym_preproc_elif_token1] = ACTIONS(233),
    [sym_nothing] = ACTIONS(241),
    [sym_integer_literal] = ACTIONS(224),
  },
  [53] = {
    [sym_raw_string_literal] = ACTIONS(168),
    [aux_sym_undef_token1] = ACTIONS(166),
    [aux_sym_char_literal_token1] = ACTIONS(166),
    [aux_sym_preproc_if_token2] = ACTIONS(166),
    [aux_sym_define_token1] = ACTIONS(166),
    [aux_sym_preproc_else_token1] = ACTIONS(166),
    [aux_sym_preproc_include_token1] = ACTIONS(166),
    [sym_comment] = ACTIONS(166),
    [aux_sym_string_literal_token1] = ACTIONS(166),
    [aux_sym_preproc_if_token1] = ACTIONS(166),
    [aux_sym_preproc_nothing_token1] = ACTIONS(166),
    [aux_sym_preproc_elif_token1] = ACTIONS(166),
    [sym_nothing] = ACTIONS(166),
    [sym_integer_literal] = ACTIONS(168),
  },
  [54] = {
    [sym_raw_string_literal] = ACTIONS(179),
    [aux_sym_undef_token1] = ACTIONS(177),
    [aux_sym_char_literal_token1] = ACTIONS(177),
    [aux_sym_preproc_if_token2] = ACTIONS(177),
    [aux_sym_define_token1] = ACTIONS(177),
    [aux_sym_preproc_else_token1] = ACTIONS(177),
    [aux_sym_preproc_include_token1] = ACTIONS(177),
    [sym_comment] = ACTIONS(177),
    [aux_sym_string_literal_token1] = ACTIONS(177),
    [aux_sym_preproc_if_token1] = ACTIONS(177),
    [aux_sym_preproc_nothing_token1] = ACTIONS(177),
    [aux_sym_preproc_elif_token1] = ACTIONS(177),
    [sym_nothing] = ACTIONS(177),
    [sym_integer_literal] = ACTIONS(179),
  },
  [55] = {
    [sym_raw_string_literal] = ACTIONS(192),
    [aux_sym_undef_token1] = ACTIONS(190),
    [aux_sym_char_literal_token1] = ACTIONS(190),
    [aux_sym_preproc_if_token2] = ACTIONS(190),
    [aux_sym_define_token1] = ACTIONS(190),
    [aux_sym_preproc_else_token1] = ACTIONS(190),
    [aux_sym_preproc_include_token1] = ACTIONS(190),
    [sym_comment] = ACTIONS(190),
    [aux_sym_string_literal_token1] = ACTIONS(190),
    [aux_sym_preproc_if_token1] = ACTIONS(190),
    [aux_sym_preproc_nothing_token1] = ACTIONS(190),
    [aux_sym_preproc_elif_token1] = ACTIONS(190),
    [sym_nothing] = ACTIONS(190),
    [sym_integer_literal] = ACTIONS(192),
  },
  [56] = {
    [sym_raw_string_literal] = ACTIONS(196),
    [aux_sym_undef_token1] = ACTIONS(194),
    [aux_sym_char_literal_token1] = ACTIONS(194),
    [aux_sym_preproc_if_token2] = ACTIONS(194),
    [aux_sym_define_token1] = ACTIONS(194),
    [aux_sym_preproc_else_token1] = ACTIONS(194),
    [aux_sym_preproc_include_token1] = ACTIONS(194),
    [sym_comment] = ACTIONS(194),
    [aux_sym_string_literal_token1] = ACTIONS(194),
    [aux_sym_preproc_if_token1] = ACTIONS(194),
    [aux_sym_preproc_nothing_token1] = ACTIONS(194),
    [aux_sym_preproc_elif_token1] = ACTIONS(194),
    [sym_nothing] = ACTIONS(194),
    [sym_integer_literal] = ACTIONS(196),
  },
  [57] = {
    [sym_raw_string_literal] = ACTIONS(202),
    [aux_sym_undef_token1] = ACTIONS(200),
    [aux_sym_char_literal_token1] = ACTIONS(200),
    [aux_sym_preproc_if_token2] = ACTIONS(200),
    [aux_sym_define_token1] = ACTIONS(200),
    [aux_sym_preproc_else_token1] = ACTIONS(200),
    [aux_sym_preproc_include_token1] = ACTIONS(200),
    [sym_comment] = ACTIONS(200),
    [aux_sym_string_literal_token1] = ACTIONS(200),
    [aux_sym_preproc_if_token1] = ACTIONS(200),
    [aux_sym_preproc_nothing_token1] = ACTIONS(200),
    [aux_sym_preproc_elif_token1] = ACTIONS(200),
    [sym_nothing] = ACTIONS(200),
    [sym_integer_literal] = ACTIONS(202),
  },
  [58] = {
    [sym_raw_string_literal] = ACTIONS(208),
    [aux_sym_undef_token1] = ACTIONS(206),
    [aux_sym_char_literal_token1] = ACTIONS(206),
    [aux_sym_preproc_if_token2] = ACTIONS(206),
    [aux_sym_define_token1] = ACTIONS(206),
    [aux_sym_preproc_else_token1] = ACTIONS(206),
    [aux_sym_preproc_include_token1] = ACTIONS(206),
    [sym_comment] = ACTIONS(206),
    [aux_sym_string_literal_token1] = ACTIONS(206),
    [aux_sym_preproc_if_token1] = ACTIONS(206),
    [aux_sym_preproc_nothing_token1] = ACTIONS(206),
    [aux_sym_preproc_elif_token1] = ACTIONS(206),
    [sym_nothing] = ACTIONS(206),
    [sym_integer_literal] = ACTIONS(208),
  },
  [59] = {
    [sym_raw_string_literal] = ACTIONS(214),
    [aux_sym_undef_token1] = ACTIONS(212),
    [aux_sym_char_literal_token1] = ACTIONS(212),
    [aux_sym_preproc_if_token2] = ACTIONS(212),
    [aux_sym_define_token1] = ACTIONS(212),
    [aux_sym_preproc_else_token1] = ACTIONS(212),
    [aux_sym_preproc_include_token1] = ACTIONS(212),
    [sym_comment] = ACTIONS(212),
    [aux_sym_string_literal_token1] = ACTIONS(212),
    [aux_sym_preproc_if_token1] = ACTIONS(212),
    [aux_sym_preproc_nothing_token1] = ACTIONS(212),
    [aux_sym_preproc_elif_token1] = ACTIONS(212),
    [sym_nothing] = ACTIONS(212),
    [sym_integer_literal] = ACTIONS(214),
  },
  [60] = {
    [sym_raw_string_literal] = ACTIONS(218),
    [aux_sym_undef_token1] = ACTIONS(216),
    [aux_sym_char_literal_token1] = ACTIONS(216),
    [aux_sym_preproc_if_token2] = ACTIONS(216),
    [aux_sym_define_token1] = ACTIONS(216),
    [aux_sym_preproc_else_token1] = ACTIONS(216),
    [aux_sym_preproc_include_token1] = ACTIONS(216),
    [sym_comment] = ACTIONS(216),
    [aux_sym_string_literal_token1] = ACTIONS(216),
    [aux_sym_preproc_if_token1] = ACTIONS(216),
    [aux_sym_preproc_nothing_token1] = ACTIONS(216),
    [aux_sym_preproc_elif_token1] = ACTIONS(216),
    [sym_nothing] = ACTIONS(216),
    [sym_integer_literal] = ACTIONS(218),
  },
  [61] = {
    [sym_raw_string_literal] = ACTIONS(222),
    [aux_sym_undef_token1] = ACTIONS(220),
    [aux_sym_char_literal_token1] = ACTIONS(220),
    [aux_sym_preproc_if_token2] = ACTIONS(220),
    [aux_sym_define_token1] = ACTIONS(220),
    [aux_sym_preproc_else_token1] = ACTIONS(220),
    [aux_sym_preproc_include_token1] = ACTIONS(220),
    [sym_comment] = ACTIONS(220),
    [aux_sym_string_literal_token1] = ACTIONS(220),
    [aux_sym_preproc_if_token1] = ACTIONS(220),
    [aux_sym_preproc_nothing_token1] = ACTIONS(220),
    [aux_sym_preproc_elif_token1] = ACTIONS(220),
    [sym_nothing] = ACTIONS(220),
    [sym_integer_literal] = ACTIONS(222),
  },
  [62] = {
    [sym_string_literal] = STATE(49),
    [aux_sym_string_literal_token1] = ACTIONS(253),
    [sym_identifier] = ACTIONS(255),
    [anon_sym_LT] = ACTIONS(257),
  },
  [63] = {
    [sym_define] = STATE(66),
    [sym_preproc_if] = STATE(66),
    [sym_string_literal] = STATE(66),
    [sym_char_literal] = STATE(66),
    [sym__top_level_item] = STATE(66),
    [sym_preproc_include] = STATE(66),
    [sym_undef] = STATE(66),
    [sym_preproc_nothing] = STATE(66),
    [aux_sym_preproc_if_repeat1] = STATE(67),
    [sym_preproc_elif] = STATE(67),
    [sym_preproc_else] = STATE(65),
    [aux_sym_translation_unit_repeat1] = STATE(66),
    [sym_raw_string_literal] = ACTIONS(259),
    [aux_sym_undef_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(261),
    [aux_sym_define_token1] = ACTIONS(55),
    [aux_sym_preproc_else_token1] = ACTIONS(57),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [sym_comment] = ACTIONS(263),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [aux_sym_preproc_elif_token1] = ACTIONS(49),
    [sym_nothing] = ACTIONS(263),
    [sym_integer_literal] = ACTIONS(259),
  },
  [64] = {
    [aux_sym_define_repeat1] = STATE(68),
    [sym_preproc_continuation_line] = ACTIONS(265),
    [sym_preproc_line] = ACTIONS(267),
  },
  [65] = {
    [aux_sym_preproc_if_token2] = ACTIONS(269),
  },
  [66] = {
    [sym_define] = STATE(52),
    [sym_preproc_if] = STATE(52),
    [sym_string_literal] = STATE(52),
    [sym_char_literal] = STATE(52),
    [sym__top_level_item] = STATE(52),
    [sym_preproc_include] = STATE(52),
    [sym_undef] = STATE(52),
    [sym_preproc_nothing] = STATE(52),
    [aux_sym_preproc_if_repeat1] = STATE(72),
    [sym_preproc_elif] = STATE(72),
    [sym_preproc_else] = STATE(71),
    [aux_sym_translation_unit_repeat1] = STATE(52),
    [sym_raw_string_literal] = ACTIONS(105),
    [aux_sym_undef_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(51),
    [aux_sym_preproc_if_token2] = ACTIONS(271),
    [aux_sym_define_token1] = ACTIONS(55),
    [aux_sym_preproc_else_token1] = ACTIONS(57),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [sym_comment] = ACTIONS(107),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [aux_sym_preproc_if_token1] = ACTIONS(45),
    [aux_sym_preproc_nothing_token1] = ACTIONS(47),
    [aux_sym_preproc_elif_token1] = ACTIONS(49),
    [sym_nothing] = ACTIONS(107),
    [sym_integer_literal] = ACTIONS(105),
  },
  [67] = {
    [sym_preproc_else] = STATE(71),
    [aux_sym_preproc_if_repeat1] = STATE(32),
    [sym_preproc_elif] = STATE(32),
    [aux_sym_preproc_elif_token1] = ACTIONS(111),
    [aux_sym_preproc_if_token2] = ACTIONS(269),
    [aux_sym_preproc_else_token1] = ACTIONS(113),
  },
  [68] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(119),
    [sym_preproc_line] = ACTIONS(273),
  },
  [69] = {
    [anon_sym_GT] = ACTIONS(275),
  },
  [70] = {
    [anon_sym_LF] = ACTIONS(277),
  },
  [71] = {
    [aux_sym_preproc_if_token2] = ACTIONS(279),
  },
  [72] = {
    [sym_preproc_else] = STATE(75),
    [aux_sym_preproc_if_repeat1] = STATE(32),
    [sym_preproc_elif] = STATE(32),
    [aux_sym_preproc_elif_token1] = ACTIONS(111),
    [aux_sym_preproc_if_token2] = ACTIONS(279),
    [aux_sym_preproc_else_token1] = ACTIONS(113),
  },
  [73] = {
    [anon_sym_LF] = ACTIONS(281),
  },
  [74] = {
    [anon_sym_LF] = ACTIONS(283),
  },
  [75] = {
    [aux_sym_preproc_if_token2] = ACTIONS(285),
  },
  [76] = {
    [anon_sym_LF] = ACTIONS(287),
  },
  [77] = {
    [sym_path] = ACTIONS(289),
  },
  [78] = {
    [aux_sym_define_repeat1] = STATE(80),
    [sym_preproc_continuation_line] = ACTIONS(291),
    [sym_preproc_line] = ACTIONS(293),
  },
  [79] = {
    [aux_sym_define_repeat1] = STATE(81),
    [sym_preproc_continuation_line] = ACTIONS(295),
    [sym_preproc_line] = ACTIONS(297),
  },
  [80] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(119),
    [sym_preproc_line] = ACTIONS(299),
  },
  [81] = {
    [aux_sym_define_repeat1] = STATE(34),
    [sym_preproc_continuation_line] = ACTIONS(119),
    [sym_preproc_line] = ACTIONS(301),
  },
  [82] = {
    [sym_identifier] = ACTIONS(303),
  },
  [83] = {
    [sym_identifier] = ACTIONS(305),
  },
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, SHIFT(10),
  [5] = {.count = 1, .reusable = false}, SHIFT(2),
  [7] = {.count = 1, .reusable = true}, REDUCE(sym_translation_unit, 0),
  [9] = {.count = 1, .reusable = false}, SHIFT(3),
  [11] = {.count = 1, .reusable = false}, SHIFT(10),
  [13] = {.count = 1, .reusable = false}, SHIFT(4),
  [15] = {.count = 1, .reusable = false}, SHIFT(5),
  [17] = {.count = 1, .reusable = false}, SHIFT(6),
  [19] = {.count = 1, .reusable = false}, SHIFT(7),
  [21] = {.count = 1, .reusable = false}, SHIFT(8),
  [23] = {.count = 1, .reusable = true}, SHIFT(4),
  [25] = {.count = 1, .reusable = true}, SHIFT(11),
  [27] = {.count = 1, .reusable = true}, SHIFT(12),
  [29] = {.count = 1, .reusable = true}, SHIFT(13),
  [31] = {.count = 1, .reusable = false}, REDUCE(sym_string_literal, 1),
  [33] = {.count = 1, .reusable = true}, REDUCE(sym_string_literal, 1),
  [35] = {.count = 1, .reusable = true}, SHIFT(18),
  [37] = {.count = 1, .reusable = false}, SHIFT(62),
  [39] = {.count = 1, .reusable = false}, SHIFT(82),
  [41] = {.count = 1, .reusable = false}, SHIFT(18),
  [43] = {.count = 1, .reusable = false}, SHIFT(47),
  [45] = {.count = 1, .reusable = false}, SHIFT(63),
  [47] = {.count = 1, .reusable = false}, SHIFT(64),
  [49] = {.count = 1, .reusable = false}, SHIFT(14),
  [51] = {.count = 1, .reusable = false}, SHIFT(48),
  [53] = {.count = 1, .reusable = false}, SHIFT(15),
  [55] = {.count = 1, .reusable = false}, SHIFT(83),
  [57] = {.count = 1, .reusable = false}, SHIFT(16),
  [59] = {.count = 1, .reusable = true}, SHIFT(21),
  [61] = {.count = 1, .reusable = false}, SHIFT(20),
  [63] = {.count = 1, .reusable = false}, REDUCE(sym_char_literal, 1),
  [65] = {.count = 1, .reusable = true}, REDUCE(sym_char_literal, 1),
  [67] = {.count = 1, .reusable = true}, SHIFT(22),
  [69] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [71] = {.count = 1, .reusable = true}, SHIFT(23),
  [73] = {.count = 1, .reusable = true}, REDUCE(sym_translation_unit, 1),
  [75] = {.count = 1, .reusable = false}, SHIFT(23),
  [77] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_include, 2),
  [79] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_include, 2),
  [81] = {.count = 1, .reusable = true}, SHIFT(24),
  [83] = {.count = 1, .reusable = true}, SHIFT(26),
  [85] = {.count = 1, .reusable = false}, SHIFT(25),
  [87] = {.count = 1, .reusable = true}, SHIFT(27),
  [89] = {.count = 1, .reusable = false}, SHIFT(27),
  [91] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_elif, 1),
  [93] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 2),
  [95] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 2),
  [97] = {.count = 1, .reusable = true}, SHIFT(28),
  [99] = {.count = 1, .reusable = false}, SHIFT(28),
  [101] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_else, 1),
  [103] = {.count = 1, .reusable = true}, SHIFT(29),
  [105] = {.count = 1, .reusable = true}, SHIFT(52),
  [107] = {.count = 1, .reusable = false}, SHIFT(52),
  [109] = {.count = 1, .reusable = false}, SHIFT(29),
  [111] = {.count = 1, .reusable = true}, SHIFT(14),
  [113] = {.count = 1, .reusable = true}, SHIFT(16),
  [115] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_nothing, 2),
  [117] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_nothing, 2),
  [119] = {.count = 1, .reusable = true}, SHIFT(34),
  [121] = {.count = 1, .reusable = false}, SHIFT(33),
  [123] = {.count = 1, .reusable = true}, SHIFT(36),
  [125] = {.count = 1, .reusable = false}, SHIFT(35),
  [127] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(2),
  [130] = {.count = 2, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(23),
  [133] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(3),
  [136] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(23),
  [139] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(4),
  [142] = {.count = 1, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [144] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(5),
  [147] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(6),
  [150] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(7),
  [153] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(8),
  [156] = {.count = 1, .reusable = true}, SHIFT(37),
  [158] = {.count = 1, .reusable = true}, SHIFT(38),
  [160] = {.count = 1, .reusable = false}, SHIFT(39),
  [162] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_elif, 2),
  [164] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_else, 2),
  [166] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 3),
  [168] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 3),
  [170] = {.count = 1, .reusable = true}, SHIFT(40),
  [172] = {.count = 2, .reusable = true}, REDUCE(aux_sym_preproc_if_repeat1, 2), SHIFT_REPEAT(14),
  [175] = {.count = 1, .reusable = true}, REDUCE(aux_sym_preproc_if_repeat1, 2),
  [177] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_nothing, 3),
  [179] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_nothing, 3),
  [181] = {.count = 2, .reusable = true}, REDUCE(aux_sym_define_repeat1, 2), SHIFT_REPEAT(34),
  [184] = {.count = 1, .reusable = false}, REDUCE(aux_sym_define_repeat1, 2),
  [186] = {.count = 1, .reusable = true}, SHIFT(42),
  [188] = {.count = 1, .reusable = false}, SHIFT(43),
  [190] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_include, 4),
  [192] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_include, 4),
  [194] = {.count = 1, .reusable = false}, REDUCE(sym_undef, 4),
  [196] = {.count = 1, .reusable = true}, REDUCE(sym_undef, 4),
  [198] = {.count = 1, .reusable = true}, SHIFT(44),
  [200] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 4),
  [202] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 4),
  [204] = {.count = 1, .reusable = true}, SHIFT(45),
  [206] = {.count = 1, .reusable = false}, REDUCE(sym_define, 4),
  [208] = {.count = 1, .reusable = true}, REDUCE(sym_define, 4),
  [210] = {.count = 1, .reusable = true}, SHIFT(46),
  [212] = {.count = 1, .reusable = false}, REDUCE(sym_undef, 5),
  [214] = {.count = 1, .reusable = true}, REDUCE(sym_undef, 5),
  [216] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 5),
  [218] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 5),
  [220] = {.count = 1, .reusable = false}, REDUCE(sym_define, 5),
  [222] = {.count = 1, .reusable = true}, REDUCE(sym_define, 5),
  [224] = {.count = 2, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(52),
  [227] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(82),
  [230] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(48),
  [233] = {.count = 1, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [235] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(83),
  [238] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(62),
  [241] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(52),
  [244] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(47),
  [247] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(63),
  [250] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(64),
  [253] = {.count = 1, .reusable = true}, SHIFT(47),
  [255] = {.count = 1, .reusable = true}, SHIFT(49),
  [257] = {.count = 1, .reusable = true}, SHIFT(77),
  [259] = {.count = 1, .reusable = true}, SHIFT(66),
  [261] = {.count = 1, .reusable = false}, SHIFT(50),
  [263] = {.count = 1, .reusable = false}, SHIFT(66),
  [265] = {.count = 1, .reusable = true}, SHIFT(68),
  [267] = {.count = 1, .reusable = false}, SHIFT(51),
  [269] = {.count = 1, .reusable = true}, SHIFT(53),
  [271] = {.count = 1, .reusable = false}, SHIFT(53),
  [273] = {.count = 1, .reusable = false}, SHIFT(54),
  [275] = {.count = 1, .reusable = true}, SHIFT(55),
  [277] = {.count = 1, .reusable = true}, SHIFT(56),
  [279] = {.count = 1, .reusable = true}, SHIFT(57),
  [281] = {.count = 1, .reusable = true}, SHIFT(58),
  [283] = {.count = 1, .reusable = true}, SHIFT(59),
  [285] = {.count = 1, .reusable = true}, SHIFT(60),
  [287] = {.count = 1, .reusable = true}, SHIFT(61),
  [289] = {.count = 1, .reusable = true}, SHIFT(69),
  [291] = {.count = 1, .reusable = true}, SHIFT(80),
  [293] = {.count = 1, .reusable = false}, SHIFT(70),
  [295] = {.count = 1, .reusable = true}, SHIFT(81),
  [297] = {.count = 1, .reusable = false}, SHIFT(73),
  [299] = {.count = 1, .reusable = false}, SHIFT(74),
  [301] = {.count = 1, .reusable = false}, SHIFT(76),
  [303] = {.count = 1, .reusable = true}, SHIFT(78),
  [305] = {.count = 1, .reusable = true}, SHIFT(79),
};

void *tree_sitter_preproc_external_scanner_create(void);
void tree_sitter_preproc_external_scanner_destroy(void *);
bool tree_sitter_preproc_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_preproc_external_scanner_serialize(void *, char *);
void tree_sitter_preproc_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_preproc(void) {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const unsigned short *)ts_parse_table,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .symbol_names = ts_symbol_names,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .external_scanner = {
      (const bool *)ts_external_scanner_states,
      ts_external_scanner_symbol_map,
      tree_sitter_preproc_external_scanner_create,
      tree_sitter_preproc_external_scanner_destroy,
      tree_sitter_preproc_external_scanner_scan,
      tree_sitter_preproc_external_scanner_serialize,
      tree_sitter_preproc_external_scanner_deserialize,
    },
  };
  return &language;
}
