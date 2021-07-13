//! Macros for opening files and directories using ambient authority, with
//! paths known at compile time.

/// Opens a file in read-only mode with ambient authority, using a path known
/// at compile time.
///
/// # Examples
///
/// ```rust
/// use open_ambient::open_ambient_file;
///
/// let file = open_ambient_file!("Cargo.toml").unwrap();
/// ```
#[macro_export]
macro_rules! open_ambient_file {
    ($path:literal) => {
        ::cap_std::fs::File::open_ambient(
            $path,
            ::cap_std::ambient_authority_known_at_compile_time(),
        )
    };
}

/// Opens a file with ambient authority with the specified options, but using a
/// path known at compile time.
///
/// # Examples
///
/// ```rust
/// use cap_std::fs::OpenOptions;
/// use open_ambient::open_ambient_file_with;
///
/// let file = open_ambient_file_with!("Cargo.toml", &OpenOptions::new().read(true)).unwrap();
/// ```
#[macro_export]
macro_rules! open_ambient_file_with {
    ($path:literal, $opts:expr) => {
        ::cap_std::fs::File::open_ambient_with(
            $path,
            $opts,
            ::cap_std::ambient_authority_known_at_compile_time(),
        )
    };
}

/// Opens a directory with ambient authority, using a path known at compile
/// time.
///
/// # Examples
///
/// ```rust
/// use open_ambient::open_ambient_dir;
///
/// let dir = open_ambient_dir!("src").unwrap();
/// ```
#[macro_export]
macro_rules! open_ambient_dir {
    ($path:literal) => {
        ::cap_std::fs::Dir::open_ambient_dir(
            $path,
            ::cap_std::ambient_authority_known_at_compile_time(),
        )
    };
}
