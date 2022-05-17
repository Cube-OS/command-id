// mod command_id;
pub use ::variant_count;
pub use ::cubeos_error;

#[macro_export]
macro_rules! command_id{
    (
        $($type: ident,)+
    ) => {
        use std::convert::{TryFrom,TryInto};
        use variant_count::VariantCount;
        use cubeos_error::{Error as CubeOSError, Result as CubeOSResult};
        use std::ops::AddAssign;
        use std::fmt;
        use serde::Serialize;

        // helper functions to implement the TryFrom<u16> for udp and ground macros
        // increments a usize and outputs the value
        // needed to increment a counter inside the macro expression $()+ 
        pub fn increment(i: &mut usize) -> usize {
            i.add_assign(1);
            *i-1
        }

        // Construct CommandID Enum
        #[derive(Clone,Copy,Debug,PartialEq,VariantCount,Serialize)]
        pub enum CommandID {
            $(
                $type,
            )+
        }
        // implementation of conversion of u16 to CommandID
        impl TryFrom<u16> for CommandID {
            type Error = CubeOSError;

            fn try_from(cmd: u16) -> CubeOSResult<Self> {
                let mut i: usize = 0;
                let h_field: Vec<u16> = (1..1+CommandID::VARIANT_COUNT as u16).collect();
                match cmd {
                    $(x if x == h_field[increment(&mut i)] => Ok(CommandID::$type),)+
                    _ => Err(CubeOSError::NoCmd),
                }
            }
        }  
        
        // implement conversion of CommandID to u16
        impl TryFrom<CommandID> for u16 {
            type Error = CubeOSError;

            fn try_from(c: CommandID) -> CubeOSResult<u16> {
                let mut i: usize = 0;
                let h_field: Vec<u16> = (1..1+CommandID::VARIANT_COUNT as u16).collect();
                match c {
                    $(CommandID::$type => Ok(h_field[CommandID::$type as usize]),)*
                    _ => Err(CubeOSError::NoCmd),
                }
            }
        }
    }
}