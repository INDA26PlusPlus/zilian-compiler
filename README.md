# zilian-compiler
This is a custom compiler made in rust with the goal of being able to run the fibonacci sequence.

## Example code

The compiler complies functions and expressions written in LaTeX-esq syntax.

```rust
// How many times to run the loop
// aka the fibonacci number you want
$ runTime = 0 $

// Set starting values
$ a = 0 $
$ b = 1 $

// Run through the fibonacci sequence a set number of times
\begin{loop}{runTime}

// Count up fibonacci numbers
$ temp = a$
$ a = b$
$ b = a + temp $

\end{loop}
```

## Syntax
The programming language is based of LaTeX code and as such a few words and phrases show up multiple times and represent a certain thing.

### Keywords
The compiler will handle certain keywords/tokens as follows:
| Keyword     | Description                                        |
|-------------|----------------------------------------------------|
| \$...\$     | The \$ wraps an expression                         |
| \begin      | Beginning of a method or function                  |
| \end        | End of a method or function                        |

### Other Phrases
Some other notable phrases that we can use include the following:
| Phrase      | Description                                        |
|-------------|----------------------------------------------------|
| loop        | Will loop until the condition in the {} is meet    |

### Standard operators
The compiler includes support for basic operations such as:
| Syntax      | Description                                        |
|-------------|----------------------------------------------------|
| a = b       | Sets a to equal the value of b                     |
| a + b       | Adds value of b to a                               |
| a == b      | Will see if a = b                                  |

