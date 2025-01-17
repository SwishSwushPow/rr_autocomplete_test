# rr_autocomplete_test

This repo showcases the issue I have with RustRover and its code auto completion. When using `color_eyre` the code
autocomplete is filled with items coming from `owo_colors::OwoColorize` (e.g. `black()`). I was not able to remove these
methods from the autocomplete by adding them to the "exclude" section found in `Editor | General | Auto Import` (I've
tried a couple of different approaches to make sure I'm not missing anything).

```
owo_colors::OwoColorize
owo_colors::OwoColorize::*
owo_colors
owo_colors::*
```

I am not adding any `use` lines that import `owo_colors` directly, yet I am not able to remove these items from the auto
complete list.

As far as I understand many (if not all) of these functions come
from https://docs.rs/owo-colors/latest/owo_colors/trait.OwoColorize.html and are implemented with a blanket
implementation looking like `impl<D: Sized> OwoColorize for D`.

Please take a look at the `lib.rs` file where I've added a comment how to reproduce the issue.

I've also tried to exclude `owo_colors::OwoColorize::black` directly, but that wouldn't work either. Ideally there
is a way to ignore all functions coming from `OwoColorize` because it is not really practical to add each individual
entry manually.

# Update: Currently not possible (RustRover 2024.3.2)

After opening a ticket with Jetbrains in January 2025, I got an answer which roughly comes down to this: Since
`owo_colors` is not directly part of the project, but is basically a re-export from/dependency of `color-eyre`, it is
very difficult to fully ignore methods coming from this crate. Implementation would be complicated, and it could affect
code completion in unforeseen ways. This is why, at the moment, Jetbrains will refrain from implementing functionality
that would allow us to do something like I've described above.

I have suggested to mabye update their docs
(https://www.jetbrains.com/help/rust/rust-autocomplete-code.html#disable-auto-import) to state more clearly that only
items and modules from crates directly part of the `Cargo.toml` file can be excluded. 