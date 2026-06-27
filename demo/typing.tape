# Demonstrates verve in action: live coloring, an error and correction, the
# blinking caret and live WPM. Render with:  vhs demo/typing.tape
# Run `cargo build --release` first so the binary below exists.

Output demo/typing.gif

Set Shell "bash"
Set FontSize 22
Set Width 1200
Set Height 640
Set Padding 24
Set TypingSpeed 75ms

Hide
Type "./target/release/verve --text 'the quick brown fox jumps over the lazy dog and runs away'"
Enter
Sleep 1s
Show

Sleep 700ms
Type@75ms "the quick brown fox "
Sleep 200ms
Type@75ms "jukps"
Sleep 500ms
Backspace 3
Type@75ms "mps "
Type@75ms "over the lazy dog"
Sleep 1500ms
Escape
Sleep 600ms
