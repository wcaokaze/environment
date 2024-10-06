#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_symbollayer.svg",
      [
         [X     , X     , X     , X     , X     , n("/*")],
         [X     , X     , X     , X     , X     , X      ],
         [n("~"), n("!"), n("@"), n("#"), n("$"), n("%") ],
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ]
      ], [
                                 [X     , X   , h("記号L"), X],
                                 [X     , X   , X         , X]
      ],

      [
                 [n("]"), n("-&gt;"), n("=&gt;"), X     , X     , X     ],
                 [X     , X         , X         , X     , X     , X     ],
                 [n("^"), n("&amp;"), n("*")    , n("("), n(")"), n("?")],
                 [X     , X         , X         , X     , X     , X     ],
                 [X     , X         , X         , X     , X     , X     ],
                 [X     , X         , X         , X     , X     , X     ]
      ],
      [
         [X     , X     , X     , X],
         [X     , X     , X     , X]
      ]
   )
}
