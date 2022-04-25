use std::io::{
    Write,
    BufRead,
};

#[derive(Debug)]
pub struct TelnetClient<'a, W, T>
where
    T : BufRead + Sync + Send,
    W  : Write + Sync + Send
{
    pub reader: &'a mut T,
    pub writer: &'a mut W
}
