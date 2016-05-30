# Herbstmove - Mouse Follows Focus

Herbstmove is a small application that will wait for focus changes in
X11 and move the mouse to the center of the newly focused window.

To be less annoying it will ignore focus-changes that happen within a
predefined time after the last cursor movement.

It's written in Rust.

# Installation

    cargo build -- release

# Usage

    ./herbstmove
