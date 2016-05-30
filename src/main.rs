extern crate x11;
extern crate libc;

use x11::xlib::*;
use std::os::raw::{c_uchar};
use std::ptr::{null,null_mut};

use std::{mem};
use std::time::{Duration,Instant};

fn main () {
  
  unsafe {
    let display = XOpenDisplay(null());
    if display == null_mut() {
      panic!("can't open display");
    }

    let root_window = XRootWindow(display, 0);

    {
      use x11::xinput2::*;
      let mut mask = XIEventMask::default();
      let mut mask1: [u8; 4] = [0,0,0,0];
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
      if event.get_type() == FocusIn {
        if (Instant::now() - last_movement) > Duration::from_millis(50) {
          // Sleep to give the window manager time to change the focus
          std::thread::sleep(Duration::from_millis(20));

          let mut window = mem::uninitialized();
          let mut ret = 0;
          XGetInputFocus(display, &mut window, &mut ret);
          
          let mut attrs: XWindowAttributes = std::mem::uninitialized();
          if XGetWindowAttributes(display, window, &mut attrs) != 1 {
            panic!("Failed to get window attributes")
          }

          XWarpPointer(display, 0, window,
                       0, 0,
                       0, 0,
                       attrs.width/2, attrs.height/2);

        }
      } else if event.get_type() == GenericEvent {
        last_movement = Instant::now();
      }
    }
  }
}
