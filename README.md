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

##### Chapter 23 - How Suite It Is

Smart progression here - left it open ended for us to actually implement what he's been teaching us. Much nicer to end up with the suite having responsibility for the set of tests to be run.

##### Chapter 24 - xUnit Retrospective

Very brief commentary more about xUnit and the benefits of implementing it as an exercise than comments about TDD.

## Part 3 - Patterns For Test-Driven Development

##### Chapter 25 - Test-Driven Development Patterns

Relationship between testing, confidence in code or likelihood of bugs, and feedback via stress.
If you don't do `test isolation`, you get bitten by sooner or later.
The notion of a `test list` is new to me from this book, and something I plan to try.

`Test first` I have always struggled with. It certainly seems more honest and perhaps you end up with more robust tests.
But it also feels slower to implement to me. That's probably my own biases though - the time saved is probably more than swallowed by figuring out what tests I need to write after the fact.
`Assert first` is interesting: writing tests by first writing the assert, then getting hold of the data for it; another one to try out.

Keeping each constant for a single use in a particular test is something I suppose I already do - and I err towards named constants too. I think this helps document the intended behaviour better.
I certainly agree that pre-calculated exepected values are harder to understand (as well as update) and I prefer a sum that shows where the expected value comes from (`evident data`).

##### Chapter 26 - Red Bar Patterns

Advice:
- when looking for the next test from the list, pick the first test that you are confident you can implement, but you will learn something about the system you are implementing.
- start with a do-nothing test
- coaching to introduce TDD. Avoid evangelising or insisting, instead start asking for explanations of code in terms of tests.
- `learning tests` of 3rd party interfaces you plan to use. Not sure whether better running prod code for this, to avoid being left with a unit test of the 3rd party. I would tend to incorporate this kind of test into my e2e tests. It can otherwise be hard to know how extensively to test a 3rd party lib.
- writing a test when fixing a regression: generally I try to aim low in the testing pyramid here, but it's hard sometimes. And sometimes you can make a test that will prevent the issue in future, but doesn't tackle the root cause. So this is not always simple.

Curious that advice around taking breaks and prioritising chairs in equipment budgets ended up in this chapter!

##### Chapter 27 - Testing Patterns

_Child Test_: advises scrapping a high-level test in favour of red-green-refactor cycle.
Generally I agree, but try to avoid writing the large test in the first place.
By the time you are having to split up the work on a branch, you are often heading for a large PR.

_Mock Object_: Absolutely, but mocked dependencies do not test the interaction boundary between classes.
They are great for verifying granular behaviour, but it is easy for those tests to pass and still have a bug.
Fewer, more sociable tests need to cover off key features, especially in a duck-typed language like Python.
I have used the notion of a test suite applied to both mock and production objects for API testing, with some success, but a lot of boilerplate.

