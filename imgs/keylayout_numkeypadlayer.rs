#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_numkeypadlayer.svg",
      [
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ],
         [X     , X     , X     , X     , X     , X     ]
      ], [
                                 [X     , X     , X     , X],
                                 [X     , X     , X     , X]
      ],

      [
                 [n("NumLock"), n("7"), n("8"), n("9"), X     , X     ],
                 [X           , X     , X     , X     , X     , X     ],
                 [X           , n("4"), n("5"), n("6"), X     , X     ],
                 [X           , X     , X     , X     , X     , X     ],
                 [n("0")      , n("1"), n("2"), n("3"), X     , X     ],
                 [X           , X     , X     , X     , X     , X     ]
      ],
      [
         [X     , X     , h("fnL"), X],
         [X     , X     , X       , X]
      ]
   )
}
