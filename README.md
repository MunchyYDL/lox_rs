# Crafting Interpreters

https://craftinginterpreters.com/

This awesome resource works through the implementation of an interpreter for the Lox programming language, but does so in Java, which is not my language of choice.

So this is an attempt to convert it to Rust while following along with the book.

> Disclaimer! 
I'm new to the concept of interpreters and programming language design, while also not an experienced Rust user. I won't take responsibility for any harm caused by reading this source! :)

I will try to keep the source close to the original design at first, while keeping the chapter numbers in my commit messages, to be easily cross-referenced by anyone interested.

I might change some things after the fact, to be more Rusty 🦀, but I will try to keep that in separate commits after the first straight conversion from Java is done.

Keep your fingers crossed, and that might actually be something I remember to do! :)


## Rust and parsers

Yes, I do know about the nom crate, but as I'm trying to learn how interpreters work, including all the stages of what goes into that, I don't want to use something prebuilt right now.


## My first impressions

You will notice that I have had to do minor tweaks and adjustments from the start, but I will try to find a good balance between staying close to the example-code in the book, getting it to compile/run in Rust and writing better/nicer Rust. So far I have tried to leave types as close to the original as possible, even if this means that I pass Strings around and Clone things without hesitation.

I do this for a couple of reasons:

1. I think it's easier to read for any Rust newcomers.
2. To show that this is a valid way to start writing Rust code.
3. When you develop something, you should always strive for functionality first. -"Make it work, make it nice, make it fast."

I had to bring in the Option type to handle the Token.literal field, it's typed as an Object in the example code, and was set to null in certain cases, but I still don't know how it looks when it's not null, so let's push on and wait for that to reveal itself.

Let's hope that I get back to this soon again! :)

## Chapter 4 - Scanner

When having completed the chapter, and implemented the code, there were quite a lot of small changes that I wanted to do. Refactoring code is a good way to learn both the subject matter, and the code itself, but the Scanner and Token classes are quite easy to understand at this point, and I'd like to keep it that way.

While there is more things we could change, I think I rather continue with the next chapter at this point, as I'm eager to start looking at Chapter 5 - Representing Code!

See you there! :)
