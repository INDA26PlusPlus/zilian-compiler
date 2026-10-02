# zilian-compiler
This is a custom compiler made in rust with the goal of being able to run the fibonacci sequence.
The compiler complies functions and expressions written in LaTeX-esq syntax.

```latex

# Begin a function with a start argument and end argument
\begin{function}{fibonacci_calculator}{k \inn \mathbb{N} -> \mathbb{N}}

# Declare starting values
$ n_{0} = 0 $
$ n_{1} = 1 $
# Declare a placeholder
$ n_{p} = 0 $

# Begin a loop (i must be \mathbb{N} by default since we can't run a loop an non positive, irrational amount of times) 

\begin{loop}{i = k}

# If it's the first two numbers
\begin{if}{k < 2}
# Return number
n_{i}
\end{if}

# Will run if k >= 2 (since we past the if-statement)

$ n_{p} = n_{0} $           # Set placeholder to i-2
$ n_{0} = n_{1} $           # Set i-2 to i-1
$ n_{1} = n_{1} + n_{p} $   # Set i-1 to i

\end{recursion}

# Return number
n_{1}

\end{function}
```

## Syntax
The types of `MoveError`s are as follows:
| Syntax    | Description                                        |
|-----------|----------------------------------------------------|
| \$...\$     | The \$ wraps an expression, if not it's a return |