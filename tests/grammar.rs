// pest. Smart PEGs in Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate pest;

use pest::Parser;
use pest::Queues;
use pest::Input;
use pest::StringInput;

impl_rdp! {
    grammar! {
        exp = _{ paren ~ exp | [""] }
        paren = { ["("] ~ exp ~ [")"] }
        rep_zero = { ["a"]* ~ eoi }
        rep_one = { ["a"]+ }
        opt = { ["a"]? }
        pres = { &["a"] }
        abs = { !(["a"] | ["b"]) ~ any }
        digit = { ['0'..'9'] }
    }
}

#[test]
fn basic() {
    let mut parser = Rdp::new(Box::new(StringInput::new("(())((())())()")));

    assert!(parser.exp());
    assert!(parser.end());

    let queue = vec![
        Rules::paren(0, 4),
        Rules::paren(1, 2),
        Rules::paren(4, 8),
        Rules::paren(5, 4),
        Rules::paren(6, 2),
        Rules::paren(9, 2),
        Rules::paren(12, 2)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn fail() {
    let mut parser = Rdp::new(Box::new(StringInput::new("(())((())())(")));

    assert!(parser.exp());
    assert!(!parser.end());

    let queue = vec![
        Rules::paren(0, 4),
        Rules::paren(1, 2),
        Rules::paren(4, 8),
        Rules::paren(5, 4),
        Rules::paren(6, 2),
        Rules::paren(9, 2)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn rep_zero_empty() {
    let mut parser = Rdp::new(Box::new(StringInput::new("")));

    assert!(parser.rep_zero());

    let queue = vec![
        Rules::rep_zero(0, 0)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn rep_zero_long() {
    let mut parser = Rdp::new(Box::new(StringInput::new("aaaa")));

    assert!(parser.rep_zero());

    let queue = vec![
        Rules::rep_zero(0, 4)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn rep_zero_wrong() {
    let mut parser = Rdp::new(Box::new(StringInput::new("aaaab")));

    assert!(!parser.rep_zero());

    let queue = vec![];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn rep_one_empty() {
    let mut parser = Rdp::new(Box::new(StringInput::new("")));

    assert!(!parser.rep_one());

    let queue = vec![];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn rep_one_long() {
    let mut parser = Rdp::new(Box::new(StringInput::new("aaaa")));

    assert!(parser.rep_one());
    assert!(parser.end());

    let queue = vec![
        Rules::rep_one(0, 4)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn rep_one_wrong() {
    let mut parser = Rdp::new(Box::new(StringInput::new("b")));

    assert!(!parser.rep_one());
    assert!(!parser.end());

    let queue = vec![];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn opt_empty() {
    let mut parser = Rdp::new(Box::new(StringInput::new("")));

    assert!(parser.opt());
    assert!(parser.end());

    let queue = vec![
        Rules::opt(0, 0)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn opt_right() {
    let mut parser = Rdp::new(Box::new(StringInput::new("a")));

    assert!(parser.opt());
    assert!(parser.end());

    let queue = vec![
        Rules::opt(0, 1)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn opt_wrong() {
    let mut parser = Rdp::new(Box::new(StringInput::new("b")));

    assert!(parser.opt());
    assert!(!parser.end());

    let queue = vec![
        Rules::opt(0, 0)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn pres_right() {
    let mut parser = Rdp::new(Box::new(StringInput::new("a")));

    assert!(parser.pres());
    assert!(!parser.end());

    let queue = vec![
        Rules::pres(0, 0)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn pres_wrong() {
    let mut parser = Rdp::new(Box::new(StringInput::new("b")));

    assert!(!parser.pres());
    assert!(!parser.end());

    let queue = vec![];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn abs_right() {
    let mut parser = Rdp::new(Box::new(StringInput::new("c")));

    assert!(parser.abs());
    assert!(parser.end());

    let queue = vec![
        Rules::abs(0, 1)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn abs_wrong() {
    let mut parser = Rdp::new(Box::new(StringInput::new("a")));

    assert!(!parser.abs());
    assert!(!parser.end());

    let queue = vec![];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn digit_right() {
    let mut parser = Rdp::new(Box::new(StringInput::new("0")));

    assert!(parser.digit());
    assert!(parser.end());

    let queue = vec![
        Rules::digit(0, 1)
    ];

    assert!(parser.queue().iter().eq(&queue));
}

#[test]
fn digit_wrong() {
    let mut parser = Rdp::new(Box::new(StringInput::new("a")));

    assert!(!parser.digit());
    assert!(!parser.end());

    let queue = vec![];

    assert!(parser.queue().iter().eq(&queue));
}