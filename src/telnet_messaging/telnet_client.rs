use std::io::{
    Write,
    BufRead,
};

#[derive(Debug)]
pub struct TelnetClient<W, T>
where
    T : BufRead + Sync + Send + 'static,
    W  : Write + Sync + Send + 'static
{
    pub reader: Option<T>,
    pub writer: W
}
