//! Library for Signals and Slots

// TODO: Remove the following allow directives
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use std::vec::Vec;
use std::iter::Iterator;
use std::marker::Sized;
use std::collections::VecDeque;
use std::ops::{Shl, Shr};

macro_rules! trait_alias {
    ($name:ident = $base1:ident + $($base2:ident +)+) => {
        trait $name: $base1 $(+ $base2)+ { }
        impl<T: $base1 $(+ $base2)+> $name for T { }
    };
}
//trait_alias!(DSL = Shl + Shr +);

macro_rules! signal {
    ($widget:) => {
    };
}

pub trait Slot : Sized {
    type Message;
    
    fn mess_received(mess: Self::Message) where Self: Sized {
    }
}

pub trait Signal<'a,  SLOT: 'a>  : Shr<&'a SLOT> + Sized {
    type Message;
    type List : Iterator;
    type Output;
    
    /// Add this to the list of slots
    fn shr(&'a self, slot: &SLOT) -> &'a Self where Self: Sized {
        &self
    }

    /// remove the slot from receiving signals
    fn remove(slot: &SLOT) where Self: Sized {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct Widget {
        name: String,
    }

    impl Widget {
        fn new(name: String) -> Widget {
            Widget{name}
        }
    }

    impl<'a> Signal<'a, &dyn Slot<Message = String>> for Widget {
        type Output = Self;
    }

    impl Slot for Widget {
    }
    
    #[test]
    fn test_basic_signal_slot() {
        let a = Widget::new("alpha");
        let b = Widget::new("beta");
        let c = Widget::new("gamma");
        // a is the signal, both b and c are slots to receive a's signals
        a >> b >> c;
        
        // b removes itself from receiving a's signals
        a.remove(b);        
    }
}
