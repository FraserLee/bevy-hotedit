wip. Ignore for the moment, I'll work on this and see where it goes.

test with `cargo test -- --test-threads=1` from project root.


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
     - [x] tests (bevy systems, edit file)

 - [x] auto-write consts to toml
     - [x] auto-gen non-existent file

 - [x] rewrite - pure serialization (de-serialize to retrieve value both in
                 debug and release) without any other weird formats or hacky
                 representations

 - [x] local webpage to edit consts
     - [x] serve webpage from thread
     - [x] auto-open on startup in browser
     - [x] basic HTML
     - [x] gen html by items
     - [x] live-set items on forum response
     - [x] write toml on forum response
     - [ ] macro-parameters to set input types (min, max, slider, etc)
     - [ ] group stuff in-order, sections by adj lines
     - [x] css
     - [ ] templates load not affected by run-directory
     - [ ] disable rocket output
     - [x] html start-bit only without any hot values
     - [x] initial html-load from file, write to file after.

 - [ ] tagged refresh systems

 - [ ] support enums
     - [ ] tests

 - [ ] write readme

 - [ ] record video highlighting basic workflow

 - [ ] write documentation

 - [ ] publish crate

 - [ ] mouse paralax code snippet behind (juice)


