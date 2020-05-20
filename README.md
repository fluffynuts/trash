# trash
A small utility written in Rust to quickly trash files or dirs

# WARNING
This will trash whatever paths it's given, no hesitation, no feedback

# Why?
Because `rm -rf node_modules` on a windows box takes approximately forever. This doesn't.
Also, I wanted to learn some Rust, and I learn by doing -- so I've made something I can
use and gained a little Rust XP. Win-Win.

side-notes: 
- in `cmd` `del /q /f /r` also takes forever -- probably because of too much feedback
- in `pwsh` `del -r` takes about the same time, so that's nice (: 


# How to use?

`trash.exe {path1} ... {path2 ..}`

# What does it trash?

Files, dirs, globs that match stuff.

# ALL MAH SHIT IS GONE, YO!

I warned you above. Use at your own peril. This util has no safety measures. If
the file / folder _can_ be deleted, it _will_ be deleted.
