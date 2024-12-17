use std::io::Bytes;
use std::io::Read;
use std::iter::Peekable;

pub fn test_io_string(
    input: &str,
    f: &mut dyn for<'b> FnMut(
        &'b mut Option<Peekable<Bytes<Box<&[u8]>>>>, // Box<dyn Read>
        &'b mut Option<Box<&mut Vec<u8>>>,
    ) -> (),
) -> String {
    let mut piet_byt_out = vec![];
    {
        let _ = f(
            &mut Some(Box::new(input.as_bytes()).bytes().peekable()),
            &mut Some(Box::new(&mut piet_byt_out)),
        );
    }

    String::from_utf8(piet_byt_out).unwrap()
}
