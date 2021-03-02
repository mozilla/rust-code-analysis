#include <tree_sitter/parser.h>

#include "./tree_sitter_javascript_scanner.c"

void *tree_sitter_mozjs_external_scanner_create() { return NULL; }
void tree_sitter_mozjs_external_scanner_destroy(void *p) {}
void tree_sitter_mozjs_external_scanner_reset(void *p) {}
unsigned tree_sitter_mozjs_external_scanner_serialize(void *p, char *buffer) { return 0; }
void tree_sitter_mozjs_external_scanner_deserialize(void *p, const char *b, unsigned n) {}

bool tree_sitter_mozjs_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    return tree_sitter_javascript_external_scanner_mozjs_scan(payload, lexer, valid_symbols);
}
