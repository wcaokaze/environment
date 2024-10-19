#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_numlayer.svg",
      [
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ],
         [n("`"), n("1"), n("2"), n("3"), n("4"), n("5")],
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ]
      ], [
                                 [X     , X     , X     , X],
                                 [X     , X     , X     , X]
      ],

      [
                 [X     , X     , X     , X     , X     , X     ],
                 [X     , X     , X     , X     , X     , X     ],
                 [n("6"), n("7"), n("8"), n("9"), n("0"), n("/")],
                 [X     , X     , X     , X     , X     , X     ],
                 [X     , X     , X     , X     , X     , X     ],
                 [X     , X     , X     , X     , X     , X     ]
      ],
      [
         [X     , h("数字L"), X     , X],
         [X     , X         , X     , X]
      ]
   )
}
