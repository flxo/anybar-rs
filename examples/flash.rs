extern crate anybar;
use anybar::{Anybar, Color};
use std::{thread, time};

const N: u16 = 5;

fn main() {
    let mut bars: Vec<Anybar> = (1739..1739 + N)
        .map(|p| Anybar::start(p as u16).unwrap())
        .map(|p| { thread::sleep(time::Duration::from_millis(200)); p })
        .map(|mut b| { b.set_color(Color::Orange).unwrap(); b })
        .collect::<Vec<Anybar>>();

    bars.drain(..).rev()
        .map(|mut b| { b.set_color(Color::Blue).unwrap(); b })
        .map(|p| { thread::sleep(time::Duration::from_millis(200)); p })
        .map(|b| b.quit().unwrap())
        .collect::<Vec<()>>();
}
