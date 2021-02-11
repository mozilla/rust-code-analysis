const JS = require("./tree-sitter-javascript/grammar.js")

module.exports = grammar(JS, {
  name: 'mozjs',

  /*extras: ($, original) => original.concat([
    $.preproc,
  ]),*/

  rules: {
    preproc: $ => token(
      /#.*\n/,
    ),

    _statement: ($, original) => choice(
      original,
      $.preproc,
    ),
  }
});
