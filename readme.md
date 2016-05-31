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

# Bugs

- Moving the focus to an empty frame in Herbstluftwm isn't handled
  gracefully (will center the cursor on the center of the root
  window).

- Error Handling is missing for most calls to X11.
