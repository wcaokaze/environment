#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_fnlayer.svg",
      [
         [n("F12"), n("F1"), n("F2"), n("F3")         , n("F4"), n("F5")],
         [X       , X      , X      , X               , X      , X      ],
         [X       , X      , X      , n("PrintScreen"), X      , X      ],
         [X       , X      , X      , X               , X      , X      ],
         [X       , X      , X      , X               , X      , X      ],
         [X       , X      , X      , X               , X      , X      ]
      ], [
                                             [X     , X     , X     , n("Delete")],
                                             [X     , X     , X     , X          ]
      ],

      [
                 [n("F6"), n("F7")  , n("F8") , n("F9"), n("F10"), n("F11")],
                 [X      , X        , X       , X      , X       , X       ],
                 [n("←") , n("↓")   , n("↑")  , n("→") , X       , X       ],
                 [X      , X        , X       , X      , X       , X       ],
                 [X      , n("Home"), n("End"), X      , X       , X       ],
                 [X      , X        , X       , X      , X       , X       ]
      ],
      [
         [X     , h("テンキーL"), h("fnL"), X],
         [X     , X             , X       , X]
      ]
   )
}
