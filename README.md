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