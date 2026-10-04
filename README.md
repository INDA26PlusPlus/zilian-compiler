# zilian-compiler
This is a custom compiler made in rust with the goal of being able to run the fibonacci sequence.

## Example code

The compiler complies functions and expressions written in LaTeX-esq syntax.

```rust

// Begin a function called  with a start argument and end argument
\begin{function; fibonacci}{k \inn \mathbb{N} -> \mathbb{N}}

// Declare starting values
$ n_{0} = 0 $
$ n_{1} = 1 $
// Declare a placeholder
$ n_{p} = 0 $

// If it's the zero-eth number
\begin{if}{k == 0}
// Return number to the function
\return{n_{0}}
\end{if}

// If it's the first number
\begin{if}{k == 1}
// Return number
\return{n_{1}}
\end{if}

// Begin a loop (i must be \mathbb{N} by default since we can't run a loop an non positive, irrational amount of times) 

\begin{loop}{k}

// Will run if k > 1 (since we past the if-statements)

$ n_{p} = n_{0} $           // Set placeholder to i-2
$ n_{0} = n_{1} $           // Set i-2 to i-1
$ n_{1} = n_{1} + n_{p} $   // Set i-1 to i

\end{recursion}

// Return number to the function
\return{n_{1}}
\end{loop}

\end{function}

$ write{fibonacci{5}} $
```

## Syntax
The programming language is based of LaTeX code and as such a few words and phrases show up multiple times and represent a certain thing.

## Keywords
The compiler will handle certain keywords/tokens as follows:
| Keyword     | Description                                        |
|-------------|----------------------------------------------------|
| \$...\$     | The \$ wraps an expression                         |
| \begin      | Beginning of a method or function                  |
| \end        | End of a method or function                        |
| \return     | Something to be returned to a function             |
| \inn        | Similar to ":" in rust, set argument types         |
| \mathbb     | Instead of using f64, or u8 we use number-sets     |

## Other Phrases
Some other notable phrases that we can use include the following:
| Phrase      | Description                                        |
|-------------|----------------------------------------------------|
| if          | And if will run only if whats in the {} is true    |
| loop        | Will loop until the condition in the {} is meet    |
| function    | Generates a function with a name and parameters    |
| write       | Outputs something in terminal                      |

## Standard operators
The compiler includes support for basic operations such as:
| Syntax      | Description                                        |
|-------------|----------------------------------------------------|
| a = b       | Sets a to equal the value of b                     |
| a + b       | Adds value of b to a                               |
| a == b      | Will see if a = b                                  |

