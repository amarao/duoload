# Current state and plans


* I got it working on my machine (that means, I get apkg file with all my words from my Deck). `[done]`.
* I want to cleanup code. There are multiple warnings I want to cleanup before doing anything else. `[plan]`
* I want to validate it works on someone's else machine and deck `[plan]`
* I want to support json and csv outputs. I found apkg a bit hard to validate, and multiple outputs will be of a great help. `[plan]`
* I want to support stdout in output. Trivial. `[plan]`
* I want to add a proper CI (there is plenty of tests to run) `[plan]`
* I want to provide releases for all major architectures `[plan]`. I never cross-build rust applications before, going to be fun.
* I want to provide a docker image (multi-arch) `[plan]`
* I want to write/generate documentation on how to use it. Extracting deck id is not the simplest task. `[plan]`. Or should I find a way to automate it?

# Duoload

Duoload is an utility to transfer a user's vocabulary data from the [Duocards](https://duocards.com/)
into the Anki flashcards local database.

This application is intended to be used only with user-created cards. Please, respect copyright and
don't download third party Duocards card stacks.

## Vibe coding

This utility was vibe coded using:

* Gemini 2.5 Pro
* Cloudie 4.0 sonnet
* Cursor using 'auto' mode

It also got some manual polishing (mostly in specs, PRDs and tests) and was completely reviewed by humans.

It also was tested to work with Duocards as per Jule 2025 with a personal database of 2k+ words.

