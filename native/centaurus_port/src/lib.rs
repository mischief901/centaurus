/** This library translates and maintains the port based communications
between Elixir or Erlang and Rust into something that centaurus_common can
understand.

See here (http://erlang.org/doc/tutorial/c_port.html) for some details.
 **/

/*
Internal ideas.

This file should contain various ways of interpretting and storing data.
The data should be stored in a struct with a buffer.
read_cmd should return an exit code when the file descriptor is closed.
read_cmd reads from file descriptor 0.
write_cmd should write an exit code when an internal error is encountered.
write_cmd writes to file descriptor 1.

The struct should include an enum of potential commands with the number 
and type of arguments each command needs.
Serde can be used to deserialize the commands once all the necessary info
has been read into the temporary buffer.
Serde can also be used to serialize the return values.

*/

use std::io::{ BufReader, BufWriter };
use std::os::unix::io::{ FromRawFd };
use std::fs::File;
use std::error::Error;

struct PortBuffer {
    read_buffer : BufReader<File>,
    write_buffer : BufWriter<File>,
}

enum PortBufferError {
    ReadError,
    WriteError,
    UnknownError,
}

trait PortInterface <T> where T : Sized {
    type Error;
    fn new() -> Self;
    fn read(self) -> Option<T>;
    fn write(self, values : T) -> Result<(), dyn Error>;
}

impl <T> PortInterface <T> for PortBuffer {
    type Error = PortBufferError;
    
    // Open the raw fd and wrap it in a BufReader/BufWriter
    fn new() -> Self {
        let read_buffer = BufReader::new(File::from_raw_fd(0));
        let write_buffer = BufWriter::new(File::from_raw_fd(1));
        Self {
            read_buffer : read_buffer,
            write_buffer : write_buffer,
        }
    }

    // Read a command.
    fn read(self) -> Option<PortBuffer> {
        let mut raw_command = String::new();
        let len : i32 = self.read_buffer.read_line(&mut raw_command)?;
        match len {
            0 => None,
            _ => { let command : T = T::new(&mut raw_command)?;
                   Some(command)
            }
        }
    }

    // Write the specified value to the buffer.
    // Maybe move this to be type generic with an Into<String> impl.
    fn write(self, values : PortBuffer) -> Result<(), dyn Error> {
        let raw_values : String = T::from(values)?;
        let len : i32 = self.write_buffer.write(raw_values);
        match len {
            l if l < 0 => Result::Err(Error::WriteError),
            _ => Ok(),
        }
    }
}
/*
enum Command {
    Open(Address, Port),
    OpenStreamUni,
    OpenStreamBi,
    Close,
    Write(String),
    WriteStream(Stream, String),
    Read,
    ReadStream(Stream, String),
}
*/
