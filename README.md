# Regex automata toolkit

A lightweight crate implementing primitives for producing and manipulating [Regex](https://en.wikipedia.org/wiki/Regular_expression), [NFA](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton), and [DFA](https://en.wikipedia.org/wiki/Deterministic_finite_automaton).

This crate can be used to execute DFAs on uncommon data structures or to build tools for low-level regex manipulation.

## Features

- `regex_syntax::Hir` -> `NFA` ([Thompson's construction](https://en.wikipedia.org/wiki/Thompson%27s_construction))
- `NFA` -> `DFA` ([Brzozowski's algorithm](https://en.wikipedia.org/wiki/DFA_minimization#Brzozowski's_algorithm))
- `DFA` -> `regex_syntax::Hir`
- Minimal DFA execution routine

Regex parsing is not reimplemented; instead, the standard crate [regex-syntax](https://docs.rs/regex-syntax/latest/regex_syntax/) is used.
