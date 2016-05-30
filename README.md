# VL Interpreter
Interpreter for the VL language

v.0.2ALPHA

Needs Rust installed to run atm.

Build with `cargo build --release`

Then run using binary in ./targets/release

usage:

- vl sourcefile

- vl -s 'source string'

- vl -r  (repl mode)

In source string mode (-s) when a ' is wanted in the string you must replace it with: '\'' (thanks bash)

(repl mode can only evaluate single line statements at a time)

see examples folder for source code examples

# VL specification

Commands:

Manipulate \<value>:

`w` Puts \<value> to stdout

`W` Puts \<value> to stdout new line version

`e` Assigns stdin to \<value>

`p` Puts \<value> into currently selected memory

`P` Puts \<int> into currently selected memory

`y` Copies current selected memory value into \<value>

`Y` Copies current selected memory value into \<int>

`i` Enter insert mode, insert characters following i up until unescaped ('\') ';' into \<value> and currently selected memory

`;` (insert mode only) Escape insert mode setting \<value>

`\` (insert mode only) Treat next ; as normal part of string

`a` Increment currently selected memory value by 1 if Integer or Character type and set \<value>

`x` Decrement currently selected memory value by 1 if Integer or Character type and set \<value>

Arithmetic

`+` Set \<value> to \<value> + \<int>

`-` Set \<value> to \<value> - \<int>

`*` Set \<value> to \<value> * \<int>

`/` Set \<value> to \<value> / \<int> and set \<int> to \<value> % \<int>

Divide is unique in that it sets the internal \<int> register and the \<value> register

Manipulate \<pointer>

`'` Set \<pointer> to character following ' and \<index> to 0

`` ` `` Set \<pointer> to character following \` and \<index> to \<int>

`]` Increase \<pointer> to next mark (a -> b -> c etc)

`[` Decrease \<pointer> to previous mark

`}` Increase \<index>

`{` Decrease \<index>

Program Flow

`^` Jump to beginning of current line

`j` Jump \<int> lines down

`k` Jump \<int> lines up

`f` *NOT YET IMPLEMENTED* Jump to \<int>th next instance of character following f on same line

`F` *NOT YET IMPLEMENTED* Jump to \<int>th previous instance of character following F on same line

`?` Do following jump only if \<value> == \<int>

`!` Do following jump only if \<value> != \<int>

`<` Do following jump only if \<value> > \<int>

`>` Do following jump only if \<value> < \<int>

`v` Copy \<int> to \<value>

`V` Copy \<value> to \<int>

`(` Begin group

`)` End group

`.` Repeat the previous repeatable command

`$` Comment, anything between $ pairs is ignored, must always be in pairs

Groups are performed \<int> times.

\<int> can be set directly by an integer i.e.

`10(ap)`

This sets \<int> to 10 then performs the functions ap ten times, resulting in the numbers 1 to 10 being outputted.

\<int> is 1 by default and reset to 1 after each command


# VL structure

Programs can access 26 'marks', named a to z by setting the pointer i.e. `'a`

Each mark is an array which can hold Strings, Integers, or Characters

`'b}}}yw`

This is one way of outputting b[3].

Values are 0 by default and \<pointer>\<index> points to a[0] by default
