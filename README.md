# VL Interpreter
Interpreter for the VL language

WIP: CURRENTLY ONLY PARSES BUT DOES NOT INTERPRET

usage:

- vl filename.vl

or

- cargo run filename.vl

# VL specification

Commands:

Manipulate \<value>:

`p` Puts \<value> to stdout

`y` Assigns stdin to \<value>

`i` Enter insert mode, insert characters following i up until unescaped ('\') ';' into \<value>

`;` (insert mode only) Escape insert mode setting \<value>

`\` (insert mode only) Treat next ; as normal part of string

`a` Increment \<value> by 1 if Integer or Character type

`x` Decrement \<value> by 1 if Integer or Character type

Manipulate \<pointer>

`'` Set \<pointer> to character following ' and \<index> to 0

`\` Set \<pointer> to character following \` and \<index> to \<value>

`]` Increase \<pointer> to next mark (a -> b -> c etc)

`[` Decrease \<pointer> to previous mark

`}` Increase \<index>

`{` Decrease \<index>

Program Flow

`j` Jump \<int> lines down

`k` Jump \<int> lines up

`f` Jump to \<int>th next instance of character following f on same line

`F` Jump to \<int>th previous instance of character following F on same line

`?` Do following jump only if \<value> == \<int>

`!` Do following jump only if \<value> != \<int>

`v` Copy \<value> to \<int>

`.` Repeat last non-move command or group of commands

`(` Begin group

`)` End group

`/` Comment, anything between / pairs is ignored, must always be in pairs

Groups are performed \<int> times.

\<int> can be set directly by an integer i.e.

`10(ap)`

This sets \<int> to 10 then performs the functions ap ten times, resulting in the numbers 1 to 10 being outputted.

\<int> is 1 by default and reset to 1 after each command


# VL structure

Programs can access 26 'marks', named a to z by setting the pointer i.e. `'a`

Each mark is an array which can hold Strings, Integers, or Characters

The \<value> register always points to the \<pointer> \<index> value.

For example if \<pointer> = b and \<index> = 3 \<value> will be whatever value is stored at b[3]

`'b}}}p`

This is one way of outputting b[3].

Values are 0 by default and \<pointer>\<index> points to a[0] by default
