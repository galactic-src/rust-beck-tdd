# rust-beck-tdd
Working through Kent Beck's TDD By Example for training with work.


## Part 1 - The Money Example

##### Chapter 1 - Multi-Currency Money
Interesting thought leap to consider placement of a calculated value in a test as duplicated code.
I'm sure this will apply to many other things too.

##### Chapter 2 - Degenerate Objects
2 strategies for quick green:
- fake it (return constant)
- obvious impl (directly type simple, clean implementation)

Mentions fluidly responding to how obvious the implementation is.
When hit an *unexpected* test failure, shifting to faking it, then refactoring to make the code cleaner.
Translating *aesthetic judgements* into tests.

Design objections (e.g. side effects) may form new test cases, or modify existing ones.

##### Chapter 3 - Equality for All

Triangulation approach: generalising code when we have 2 examples or more (at which point the general solution is also the simplest).
Recommended as a backup when unsure how to refactor. Makes *axes of variability* more explicit.

##### Chapter 4 - Privacy

