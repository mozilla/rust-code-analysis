#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 10
#define STATE_COUNT 70
#define SYMBOL_COUNT 34
#define ALIAS_COUNT 0
#define TOKEN_COUNT 21
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
  aux_sym_preproc_nothing_token1 = 15,
  aux_sym_string_literal_token1 = 16,
  aux_sym_char_literal_token1 = 17,
  sym_integer_literal = 18,
  sym_comment = 19,
  sym_raw_string_literal = 20,
  sym_translation_unit = 21,
  sym__top_level_item = 22,
  sym_preproc_include = 23,
  sym_define = 24,
  sym_preproc_if = 25,
  sym_preproc_elif = 26,
  sym_preproc_else = 27,
  sym_preproc_nothing = 28,
  sym_string_literal = 29,
  sym_char_literal = 30,
  aux_sym_translation_unit_repeat1 = 31,
  aux_sym_define_repeat1 = 32,
  aux_sym_preproc_if_repeat1 = 33,
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
      if (lookahead == 0) ADVANCE(63);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(78);
      if (lookahead == '>') ADVANCE(79);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(91);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(64);
      END_STATE();
    case 1:
      if (lookahead == 0) ADVANCE(63);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(67);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(70);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(68);
      if (lookahead == 'R') ADVANCE(65);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(66);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(91);
      if (lookahead != 0) ADVANCE(71);
      END_STATE();
    case 2:
      if (lookahead == 0) ADVANCE(63);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(78);
      if (lookahead == '>') ADVANCE(79);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(12);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(2)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(64);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(84);
      if (lookahead == 'd') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(4);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(84);
      if (lookahead == 'd') ADVANCE(5);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(84);
      if (lookahead == 'e') ADVANCE(6);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 6:
      if (lookahead == '\n') ADVANCE(84);
      if (lookahead == 'f') ADVANCE(7);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 7:
      if (lookahead == '\n') ADVANCE(84);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 8:
      if (lookahead == '\n') ADVANCE(83);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      END_STATE();
    case 9:
      if (lookahead == '\n') ADVANCE(86);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 10:
      if (lookahead == '\n') ADVANCE(87);
      if (lookahead != 0) ADVANCE(10);
      END_STATE();
    case 11:
      if (lookahead == '\n') ADVANCE(85);
      if (lookahead != 0) ADVANCE(11);
      END_STATE();
    case 12:
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(78);
      if (lookahead == '>') ADVANCE(79);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(12);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(12)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(64);
      END_STATE();
    case 13:
      if (lookahead == '"') ADVANCE(14);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '<') ADVANCE(78);
      if (lookahead == '>') ADVANCE(79);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(12)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(64);
      END_STATE();
    case 14:
      if (lookahead == '"') ADVANCE(89);
      if (lookahead == '\\') ADVANCE(60);
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 15:
      if (lookahead == '\'') ADVANCE(90);
      if (lookahead == '\\') ADVANCE(61);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 16:
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(94);
      END_STATE();
    case 17:
      if (lookahead == '*') ADVANCE(17);
      if (lookahead == '/') ADVANCE(93);
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
      if (lookahead == 'a') ADVANCE(88);
      END_STATE();
    case 21:
      if (lookahead == 'c') ADVANCE(44);
      END_STATE();
    case 22:
      if (lookahead == 'd') ADVANCE(28);
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(22);
      END_STATE();
    case 23:
      if (lookahead == 'd') ADVANCE(28);
      if (lookahead == 'e') ADVANCE(46);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'p') ADVANCE(55);
      if (lookahead == 'u') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(23);
      END_STATE();
    case 24:
      if (lookahead == 'd') ADVANCE(43);
      END_STATE();
    case 25:
      if (lookahead == 'd') ADVANCE(32);
      END_STATE();
    case 26:
      if (lookahead == 'd') ADVANCE(27);
      END_STATE();
    case 27:
      if (lookahead == 'e') ADVANCE(76);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(59);
      END_STATE();
    case 30:
      if (lookahead == 'e') ADVANCE(88);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(35);
      END_STATE();
    case 33:
      if (lookahead == 'e') ADVANCE(10);
      END_STATE();
    case 34:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(21);
      END_STATE();
    case 35:
      if (lookahead == 'f') ADVANCE(88);
      END_STATE();
    case 36:
      if (lookahead == 'f') ADVANCE(9);
      END_STATE();
    case 37:
      if (lookahead == 'f') ADVANCE(11);
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
      if (lookahead == 'l') ADVANCE(57);
      END_STATE();
    case 45:
      if (lookahead == 'l') ADVANCE(41);
      if (lookahead == 'n') ADVANCE(24);
      END_STATE();
    case 46:
      if (lookahead == 'l') ADVANCE(41);
      if (lookahead == 'n') ADVANCE(24);
      if (lookahead == 'r') ADVANCE(53);
      END_STATE();
    case 47:
      if (lookahead == 'm') ADVANCE(20);
      END_STATE();
    case 48:
      if (lookahead == 'n') ADVANCE(25);
      END_STATE();
    case 49:
      if (lookahead == 'n') ADVANCE(30);
      END_STATE();
    case 50:
      if (lookahead == 'n') ADVANCE(29);
      END_STATE();
    case 51:
      if (lookahead == 'n') ADVANCE(31);
      END_STATE();
    case 52:
      if (lookahead == 'o') ADVANCE(54);
      END_STATE();
    case 53:
      if (lookahead == 'r') ADVANCE(52);
      END_STATE();
    case 54:
      if (lookahead == 'r') ADVANCE(88);
      END_STATE();
    case 55:
      if (lookahead == 'r') ADVANCE(19);
      END_STATE();
    case 56:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 57:
      if (lookahead == 'u') ADVANCE(26);
      END_STATE();
    case 58:
      if (lookahead == 'x') ADVANCE(56);
      END_STATE();
    case 59:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(82);
      END_STATE();
    case 60:
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 61:
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 62:
      if (lookahead != 0 &&
          lookahead != '\r') ADVANCE(94);
      if (lookahead == '\r') ADVANCE(95);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(64);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_nothing);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(67);
      if (lookahead == '#') ADVANCE(70);
      if (lookahead == '/') ADVANCE(68);
      if (lookahead == 'R') ADVANCE(65);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(66);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(71);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '\n') ADVANCE(67);
      if (lookahead == '\r') ADVANCE(67);
      if (lookahead == '#') ADVANCE(69);
      if (lookahead == '/') ADVANCE(68);
      if (lookahead == 'R') ADVANCE(65);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(67);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '0' || '9' < lookahead)) ADVANCE(71);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(94);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(28);
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(22);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(28);
      if (lookahead == 'e') ADVANCE(46);
      if (lookahead == 'i') ADVANCE(34);
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'p') ADVANCE(55);
      if (lookahead == 'u') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(23);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '#' &&
          lookahead != '\'' &&
          (lookahead < '/' || '9' < lookahead) &&
          lookahead != 'R') ADVANCE(71);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_preproc_continuation_line);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(72);
      if (lookahead == '\r') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(73);
      if (lookahead != 0) ADVANCE(75);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(72);
      if (lookahead == '\\') ADVANCE(73);
      if (lookahead != 0) ADVANCE(75);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\\') ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(75);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(aux_sym_preproc_include_token1);
      if (lookahead == '_') ADVANCE(51);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(aux_sym_preproc_include_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_path);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(80);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(81);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_path);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(81);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(aux_sym_define_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(82);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(83);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(aux_sym_preproc_if_token1);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(aux_sym_preproc_if_token2);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(aux_sym_preproc_elif_token1);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(aux_sym_preproc_else_token1);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(aux_sym_preproc_nothing_token1);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(aux_sym_char_literal_token1);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_integer_literal);
      if (lookahead == '\'') ADVANCE(92);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(91);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_integer_literal);
      if (lookahead == '\'' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\\') ADVANCE(62);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(94);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(94);
      if (lookahead == '\\') ADVANCE(62);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 1, .external_lex_state = 1},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 75},
  [4] = {.lex_state = 1, .external_lex_state = 1},
  [5] = {.lex_state = 1, .external_lex_state = 1},
  [6] = {.lex_state = 1, .external_lex_state = 1},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 1, .external_lex_state = 1},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 1, .external_lex_state = 1},
  [11] = {.lex_state = 80},
  [12] = {.lex_state = 1, .external_lex_state = 1},
  [13] = {.lex_state = 75},
  [14] = {.lex_state = 1, .external_lex_state = 1},
  [15] = {.lex_state = 1, .external_lex_state = 1},
  [16] = {.lex_state = 1, .external_lex_state = 1},
  [17] = {.lex_state = 1, .external_lex_state = 1},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 75},
  [21] = {.lex_state = 1, .external_lex_state = 1},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 1, .external_lex_state = 1},
  [24] = {.lex_state = 75},
  [25] = {.lex_state = 1, .external_lex_state = 1},
  [26] = {.lex_state = 1, .external_lex_state = 1},
  [27] = {.lex_state = 1, .external_lex_state = 1},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 8},
  [32] = {.lex_state = 75},
  [33] = {.lex_state = 1, .external_lex_state = 1},
  [34] = {.lex_state = 1, .external_lex_state = 1},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 1, .external_lex_state = 1},
  [37] = {.lex_state = 8},
  [38] = {.lex_state = 1, .external_lex_state = 1},
  [39] = {.lex_state = 1, .external_lex_state = 1},
  [40] = {.lex_state = 1, .external_lex_state = 1},
  [41] = {.lex_state = 1, .external_lex_state = 1},
  [42] = {.lex_state = 1, .external_lex_state = 1},
  [43] = {.lex_state = 1, .external_lex_state = 1},
  [44] = {.lex_state = 1, .external_lex_state = 1},
  [45] = {.lex_state = 1, .external_lex_state = 1},
  [46] = {.lex_state = 1, .external_lex_state = 1},
  [47] = {.lex_state = 1, .external_lex_state = 1},
  [48] = {.lex_state = 1, .external_lex_state = 1},
  [49] = {.lex_state = 1, .external_lex_state = 1},
  [50] = {.lex_state = 1, .external_lex_state = 1},
  [51] = {.lex_state = 1, .external_lex_state = 1},
  [52] = {.lex_state = 1, .external_lex_state = 1},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 75},
  [55] = {.lex_state = 1, .external_lex_state = 1},
  [56] = {.lex_state = 75},
  [57] = {.lex_state = 1, .external_lex_state = 1},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 8},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 8},
  [66] = {.lex_state = 80},
  [67] = {.lex_state = 75},
  [68] = {.lex_state = 75},
  [69] = {.lex_state = 0},
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
    [aux_sym_preproc_nothing_token1] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [sym_integer_literal] = ACTIONS(1),
    [aux_sym_preproc_if_token2] = ACTIONS(1),
    [aux_sym_define_token1] = ACTIONS(1),
    [aux_sym_preproc_else_token1] = ACTIONS(1),
    [aux_sym_preproc_include_token1] = ACTIONS(1),
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_char_literal_token1] = ACTIONS(1),
    [aux_sym_preproc_if_token1] = ACTIONS(1),
    [aux_sym_string_literal_token1] = ACTIONS(1),
    [aux_sym_preproc_elif_token1] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
  },
  [1] = {
    [sym_define] = STATE(8),
    [sym_preproc_if] = STATE(8),
    [sym_char_literal] = STATE(8),
    [aux_sym_translation_unit_repeat1] = STATE(8),
    [sym_translation_unit] = STATE(9),
    [sym__top_level_item] = STATE(8),
    [sym_preproc_include] = STATE(8),
    [sym_preproc_nothing] = STATE(8),
    [sym_string_literal] = STATE(8),
    [sym_raw_string_literal] = ACTIONS(3),
    [aux_sym_preproc_include_token1] = ACTIONS(5),
    [ts_builtin_sym_end] = ACTIONS(7),
    [aux_sym_preproc_nothing_token1] = ACTIONS(9),
    [aux_sym_char_literal_token1] = ACTIONS(11),
    [aux_sym_preproc_if_token1] = ACTIONS(13),
    [aux_sym_string_literal_token1] = ACTIONS(15),
    [sym_nothing] = ACTIONS(17),
    [sym_integer_literal] = ACTIONS(3),
    [aux_sym_define_token1] = ACTIONS(19),
    [sym_comment] = ACTIONS(17),
  },
  [2] = {
    [sym_string_literal] = STATE(10),
    [sym_identifier] = ACTIONS(21),
    [anon_sym_LT] = ACTIONS(23),
    [aux_sym_string_literal_token1] = ACTIONS(25),
  },
  [3] = {
    [aux_sym_define_repeat1] = STATE(13),
    [sym_preproc_continuation_line] = ACTIONS(27),
    [sym_preproc_line] = ACTIONS(29),
  },
  [4] = {
    [aux_sym_preproc_include_token1] = ACTIONS(31),
    [sym_raw_string_literal] = ACTIONS(33),
    [aux_sym_preproc_nothing_token1] = ACTIONS(31),
    [ts_builtin_sym_end] = ACTIONS(33),
    [aux_sym_char_literal_token1] = ACTIONS(31),
    [aux_sym_preproc_if_token1] = ACTIONS(31),
    [aux_sym_string_literal_token1] = ACTIONS(31),
    [sym_nothing] = ACTIONS(31),
    [sym_integer_literal] = ACTIONS(33),
    [aux_sym_define_token1] = ACTIONS(31),
    [sym_comment] = ACTIONS(31),
  },
  [5] = {
    [sym_define] = STATE(17),
    [sym_preproc_if] = STATE(17),
    [sym_char_literal] = STATE(17),
    [aux_sym_translation_unit_repeat1] = STATE(17),
    [sym_preproc_elif] = STATE(19),
    [sym_preproc_else] = STATE(18),
    [aux_sym_preproc_if_repeat1] = STATE(19),
    [sym__top_level_item] = STATE(17),
    [sym_preproc_include] = STATE(17),
    [sym_preproc_nothing] = STATE(17),
    [sym_string_literal] = STATE(17),
    [sym_raw_string_literal] = ACTIONS(35),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [aux_sym_preproc_elif_token1] = ACTIONS(47),
    [sym_nothing] = ACTIONS(49),
    [sym_integer_literal] = ACTIONS(35),
    [aux_sym_preproc_if_token2] = ACTIONS(51),
    [aux_sym_define_token1] = ACTIONS(53),
    [aux_sym_preproc_else_token1] = ACTIONS(55),
    [sym_comment] = ACTIONS(49),
  },
  [6] = {
    [aux_sym_preproc_include_token1] = ACTIONS(57),
    [sym_raw_string_literal] = ACTIONS(59),
    [aux_sym_preproc_nothing_token1] = ACTIONS(57),
    [ts_builtin_sym_end] = ACTIONS(59),
    [aux_sym_char_literal_token1] = ACTIONS(57),
    [aux_sym_preproc_if_token1] = ACTIONS(57),
    [aux_sym_string_literal_token1] = ACTIONS(57),
    [sym_nothing] = ACTIONS(57),
    [sym_integer_literal] = ACTIONS(59),
    [aux_sym_define_token1] = ACTIONS(57),
    [sym_comment] = ACTIONS(57),
  },
  [7] = {
    [sym_identifier] = ACTIONS(61),
  },
  [8] = {
    [sym_define] = STATE(21),
    [sym_preproc_if] = STATE(21),
    [sym_char_literal] = STATE(21),
    [aux_sym_translation_unit_repeat1] = STATE(21),
    [sym__top_level_item] = STATE(21),
    [sym_preproc_include] = STATE(21),
    [sym_preproc_nothing] = STATE(21),
    [sym_string_literal] = STATE(21),
    [sym_raw_string_literal] = ACTIONS(63),
    [aux_sym_preproc_include_token1] = ACTIONS(5),
    [ts_builtin_sym_end] = ACTIONS(65),
    [aux_sym_preproc_nothing_token1] = ACTIONS(9),
    [aux_sym_char_literal_token1] = ACTIONS(11),
    [aux_sym_preproc_if_token1] = ACTIONS(13),
    [aux_sym_string_literal_token1] = ACTIONS(15),
    [sym_nothing] = ACTIONS(67),
    [sym_integer_literal] = ACTIONS(63),
    [aux_sym_define_token1] = ACTIONS(19),
    [sym_comment] = ACTIONS(67),
  },
  [9] = {
    [ts_builtin_sym_end] = ACTIONS(69),
  },
  [10] = {
    [aux_sym_preproc_include_token1] = ACTIONS(71),
    [sym_raw_string_literal] = ACTIONS(73),
    [aux_sym_preproc_nothing_token1] = ACTIONS(71),
    [ts_builtin_sym_end] = ACTIONS(73),
    [aux_sym_char_literal_token1] = ACTIONS(71),
    [aux_sym_preproc_if_token1] = ACTIONS(71),
    [aux_sym_string_literal_token1] = ACTIONS(71),
    [sym_nothing] = ACTIONS(71),
    [sym_integer_literal] = ACTIONS(73),
    [aux_sym_define_token1] = ACTIONS(71),
    [sym_comment] = ACTIONS(71),
  },
  [11] = {
    [sym_path] = ACTIONS(75),
  },
  [12] = {
    [aux_sym_preproc_include_token1] = ACTIONS(77),
    [sym_raw_string_literal] = ACTIONS(79),
    [aux_sym_preproc_nothing_token1] = ACTIONS(77),
    [ts_builtin_sym_end] = ACTIONS(79),
    [aux_sym_char_literal_token1] = ACTIONS(77),
    [aux_sym_preproc_if_token1] = ACTIONS(77),
    [aux_sym_string_literal_token1] = ACTIONS(77),
    [sym_nothing] = ACTIONS(77),
    [sym_integer_literal] = ACTIONS(79),
    [aux_sym_define_token1] = ACTIONS(77),
    [sym_comment] = ACTIONS(77),
  },
  [13] = {
    [aux_sym_define_repeat1] = STATE(24),
    [sym_preproc_continuation_line] = ACTIONS(81),
    [sym_preproc_line] = ACTIONS(83),
  },
  [14] = {
    [sym_define] = STATE(25),
    [sym_preproc_if] = STATE(25),
    [sym_char_literal] = STATE(25),
    [aux_sym_translation_unit_repeat1] = STATE(25),
    [sym__top_level_item] = STATE(25),
    [sym_preproc_include] = STATE(25),
    [sym_preproc_nothing] = STATE(25),
    [sym_string_literal] = STATE(25),
    [sym_raw_string_literal] = ACTIONS(85),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [aux_sym_preproc_elif_token1] = ACTIONS(87),
    [sym_nothing] = ACTIONS(89),
    [sym_integer_literal] = ACTIONS(85),
    [aux_sym_preproc_if_token2] = ACTIONS(87),
    [aux_sym_define_token1] = ACTIONS(53),
    [aux_sym_preproc_else_token1] = ACTIONS(87),
    [sym_comment] = ACTIONS(89),
  },
  [15] = {
    [aux_sym_preproc_include_token1] = ACTIONS(91),
    [sym_raw_string_literal] = ACTIONS(93),
    [aux_sym_preproc_nothing_token1] = ACTIONS(91),
    [ts_builtin_sym_end] = ACTIONS(93),
    [aux_sym_char_literal_token1] = ACTIONS(91),
    [aux_sym_preproc_if_token1] = ACTIONS(91),
    [aux_sym_string_literal_token1] = ACTIONS(91),
    [sym_nothing] = ACTIONS(91),
    [sym_integer_literal] = ACTIONS(93),
    [aux_sym_define_token1] = ACTIONS(91),
    [sym_comment] = ACTIONS(91),
  },
  [16] = {
    [sym_define] = STATE(26),
    [sym_preproc_if] = STATE(26),
    [sym_char_literal] = STATE(26),
    [aux_sym_translation_unit_repeat1] = STATE(26),
    [sym__top_level_item] = STATE(26),
    [sym_preproc_include] = STATE(26),
    [sym_preproc_nothing] = STATE(26),
    [sym_string_literal] = STATE(26),
    [sym_raw_string_literal] = ACTIONS(95),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [sym_nothing] = ACTIONS(97),
    [sym_integer_literal] = ACTIONS(95),
    [aux_sym_preproc_if_token2] = ACTIONS(99),
    [aux_sym_define_token1] = ACTIONS(53),
    [sym_comment] = ACTIONS(97),
  },
  [17] = {
    [sym_define] = STATE(45),
    [sym_preproc_if] = STATE(45),
    [sym_char_literal] = STATE(45),
    [aux_sym_translation_unit_repeat1] = STATE(45),
    [sym_preproc_elif] = STATE(29),
    [sym_preproc_else] = STATE(28),
    [aux_sym_preproc_if_repeat1] = STATE(29),
    [sym__top_level_item] = STATE(45),
    [sym_preproc_include] = STATE(45),
    [sym_preproc_nothing] = STATE(45),
    [sym_string_literal] = STATE(45),
    [sym_raw_string_literal] = ACTIONS(101),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [aux_sym_preproc_elif_token1] = ACTIONS(47),
    [sym_nothing] = ACTIONS(103),
    [sym_integer_literal] = ACTIONS(101),
    [aux_sym_preproc_if_token2] = ACTIONS(105),
    [aux_sym_define_token1] = ACTIONS(53),
    [aux_sym_preproc_else_token1] = ACTIONS(55),
    [sym_comment] = ACTIONS(103),
  },
  [18] = {
    [aux_sym_preproc_if_token2] = ACTIONS(107),
  },
  [19] = {
    [sym_preproc_else] = STATE(28),
    [aux_sym_preproc_if_repeat1] = STATE(30),
    [sym_preproc_elif] = STATE(30),
    [aux_sym_preproc_elif_token1] = ACTIONS(109),
    [aux_sym_preproc_else_token1] = ACTIONS(111),
    [aux_sym_preproc_if_token2] = ACTIONS(107),
  },
  [20] = {
    [aux_sym_define_repeat1] = STATE(32),
    [sym_preproc_continuation_line] = ACTIONS(113),
    [sym_preproc_line] = ACTIONS(115),
  },
  [21] = {
    [sym_define] = STATE(21),
    [sym_preproc_if] = STATE(21),
    [sym_char_literal] = STATE(21),
    [aux_sym_translation_unit_repeat1] = STATE(21),
    [sym__top_level_item] = STATE(21),
    [sym_preproc_include] = STATE(21),
    [sym_preproc_nothing] = STATE(21),
    [sym_string_literal] = STATE(21),
    [aux_sym_preproc_include_token1] = ACTIONS(117),
    [sym_raw_string_literal] = ACTIONS(120),
    [aux_sym_preproc_nothing_token1] = ACTIONS(123),
    [ts_builtin_sym_end] = ACTIONS(126),
    [aux_sym_char_literal_token1] = ACTIONS(128),
    [aux_sym_preproc_if_token1] = ACTIONS(131),
    [aux_sym_string_literal_token1] = ACTIONS(134),
    [sym_nothing] = ACTIONS(137),
    [sym_integer_literal] = ACTIONS(120),
    [aux_sym_define_token1] = ACTIONS(140),
    [sym_comment] = ACTIONS(137),
  },
  [22] = {
    [anon_sym_GT] = ACTIONS(143),
  },
  [23] = {
    [aux_sym_preproc_include_token1] = ACTIONS(145),
    [sym_raw_string_literal] = ACTIONS(147),
    [aux_sym_preproc_nothing_token1] = ACTIONS(145),
    [ts_builtin_sym_end] = ACTIONS(147),
    [aux_sym_char_literal_token1] = ACTIONS(145),
    [aux_sym_preproc_if_token1] = ACTIONS(145),
    [aux_sym_string_literal_token1] = ACTIONS(145),
    [sym_nothing] = ACTIONS(145),
    [sym_integer_literal] = ACTIONS(147),
    [aux_sym_define_token1] = ACTIONS(145),
    [sym_comment] = ACTIONS(145),
  },
  [24] = {
    [aux_sym_define_repeat1] = STATE(24),
    [sym_preproc_continuation_line] = ACTIONS(149),
    [sym_preproc_line] = ACTIONS(152),
  },
  [25] = {
    [sym_define] = STATE(45),
    [sym_preproc_if] = STATE(45),
    [sym_char_literal] = STATE(45),
    [aux_sym_translation_unit_repeat1] = STATE(45),
    [sym__top_level_item] = STATE(45),
    [sym_preproc_include] = STATE(45),
    [sym_preproc_nothing] = STATE(45),
    [sym_string_literal] = STATE(45),
    [sym_raw_string_literal] = ACTIONS(101),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [aux_sym_preproc_elif_token1] = ACTIONS(154),
    [sym_nothing] = ACTIONS(103),
    [sym_integer_literal] = ACTIONS(101),
    [aux_sym_preproc_if_token2] = ACTIONS(154),
    [aux_sym_define_token1] = ACTIONS(53),
    [aux_sym_preproc_else_token1] = ACTIONS(154),
    [sym_comment] = ACTIONS(103),
  },
  [26] = {
    [sym_define] = STATE(45),
    [sym_preproc_if] = STATE(45),
    [sym_char_literal] = STATE(45),
    [aux_sym_translation_unit_repeat1] = STATE(45),
    [sym__top_level_item] = STATE(45),
    [sym_preproc_include] = STATE(45),
    [sym_preproc_nothing] = STATE(45),
    [sym_string_literal] = STATE(45),
    [sym_raw_string_literal] = ACTIONS(101),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [sym_nothing] = ACTIONS(103),
    [sym_integer_literal] = ACTIONS(101),
    [aux_sym_preproc_if_token2] = ACTIONS(156),
    [aux_sym_define_token1] = ACTIONS(53),
    [sym_comment] = ACTIONS(103),
  },
  [27] = {
    [aux_sym_preproc_include_token1] = ACTIONS(158),
    [sym_raw_string_literal] = ACTIONS(160),
    [aux_sym_preproc_nothing_token1] = ACTIONS(158),
    [ts_builtin_sym_end] = ACTIONS(160),
    [aux_sym_char_literal_token1] = ACTIONS(158),
    [aux_sym_preproc_if_token1] = ACTIONS(158),
    [aux_sym_string_literal_token1] = ACTIONS(158),
    [sym_nothing] = ACTIONS(158),
    [sym_integer_literal] = ACTIONS(160),
    [aux_sym_define_token1] = ACTIONS(158),
    [sym_comment] = ACTIONS(158),
  },
  [28] = {
    [aux_sym_preproc_if_token2] = ACTIONS(162),
  },
  [29] = {
    [sym_preproc_else] = STATE(35),
    [aux_sym_preproc_if_repeat1] = STATE(30),
    [sym_preproc_elif] = STATE(30),
    [aux_sym_preproc_elif_token1] = ACTIONS(109),
    [aux_sym_preproc_else_token1] = ACTIONS(111),
    [aux_sym_preproc_if_token2] = ACTIONS(162),
  },
  [30] = {
    [aux_sym_preproc_if_repeat1] = STATE(30),
    [sym_preproc_elif] = STATE(30),
    [aux_sym_preproc_elif_token1] = ACTIONS(164),
    [aux_sym_preproc_else_token1] = ACTIONS(167),
    [aux_sym_preproc_if_token2] = ACTIONS(167),
  },
  [31] = {
    [anon_sym_LF] = ACTIONS(169),
  },
  [32] = {
    [aux_sym_define_repeat1] = STATE(24),
    [sym_preproc_continuation_line] = ACTIONS(81),
    [sym_preproc_line] = ACTIONS(171),
  },
  [33] = {
    [aux_sym_preproc_include_token1] = ACTIONS(173),
    [sym_raw_string_literal] = ACTIONS(175),
    [aux_sym_preproc_nothing_token1] = ACTIONS(173),
    [ts_builtin_sym_end] = ACTIONS(175),
    [aux_sym_char_literal_token1] = ACTIONS(173),
    [aux_sym_preproc_if_token1] = ACTIONS(173),
    [aux_sym_string_literal_token1] = ACTIONS(173),
    [sym_nothing] = ACTIONS(173),
    [sym_integer_literal] = ACTIONS(175),
    [aux_sym_define_token1] = ACTIONS(173),
    [sym_comment] = ACTIONS(173),
  },
  [34] = {
    [aux_sym_preproc_include_token1] = ACTIONS(177),
    [sym_raw_string_literal] = ACTIONS(179),
    [aux_sym_preproc_nothing_token1] = ACTIONS(177),
    [ts_builtin_sym_end] = ACTIONS(179),
    [aux_sym_char_literal_token1] = ACTIONS(177),
    [aux_sym_preproc_if_token1] = ACTIONS(177),
    [aux_sym_string_literal_token1] = ACTIONS(177),
    [sym_nothing] = ACTIONS(177),
    [sym_integer_literal] = ACTIONS(179),
    [aux_sym_define_token1] = ACTIONS(177),
    [sym_comment] = ACTIONS(177),
  },
  [35] = {
    [aux_sym_preproc_if_token2] = ACTIONS(181),
  },
  [36] = {
    [aux_sym_preproc_include_token1] = ACTIONS(183),
    [sym_raw_string_literal] = ACTIONS(185),
    [aux_sym_preproc_nothing_token1] = ACTIONS(183),
    [ts_builtin_sym_end] = ACTIONS(185),
    [aux_sym_char_literal_token1] = ACTIONS(183),
    [aux_sym_preproc_if_token1] = ACTIONS(183),
    [aux_sym_string_literal_token1] = ACTIONS(183),
    [sym_nothing] = ACTIONS(183),
    [sym_integer_literal] = ACTIONS(185),
    [aux_sym_define_token1] = ACTIONS(183),
    [sym_comment] = ACTIONS(183),
  },
  [37] = {
    [anon_sym_LF] = ACTIONS(187),
  },
  [38] = {
    [aux_sym_preproc_include_token1] = ACTIONS(189),
    [sym_raw_string_literal] = ACTIONS(191),
    [aux_sym_preproc_nothing_token1] = ACTIONS(189),
    [ts_builtin_sym_end] = ACTIONS(191),
    [aux_sym_char_literal_token1] = ACTIONS(189),
    [aux_sym_preproc_if_token1] = ACTIONS(189),
    [aux_sym_string_literal_token1] = ACTIONS(189),
    [sym_nothing] = ACTIONS(189),
    [sym_integer_literal] = ACTIONS(191),
    [aux_sym_define_token1] = ACTIONS(189),
    [sym_comment] = ACTIONS(189),
  },
  [39] = {
    [aux_sym_preproc_include_token1] = ACTIONS(193),
    [sym_raw_string_literal] = ACTIONS(195),
    [aux_sym_preproc_nothing_token1] = ACTIONS(193),
    [ts_builtin_sym_end] = ACTIONS(195),
    [aux_sym_char_literal_token1] = ACTIONS(193),
    [aux_sym_preproc_if_token1] = ACTIONS(193),
    [aux_sym_string_literal_token1] = ACTIONS(193),
    [sym_nothing] = ACTIONS(193),
    [sym_integer_literal] = ACTIONS(195),
    [aux_sym_define_token1] = ACTIONS(193),
    [sym_comment] = ACTIONS(193),
  },
  [40] = {
    [sym_raw_string_literal] = ACTIONS(33),
    [aux_sym_preproc_nothing_token1] = ACTIONS(31),
    [sym_integer_literal] = ACTIONS(33),
    [aux_sym_preproc_if_token2] = ACTIONS(31),
    [aux_sym_define_token1] = ACTIONS(31),
    [aux_sym_preproc_else_token1] = ACTIONS(31),
    [aux_sym_preproc_include_token1] = ACTIONS(31),
    [aux_sym_char_literal_token1] = ACTIONS(31),
    [aux_sym_preproc_if_token1] = ACTIONS(31),
    [aux_sym_string_literal_token1] = ACTIONS(31),
    [aux_sym_preproc_elif_token1] = ACTIONS(31),
    [sym_nothing] = ACTIONS(31),
    [sym_comment] = ACTIONS(31),
  },
  [41] = {
    [sym_raw_string_literal] = ACTIONS(59),
    [aux_sym_preproc_nothing_token1] = ACTIONS(57),
    [sym_integer_literal] = ACTIONS(59),
    [aux_sym_preproc_if_token2] = ACTIONS(57),
    [aux_sym_define_token1] = ACTIONS(57),
    [aux_sym_preproc_else_token1] = ACTIONS(57),
    [aux_sym_preproc_include_token1] = ACTIONS(57),
    [aux_sym_char_literal_token1] = ACTIONS(57),
    [aux_sym_preproc_if_token1] = ACTIONS(57),
    [aux_sym_string_literal_token1] = ACTIONS(57),
    [aux_sym_preproc_elif_token1] = ACTIONS(57),
    [sym_nothing] = ACTIONS(57),
    [sym_comment] = ACTIONS(57),
  },
  [42] = {
    [sym_raw_string_literal] = ACTIONS(73),
    [aux_sym_preproc_nothing_token1] = ACTIONS(71),
    [sym_integer_literal] = ACTIONS(73),
    [aux_sym_preproc_if_token2] = ACTIONS(71),
    [aux_sym_define_token1] = ACTIONS(71),
    [aux_sym_preproc_else_token1] = ACTIONS(71),
    [aux_sym_preproc_include_token1] = ACTIONS(71),
    [aux_sym_char_literal_token1] = ACTIONS(71),
    [aux_sym_preproc_if_token1] = ACTIONS(71),
    [aux_sym_string_literal_token1] = ACTIONS(71),
    [aux_sym_preproc_elif_token1] = ACTIONS(71),
    [sym_nothing] = ACTIONS(71),
    [sym_comment] = ACTIONS(71),
  },
  [43] = {
    [sym_raw_string_literal] = ACTIONS(79),
    [aux_sym_preproc_nothing_token1] = ACTIONS(77),
    [sym_integer_literal] = ACTIONS(79),
    [aux_sym_preproc_if_token2] = ACTIONS(77),
    [aux_sym_define_token1] = ACTIONS(77),
    [aux_sym_preproc_else_token1] = ACTIONS(77),
    [aux_sym_preproc_include_token1] = ACTIONS(77),
    [aux_sym_char_literal_token1] = ACTIONS(77),
    [aux_sym_preproc_if_token1] = ACTIONS(77),
    [aux_sym_string_literal_token1] = ACTIONS(77),
    [aux_sym_preproc_elif_token1] = ACTIONS(77),
    [sym_nothing] = ACTIONS(77),
    [sym_comment] = ACTIONS(77),
  },
  [44] = {
    [sym_raw_string_literal] = ACTIONS(93),
    [aux_sym_preproc_nothing_token1] = ACTIONS(91),
    [sym_integer_literal] = ACTIONS(93),
    [aux_sym_preproc_if_token2] = ACTIONS(91),
    [aux_sym_define_token1] = ACTIONS(91),
    [aux_sym_preproc_else_token1] = ACTIONS(91),
    [aux_sym_preproc_include_token1] = ACTIONS(91),
    [aux_sym_char_literal_token1] = ACTIONS(91),
    [aux_sym_preproc_if_token1] = ACTIONS(91),
    [aux_sym_string_literal_token1] = ACTIONS(91),
    [aux_sym_preproc_elif_token1] = ACTIONS(91),
    [sym_nothing] = ACTIONS(91),
    [sym_comment] = ACTIONS(91),
  },
  [45] = {
    [sym_define] = STATE(45),
    [sym_preproc_if] = STATE(45),
    [sym_char_literal] = STATE(45),
    [aux_sym_translation_unit_repeat1] = STATE(45),
    [sym__top_level_item] = STATE(45),
    [sym_preproc_include] = STATE(45),
    [sym_preproc_nothing] = STATE(45),
    [sym_string_literal] = STATE(45),
    [sym_raw_string_literal] = ACTIONS(197),
    [aux_sym_preproc_nothing_token1] = ACTIONS(200),
    [sym_integer_literal] = ACTIONS(197),
    [aux_sym_preproc_if_token2] = ACTIONS(203),
    [aux_sym_define_token1] = ACTIONS(205),
    [aux_sym_preproc_else_token1] = ACTIONS(203),
    [aux_sym_preproc_include_token1] = ACTIONS(208),
    [aux_sym_char_literal_token1] = ACTIONS(211),
    [aux_sym_preproc_if_token1] = ACTIONS(214),
    [aux_sym_string_literal_token1] = ACTIONS(217),
    [aux_sym_preproc_elif_token1] = ACTIONS(203),
    [sym_nothing] = ACTIONS(220),
    [sym_comment] = ACTIONS(220),
  },
  [46] = {
    [sym_raw_string_literal] = ACTIONS(147),
    [aux_sym_preproc_nothing_token1] = ACTIONS(145),
    [sym_integer_literal] = ACTIONS(147),
    [aux_sym_preproc_if_token2] = ACTIONS(145),
    [aux_sym_define_token1] = ACTIONS(145),
    [aux_sym_preproc_else_token1] = ACTIONS(145),
    [aux_sym_preproc_include_token1] = ACTIONS(145),
    [aux_sym_char_literal_token1] = ACTIONS(145),
    [aux_sym_preproc_if_token1] = ACTIONS(145),
    [aux_sym_string_literal_token1] = ACTIONS(145),
    [aux_sym_preproc_elif_token1] = ACTIONS(145),
    [sym_nothing] = ACTIONS(145),
    [sym_comment] = ACTIONS(145),
  },
  [47] = {
    [sym_raw_string_literal] = ACTIONS(160),
    [aux_sym_preproc_nothing_token1] = ACTIONS(158),
    [sym_integer_literal] = ACTIONS(160),
    [aux_sym_preproc_if_token2] = ACTIONS(158),
    [aux_sym_define_token1] = ACTIONS(158),
    [aux_sym_preproc_else_token1] = ACTIONS(158),
    [aux_sym_preproc_include_token1] = ACTIONS(158),
    [aux_sym_char_literal_token1] = ACTIONS(158),
    [aux_sym_preproc_if_token1] = ACTIONS(158),
    [aux_sym_string_literal_token1] = ACTIONS(158),
    [aux_sym_preproc_elif_token1] = ACTIONS(158),
    [sym_nothing] = ACTIONS(158),
    [sym_comment] = ACTIONS(158),
  },
  [48] = {
    [sym_raw_string_literal] = ACTIONS(175),
    [aux_sym_preproc_nothing_token1] = ACTIONS(173),
    [sym_integer_literal] = ACTIONS(175),
    [aux_sym_preproc_if_token2] = ACTIONS(173),
    [aux_sym_define_token1] = ACTIONS(173),
    [aux_sym_preproc_else_token1] = ACTIONS(173),
    [aux_sym_preproc_include_token1] = ACTIONS(173),
    [aux_sym_char_literal_token1] = ACTIONS(173),
    [aux_sym_preproc_if_token1] = ACTIONS(173),
    [aux_sym_string_literal_token1] = ACTIONS(173),
    [aux_sym_preproc_elif_token1] = ACTIONS(173),
    [sym_nothing] = ACTIONS(173),
    [sym_comment] = ACTIONS(173),
  },
  [49] = {
    [sym_raw_string_literal] = ACTIONS(179),
    [aux_sym_preproc_nothing_token1] = ACTIONS(177),
    [sym_integer_literal] = ACTIONS(179),
    [aux_sym_preproc_if_token2] = ACTIONS(177),
    [aux_sym_define_token1] = ACTIONS(177),
    [aux_sym_preproc_else_token1] = ACTIONS(177),
    [aux_sym_preproc_include_token1] = ACTIONS(177),
    [aux_sym_char_literal_token1] = ACTIONS(177),
    [aux_sym_preproc_if_token1] = ACTIONS(177),
    [aux_sym_string_literal_token1] = ACTIONS(177),
    [aux_sym_preproc_elif_token1] = ACTIONS(177),
    [sym_nothing] = ACTIONS(177),
    [sym_comment] = ACTIONS(177),
  },
  [50] = {
    [sym_raw_string_literal] = ACTIONS(185),
    [aux_sym_preproc_nothing_token1] = ACTIONS(183),
    [sym_integer_literal] = ACTIONS(185),
    [aux_sym_preproc_if_token2] = ACTIONS(183),
    [aux_sym_define_token1] = ACTIONS(183),
    [aux_sym_preproc_else_token1] = ACTIONS(183),
    [aux_sym_preproc_include_token1] = ACTIONS(183),
    [aux_sym_char_literal_token1] = ACTIONS(183),
    [aux_sym_preproc_if_token1] = ACTIONS(183),
    [aux_sym_string_literal_token1] = ACTIONS(183),
    [aux_sym_preproc_elif_token1] = ACTIONS(183),
    [sym_nothing] = ACTIONS(183),
    [sym_comment] = ACTIONS(183),
  },
  [51] = {
    [sym_raw_string_literal] = ACTIONS(191),
    [aux_sym_preproc_nothing_token1] = ACTIONS(189),
    [sym_integer_literal] = ACTIONS(191),
    [aux_sym_preproc_if_token2] = ACTIONS(189),
    [aux_sym_define_token1] = ACTIONS(189),
    [aux_sym_preproc_else_token1] = ACTIONS(189),
    [aux_sym_preproc_include_token1] = ACTIONS(189),
    [aux_sym_char_literal_token1] = ACTIONS(189),
    [aux_sym_preproc_if_token1] = ACTIONS(189),
    [aux_sym_string_literal_token1] = ACTIONS(189),
    [aux_sym_preproc_elif_token1] = ACTIONS(189),
    [sym_nothing] = ACTIONS(189),
    [sym_comment] = ACTIONS(189),
  },
  [52] = {
    [sym_raw_string_literal] = ACTIONS(195),
    [aux_sym_preproc_nothing_token1] = ACTIONS(193),
    [sym_integer_literal] = ACTIONS(195),
    [aux_sym_preproc_if_token2] = ACTIONS(193),
    [aux_sym_define_token1] = ACTIONS(193),
    [aux_sym_preproc_else_token1] = ACTIONS(193),
    [aux_sym_preproc_include_token1] = ACTIONS(193),
    [aux_sym_char_literal_token1] = ACTIONS(193),
    [aux_sym_preproc_if_token1] = ACTIONS(193),
    [aux_sym_string_literal_token1] = ACTIONS(193),
    [aux_sym_preproc_elif_token1] = ACTIONS(193),
    [sym_nothing] = ACTIONS(193),
    [sym_comment] = ACTIONS(193),
  },
  [53] = {
    [sym_string_literal] = STATE(42),
    [sym_identifier] = ACTIONS(223),
    [anon_sym_LT] = ACTIONS(225),
    [aux_sym_string_literal_token1] = ACTIONS(227),
  },
  [54] = {
    [aux_sym_define_repeat1] = STATE(56),
    [sym_preproc_continuation_line] = ACTIONS(229),
    [sym_preproc_line] = ACTIONS(231),
  },
  [55] = {
    [sym_define] = STATE(57),
    [sym_preproc_if] = STATE(57),
    [sym_char_literal] = STATE(57),
    [aux_sym_translation_unit_repeat1] = STATE(57),
    [sym__top_level_item] = STATE(57),
    [sym_preproc_include] = STATE(57),
    [sym_preproc_nothing] = STATE(57),
    [sym_string_literal] = STATE(57),
    [sym_preproc_elif] = STATE(59),
    [sym_preproc_else] = STATE(58),
    [aux_sym_preproc_if_repeat1] = STATE(59),
    [sym_raw_string_literal] = ACTIONS(233),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [sym_integer_literal] = ACTIONS(233),
    [aux_sym_preproc_if_token2] = ACTIONS(235),
    [aux_sym_define_token1] = ACTIONS(53),
    [aux_sym_preproc_else_token1] = ACTIONS(55),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [aux_sym_preproc_elif_token1] = ACTIONS(47),
    [sym_nothing] = ACTIONS(237),
    [sym_comment] = ACTIONS(237),
  },
  [56] = {
    [aux_sym_define_repeat1] = STATE(24),
    [sym_preproc_continuation_line] = ACTIONS(81),
    [sym_preproc_line] = ACTIONS(239),
  },
  [57] = {
    [sym_define] = STATE(45),
    [sym_preproc_if] = STATE(45),
    [sym_char_literal] = STATE(45),
    [aux_sym_translation_unit_repeat1] = STATE(45),
    [sym__top_level_item] = STATE(45),
    [sym_preproc_include] = STATE(45),
    [sym_preproc_nothing] = STATE(45),
    [sym_string_literal] = STATE(45),
    [sym_preproc_elif] = STATE(62),
    [sym_preproc_else] = STATE(61),
    [aux_sym_preproc_if_repeat1] = STATE(62),
    [sym_raw_string_literal] = ACTIONS(101),
    [aux_sym_preproc_nothing_token1] = ACTIONS(39),
    [sym_integer_literal] = ACTIONS(101),
    [aux_sym_preproc_if_token2] = ACTIONS(241),
    [aux_sym_define_token1] = ACTIONS(53),
    [aux_sym_preproc_else_token1] = ACTIONS(55),
    [aux_sym_preproc_include_token1] = ACTIONS(37),
    [aux_sym_char_literal_token1] = ACTIONS(41),
    [aux_sym_preproc_if_token1] = ACTIONS(43),
    [aux_sym_string_literal_token1] = ACTIONS(45),
    [aux_sym_preproc_elif_token1] = ACTIONS(47),
    [sym_nothing] = ACTIONS(103),
    [sym_comment] = ACTIONS(103),
  },
  [58] = {
    [aux_sym_preproc_if_token2] = ACTIONS(243),
  },
  [59] = {
    [sym_preproc_else] = STATE(61),
    [aux_sym_preproc_if_repeat1] = STATE(30),
    [sym_preproc_elif] = STATE(30),
    [aux_sym_preproc_elif_token1] = ACTIONS(109),
    [aux_sym_preproc_if_token2] = ACTIONS(243),
    [aux_sym_preproc_else_token1] = ACTIONS(111),
  },
  [60] = {
    [anon_sym_GT] = ACTIONS(245),
  },
  [61] = {
    [aux_sym_preproc_if_token2] = ACTIONS(247),
  },
  [62] = {
    [sym_preproc_else] = STATE(64),
    [aux_sym_preproc_if_repeat1] = STATE(30),
    [sym_preproc_elif] = STATE(30),
    [aux_sym_preproc_elif_token1] = ACTIONS(109),
    [aux_sym_preproc_if_token2] = ACTIONS(247),
    [aux_sym_preproc_else_token1] = ACTIONS(111),
  },
  [63] = {
    [anon_sym_LF] = ACTIONS(249),
  },
  [64] = {
    [aux_sym_preproc_if_token2] = ACTIONS(251),
  },
  [65] = {
    [anon_sym_LF] = ACTIONS(253),
  },
  [66] = {
    [sym_path] = ACTIONS(255),
  },
  [67] = {
    [aux_sym_define_repeat1] = STATE(68),
    [sym_preproc_continuation_line] = ACTIONS(257),
    [sym_preproc_line] = ACTIONS(259),
  },
  [68] = {
    [aux_sym_define_repeat1] = STATE(24),
    [sym_preproc_continuation_line] = ACTIONS(81),
    [sym_preproc_line] = ACTIONS(261),
  },
  [69] = {
    [sym_identifier] = ACTIONS(263),
  },
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, SHIFT(8),
  [5] = {.count = 1, .reusable = false}, SHIFT(2),
  [7] = {.count = 1, .reusable = true}, REDUCE(sym_translation_unit, 0),
  [9] = {.count = 1, .reusable = false}, SHIFT(3),
  [11] = {.count = 1, .reusable = false}, SHIFT(4),
  [13] = {.count = 1, .reusable = false}, SHIFT(5),
  [15] = {.count = 1, .reusable = false}, SHIFT(6),
  [17] = {.count = 1, .reusable = false}, SHIFT(8),
  [19] = {.count = 1, .reusable = false}, SHIFT(7),
  [21] = {.count = 1, .reusable = true}, SHIFT(10),
  [23] = {.count = 1, .reusable = true}, SHIFT(11),
  [25] = {.count = 1, .reusable = true}, SHIFT(6),
  [27] = {.count = 1, .reusable = true}, SHIFT(13),
  [29] = {.count = 1, .reusable = false}, SHIFT(12),
  [31] = {.count = 1, .reusable = false}, REDUCE(sym_char_literal, 1),
  [33] = {.count = 1, .reusable = true}, REDUCE(sym_char_literal, 1),
  [35] = {.count = 1, .reusable = true}, SHIFT(17),
  [37] = {.count = 1, .reusable = false}, SHIFT(53),
  [39] = {.count = 1, .reusable = false}, SHIFT(54),
  [41] = {.count = 1, .reusable = false}, SHIFT(40),
  [43] = {.count = 1, .reusable = false}, SHIFT(55),
  [45] = {.count = 1, .reusable = false}, SHIFT(41),
  [47] = {.count = 1, .reusable = false}, SHIFT(14),
  [49] = {.count = 1, .reusable = false}, SHIFT(17),
  [51] = {.count = 1, .reusable = false}, SHIFT(15),
  [53] = {.count = 1, .reusable = false}, SHIFT(69),
  [55] = {.count = 1, .reusable = false}, SHIFT(16),
  [57] = {.count = 1, .reusable = false}, REDUCE(sym_string_literal, 1),
  [59] = {.count = 1, .reusable = true}, REDUCE(sym_string_literal, 1),
  [61] = {.count = 1, .reusable = true}, SHIFT(20),
  [63] = {.count = 1, .reusable = true}, SHIFT(21),
  [65] = {.count = 1, .reusable = true}, REDUCE(sym_translation_unit, 1),
  [67] = {.count = 1, .reusable = false}, SHIFT(21),
  [69] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [71] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_include, 2),
  [73] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_include, 2),
  [75] = {.count = 1, .reusable = true}, SHIFT(22),
  [77] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_nothing, 2),
  [79] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_nothing, 2),
  [81] = {.count = 1, .reusable = true}, SHIFT(24),
  [83] = {.count = 1, .reusable = false}, SHIFT(23),
  [85] = {.count = 1, .reusable = true}, SHIFT(25),
  [87] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_elif, 1),
  [89] = {.count = 1, .reusable = false}, SHIFT(25),
  [91] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 2),
  [93] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 2),
  [95] = {.count = 1, .reusable = true}, SHIFT(26),
  [97] = {.count = 1, .reusable = false}, SHIFT(26),
  [99] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_else, 1),
  [101] = {.count = 1, .reusable = true}, SHIFT(45),
  [103] = {.count = 1, .reusable = false}, SHIFT(45),
  [105] = {.count = 1, .reusable = false}, SHIFT(27),
  [107] = {.count = 1, .reusable = true}, SHIFT(27),
  [109] = {.count = 1, .reusable = true}, SHIFT(14),
  [111] = {.count = 1, .reusable = true}, SHIFT(16),
  [113] = {.count = 1, .reusable = true}, SHIFT(32),
  [115] = {.count = 1, .reusable = false}, SHIFT(31),
  [117] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(2),
  [120] = {.count = 2, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(21),
  [123] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(3),
  [126] = {.count = 1, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [128] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(4),
  [131] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(5),
  [134] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(6),
  [137] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(21),
  [140] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(7),
  [143] = {.count = 1, .reusable = true}, SHIFT(33),
  [145] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_nothing, 3),
  [147] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_nothing, 3),
  [149] = {.count = 2, .reusable = true}, REDUCE(aux_sym_define_repeat1, 2), SHIFT_REPEAT(24),
  [152] = {.count = 1, .reusable = false}, REDUCE(aux_sym_define_repeat1, 2),
  [154] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_elif, 2),
  [156] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_else, 2),
  [158] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 3),
  [160] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 3),
  [162] = {.count = 1, .reusable = true}, SHIFT(34),
  [164] = {.count = 2, .reusable = true}, REDUCE(aux_sym_preproc_if_repeat1, 2), SHIFT_REPEAT(14),
  [167] = {.count = 1, .reusable = true}, REDUCE(aux_sym_preproc_if_repeat1, 2),
  [169] = {.count = 1, .reusable = true}, SHIFT(36),
  [171] = {.count = 1, .reusable = false}, SHIFT(37),
  [173] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_include, 4),
  [175] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_include, 4),
  [177] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 4),
  [179] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 4),
  [181] = {.count = 1, .reusable = true}, SHIFT(38),
  [183] = {.count = 1, .reusable = false}, REDUCE(sym_define, 4),
  [185] = {.count = 1, .reusable = true}, REDUCE(sym_define, 4),
  [187] = {.count = 1, .reusable = true}, SHIFT(39),
  [189] = {.count = 1, .reusable = false}, REDUCE(sym_preproc_if, 5),
  [191] = {.count = 1, .reusable = true}, REDUCE(sym_preproc_if, 5),
  [193] = {.count = 1, .reusable = false}, REDUCE(sym_define, 5),
  [195] = {.count = 1, .reusable = true}, REDUCE(sym_define, 5),
  [197] = {.count = 2, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(45),
  [200] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(54),
  [203] = {.count = 1, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [205] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(69),
  [208] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(53),
  [211] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(40),
  [214] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(55),
  [217] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(41),
  [220] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(45),
  [223] = {.count = 1, .reusable = true}, SHIFT(42),
  [225] = {.count = 1, .reusable = true}, SHIFT(66),
  [227] = {.count = 1, .reusable = true}, SHIFT(41),
  [229] = {.count = 1, .reusable = true}, SHIFT(56),
  [231] = {.count = 1, .reusable = false}, SHIFT(43),
  [233] = {.count = 1, .reusable = true}, SHIFT(57),
  [235] = {.count = 1, .reusable = false}, SHIFT(44),
  [237] = {.count = 1, .reusable = false}, SHIFT(57),
  [239] = {.count = 1, .reusable = false}, SHIFT(46),
  [241] = {.count = 1, .reusable = false}, SHIFT(47),
  [243] = {.count = 1, .reusable = true}, SHIFT(47),
  [245] = {.count = 1, .reusable = true}, SHIFT(48),
  [247] = {.count = 1, .reusable = true}, SHIFT(49),
  [249] = {.count = 1, .reusable = true}, SHIFT(50),
  [251] = {.count = 1, .reusable = true}, SHIFT(51),
  [253] = {.count = 1, .reusable = true}, SHIFT(52),
  [255] = {.count = 1, .reusable = true}, SHIFT(60),
  [257] = {.count = 1, .reusable = true}, SHIFT(68),
  [259] = {.count = 1, .reusable = false}, SHIFT(63),
  [261] = {.count = 1, .reusable = false}, SHIFT(65),
  [263] = {.count = 1, .reusable = true}, SHIFT(67),
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
