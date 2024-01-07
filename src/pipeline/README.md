# Pipeline

The pipeline contains multiple structs that form the "frontend" of the interpreter. In order:

## Scanner

The scanner struct processes a file into a series of tokens. Tokens describe the source code by grouping each character or groups of characters by a token type. Tokens are really basic, they hold:

- The type of token.
- The lexeme; the literal set of characters matched to identify this token.
- The literal; the casted value this token represents. If `'5'` is matched as a char, this should be a literal type of number holding the value of 5.
- The line; the line in which this token was identified.

The scanner struct has a few important fields:

- `source`; A vector of UTF-8 chars.
- `tokens`; This contains the tokens identified from the `source`. When the scanner struct is initialised, this is empty.
- `start`; The index of the char in the source of the current token that is being processed. For example, if the scanner matches a string `"` char, then `start` will be the index of that `"` char, then, when the string is closed because the scanner found the last `"`, we can use `start` to pull a slice out of the source (which is a vector of chars) to create the token's lexeme.
- `current`; This is the index of the current token that is being processed. It's updated more often than `start` as some characters in the source are consumed while a token is being processed, incrementing the `current` by 1 for each character that is consumed.

A high-level overview of the scanner process:

1. It starts with a loop that will only terminate when we have reached the end of the file.
2. If the end of the file has not yet been reached, it will read the current token using the `current` index with a `match` block in that will peek at the current character (or characters) in the source to pair it with a token.
3. If the `match` block finds a complex character (like a `'"'` or `'!'`) then it will do a little extra work process the character as it may represent something else (like a `!= or "string"`).

It is important to note the scanner code is quite imperative in that many methods may increment `current`. The method that scans a string will keep incrementing it until it hits the final `'"'` char, so that the next scanner loop iteration can pick up from where the string terminates.

## Parser

This struct takes a series of tokens and creates an "abstract syntax tree". This tree is formulated from the grammar of our interpreter.