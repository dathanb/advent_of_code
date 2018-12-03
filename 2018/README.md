Advent of Code 2018
===================

Instead of doing each day in a different language like I did last year,
this year I'd like to pick a single language and/or paradigm and stick
with it -- 50 projects is enough to start to feel comfortable with a
language or pattern/ paradigm, so this seems like a great opportunity.

But what to pick?

Last year, Elixir, Haskell, and Rust were the languages that left me
wanting more. So from a personal learning perspective, any of those
three would be good.

There are also some paradigms that I'd like to check out -- namely the actor
model and event-driven architectures. So I could also get some profesional
value out of setting up a dockerized Kafka instance for each day and doing
aggressive decoupling of workflows.

That kinda sounds found -- let's do that.

Should I combine two of these? e.g., Rust and Kafka? That's probably biting off
too much... Let's just stick with one. So Kafka and... what?  I guess Ruby
makes the most sense, since I'm most familiar with the language itself right
now, so coding in Ruby imposes the least cognitive load out of the languages I
could pick, and potentially leaves me the most mental bandwidth to devote to
Kafka-specific concerns.

So I guess let's do that -- dockerized Kafka and Ruby, and our workflows
for processing input will be decoupled, using Kafka messages to drive
processing.


