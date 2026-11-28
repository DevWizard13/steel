# Steel Programming Language

Steel is a programming language, as you can probably already tell.

## Goal

The goal of steel is maximum performance while maintaining saftey,
reliability, and ease of use to a reasonable extent. We achive this
goal through massively parralel processing by default. For this
reason, steel will likely not feel like many other programming
languages you used in the past.

Currently, our next goal is to have a parser that can succesfully
parse all files in `./examples`.

## Status

As mentioned above, the steel compiler is currently a half-functional
parser that can only parse 2 of 6 of the files serving as a basic
reference for the language. As mentioned, these files are in `./examples`.
These are the reference that the language will be built off.

Long-term ideas of mine include:
- object oriented features that work on features
  similar to the principles of Bevy's ECS, where an object is a group of
  classes, groups of classes are assigned methods to them, and any object
  with a certain group of objects can have the associated methods called
  on it
- Dependant types and built-in formal verification (maybe)
- Runtime with distributed processing built in

## Learn

Learning steel is probably a horrible idea, given the stage of development.
If you wanna learn a *real* high-performance language, then you should probably
look at rust. But say you do wanna learn steel. Sadly, there isn't much
documentation yet. But there is a lot of stuff in `./examples`, which is
there for learning in addition to reference for parser creation.

If you do decide to read it, start with `cat.steel`. Then read the others
in whatever order. Read the comments.

### Compiler

The compiler here is a little unusual. We trade speed for flexibility.
Instead of running a single monolithic compiler, we have a seperate
parser, type checker, compiler, and interpreter. This is to make it easier
to add various tools. Currently, only the parser and interpreter exist.

To run the parser and write the AST to a file (in RON format, which is
the universal language of the compiler ecosystem), you'd run `mine file 
| save ast.ron` (or `mine > ast.ron` if you use a POSIX shell). The names of all
these programs are steel-themed, so the parser is mine and the runtime is
dagger. The checker will be smelt and the compiler forge.

To give an overview of most commands you may want to run (written for
nushell, you may have to adjust based on your shell):

Parse and run directly:
```
mine file | dagger
```

Parse and save to a file:
```
mine | save ast.ron
```

Run from a file containing an AST:
```
cat ast.ron | dagger
```

## Editor

Currently, no editors have LSP support or syntax highlighting. This isn't
exactly a priority either. I recommend having an editor with support for
ligatures (the thing that makes it so that -> appears as an arrow rather than
a - and a >, or != appear as a not equals sign rather than a ! and a =).

Zed does this, and Zed is also the best editor I've ever used. It's like if
the people who made VS Code actually cared, used high-performance tools, and
actually considered people who like vim keybindings. I use and recommend it. IMO,
it's for you regardless of if you're a power user or just a regular programmer.

## Style Guides

Since I'm the one running the project, I made the style guide a collection
of all my hottest takes and wildest ideas. Be warned, it's very unusual.
Basically, it's as follows:

- snake_case for variable and system names
- Try to make all variables the same length for alignment
- Use spaces as padding to keep everything aligned
- Use tabs as indent so it displays well on all screen sizes

So good code will look as follows:

```
first = tru
secnd = fal
in |
	"hi" => "hello" | out
	"by" => "bye"   | out
```

and bad code would be:

```
first = tru
second = fal
in |
    "hi" => "hello" | out
    "bye" => "bye" | out
```

One could argue that the second is more readable to
non-programmers than the first, and they'd be correct!
But guess what: we're all programmers here! The reason
I picked this is because the second is *much* easier to
edit with a column editor like Ctrl+V in (n)vim or Zed.
Say you wanted to change all instances of out to, say,
print or something (maybe you wrote a custom function).
Properly following the style guides makes that much easier.

I picked tabs because they can be adjusted to display
in a way that's best for the window size rather than
whatever one person's editor window size is. In multi-gig
projects, tabs can also make a noticable difference in
project size. They're just better overall.

It'll feel natural once you get used to it.

## Copyright

Copyright (c) 2026-present DevWizard and contributers

This is free software, provided under GPL v3 or a later
version of the GPL licence. See LICENCE for details. Some
sections of code may be provided under a different licence.
See those individually.
