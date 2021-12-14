use std::fs::File;
use std::io;
use std::io::{Read, Error};
use std::io::{Stdout, Stderr};
use std::os::unix::prelude::{FromRawFd, RawFd, AsRawFd};

/// Silence and redirect output from stdout and stderr
/// into pipe
/// `Devertion` implements io::Read
/// # Example
/// ```rust
/// crate::fd_divertion::fd_divertion::Divertion;
/// use std::str;
/// 
/// let mut div = Divertion::new().unwrap();
/// div.divert();
/// println!("print after divert");
/// let mut buf: Vec<u8> = Vec::new();
/// div.read_to_end(&mut buf).unwrap();
/// drop (div);
/// println!("what comes from pipe: {}", str::from_utf8(&buf).unwrap());
/// ```
pub(crate) struct Divertion
{
    original_out: i32,
    original_err: i32,
    read_file: File,
    write_file: File,
}

impl Divertion
{
    pub fn new() -> Result<Self, Error>
    {
        let original_out = match unsafe { libc::dup(libc::STDOUT_FILENO)}
        {
            -1 => return Err(Error::last_os_error()),
            fd => fd,
        };
        let original_err = match unsafe {libc::dup(libc::STDERR_FILENO)}
        {
            -1 => return Err(Error::last_os_error()),
            fd => fd,
        };
        let (read_file, write_file) = Self::get_pipe()?;
        Ok
        (
            Self 
            {
                original_out,
                original_err,
                read_file,
                write_file
            }
        )
    }

    fn get_pipe() -> Result<(File, File), Error>
    {
        let mut outputs = [0;2];
        if unsafe { libc::pipe(outputs.as_mut_ptr())} == -1 
        {
            return Err(Error::last_os_error())
        }
        let read_file = unsafe { FromRawFd::from_raw_fd(outputs[0])};
        let write_file = unsafe { FromRawFd::from_raw_fd(outputs[1])};
        Ok((read_file, write_file))
    }

    pub fn divert(&self) -> Result<(), Error>
    {
        if -1 == unsafe { libc::dup2(self.write_file.as_raw_fd(), libc::STDOUT_FILENO) } {
            return Err(Error::last_os_error());
        };
        if -1 == unsafe { libc::dup2(self.write_file.as_raw_fd(), libc::STDERR_FILENO) } {
            return Err(Error::last_os_error());
        };

        Ok(())
    }

    pub fn to_string(mut self) -> String 
    {
        let mut output = String::new();
        self.read_to_string(&mut output);

        output
    }

    fn _read(&mut self, buf: &mut [u8]) -> Result<usize, Error>
    {
        let mut avail = 0;
        let res = unsafe {libc::ioctl(self.read_file.as_raw_fd(), libc::FIONREAD, &mut avail)};
        if res == -1 
        {
            return Err(Error::last_os_error());
        } 
        else if avail == 0
        {
            Ok(0)
        } 
        else 
        {
            self.read_file.read(buf)
        }
    }
}


impl Drop for Divertion
{
    fn drop(&mut self) {
        unsafe { 
            libc::dup2(self.original_out, libc::STDOUT_FILENO);
            libc::dup2(self.original_err, libc::STDERR_FILENO);
            libc::close(self.original_out);
            libc::close(self.original_err);
        }

    }
}

impl Read for Divertion
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>
    {
        self._read(buf)
    }
}

mod tests
{
    use super::Divertion;
    use std::io::Read;
    use std::str;

    #[test]
    fn test_read()
    {
        let mut div = Divertion::new().unwrap();
        div.divert();
        println!("print after divert");
        let mut buf: Vec<u8> = Vec::new();
        div.read_to_end(&mut buf).unwrap();
        drop (div);
        println!("what comes from pipe: {}", str::from_utf8(&buf).unwrap());
    }
}