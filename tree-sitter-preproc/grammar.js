module.exports = grammar({
  name: 'preproc',

  externals: $ => [
    $.raw_string_literal,
  ],

  rules: {
    translation_unit: $ => repeat($._top_level_item),

    _top_level_item: $ => choice(
      $.string_literal,
      $.char_literal,
      $.raw_string_literal,
      $.comment,
      $.nothing,
      $.define,
      $.preproc_if,
      $.preproc_include,
      $.preproc_nothing,
      $.integer_literal,
    ),

    identifier: $ => /[a-zA-Z_]\w*/,

    nothing: $ => token(
      choice(
        /[^R"'\/#0-9]+/,
        'R',
        '#',
        '/',
        '\n',
      )
    ),

    preproc_continuation_line: $ => token.immediate(
      /.*\\\r?\n/
    ),

    preproc_line: $ => token.immediate(
      /.*/,
    ),

    preproc_include: $ => seq(
      /[ \t]*#[ \t]*include(_next)?[ \t]*/,
      choice(
        $.string_literal,
        seq(
          '<',
          $.path,
          '>',
        ),
        $.identifier,
      ),
    ),

    path: $ => token(
      /[^>]*/,
    ),

    define: $ => seq(
      /[ \t]*#[ \t]*define[ \t]+/,
      $.identifier,
      repeat($.preproc_continuation_line),
      $.preproc_line,
      '\n',
    ),

    preproc_if: $ => seq(
      /[ \t]*#[ \t]*(ifdef|ifndef|if).*\n/,
      repeat($._top_level_item),
      repeat($.preproc_elif),
      optional($.preproc_else),
      /[ \t]*#[ \t]*endif.*\n/,
    ),

    preproc_elif: $ => seq(
      /[ \t]*#[ \t]*elif.*\n/,
      repeat($._top_level_item),
    ),

    preproc_else: $ => seq(
      /[ \t]*#[ \t]*else.*\n/,
      repeat($._top_level_item),
    ),

    preproc_nothing: $ => seq(
      token.immediate(/[ \t]*#[ \t]*(error|pragma|undef|line)/),
      repeat($.preproc_continuation_line),
      $.preproc_line,
    ),

    string_literal: $ => seq(
      /"([^\\"]|\\(.|\n))*"/,
    ),

    char_literal: $ => seq(
      /'([^\\']|\\(.|\n))*'/,
    ),

    integer_literal: $ => token.immediate(
      /[0-9]+[0-9']*/
    ),

    // static constexpr uint8_t Global = 0b0'0010; ^^ pas compatible avec char

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: $ => token(choice(
      seq('//', /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
      '/'
    )
    )),
  },

});
