extern crate x11;
extern crate getopts;

use x11::xlib::*;
use std::os::raw::c_uchar;
use std::ptr::{null,null_mut};

use std::{env, mem, u64};
use std::str::FromStr;
use std::time::{Duration,Instant};

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main () {
  let args: Vec<String> = env::args().collect();

  let mut opts = Options::new();
  opts.optopt("c", "cooldown", "Sets cooldown period after mouse movements", "MILLIS");
  opts.optflag("v", "verbose", "Verbose Output");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(&args[0], opts);
    return;
  }

  let verbose = matches.opt_present("v");
  macro_rules! vp {
    ( $( $x:expr ),* ) => {
      if verbose { println!( $( $x, )* ); }
    }
  }

  let cooldown = matches.opt_str("c")
    .and_then(|s| u64::from_str(&s).ok())
    .unwrap_or(50);
  println!("Using a cooldown period of {}ms", cooldown);
  let cooldown = Duration::from_millis(cooldown);

  unsafe {
    let display = XOpenDisplay(null());
    if display == null_mut() {
      panic!("can't open display");
    }

    let root_window = XRootWindow(display, 0);
    vp!("root window id: {:#x}", root_window);

    {
      use x11::xinput2::*;
      let mut mask = XIEventMask::default();
      let mut mask1 = [0; 4];
      XISetMask(&mut mask1, XI_RawMotion);
      mask.deviceid = XIAllMasterDevices;
      mask.mask_len = mem::size_of::<[c_uchar; 4]>() as i32;
      mask.mask = mem::transmute::<&[u8; 4], *mut u8>(&mask1);
      XISelectEvents(display, root_window, &mut mask as *mut XIEventMask, 1);
      XFlush(display);
    };

    XSelectInput(display, root_window, FocusChangeMask);

    let mut event: XEvent = std::mem::uninitialized();
    let mut last_movement = Instant::now();

    loop {
      XNextEvent(display, &mut event);
      vp!("event type: {:?}", event.get_type());

      if event.get_type() == FocusIn {
        if (Instant::now() - last_movement) > cooldown {
          // Sleep to give the window manager time to change the focus
          std::thread::sleep(Duration::from_millis(20));

          // Get focused Window
          let mut window = mem::uninitialized();
          let mut ret = 0;
          XGetInputFocus(display, &mut window, &mut ret);
          vp!("Focused window: {:#x}", window);

          if window == root_window {
            vp!("Focused root window. Ignoring.");
          } else {
            // Move Focus
            move_focus(display, window, verbose);
          }
        }
      } else if event.get_type() == GenericEvent {
        last_movement = Instant::now();
      }
    }
  }
}

unsafe fn move_focus(display: *mut Display, window: Window, verbose: bool) {
  let mut attrs: XWindowAttributes = std::mem::uninitialized();
  if XGetWindowAttributes(display, window, &mut attrs) != 1 {
    panic!("Failed to get window attributes");
  }

  if verbose {
    println!("Focusing [pos: {}x{}, size: {}x{}]", attrs.x, attrs.y, attrs.width, attrs.height);
  }

  XWarpPointer(display, 0, window,
               0, 0,
               0, 0,
               attrs.width/2, attrs.height/2);
}
