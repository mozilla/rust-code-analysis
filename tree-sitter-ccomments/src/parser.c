#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 10
#define STATE_COUNT 12
#define SYMBOL_COUNT 16
#define ALIAS_COUNT 0
#define TOKEN_COUNT 9
#define EXTERNAL_TOKEN_COUNT 1
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  sym_nothing = 1,
  sym_preproc_continuation_line = 2,
  sym_preproc_line = 3,
  aux_sym_define_token1 = 4,
  aux_sym_string_literal_token1 = 5,
  aux_sym_char_literal_token1 = 6,
  sym_comment = 7,
  sym_raw_string_literal = 8,
  sym_translation_unit = 9,
  sym__top_level_item = 10,
  sym_define = 11,
  sym_string_literal = 12,
  sym_char_literal = 13,
  aux_sym_translation_unit_repeat1 = 14,
  aux_sym_define_repeat1 = 15,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_nothing] = "nothing",
  [sym_preproc_continuation_line] = "preproc_continuation_line",
  [sym_preproc_line] = "preproc_line",
  [aux_sym_define_token1] = "define_token1",
  [aux_sym_string_literal_token1] = "string_literal_token1",
  [aux_sym_char_literal_token1] = "char_literal_token1",
  [sym_comment] = "comment",
  [sym_raw_string_literal] = "raw_string_literal",
  [sym_translation_unit] = "translation_unit",
  [sym__top_level_item] = "_top_level_item",
  [sym_define] = "define",
  [sym_string_literal] = "string_literal",
  [sym_char_literal] = "char_literal",
  [aux_sym_translation_unit_repeat1] = "translation_unit_repeat1",
  [aux_sym_define_repeat1] = "define_repeat1",
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
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
  [aux_sym_define_token1] = {
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
  [sym_define] = {
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  switch (state) {
    case 0:
      if (lookahead == 0) ADVANCE(20);
      if (lookahead == '"') ADVANCE(5);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(6);
      if (lookahead == '/') ADVANCE(7);
      if (lookahead == 'R') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(22);
      if (lookahead != 0) ADVANCE(24);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(1);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(25);
      if (lookahead == '\r') ADVANCE(3);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(25);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(26);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '"') ADVANCE(29);
      if (lookahead == '\\') ADVANCE(17);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '\'') ADVANCE(30);
      if (lookahead == '\\') ADVANCE(18);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == '*') ADVANCE(9);
      if (lookahead == '/') ADVANCE(32);
      END_STATE();
    case 8:
      if (lookahead == '*') ADVANCE(8);
      if (lookahead == '/') ADVANCE(31);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 9:
      if (lookahead == '*') ADVANCE(8);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 10:
      if (lookahead == 'd') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(10);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 12:
      if (lookahead == 'e') ADVANCE(16);
      END_STATE();
    case 13:
      if (lookahead == 'f') ADVANCE(14);
      END_STATE();
    case 14:
      if (lookahead == 'i') ADVANCE(15);
      END_STATE();
    case 15:
      if (lookahead == 'n') ADVANCE(12);
      END_STATE();
    case 16:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(28);
      END_STATE();
    case 17:
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 18:
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 19:
      if (lookahead != 0 &&
          lookahead != '\r') ADVANCE(32);
      if (lookahead == '\r') ADVANCE(33);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_nothing);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == '#') ADVANCE(23);
      if (lookahead == 'R') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(22);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          lookahead != '/') ADVANCE(24);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead == 'd') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(10);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_nothing);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '#' &&
          lookahead != '\'' &&
          lookahead != '/' &&
          lookahead != 'R') ADVANCE(24);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_preproc_continuation_line);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_preproc_line);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_preproc_line);
      if (lookahead == '\n') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(1);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym_define_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(aux_sym_char_literal_token1);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\\') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(32);
      if (lookahead == '\\') ADVANCE(19);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 0, .external_lex_state = 1},
  [2] = {.lex_state = 0, .external_lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 0, .external_lex_state = 1},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0, .external_lex_state = 1},
  [7] = {.lex_state = 0, .external_lex_state = 1},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 0, .external_lex_state = 1},
  [10] = {.lex_state = 0, .external_lex_state = 1},
  [11] = {.lex_state = 1},
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
    [aux_sym_string_literal_token1] = ACTIONS(1),
    [sym_raw_string_literal] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
    [aux_sym_define_token1] = ACTIONS(1),
    [sym_nothing] = ACTIONS(1),
    [aux_sym_char_literal_token1] = ACTIONS(1),
    [ts_builtin_sym_end] = ACTIONS(1),
  },
  [1] = {
    [sym_string_literal] = STATE(6),
    [sym_char_literal] = STATE(6),
    [sym__top_level_item] = STATE(6),
    [sym_define] = STATE(6),
    [sym_translation_unit] = STATE(5),
    [aux_sym_translation_unit_repeat1] = STATE(6),
    [sym_raw_string_literal] = ACTIONS(3),
    [aux_sym_string_literal_token1] = ACTIONS(5),
    [ts_builtin_sym_end] = ACTIONS(7),
    [sym_comment] = ACTIONS(9),
    [aux_sym_define_token1] = ACTIONS(11),
    [sym_nothing] = ACTIONS(9),
    [aux_sym_char_literal_token1] = ACTIONS(13),
  },
  [2] = {
    [aux_sym_string_literal_token1] = ACTIONS(15),
    [sym_raw_string_literal] = ACTIONS(17),
    [sym_comment] = ACTIONS(15),
    [aux_sym_define_token1] = ACTIONS(15),
    [sym_nothing] = ACTIONS(15),
    [aux_sym_char_literal_token1] = ACTIONS(15),
    [ts_builtin_sym_end] = ACTIONS(17),
  },
  [3] = {
    [aux_sym_define_repeat1] = STATE(8),
    [sym_preproc_continuation_line] = ACTIONS(19),
    [sym_preproc_line] = ACTIONS(21),
  },
  [4] = {
    [aux_sym_string_literal_token1] = ACTIONS(23),
    [sym_raw_string_literal] = ACTIONS(25),
    [sym_comment] = ACTIONS(23),
    [aux_sym_define_token1] = ACTIONS(23),
    [sym_nothing] = ACTIONS(23),
    [aux_sym_char_literal_token1] = ACTIONS(23),
    [ts_builtin_sym_end] = ACTIONS(25),
  },
  [5] = {
    [ts_builtin_sym_end] = ACTIONS(27),
  },
  [6] = {
    [sym_string_literal] = STATE(9),
    [sym_char_literal] = STATE(9),
    [sym__top_level_item] = STATE(9),
    [sym_define] = STATE(9),
    [aux_sym_translation_unit_repeat1] = STATE(9),
    [sym_raw_string_literal] = ACTIONS(29),
    [aux_sym_string_literal_token1] = ACTIONS(5),
    [ts_builtin_sym_end] = ACTIONS(31),
    [sym_comment] = ACTIONS(33),
    [aux_sym_define_token1] = ACTIONS(11),
    [sym_nothing] = ACTIONS(33),
    [aux_sym_char_literal_token1] = ACTIONS(13),
  },
  [7] = {
    [aux_sym_string_literal_token1] = ACTIONS(35),
    [sym_raw_string_literal] = ACTIONS(37),
    [sym_comment] = ACTIONS(35),
    [aux_sym_define_token1] = ACTIONS(35),
    [sym_nothing] = ACTIONS(35),
    [aux_sym_char_literal_token1] = ACTIONS(35),
    [ts_builtin_sym_end] = ACTIONS(37),
  },
  [8] = {
    [aux_sym_define_repeat1] = STATE(11),
    [sym_preproc_continuation_line] = ACTIONS(39),
    [sym_preproc_line] = ACTIONS(41),
  },
  [9] = {
    [sym_string_literal] = STATE(9),
    [sym_char_literal] = STATE(9),
    [sym__top_level_item] = STATE(9),
    [sym_define] = STATE(9),
    [aux_sym_translation_unit_repeat1] = STATE(9),
    [aux_sym_string_literal_token1] = ACTIONS(43),
    [sym_raw_string_literal] = ACTIONS(46),
    [sym_comment] = ACTIONS(49),
    [aux_sym_define_token1] = ACTIONS(52),
    [sym_nothing] = ACTIONS(49),
    [aux_sym_char_literal_token1] = ACTIONS(55),
    [ts_builtin_sym_end] = ACTIONS(58),
  },
  [10] = {
    [aux_sym_string_literal_token1] = ACTIONS(60),
    [sym_raw_string_literal] = ACTIONS(62),
    [sym_comment] = ACTIONS(60),
    [aux_sym_define_token1] = ACTIONS(60),
    [sym_nothing] = ACTIONS(60),
    [aux_sym_char_literal_token1] = ACTIONS(60),
    [ts_builtin_sym_end] = ACTIONS(62),
  },
  [11] = {
    [aux_sym_define_repeat1] = STATE(11),
    [sym_preproc_continuation_line] = ACTIONS(64),
    [sym_preproc_line] = ACTIONS(67),
  },
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, SHIFT(6),
  [5] = {.count = 1, .reusable = false}, SHIFT(2),
  [7] = {.count = 1, .reusable = true}, REDUCE(sym_translation_unit, 0),
  [9] = {.count = 1, .reusable = false}, SHIFT(6),
  [11] = {.count = 1, .reusable = false}, SHIFT(3),
  [13] = {.count = 1, .reusable = false}, SHIFT(4),
  [15] = {.count = 1, .reusable = false}, REDUCE(sym_string_literal, 1),
  [17] = {.count = 1, .reusable = true}, REDUCE(sym_string_literal, 1),
  [19] = {.count = 1, .reusable = false}, SHIFT(8),
  [21] = {.count = 1, .reusable = false}, SHIFT(7),
  [23] = {.count = 1, .reusable = false}, REDUCE(sym_char_literal, 1),
  [25] = {.count = 1, .reusable = true}, REDUCE(sym_char_literal, 1),
  [27] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [29] = {.count = 1, .reusable = true}, SHIFT(9),
  [31] = {.count = 1, .reusable = true}, REDUCE(sym_translation_unit, 1),
  [33] = {.count = 1, .reusable = false}, SHIFT(9),
  [35] = {.count = 1, .reusable = false}, REDUCE(sym_define, 2),
  [37] = {.count = 1, .reusable = true}, REDUCE(sym_define, 2),
  [39] = {.count = 1, .reusable = false}, SHIFT(11),
  [41] = {.count = 1, .reusable = false}, SHIFT(10),
  [43] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(2),
  [46] = {.count = 2, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(9),
  [49] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(9),
  [52] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(3),
  [55] = {.count = 2, .reusable = false}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(4),
  [58] = {.count = 1, .reusable = true}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [60] = {.count = 1, .reusable = false}, REDUCE(sym_define, 3),
  [62] = {.count = 1, .reusable = true}, REDUCE(sym_define, 3),
  [64] = {.count = 2, .reusable = false}, REDUCE(aux_sym_define_repeat1, 2), SHIFT_REPEAT(11),
  [67] = {.count = 1, .reusable = false}, REDUCE(aux_sym_define_repeat1, 2),
};

void *tree_sitter_ccomment_external_scanner_create(void);
void tree_sitter_ccomment_external_scanner_destroy(void *);
bool tree_sitter_ccomment_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_ccomment_external_scanner_serialize(void *, char *);
void tree_sitter_ccomment_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_ccomment(void) {
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
      tree_sitter_ccomment_external_scanner_create,
      tree_sitter_ccomment_external_scanner_destroy,
      tree_sitter_ccomment_external_scanner_scan,
      tree_sitter_ccomment_external_scanner_serialize,
      tree_sitter_ccomment_external_scanner_deserialize,
    },
  };
  return &language;
}
