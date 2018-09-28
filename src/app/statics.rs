pub const long_desc: &str =
    "This is a command line app to generate gitignores. You can use it easily right
 along with command line git with the `gi` command. It uses the public API to 
 generate gitignores, and therefore requires internet connection. Requests can 
 sometimes be cached for offline usage.";

pub const SYMDIR_HELP: &str = "Choose a symbolic install directories. A symlink will be created there; PATH can then be linked to the symlink instead of this executable. `gi` will not explicitly try to, though.";

pub const DIR_HELP: &str = "Choose a directory in which to copy (install) this executable. `gi` will then try to link PATH to that executable instead of this one.";

pub const NOLINK_HELP: &str =
    "Turns off automatic linking. `gi` will no longer try and link PATH to an executable";

pub const NODIRWRAP_HELP: &str = "Turns off wrapping of executable in directories. `gi` would wrap the executable into a folder (`bin` or `gi/bin`) for safety, except when you link PATH to this executable";

pub const PROFILES: [&str; 3] = ["~/.bash_profile", "~/.profile", "~/.bashrc"];
