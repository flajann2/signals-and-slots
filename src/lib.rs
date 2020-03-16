//! Library for Signals and Slots
#![feature(associated_type_defaults)]
#![feature(concat_idents)]

// TODO: Remove the following allow directives
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

pub mod rrcell;

use rrcell::RRCell;
use std::vec::Vec;
use std::iter::Iterator;
use std::marker::Sized;
use std::collections::VecDeque;
use std::boxed::Box;
use std::any::Any;
use std::default::Default;

use std::rc::Rc;
use std::cell::{RefCell,Ref, RefMut};
    
extern crate ident;
use ident::*;


pub trait Gizmo<'a, T> {
    fn emit_message(&self, mess: &'a T);
    fn receive_message(&mut self, mess: &'a T);
    fn remove(&self, slot: &dyn Gizmo<T>);
}

#[macro_export]
macro_rules! gizmo {
    (struct $widget:ident<$a:tt> { $($tt:tt)* } with_message = $message:path; ) => {
        #[derive(Clone)]
        struct $widget<$a> { $($tt)*
                         boilerplate: i32,
                         slots: Vec<RRCell<$widget<$a>>>,
                         messages: VecDeque<&$a $message>,
        }

        impl <$a> Gizmo<$a, $message> for $widget<$a> {
            fn emit_message(&self, mess: &$a $message) {
                self.slots
                    .iter()
                    .for_each(|w| w.receive_message(mess));
            }
            
            fn receive_message(&mut self, mess: &$a $message) {
                self.messages.push_back(mess);
            }

            fn remove(&self, slot: &dyn Gizmo<$message>) {
            }
        }

        impl <$a> Default for $widget<$a> {
            fn default() -> $widget<$a> {
                $widget {
                    boilerplate: 0,
                    slots: vec![],
                    messages: VecDeque::from(vec![]),
                    name: String::from("")                        
                }                
            }
        }
    };
}

#[macro_export]
macro_rules! wire {
    ($emitter:ident to $head:ident $(+ $tail:ident)*) => {{
        $emitter.borrow_mut().slots.push($head.clone());
        $($emitter.borrow_mut().slots.push($tail.clone());)*
    }}
}

#[macro_export]
macro_rules! snip {
    ($this_widget:ident from $emmiter:ident) => {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum SlideMessage {
        Str(String),
        Num(i32),
        Empty
    }
 
    gizmo! {
        struct SlideWidget<'a> {
            name: String,
        } with_message = SlideMessage;
    }

    impl <'a> SlideWidget<'a> {
        fn new(name: &str) -> RRCell<SlideWidget<'a>> {
            RRCell::new(SlideWidget{name: name.to_string(), ..Default::default()})
        }

        fn send(&self, mess: &'a SlideMessage) {
            self.emit_message(mess);
        }
    }

    
    #[test]
    fn test_basic_signal_slot() {
        let a = SlideWidget::new("alpha");
        let b = SlideWidget::new("beta");
        let c = SlideWidget::new("gamma");

        use SlideMessage::*;

        let m1 = Str("Come to mama".to_string());
        let m2 = Num(2001);
        let m3 = Empty;
        
        // a is the signal, both b and c are slots to receive a's signals
        wire!{ a to b + c };
        wire!{ c to a + b };

        // Send messages to listeners
        a.borrow_mut().send(&m1);
        a.borrow_mut().send(&m2);
        a.borrow_mut().send(&m3);
        
        // b removes itself from receiving a's signals
        snip!(b from a);        
    }
}
