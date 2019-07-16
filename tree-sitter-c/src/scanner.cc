#include <tree_sitter/parser.h>
#include <string>
#include <cstring>
#include <cwctype>

namespace {

using std::wstring;
using std::iswspace;

enum TokenType {
  MACRO_ANNOTATION,
  MACRO_CALL,
  MACRO_SPECIAL,
};

extern "C" unsigned tree_sitter_c_is_macro(const char *, unsigned length);

struct Scanner {

  bool scan(TSLexer *lexer, const bool *valid_symbols) {
      while (iswspace(lexer->lookahead)) {
          lexer->advance(lexer, true);
      }

      // First, we get a valid identifier
      if ((valid_symbols[MACRO_ANNOTATION] || valid_symbols[MACRO_CALL]) &&
          (('a' <= lexer->lookahead && lexer->lookahead <= 'z') ||
          ('A' <= lexer->lookahead && lexer->lookahead <= 'Z') ||
           lexer->lookahead == '_')) {
          char identifier[1024];
          int32_t len = 1;
          identifier[0] = lexer->lookahead;
          lexer->advance(lexer, false);
          while (('a' <= lexer->lookahead && lexer->lookahead <= 'z') ||
                 ('A' <= lexer->lookahead && lexer->lookahead <= 'Z') ||
                 ('0' <= lexer->lookahead && lexer->lookahead <= '9') ||
                 lexer->lookahead == '_') {
              identifier[len++] = lexer->lookahead;
              lexer->advance(lexer, false);
          }
          
          // Second, we check if it's a macro name
          if (tree_sitter_c_is_macro((const char *)&identifier, len)) {
              lexer->mark_end(lexer);
              lexer->result_symbol = MACRO_ANNOTATION;
              if (valid_symbols[MACRO_SPECIAL]) {
                  return true;
              }              
  
              while (iswspace(lexer->lookahead)) {
                  lexer->advance(lexer, false);
              }

              const int32_t c = lexer->lookahead;

              if (valid_symbols[MACRO_CALL] && c == '(') {
                  lexer->result_symbol = MACRO_CALL;
                  return true;
              }
              
              if ((c >= 'a' && c <= 'z') ||
                  (c >= 'A' && c <= 'Z') ||
                  (c >= '0' && c <= '9') ||
                  (c == '"' || c == '\'') || c == '{') {
                  return true;
              }
          }
      }
      
      return false;
  }
};

}

extern "C" {

void *tree_sitter_c_external_scanner_create() {
  return new Scanner();
}

bool tree_sitter_c_external_scanner_scan(void *payload, TSLexer *lexer,
                                            const bool *valid_symbols) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  return scanner->scan(lexer, valid_symbols);
}

unsigned tree_sitter_c_external_scanner_serialize(void *payload, char *buffer) {
  return 0;
}

void tree_sitter_c_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
}

void tree_sitter_c_external_scanner_destroy(void *payload) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  delete scanner;
}

}
