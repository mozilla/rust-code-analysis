module.exports = grammar({
  name: 'ccomment',

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
    ),

    nothing: $ => token(
      choice(
        /[^R"'\/#]+/,
        'R',
        '#',
      )
    ),

    preproc_continuation_line: $ => token(
      ///([^\\\n]|\\[^\n])*\\\r*\n/
      /.*\\\r?\n/
    ),

    preproc_line: $ => token(
      ///[^\n]*\n/
      /.*\n/
    ),
    
    define: $ => seq(
      /#[ \t]*define[ \t]+/,
      repeat($.preproc_continuation_line),
      $.preproc_line,
    ),
    
    string_literal: $ => seq(
      /"([^\\"]|\\(.|\n))*"/,
    ),
    
    char_literal: $ => seq(
      /'([^\\']|\\(.|\n))*'/,
    ),
    
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
