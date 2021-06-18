# winit-fullscreen

This is a simple crate that manages toggling of fullscreen in a winit-based
application.  Given a borrowed immutable reference to your
`winit::window::Window` instance, you can call:

```rust
window.toggle_fullscreen();
```

If your window is not fullscreen, it will be now, otherwise the reverse will be
true.

# Motivation

I found the fullscreen interface difficult to remember, the documentation
lacking and there are some complexities involving different platforms (some do
not support full exclusive mode, for example).  So I decided to wrap up the
functionality under a simple, argumentless, single function.

I am happy to contribute this back into the Winit crate if there is interest.

# Issues and ideas

Please post any issues or ideas on the GitHub site on
https://github.com/Cthutu/winit-fullscreen.

# Credits

Written by Matt Davies, copyright ©2021, all rights reserved.