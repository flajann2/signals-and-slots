//! Library for Signals and Slots

// TODO: Remove the following allow directives
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use std::collections::VecDeque;
use std::ops::{Shl, Shr};

macro_rules! trait_alias {
    ($name:ident = $base1:ident + $($base2:ident +)+) => {
        trait $name: $base1 $(+ $base2)+ { }
        impl<T: $base1 $(+ $base2)+> $name for T { }
    };
}


//trait_alias!(DSL = Shl + Shr +);

pub trait Slot : std::marker::Sized {
    type Message;
    
    fn mess_received(mess: Self::Message) {
    }
}

pub trait Signal<'a, RHS>  : Shl<RHS> + Shr<RHS> + std::marker::Sized {
    type Message;
    type Output;
    
    /// Add this to the list of signals
    fn shl(&'a self, slot: &RHS) -> &'a Self where Self: std::marker::Sized {
        &self
    }
    
    fn shr(&'a self, slot: &RHS) -> &'a Self where Self: std::marker::Sized {
        &self
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    struct Widget {
    }

    impl<'a> Signal<'a, Slot<Message = String>> for Widget {
    }
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
