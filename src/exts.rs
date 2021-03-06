/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                                                                            │ *
 * │ This Source Code Form is subject to the terms of the Mozilla Public                        │ *
 * │ License, v. 2.0. If a copy of the MPL was not distributed with this                        │ *
 * │ file, You can obtain one at http://mozilla.org/MPL/2.0/.                                   │ *
 * │                                                                                            │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                          Imports                                           │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

use cfg_if::cfg_if;

/* ┌────────────────────────────────────────────────────────────────────────────────────────────┐ *\
 * │                                       cfg_if! { .. }                                       │ *
\* └────────────────────────────────────────────────────────────────────────────────────────────┘ */

cfg_if! {
    if #[cfg(feature = "chrono")] {
        mod chrono;
    }
}

cfg_if! {
    if #[cfg(feature = "collections")] {
        mod deque;
        mod heap;
        mod list;
        mod map;
        mod set;
        mod vec;
    }
}

cfg_if! {
    if #[cfg(feature = "ed25519")] {
        mod ed25519;
    }
}

cfg_if! {
    if #[cfg(feature = "net")] {
        mod net;
    }
}

cfg_if! {
    if #[cfg(feature = "pow")] {
        mod pow;
    }
}

cfg_if! {
    if #[cfg(feature = "sparse")] {
        mod sparse;
    }
}

cfg_if! {
    if #[cfg(feature = "x25519")] {
        mod x25519;
    }
}
