# Brainfuck Interpreter

[![Build](../../actions/workflows/build.yaml/badge.svg)](../../actions/workflows/build.yaml)

This is a simple Brainfuck interpreter implemented in Rust. Brainfuck is an esoteric programming language with a minimalistic set of commands. Despite its simplicity, it is Turing-complete, meaning it can compute any computable function.

## Basics

Brainfuck defines its entire language using just eight commands:

- `>`: Increment the memory pointer, moving it one block to the right.
- `<`: Decrement the memory pointer, moving it one block to the left.
- `+`: Increment the value stored at the memory pointer.
- `-`: Decrement the value stored at the memory pointer.
- `[`: If the value at the current block is zero, jump to the corresponding `]` command. Otherwise, continue execution.
- `]`: If the value at the current block is nonzero, jump back to the corresponding `[` command. Otherwise, continue execution.
- `,`: Input an ASCII character and store its value in the current block.
- `.`: Output the ASCII character represented by the value at the current block.


## Hello World

To output the famous "Hello World!" phrase in Brainfuck, you can use the following code:
```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

Here's a simplified version of the code with comments to explain its functionality:
```brainfuck
+++++ +++++             initialize counter (cell #0) to 10
[                       use loop to set the next four cells to 70/100/30/10
    > +++++ ++              add  7 to cell #1
    > +++++ +++++           add 10 to cell #2 
    > +++                   add  3 to cell #3
    > +                     add  1 to cell #4
    <<<< -                  decrement counter (cell #0)
]                   
> ++ .                  print 'H'
> + .                   print 'e'
+++++ ++ .              print 'l'
.                       print 'l'
+++ .                   print 'o'
> ++ .                  print ' '
<< +++++ +++++ +++++ .  print 'W'
> .                     print 'o'
+++ .                   print 'r'
----- - .               print 'l'
----- --- .             print 'd'
> + .                   print '!'
> .                     print '\n'
```

# References
- [Brainfuck](http://brainfuck.org/)
- Brainfuck [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)
- Basics of Brainfuck by roachhd [Github Gist](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
