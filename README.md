# idk
idk, it is just a toy programming language

## EBNF grammar for "idk"

This grammar was built with the help of [`EBNF Evaluator`](https://mdkrajnak.github.io/ebnftest/) tool.

```
identifier ::= letter, { characters } ?
number ::= [ '-' ], digit, { digit } ?

digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ?

letter ::= "A" | "B" | "C" | "D" | "E" | "F" | "G"
       | "H" | "I" | "J" | "K" | "L" | "M" | "N"
       | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
       | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
       | "c" | "d" | "e" | "f" | "g" | "h" | "i"
       | "j" | "k" | "l" | "m" | "n" | "o" | "p"
       | "q" | "r" | "s" | "t" | "u" | "v" | "w"
       | "x" | "y" | "z" ?

characters ::= number | letter | "_"
```

## License 
This project is licensed under the Apache License 2.0. See
[LICENSE](LICENSE).
