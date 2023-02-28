// extern crate x11;
// extern crate pkg_config;

use x11::xlib::*;
use std::thread::sleep;
use std::time::Duration;
use std::mem::zeroed;

const NIL: *const i8 = 0 as *const i8;

pub unsafe fn main() {
    let display = XOpenDisplay(NIL);
    let black_color = XBlackPixel(display, XDefaultScreen(display));
    let white_color = XWhitePixel(display, XDefaultScreen(display));
    let window = XCreateSimpleWindow(
        display, XDefaultRootWindow(display),
        0, 0, 200, 100, 0, black_color, white_color);
    XSelectInput(display, window, StructureNotifyMask);
    XMapWindow(display, window);
    let graphical_context = XCreateGC(display, window, 0, NIL as *mut XGCValues );
    XSetForeground(display, graphical_context, white_color);
    loop {
        let mut event: XEvent;
        event = zeroed();
        XNextEvent(display, &mut event);
        match event.type_ {
            MapNotify => break,
            _ => ()
        }
    }
    XDrawLine(display, window, graphical_context, 10, 60, 180, 20);
    XFlush(display);
    sleep(Duration::from_secs(10));
}
