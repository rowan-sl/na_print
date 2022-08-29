//! Print to the terminal *without* allocating!

/// Unstable (ish) internals
///
/// this contains the actual write fn that is used, so it can be used for other things in users libs
pub mod internal {
    use std::{io, cmp};
    use libc::{write, c_int, c_void};

    // The maximum read limit on most POSIX-like systems is `SSIZE_MAX`,
    // with the man page quoting that if the count of bytes to read is
    // greater than `SSIZE_MAX` the result is "unspecified".
    //
    // On macOS, however, apparently the 64-bit libc is either buggy or
    // intentionally showing odd behavior by rejecting any read with a size
    // larger than or equal to INT_MAX. To handle both of these the read
    // size is capped on both platforms.
    #[cfg(target_os = "macos")]
    const READ_LIMIT: usize = libc::c_int::MAX as usize - 1;
    #[cfg(not(target_os = "macos"))]
    const READ_LIMIT: usize = libc::ssize_t::MAX as usize;


    /// writes all of `buf` to the given fd. can be used with [`libc::STDOUT_FILENO`]
    /// and [`libc::STDERR_FILENO`] to print without allocating
    ///
    /// # Saftey
    /// it seemed similar enough to what happenes in [`std::sys::unix::fd::FileDesc::write`], so it should be fine?
    /// not synced or locked in any way so expect some strange behavior
    ///
    #[cfg(unix)]
    pub unsafe fn write_all(fd: c_int, buf: &[u8]) -> io::Result<()> {
        debug_assert!(buf.len() <= READ_LIMIT, "writing more than {} bytes is unspecified behavior!", READ_LIMIT);
        let mut remaining = cmp::min(buf.len(), READ_LIMIT);
        loop {
            let res = write(fd, buf[buf.len() - remaining..].as_ptr() as *const c_void, remaining);
            if res < 0 {
                return Err(io::Error::last_os_error());
            } else if res == 0 {
                break Ok(());
            } else {
                remaining -= res as usize;
            }
        }
    }
}

#[cfg(unix)]
fn print_to(fd: libc::c_int, txt: &str) {
    unsafe { internal::write_all(fd, txt.as_bytes()) }.unwrap();
}

/// Prints `txt` to stdout. performs no allocations, and panics on error.
#[cfg(unix)]
#[inline(always)]
pub fn print(txt: &str) {
    print_to(libc::STDOUT_FILENO, txt);
}

/// [`print`], but prints a trailing newline
#[cfg(unix)]
#[inline(always)]
pub fn println(txt: &str) {
    print(txt);
    print("\n");
}

/// Prints `txt` to stderr. for mor info, see [`print`]
#[cfg(unix)]
#[inline(always)]
pub fn eprint(txt: &str) {
    print_to(libc::STDERR_FILENO, txt);
}

/// [`eprint`], but prints a trailing newline
#[cfg(unix)]
#[inline(always)]
pub fn eprintln(txt: &str) {
    print(txt);
    print("\n");
}

#[cfg(all(test, unix))]
#[test]
fn it_works() {
    print("Hello, World!");
    eprint("oh no");
}
