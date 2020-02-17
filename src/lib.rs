//! Library for Signals and Slots
#![feature(associated_type_defaults)]
#![feature(concat_idents)]

// TODO: Remove the following allow directives
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use std::vec::Vec;
use std::iter::Iterator;
use std::marker::Sized;
use std::collections::VecDeque;
use std::ops::{Shl, Shr};
use std::boxed::Box;
use std::concat_idents;

extern crate ident;
use ident::*;

pub trait Slot<MESS> {
    type Message = MESS;
    
    fn mess_received(mess: Self::Message) where Self: Sized {
    }
}

pub trait Signal<MESS>: Sized {
    type List = Vec<MESS>;

    /// remove the slot from receiving signals
    fn remove(slot: &dyn Slot<MESS, Message = MESS>) {
    }
}

#[macro_export]
macro_rules! sigdef {
    ($widget:path, $($tag:ident : $type:ty),*) => {
        $(
            impl Shr<dyn Slot<$type, Message = $type>> for $widget {
                type Output = Self;
                
                // Add this to the list of slots
                fn shr(self, slot: &dyn Slot<$type, Message = $type>) -> &Self where Self: Sized {
                    &self
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! slotdef {
    ($widget:path, $($tag:ident : $type:ty),*) => {
        $(
            impl Shr<Self> for $widget {
                type Output = Self;
                
                // Add this to the list of slots
                fn shr(self, slot: &dyn Slot<$type, Message = $type>) -> &Self where Self: Sized {
                    &self
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct Widget {
        name: String,
    }

    enum WidgetMessages {
        Str(String),
        Num(i32),
        Empty
    }

    impl Widget {
        fn new(name: String) -> Widget {
            Widget{name}
        }
    }

    sigdef!(Widget, fly:WidgetMessages, wings:String);
    slotdef!(Widget, fly:WidgetMessages);
    
    impl<'a> Signal<&dyn Slot<WidgetMessages, Message = WidgetMessages>> for Widget {
    }

    impl Slot<WidgetMessages> for Widget {
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
