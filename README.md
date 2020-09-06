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

##### Chapter 13 - Make it

Carry out reducing a sum on the Bank object, using a Sum as a type of expression.
In Rust it makes more sense to just handle Sums for now and later create an Expression enum.

##### Chapter 14 - Change

The test at the start includes the add_rate method, which doesn't get implemented until the last page of the chapter.
Introduces a HashMap of pairs to values. This feels a little complex to me, and a shame Pairs need implementing.

##### Chapter 15 - Mixed Currencies

Feels a little like you need to debug your own code after putting in the dummy implementations, unless they are resolved fairly quickly.
Finding it hard to slow down enough! Also I suspect if you have experience working in the expressions domain you'll have a fair idea of how you want to implement it.

##### Chapter 16 - Abstraction, Finally

Spotting duplication between your code and test code requires you not to move on until you are satisfied you have removed duplication.
You need to be disciplined about how much you are working on at once, and avoid rabbit holes that leave a trail of dummy implementations.

##### Chapter 17 - Money Retrospective

Commentary on necessity of completeness of tests, more lax towards "edge". Also that code is never "done".
A feature can and should be "done" though, and from that point of view the code needs to be in an acceptable state.
I think we have tighter expectations of production-ready code than presented in the book.

Interesting that feels he discovered a new way of solving the problem - I find that I tend towards familiar solutions,
so this seems a legitimate benefit of TDD, albeit expensive a lot of the time.

I absolutely agree with the power of metaphor in guiding implementation. Ultimately people need to be able to understand, modify and maintain code.
Choosing intuitive metaphors is a powerful communication tool to minimise friction.

##### Final key points summary

- 3 approaches to making a test work: fake it, triangulation, obvious implementation
- Remove duplication between test and code as a way to drive the design
- Ability to control the gap between tests to increase traction when the road gets slippery and cruise faster when conditions are clear.


## Part 2 - The xUnit Example

##### Chapter 18 - First Steps to xUnit

A little fumbling around based on prints to get us to the point that running things actually "runs a test", which makes sense. Acknowledged that actually he spent 6 hours learning some Python and trying steps out to get to refine the stepwise iteration for the book.

I wonder how much, for a project like this, you need some idea of "good" unit testing features.

Book is Python2 but I'm using Python3 here.

##### Chapter 19 - Set The Table

Good to recognise that often you can prioritise the consumer API (as written in the test code) or the code under test and there is a clear preference here while building the code to prioritise the tests - at least over performance.

Added set_up functionality, including an (untested!) do-nothing implementation in the TestCase class.

##### Chapter 20 - Cleaning Up After

Where did 'result' parameter for TestCase.run come from? And what happened to WasRun.wasRun? I'm no longer confident that my implementation matches what is expected. I think ultimately we have a single test which ensures that the 3 methods are all called in the correct order.

##### Chapter 21 - Counting

This chapter felt more enjoyable - I had a decent sense of where we were going, and it felt like we were building more directly towards a solution. On the other hand, it was odd to finish up with a failing test.

##### Chapter 22 - Dealing With Failure

There was a sneaky bit at the end of the chapter where he suggests that setup failures be handled properly. Investigating this revealed that actually, using our original running mechanism, we are in fact silently catching failures anyway.

I introduced a method into my TestCaseTest which expects that the test runs without failures off the back of this - there might be a neater way.

Adding the setup call to our try block fixed the actual issue.
