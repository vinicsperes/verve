# Demonstrates a full run finishing on a new personal best, with the confetti
# celebration. Render with:  vhs demo/record.tape
# Run `cargo build --release` first so the binary below exists.

Output demo/record.gif

Set Shell "bash"
Set FontSize 22
Set Width 1200
Set Height 640
Set Padding 24
Set TypingSpeed 60ms

Hide
Type "./target/release/verve --text 'speed comes from calm and steady hands'"
Enter
Sleep 1s
Show

Sleep 700ms
Type@60ms "speed comes from calm and steady hands"
Sleep 4500ms
Escape
Sleep 500ms
