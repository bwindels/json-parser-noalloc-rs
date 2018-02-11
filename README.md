Library for parsing JSON mostly conforming to [RFC 7159](https://tools.ietf.org/html/rfc7159) (see [exceptions](#exceptions)) in Rust without allocating any dynamic memory.

# Roadmap

## Tokenization
- tests for simple example
- tests for skipping \" in string literal
- tests for illegal numeric literals
- implement numeric literals completely
- implement error handling
- tests for error handling

## Parsing
- decode strings + tests (and add support to tokenizer for already decoded strings)
	- replace " delimiters by NUL character (and pad end with extra NULs)
- decode numbers + tests
- decode objects to structs with field name => field ref match
	- how to initialize struct? mem::uninitialized()? Option::None for every field?
	- how to be certain struct has been fully initialized? Count different offsets?
- decode properties
	- decode array
	- cast numbers to struct type
	- decode Option::None to null	

# Exceptions
- string literals that contain the NUL character (0x00) are not accepted by the tokenizer. Because the input buffer is used to decode strings into, the NUL character is used to delimit already decoded strings when parsing several times.