_Self Shunt_: I'm quite cautious about this - I like my tests to have minimal logic to them.
Ideally I'd use a spy object or mock method object to capture this kind of information, relying on battle-hardened test suite tools.
In testing tools for many languages (including strictly-typed, there are now clever objects that can be generated on the fly to implement a mock version of a class.

_Log String_: This feels at risk of needing updating and thinking about. I think method spies do this job better.

_Crash Test Dummy_: Yes, absolutely worth testing exception handling, throwing exceptions with mock objects if required.

_Broken Test_: I quite like this - I would often leave my laptop with a focused notepad open in the middle, with a sentence summarising what I was going to do next.

_Clean Check-in_: I think throwing out your work because of a failing integration test sounds counter-productive.
However, that might be because I haven't tried it! Incentivising regular check-ins is fine, but in a world with branches, CI and PR review, not sure this is necessary/helpful.


##### Chapter 28 - Green Bar Patterns

_Fake It ('Til You Make It)_: This tends to feel like one of the less efficient bits of TDD advice, to me.
If you think about it in terms of the test suite being about as important as the production code,
you can see this as a way to build/improve your test suite.
I do broadly agree with starting with the simple case and building from there. I tend to keep the future features on a card rather than a test list though.

_Triangulate_: I prefer this, generally, to faking it. And I like having multiple examples anyway, whether they are in a single or multiple tests.
I've definitely been bitten by not testing the negative case of a conditional, for example.

_Obvious Implementation_: This feels like my go-to but I'm sure that's partly confirmation bias.
I usually have a clear idea what I'm going to write, but then I probably jump too quickly towards a less good implementation too.
The red-green advice, to monitor how often you are failing tests with your "obvious" implementation is sound.

_One to Many_: This is an example where it feels "obvious" to me that we'll need the collection, so I jump to it.
Often I will split responsibility at the method level here though, so it works out, especially when I'm programming more imperatively.
In a functional context, I think I would tend to go straight to the collection.


##### Chapter 29 - xUnit Patterns

_Assertion_: Agree that testing equality/equivalence is preferable to asserting inequality.
I don't necessarily agree that everything should be tested using public-only methods.
You end up making things public just _for_ testing (eww) and some classes have a single do-everything public entry-point, which makes for less granular tests.
I've seen people introducing parameters to their production code solely used for testing too - which I would generally avoid as much as possible. 

_Fixture_: I prefer to make factory() utilities that generate test objects than introduce hierarchy to my test classes.
Like all test code I try to avoid complex logic. As such, I try not to have chains of method calls to setup data.
I quite like builder objects to set up data in a readable way. Partial<> argument objects to configure fixtures are quite neat in TypeScript too.

_External Fixture_: Yes, teardown is helpful. `pytest` also has a method `addCleanup()` to dynamically add a function call to happen after teardown.
Good for managing resources that might be created during the test, rather than having to conditionally test for their presence.

_Test Method_: Consistency is generally good for readability.
I don't feel strongly about whether "test" should appear at the start or end of test method names.
And I really love when languages give you scope to write test method names as sentences (like Kotlin).

_Exception Test_: Really important to have ways to allow the code under test to spit out Errors, capture the error object and assert on it.

_All Tests_: Having a mechanism to run everything in a consistent way (especially between local environment and CI)


##### Chapter 30 - Design Patterns

_Command_: These have become more mainstream as language features. Kotlin uses lambdas extensively, and futures/promises are commonplace.

_Value Object_: Immutable objects are fundamental to functional implementations, and some languages have moved towards immutable-by-default.
Kotlin `data classes` are very handy, for example.

_Null Object_: In a language like Java where nullability was not encodable, this kind of thing would make a huge difference to code readability and safety.
These are less essential now, with null guarded invocation etc.

_Template Method_: I don't actually use this often. There's too much nuance in the method contracts when another developer needs to write an implementation (or I come back to it in a year).

_Pluggable Object_: Fairly basic use of polymorphism to replace conditionals.

_Pluggable Selector_: Reflection has (rightfully) become a bit of a dirty topic. It makes it harder to leverage IDEs. I would inject a printer implementation in the constructor.

_Factory Method_: Absolutely, I use these all the time for test data setup.

_Imposter_: Seems to be just making a call on when to extract an interface.

_Composite_: In Java these tend to end up being rather painful, with methods wired through like React Prop Drilling.
Some languages like Kotlin now allow direct delegation of interface methods to a specified field on an object, which is great, but I don't use it often.
Most often it's when I need to decorate or modify an implementation from 3rd-party code.

_Collecting Parameter_: I have used these in the past, but not generally for testing.
They are helpful when you need to summarise several small results (e.g. trying a bulk job and reporting failures).

_Singleton_: He's advising against Singletons. They are boilerplate magnets in order to keep tests separable, requiring resetting in `setUp`s.
Most recently I've used one to set up a TestContainer to hold a database for integration testing, to be shared across the test suite.
They do have their uses but I'd tend to agree with the author.

##### Chapter 31 - Refactoring

_Reconcile Differences_: I'm not super-strict about this if I am confident in my test coverage.
If I do a big refactor and it breaks tests, and I mend the code until the tests pass, I'm usually pretty happy.
Sometimes I do a large refactor on a branch and if it doesn't go well, do it more slowly.

_Isolate Change_: Yes I totally agree with this. A preliminary part of a refactor is often to make the bit I want to change more accessible or malleable.
In fact often this change improves separation of responsibilities anyway and goes in as a separate PR.

_Migrate Data_: I don't know how valuable this is - it depends how many places need to change.
If I have sensible factory methods that really helps with making these changes - I rarely feel the need to make a data adaptor.
As for the example, I would usually jump to "many" as an "obvious" change, rather than have to do this juggling.

_Extract Method_: I love that IDEs mostly field this for us these days!

_Inline Method_: It's great when you have found some logic that you can get rid of, which simplifies a method so much it can disappear.
I actually found `Clean Code` kind of hard to stomach because of how scattered it advocated making logic.
The goal is almost always readability!

_Extract Interface_: Yeah, this is sensible. Often I'll make an interface in a strongly-typed language so I can make a test/mock implementation.
Good to wait until you need one though, rather than making them by default and cluttering things up.

_Move Method_: This feels pretty obvious as a process, and in terms of need where all the logic is managed by a field object.

_Method Object_: In my head, I don't see it so much as a method object as a class extraction, but yep.

_Add Parameter_: Straightforward.

_Method Parameter to Constructor Parameter_: This is closely tied to the _Method Object_ type change mentioned previously.
I feel more comfortable doing this is the object isn't mutated by the methods, remaining constant for the lifetime of the object.
I don't like it when the "entrypoint" to the object sets instance variables.

##### Chapter 32 - Mastering TDD

`"Write tests until fear is transformed into boredom"`: That's interesting, because fear is subjective, so different people will be comfortable with different coverage.
I think test coverage should be relatively consistent for a project, with a natural tension between writing more for confidence and fewer for expediency.
Test code needs maintaining, it's not a good idea to just keep writing them. Also for a system, tests tend to fall naturally into types of test.
E.g. anything web-facing could reasonably have a layer of tests checking that certain requests in order produce certain responses.
Part of PR should ensure that the project's test coverage expectations is being met. 

`"The tests are a canary in a coal mine revealing by their distress the presence of evil design vapors"`:
I don't know about this - I think you can write code to make it testable or not, and sure, if it's hard to test, the code should be refactored to make it more testable.
It's possible to write simple-to-read code without making it testable, and generally more concise.
But yeah, I think with the assumption that you want good regression coverage and documentation of intent, this is valid.
I've experienced all of these things.
I would add that some ways of writing tests lead to having to update a lot of tests to make a relatively simple code change.
Avoiding constants directly in tests aside from those that the test relies on, and using test factories for input data help to
make tests less fragile. Also avoiding testing order of methods called by code under test with specific arguments and rather trying to test intent helps.

_TDD and frameworks_: I've not got enough experience of TDD to really see how the end-product might differ from a designed output.

_How many tests_: I agree entirely that you think about test coverage based on severity of impact if something in the system goes wrong.
Any data type has some caveats to consider, and you can test each of those caveats, or every combination.
Ultimately for systems that _really_ matter, you can consider formal proofs, which take the caveats implicitly into account
and force you to explicitly recognise the assumptions you are making about inputs.

_Redundant tests_: I prefer to delete redundant tests to reduce the amount of code we are maintaining.

_Application Test Driven Development_: I guess this is the precursor to BDD. I haven't had much experience of it, and don't really want to.
I think it is hard for someone who isn't implementing a system to second-guess what a helpful unit to test is.

_Testing systems that weren't designed for testability_: Oh the horror. Basically yes, system-level tests especially for core paths.
Try to exercise mission-critical happy-path functionality in tests. Build lower-level test coverage towards the code you are working on.
Trace code invocation in the areas you are making changes and get a good idea of any global mutable state.

_emotional attachment to code_: Yeah I agree with this. Green field projects are a luxury because you don't have to deal with the state of the existing code.
You are free to grow in any direction without fear of implosion. Over time it's very easy to end up disenchanted with a code base and tests _really_ help.

_TDD rules by rote_: Yes I broadly agree. You can learn principles by word of mouth of what is good practice, or you can get burned along the way.
Generally it's harder to learn a lesson well by rote (using purely logical processes and memory)
and much easier to build good instincts off the back of a horrible experience wit ha code base. But it's worth a try doing it by rote in case it sticks.

_Extreme Programming_: Generally sensible advice, I think. Nothing too controversial.

_Darach's Challenge_: GUIs can be painful to to test if they are not designed with it in mind.
Integration tests are pretty important to check your code integrates with external systems the way you think it does.
However I don't think it's valuable to extensively test third-party code or generated code on a unit level - more integration/e2e.
