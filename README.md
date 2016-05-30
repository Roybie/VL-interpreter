# VL Interpreter
Interpreter for the VL language

v.0.2alpha

### Requirements

Rust nightly

### Build/Run

Build with `cargo build --release`

Then run using vl binary in ./targets/release

e.g.

- vl sourcefile

- vl -s 'source string'

- vl -r  (repl mode)

In source string mode (-s) when a ' is wanted in the string you must replace it with: '\'' (thanks bash)

(repl mode can currently only evaluate single line statements at a time)

see examples folder for source code examples

## VL language

Commands:

#### IO

`w` _(loopable, repeatable)_ Puts \<value> to stdout

`W` _(loopable, repeatable)_ Puts \<value> to stdout new line version

`e` Assigns stdin to \<value>

#### Manipulate memory address pointers

`'` Set \<pointer> to character following ' and \<index> to 0

`` ` `` _(resets int)_ Set \<pointer> to character following \` and \<index> to \<int>

`]` _(loopable)_ Increase \<pointer> to next mark (a -> b -> c etc)

`[` _(loopable)_ Decrease \<pointer> to previous mark

`}` _(loopable)_ Increase \<index>

`{` _(loopable)_ Decrease \<index>

#### Manipulate registers/memory

`y` _(repeatable)_ Copies current selected memory value into \<value>

`Y` _(repeatable)_ Copies current selected memory value into \<int>

`i` _(repeatable)_ Enter insert mode, insert characters following i up until unescaped ('\') ';' into \<value> and currently selected memory

`;` _(insert mode only)_ Escape insert mode setting \<value>

`\` _(insert mode only)_ Treat next ; as normal part of string

`a` _(loopable, repeatable)_ Increment currently selected memory value by 1 if Integer or Character type and set \<value>

`x` _(loopable, repeatable)_ Decrement currently selected memory value by 1 if Integer or Character type and set \<value>

`v` Copy \<int> to \<value>

`V` Copy \<value> to \<int>

#### Arithmetic

`+` _(resets int)_ Set \<value> to \<value> + \<int>

`-` _(resets int)_ Set \<value> to \<value> - \<int>

`*` _(resets int)_ Set \<value> to \<value> * \<int>

`/` Set \<value> to \<value> / \<int> and set \<int> to \<value> % \<int>

Divide is unique in that it sets the internal \<int> register and the \<value> register

#### Program Flow

`^` Jump to beginning of current line

`j` _(loopable)_ Jump \<int> lines down

`k` _(loopable)_ Jump \<int> lines up

`f` _(loopable)_ Jump to \<int>th next instance of character following f

`F` _(loopable)_ Jump to \<int>th previous instance of character following F

`?` _(resets int)_ Do following jump only if \<value> == \<int>

`!` _(resets int)_ Do following jump only if \<value> != \<int>

`<` _(resets int)_ Do following jump only if \<value> > \<int>

`>` _(resets int)_ Do following jump only if \<value> < \<int>

`(` Begin group

`)` End group

`.` _(loopable)_ Repeat the previous _(repeatable)_ command

`$` Comment, anything between $ pairs is ignored, must always be in pairs

Groups are treated as isolated code segments, memory and registers carry through from and over to the outside code, but program flow cannot jump out from a group until the end.

Groups are _(loopable)_ and so performed \<int> times.

\<int> can be set directly by an integer i.e.

`10(aW)`

This sets \<int> to 10 then performs the following _(loopable)_ function ten times, in this case the group `(aW)` resulting in the numbers 1 to 10 being outputted.

\<int> is 1 by default and reset to 1 after _(loopable)_ and _(resets int)_  commands, preserved otherwise.


## VL structure

Programs can access 26 'marks', named a to z by setting the pointer i.e. `'a`

Each mark is an array which can hold Strings, Integers, or Characters

`'b}}}yw`

This is one way of outputting b[3].

Values are 0 by default and \<pointer>\<index> points to a[0] by default
