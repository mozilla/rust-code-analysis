#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 12
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 16
#define ALIAS_COUNT 0
#define TOKEN_COUNT 9
#define EXTERNAL_TOKEN_COUNT 1
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

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

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_nothing] = sym_nothing,
  [sym_preproc_continuation_line] = sym_preproc_continuation_line,
  [sym_preproc_line] = sym_preproc_line,
  [aux_sym_define_token1] = aux_sym_define_token1,
  [aux_sym_string_literal_token1] = aux_sym_string_literal_token1,
  [aux_sym_char_literal_token1] = aux_sym_char_literal_token1,
  [sym_comment] = sym_comment,
  [sym_raw_string_literal] = sym_raw_string_literal,
  [sym_translation_unit] = sym_translation_unit,
  [sym__top_level_item] = sym__top_level_item,
  [sym_define] = sym_define,
  [sym_string_literal] = sym_string_literal,
  [sym_char_literal] = sym_char_literal,
  [aux_sym_translation_unit_repeat1] = aux_sym_translation_unit_repeat1,
  [aux_sym_define_repeat1] = aux_sym_define_repeat1,
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

static TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(20);
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
      if (lookahead == '\\') ADVANCE(18);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '\'') ADVANCE(30);
      if (lookahead == '\\') ADVANCE(19);
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
      if (lookahead != 0 &&
          lookahead != '\r') ADVANCE(32);
      if (lookahead == '\r') ADVANCE(33);
      END_STATE();
    case 18:
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 19:
      if (lookahead != 0) ADVANCE(6);
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
      if (lookahead == '\\') ADVANCE(17);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(32);
      if (lookahead == '\\') ADVANCE(17);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 0, .external_lex_state = 1},
  [2] = {.lex_state = 0, .external_lex_state = 1},
  [3] = {.lex_state = 0, .external_lex_state = 1},
  [4] = {.lex_state = 0, .external_lex_state = 1},
  [5] = {.lex_state = 0, .external_lex_state = 1},
  [6] = {.lex_state = 0, .external_lex_state = 1},
  [7] = {.lex_state = 0, .external_lex_state = 1},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 1},
  [10] = {.lex_state = 1},
  [11] = {.lex_state = 0},
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

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_nothing] = ACTIONS(1),
    [aux_sym_define_token1] = ACTIONS(1),
    [aux_sym_string_literal_token1] = ACTIONS(1),
    [aux_sym_char_literal_token1] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
    [sym_raw_string_literal] = ACTIONS(1),
  },
  [1] = {
    [sym_translation_unit] = STATE(11),
    [sym__top_level_item] = STATE(2),
    [sym_define] = STATE(2),
    [sym_string_literal] = STATE(2),
    [sym_char_literal] = STATE(2),
    [aux_sym_translation_unit_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_nothing] = ACTIONS(5),
    [aux_sym_define_token1] = ACTIONS(7),
    [aux_sym_string_literal_token1] = ACTIONS(9),
    [aux_sym_char_literal_token1] = ACTIONS(11),
    [sym_comment] = ACTIONS(5),
    [sym_raw_string_literal] = ACTIONS(13),
  },
  [2] = {
    [sym__top_level_item] = STATE(3),
    [sym_define] = STATE(3),
    [sym_string_literal] = STATE(3),
    [sym_char_literal] = STATE(3),
    [aux_sym_translation_unit_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(15),
    [sym_nothing] = ACTIONS(17),
    [aux_sym_define_token1] = ACTIONS(7),
    [aux_sym_string_literal_token1] = ACTIONS(9),
    [aux_sym_char_literal_token1] = ACTIONS(11),
    [sym_comment] = ACTIONS(17),
    [sym_raw_string_literal] = ACTIONS(19),
  },
  [3] = {
    [sym__top_level_item] = STATE(3),
    [sym_define] = STATE(3),
    [sym_string_literal] = STATE(3),
    [sym_char_literal] = STATE(3),
    [aux_sym_translation_unit_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(21),
    [sym_nothing] = ACTIONS(23),
    [aux_sym_define_token1] = ACTIONS(26),
    [aux_sym_string_literal_token1] = ACTIONS(29),
    [aux_sym_char_literal_token1] = ACTIONS(32),
    [sym_comment] = ACTIONS(23),
    [sym_raw_string_literal] = ACTIONS(35),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(38), 2,
      sym_raw_string_literal,
      ts_builtin_sym_end,
    ACTIONS(40), 5,
      sym_nothing,
      aux_sym_define_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [12] = 2,
    ACTIONS(42), 2,
      sym_raw_string_literal,
      ts_builtin_sym_end,
    ACTIONS(44), 5,
      sym_nothing,
      aux_sym_define_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [24] = 2,
    ACTIONS(46), 2,
      sym_raw_string_literal,
      ts_builtin_sym_end,
    ACTIONS(48), 5,
      sym_nothing,
      aux_sym_define_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [36] = 2,
    ACTIONS(50), 2,
      sym_raw_string_literal,
      ts_builtin_sym_end,
    ACTIONS(52), 5,
      sym_nothing,
      aux_sym_define_token1,
      aux_sym_string_literal_token1,
      aux_sym_char_literal_token1,
      sym_comment,
  [48] = 3,
    ACTIONS(54), 1,
      sym_preproc_continuation_line,
    ACTIONS(56), 1,
      sym_preproc_line,
    STATE(9), 1,
      aux_sym_define_repeat1,
  [58] = 3,
    ACTIONS(58), 1,
      sym_preproc_continuation_line,
    ACTIONS(60), 1,
      sym_preproc_line,
    STATE(10), 1,
      aux_sym_define_repeat1,
  [68] = 3,
    ACTIONS(62), 1,
      sym_preproc_continuation_line,
    ACTIONS(65), 1,
      sym_preproc_line,
    STATE(10), 1,
      aux_sym_define_repeat1,
  [78] = 1,
    ACTIONS(67), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 12,
  [SMALL_STATE(6)] = 24,
  [SMALL_STATE(7)] = 36,
  [SMALL_STATE(8)] = 48,
  [SMALL_STATE(9)] = 58,
  [SMALL_STATE(10)] = 68,
  [SMALL_STATE(11)] = 78,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 1),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [23] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(3),
  [26] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(8),
  [29] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(4),
  [32] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(5),
  [35] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(3),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_literal, 1),
  [40] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_literal, 1),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_char_literal, 1),
  [44] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_char_literal, 1),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_define, 2),
  [48] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_define, 2),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_define, 3),
  [52] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_define, 3),
  [54] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [56] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [58] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [60] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [62] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_define_repeat1, 2), SHIFT_REPEAT(10),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_define_repeat1, 2),
  [67] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
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
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = (const uint16_t *)ts_parse_table,
    .small_parse_table = (const uint16_t *)ts_small_parse_table,
    .small_parse_table_map = (const uint32_t *)ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = (const TSSymbol *)ts_alias_sequences,
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
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
#ifdef __cplusplus
}
#endif
