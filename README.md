wip. Ignore for the moment, I'll work on this and see where it goes.

test with `cargo test` from project root.


 -----------------------------------------------------------------------------


 - [x] plan project
     - [x] workflow document
     - [x] vague architecture
     - [x] come up with name

 - [x] figure out how structure works for cargo
     - [x] make repo

 - [x] set value to temp-lock via a tag macro

 - [x] read consts from toml
     - [x] tests, find out how test environment works

 - [x] live-refresh
     - [x] const -> function switch
     - [x] function lookup from file
         - [x] cache w/ filesystem based invalidation
     - [x] release-build replace with consts
     - [x] tests (rework first batch into single write)
     - [ ] tests (bevy systems, edit file)

 - [x] auto-write consts to toml
     - [ ] auto-gen non-existent file
     - [ ] tests

 - [ ] increase support for all primitives, enums, strings.
     - [ ] tests

 - [ ] local webpage to edit consts
     - [ ] macro-parameters to set input types (min, max, slider, etc)
     - [ ] css

 - [ ] tagged refresh systems

 - [ ] record video highlighting basic workflow

 - [ ] write readme

 - [ ] write documentation

 - [ ] publish crate

