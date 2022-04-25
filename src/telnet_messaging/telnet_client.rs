use std::io::{
    Write,
    BufRead,
    Read
};

use std::clone::Clone;

#[derive(Debug)]
pub struct TelnetClient<'a, T>
where T :  Read + Write + BufRead + Clone + Send + 'a
{
    pub stream: &'a T
}
