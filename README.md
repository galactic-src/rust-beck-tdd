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

Making tests more "speaking" - making them more readable as sentences, with less "working out" of what it means.
Certainly valuable, though interesting that we've worked towards this rather than making our tests readable from the outset.
I'm not sure if this is a general "once you've done your equals" effect - it seems like it might not apply too obviously aside from that.
Certainly testing equality in a test, effectively duplicating an implementation of equals() would be laborious to write and read.

Consider whether utilities made in prod code could make tests more readable.

##### Chapter 5 - Franc-ly speaking

Copy/paste of test and code to create a new currency type.
Interesting chapter because it takes a test which feels out of reach, and chooses an approach to get us closer to that goal.
This is often the tough bit of coding for testable code that makes for small, reviewable PRs:
working out what bitesize bit you can implement next without it spiralling into a big change.
I wonder whether TDD might makes it easier or harder to get to a sensible PR point. It feels more exploratory and so less contained.

##### Chapter 6 - Equality for All, Redux

Takes the common equals code into a superclass. We don't have common equals code, so there's nothing to do.

##### Chapter 7 - Apples and Oranges

Requires that equals take class into account. In a way this is a little like a two-part triangulation to get equals working.
Again, we get this behaviour for free by using separate structs - we can simply add a test.

##### Chapter 8 - Makin' Objects

Unifies return type on Dollar/Franc to Money, makes Money abstract and types as Money in tests. This is similar to the approach I took in maaking factory methods. I'm not sure how you justify changing the test code this way from a TDD perspective - is it removing duplicate code?

##### Chapter 9 - Times We're Livin' In

Introduces Strings as a way to distinguish Money instances, in a similar way to my enum. This lets us delegate both constructors to super constructor as they are now identical.

##### Chapter 10 - Interesting Times

Permits toString() implementation without test on grounds we would immediately see result. Money.equals() is updated to take the currency string into account.

##### Chapter 11 - The Root Of All Evil

Eliminates the two subclasses leaving only money. So he has demonstrated that it is possible to migrate in incremental steps from two separate classes to a single unified class.

##### Chapter 12 - Addition, Finally

I wouldn't have gone with Bank, because it feels like invoking an entity that isn't really there in our domain.
However, maybe it's a good choice, because it's a familiar object with a bit of personality. Might make the world easier to understand.

Quote: "I try to keep the objects at the heart as ignorant of the rest of the world as possible, so they stay flexible as long as possible (and remain easy to test, and reuse, and understand).