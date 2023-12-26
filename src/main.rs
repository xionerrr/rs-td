use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use rs_todo::*;

fn main() {
    let circle = Circle { radius: 10.0 };
    assert_eq!(circle.area(), PI * 100.0);
    let square = &Square {
        height: 5.0,
        width: 4.0,
    };
    let yo = Arc::new(Mutex::new(0));
    std::thread::scope(|cx| {
        for _ in (0..10) {
            let t = [
                cx.spawn(|| {
                    let mut yo = yo.lock().unwrap();
                    *yo += 1;
                }),
                cx.spawn(|| {
                    let mut yo = yo.lock().unwrap();
                    *yo += 1;
                }),
            ];

            for t in t {
                t.join().unwrap();
            }
        }
    });
    println!("{:#?}", yo);
}
