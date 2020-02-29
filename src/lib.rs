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
use std::any::Any;
use std::default::Default;

extern crate ident;
use ident::*;


pub trait Widget {
    fn mess_received (self, mess: &dyn Any);
    fn remove(self, slot: &dyn Widget);
}


#[macro_export]
macro_rules! transceiver {
    (struct $widget:ident<$a:tt> { $($tt:tt)* } with_message = $message:path; ) => {
        struct $widget<$a> { $($tt)*
                         boilerplate: i32,
                         slots: Vec<&$a dyn Widget>,
                         messages: Vec<$message>,
        }

        impl<$a> Shr<&$a dyn Widget> for $widget<$a> {
            type Output = Self;
            
            // Add this to the list of slots
            fn shr(&$a self, slot: &$a dyn Widget) -> &$a Self {
                &self
            }
        }

        impl <$a> Widget for $widget<$a> {
            fn mess_received(self, mess: &dyn Any) {
            }

            fn remove(self, slot: &dyn Widget) {
            }
        }

        impl <$a> Default for $widget<$a> {
            fn default() -> $widget<$a> {
                $widget {
                    boilerplate: 0,
                    slots: vec![],
                    messages: vec![],
                    name: String::from("")
                        
                };
                
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    enum SlideWMessage {
        Str(String),
        Num(i32),
        Empty
    }
 
    transceiver! {
        struct SlideW<'a> {
            name: String,
        } with_message = SlideWMessage;
    }

    impl <'a> SlideW<'a> {
        fn new(name: String) -> SlideW<'a> {
            SlideW{name, ..Default::default()}
        }
    }

    
    #[test]
    fn test_basic_signal_slot() {
        let a = SlideW::new("alpha");
        let b = SlideW::new("beta");
        let c = SlideW::new("gamma");
        // a is the signal, both b and c are slots to receive a's signals
        a >> b >> c;
        
        // b removes itself from receiving a's signals
        a.remove(b);        
    }
}
