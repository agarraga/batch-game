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
        0, 0, 480, 320, 0, black_color, white_color);
    XSelectInput(display, window, StructureNotifyMask);
    XMapWindow(display, window);
    let graphical_context = XCreateGC(display, window, 0, NIL as *mut XGCValues );
    XSetForeground(display, graphical_context, black_color);
    loop {
        let mut event: XEvent = zeroed();
        XNextEvent(display, &mut event);
        #[allow(non_upper_case_globals)]
        match event.type_ {
            MapNotify => break, 
            _ => ()
        }
    }

    let draw_rectangle = move |a: [i32; 2], b: [i32; 2], c: [i32; 2], d: [i32; 2]| {
        XDrawLine(display, window, graphical_context, a[0], a[1], b[0], b[1]);
        XDrawLine(display, window, graphical_context, b[0], b[1], d[0], d[1]);
        XDrawLine(display, window, graphical_context, d[0], d[1], c[0], c[1]);
        XDrawLine(display, window, graphical_context, c[0], c[1], a[0], a[1]);
    };

    let mut a:  [i32; 2];
    let mut b:  [i32; 2];
    let mut c:  [i32; 2];
    let mut d:  [i32; 2];
    a = [12, 12];
    b = [48, 12];
    c = [12, 48];
    d = [48, 48];
    loop {

        let mut key_event_code: u32 = 0;
        XSelectInput(display, window, KeyPressMask | KeyReleaseMask);
        while XPending(display) != 0 {
            let mut key_event: XEvent = zeroed();
            XNextEvent(display, &mut key_event);
            #[allow(non_upper_case_globals)]
            match key_event.type_ {
                KeyPress => { key_event_code = key_event.key.keycode; },
                KeyRelease => { key_event_code = key_event.key.keycode; },
                _ => ()
            }
        }
        match key_event_code {
            111 => {
                a[1] -= 6;
                b[1] -= 6;
                c[1] -= 6;
                d[1] -= 6;
            },
            116 => {
                a[1] += 6;
                b[1] += 6;
                c[1] += 6;
                d[1] += 6;
            },
            114 => {
                a[0] += 6;
                b[0] += 6;
                c[0] += 6;
                d[0] += 6;
            },
            113 => {
                a[0] -= 6;
                b[0] -= 6;
                c[0] -= 6;
                d[0] -= 6;
            },
            _ => ()
        }

        XClearWindow(display, window);
        draw_rectangle(a, b, c, d);
        sleep(Duration::from_millis(50));
        XFlush(display);
    }
}

