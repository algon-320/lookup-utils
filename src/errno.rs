#![cfg(all(
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "aarch64"
    )
))]

use box_drawing_table::{ansi_term::Style, Align, Border, Cell, CellSize, Column, Row, Table};
use clap::Parser;
use libc::{c_int, strerror};
use std::ffi::CStr;

// NOTE: based on Linux man-pages 6.01

/// A simple utility to look up Linux error numbers (errno)
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    /// errno value (e.g. "2"),
    /// or symbolic name (e.g. "ENOENT")
    query: Vec<String>,

    #[clap(long, default_value_t = false)]
    /// Disable pretty-printing
    simple: bool,

    #[clap(short, long, default_value_t = false)]
    /// List all errors
    list: bool,

    #[clap(long, default_value_t = false)]
    /// Display the description using strerror(3)
    libc: bool,
}

fn main() {
    let args = Args::parse();

    let mut rows = Vec::new();
    let mut desc_len = 0_usize;

    let queries = if args.list { list() } else { args.query };
    for q in queries {
        let sigdesc = if let Ok(num) = q.parse::<c_int>() {
            ErrnoDesc::from_number(num).ok_or(num)
        } else {
            Ok(ErrnoDesc::from_name(q.clone()))
        };

        let name: String;
        let number: String;
        let description: String;
        match sigdesc {
            Ok(sigdesc) => {
                name = sigdesc.name().to_string();
                number = sigdesc
                    .number()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "-".to_owned());

                if args.libc {
                    description = sigdesc.libc_descripton();
                } else {
                    description = sigdesc.manpages_description();
                }
            }
            Err(num) => {
                name = "-".to_owned();
                number = num.to_string();
                description = "Unknown error".to_owned();
            }
        }

        if args.simple {
            println!("{} {} {}", name, number, description);
        } else {
            desc_len = desc_len.max(description.len());

            rows.push(Row::flexible_height(vec![
                Cell {
                    value: name,
                    align: Align::Left,
                    style: Style::default().bold(),
                },
                Cell::left(number),
                Cell::left(description),
            ]));
        }
    }

    if !args.simple && !rows.is_empty() {
        create_table(rows, desc_len.min(80));
    }
}

fn create_table(rows: Vec<Row>, desc_len: usize) {
    let mut table = Table::new(vec![
        Border::Double.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(desc_len),
        },
        Border::Double.into(),
    ]);

    table.append_row(Border::Double.into());
    table.append_row(Row::flexible_height(vec![
        Cell::left("name"),
        Cell::left("number"),
        Cell::left("description"),
    ]));
    table.append_row(Border::Single.into());

    for r in rows {
        table.append_row(r);
    }

    table.append_row(Border::Double.into());

    print!("{}", table);
}

fn list() -> Vec<String> {
    [
        "E2BIG",
        "EACCES",
        "EADDRINUSE",
        "EADDRNOTAVAIL",
        "EAFNOSUPPORT",
        "EAGAIN",
        "EALREADY",
        "EBADE",
        "EBADF",
        "EBADFD",
        "EBADMSG",
        "EBADR",
        "EBADRQC",
        "EBADSLT",
        "EBUSY",
        "ECANCELED",
        "ECHILD",
        "ECHRNG",
        "ECOMM",
        "ECONNABORTED",
        "ECONNREFUSED",
        "ECONNRESET",
        "EDEADLK",
        "EDEADLOCK",
        "EDESTADDRREQ",
        "EDOM",
        "EDQUOT",
        "EEXIST",
        "EFAULT",
        "EFBIG",
        "EHOSTDOWN",
        "EHOSTUNREACH",
        "EHWPOISON",
        "EIDRM",
        "EILSEQ",
        "EINPROGRESS",
        "EINTR",
        "EINVAL",
        "EIO",
        "EISCONN",
        "EISDIR",
        "EISNAM",
        "EKEYEXPIRED",
        "EKEYREJECTED",
        "EKEYREVOKED",
        "EL2HLT",
        "EL2NSYNC",
        "EL3HLT",
        "EL3RST",
        "ELIBACC",
        "ELIBBAD",
        "ELIBMAX",
        "ELIBSCN",
        "ELIBEXEC",
        "ELNRNG",
        "ELOOP",
        "EMEDIUMTYPE",
        "EMFILE",
        "EMLINK",
        "EMSGSIZE",
        "EMULTIHOP",
        "ENAMETOOLONG",
        "ENETDOWN",
        "ENETRESET",
        "ENETUNREACH",
        "ENFILE",
        "ENOANO",
        "ENOBUFS",
        "ENODATA",
        "ENODEV",
        "ENOENT",
        "ENOEXEC",
        "ENOKEY",
        "ENOLCK",
        "ENOLINK",
        "ENOMEDIUM",
        "ENOMEM",
        "ENOMSG",
        "ENONET",
        "ENOPKG",
        "ENOPROTOOPT",
        "ENOSPC",
        "ENOSR",
        "ENOSTR",
        "ENOSYS",
        "ENOTBLK",
        "ENOTCONN",
        "ENOTDIR",
        "ENOTEMPTY",
        "ENOTRECOVERABLE",
        "ENOTSOCK",
        "ENOTSUP",
        "ENOTTY",
        "ENOTUNIQ",
        "ENXIO",
        "EOPNOTSUPP",
        "EOVERFLOW",
        "EOWNERDEAD",
        "EPERM",
        "EPFNOSUPPORT",
        "EPIPE",
        "EPROTO",
        "EPROTONOSUPPORT",
        "EPROTOTYPE",
        "ERANGE",
        "EREMCHG",
        "EREMOTE",
        "EREMOTEIO",
        "ERESTART",
        "ERFKILL",
        "EROFS",
        "ESHUTDOWN",
        "ESPIPE",
        "ESOCKTNOSUPPORT",
        "ESRCH",
        "ESTALE",
        "ESTRPIPE",
        "ETIME",
        "ETIMEDOUT",
        "ETOOMANYREFS",
        "ETXTBSY",
        "EUCLEAN",
        "EUNATCH",
        "EUSERS",
        "EWOULDBLOCK",
        "EXDEV",
        "EXFULL",
    ]
    .iter()
    .map(|name| name.to_string())
    .collect()
}

struct ErrnoDesc {
    name: String,
}

impl ErrnoDesc {
    fn from_name(name: String) -> Self {
        Self { name }
    }

    fn from_number(errno: c_int) -> Option<Self> {
        let name = match errno {
            libc::E2BIG => "E2BIG",
            libc::EACCES => "EACCES",
            libc::EADDRINUSE => "EADDRINUSE",
            libc::EADDRNOTAVAIL => "EADDRNOTAVAIL",
            libc::EAFNOSUPPORT => "EAFNOSUPPORT",
            libc::EAGAIN => "EAGAIN",
            libc::EALREADY => "EALREADY",
            libc::EBADE => "EBADE",
            libc::EBADF => "EBADF",
            libc::EBADFD => "EBADFD",
            libc::EBADMSG => "EBADMSG",
            libc::EBADR => "EBADR",
            libc::EBADRQC => "EBADRQC",
            libc::EBADSLT => "EBADSLT",
            libc::EBUSY => "EBUSY",
            libc::ECANCELED => "ECANCELED",
            libc::ECHILD => "ECHILD",
            libc::ECHRNG => "ECHRNG",
            libc::ECOMM => "ECOMM",
            libc::ECONNABORTED => "ECONNABORTED",
            libc::ECONNREFUSED => "ECONNREFUSED",
            libc::ECONNRESET => "ECONNRESET",
            libc::EDEADLK => "EDEADLK",
            // libc::EDEADLOCK => "EDEADLOCK",
            libc::EDESTADDRREQ => "EDESTADDRREQ",
            libc::EDOM => "EDOM",
            libc::EDQUOT => "EDQUOT",
            libc::EEXIST => "EEXIST",
            libc::EFAULT => "EFAULT",
            libc::EFBIG => "EFBIG",
            libc::EHOSTDOWN => "EHOSTDOWN",
            libc::EHOSTUNREACH => "EHOSTUNREACH",
            libc::EHWPOISON => "EHWPOISON",
            libc::EIDRM => "EIDRM",
            libc::EILSEQ => "EILSEQ",
            libc::EINPROGRESS => "EINPROGRESS",
            libc::EINTR => "EINTR",
            libc::EINVAL => "EINVAL",
            libc::EIO => "EIO",
            libc::EISCONN => "EISCONN",
            libc::EISDIR => "EISDIR",
            libc::EISNAM => "EISNAM",
            libc::EKEYEXPIRED => "EKEYEXPIRED",
            libc::EKEYREJECTED => "EKEYREJECTED",
            libc::EKEYREVOKED => "EKEYREVOKED",
            libc::EL2HLT => "EL2HLT",
            libc::EL2NSYNC => "EL2NSYNC",
            libc::EL3HLT => "EL3HLT",
            libc::EL3RST => "EL3RST",
            libc::ELIBACC => "ELIBACC",
            libc::ELIBBAD => "ELIBBAD",
            libc::ELIBMAX => "ELIBMAX",
            libc::ELIBSCN => "ELIBSCN",
            libc::ELIBEXEC => "ELIBEXEC",
            libc::ELNRNG => "ELNRNG",
            libc::ELOOP => "ELOOP",
            libc::EMEDIUMTYPE => "EMEDIUMTYPE",
            libc::EMFILE => "EMFILE",
            libc::EMLINK => "EMLINK",
            libc::EMSGSIZE => "EMSGSIZE",
            libc::EMULTIHOP => "EMULTIHOP",
            libc::ENAMETOOLONG => "ENAMETOOLONG",
            libc::ENETDOWN => "ENETDOWN",
            libc::ENETRESET => "ENETRESET",
            libc::ENETUNREACH => "ENETUNREACH",
            libc::ENFILE => "ENFILE",
            libc::ENOANO => "ENOANO",
            libc::ENOBUFS => "ENOBUFS",
            libc::ENODATA => "ENODATA",
            libc::ENODEV => "ENODEV",
            libc::ENOENT => "ENOENT",
            libc::ENOEXEC => "ENOEXEC",
            libc::ENOKEY => "ENOKEY",
            libc::ENOLCK => "ENOLCK",
            libc::ENOLINK => "ENOLINK",
            libc::ENOMEDIUM => "ENOMEDIUM",
            libc::ENOMEM => "ENOMEM",
            libc::ENOMSG => "ENOMSG",
            libc::ENONET => "ENONET",
            libc::ENOPKG => "ENOPKG",
            libc::ENOPROTOOPT => "ENOPROTOOPT",
            libc::ENOSPC => "ENOSPC",
            libc::ENOSR => "ENOSR",
            libc::ENOSTR => "ENOSTR",
            libc::ENOSYS => "ENOSYS",
            libc::ENOTBLK => "ENOTBLK",
            libc::ENOTCONN => "ENOTCONN",
            libc::ENOTDIR => "ENOTDIR",
            libc::ENOTEMPTY => "ENOTEMPTY",
            libc::ENOTRECOVERABLE => "ENOTRECOVERABLE",
            libc::ENOTSOCK => "ENOTSOCK",
            libc::ENOTSUP => "ENOTSUP",
            libc::ENOTTY => "ENOTTY",
            libc::ENOTUNIQ => "ENOTUNIQ",
            libc::ENXIO => "ENXIO",
            // libc::EOPNOTSUPP => "EOPNOTSUPP",
            libc::EOVERFLOW => "EOVERFLOW",
            libc::EOWNERDEAD => "EOWNERDEAD",
            libc::EPERM => "EPERM",
            libc::EPFNOSUPPORT => "EPFNOSUPPORT",
            libc::EPIPE => "EPIPE",
            libc::EPROTO => "EPROTO",
            libc::EPROTONOSUPPORT => "EPROTONOSUPPORT",
            libc::EPROTOTYPE => "EPROTOTYPE",
            libc::ERANGE => "ERANGE",
            libc::EREMCHG => "EREMCHG",
            libc::EREMOTE => "EREMOTE",
            libc::EREMOTEIO => "EREMOTEIO",
            libc::ERESTART => "ERESTART",
            libc::ERFKILL => "ERFKILL",
            libc::EROFS => "EROFS",
            libc::ESHUTDOWN => "ESHUTDOWN",
            libc::ESPIPE => "ESPIPE",
            libc::ESOCKTNOSUPPORT => "ESOCKTNOSUPPORT",
            libc::ESRCH => "ESRCH",
            libc::ESTALE => "ESTALE",
            libc::ESTRPIPE => "ESTRPIPE",
            libc::ETIME => "ETIME",
            libc::ETIMEDOUT => "ETIMEDOUT",
            libc::ETOOMANYREFS => "ETOOMANYREFS",
            libc::ETXTBSY => "ETXTBSY",
            libc::EUCLEAN => "EUCLEAN",
            libc::EUNATCH => "EUNATCH",
            libc::EUSERS => "EUSERS",
            // libc::EWOULDBLOCK => "EWOULDBLOCK",
            libc::EXDEV => "EXDEV",
            libc::EXFULL => "EXFULL",
            _ => None?,
        }
        .to_owned();
        Some(Self { name })
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn number(&self) -> Option<c_int> {
        let num = match self.name.as_str() {
            "E2BIG" => libc::E2BIG,
            "EACCES" => libc::EACCES,
            "EADDRINUSE" => libc::EADDRINUSE,
            "EADDRNOTAVAIL" => libc::EADDRNOTAVAIL,
            "EAFNOSUPPORT" => libc::EAFNOSUPPORT,
            "EAGAIN" => libc::EAGAIN,
            "EALREADY" => libc::EALREADY,
            "EBADE" => libc::EBADE,
            "EBADF" => libc::EBADF,
            "EBADFD" => libc::EBADFD,
            "EBADMSG" => libc::EBADMSG,
            "EBADR" => libc::EBADR,
            "EBADRQC" => libc::EBADRQC,
            "EBADSLT" => libc::EBADSLT,
            "EBUSY" => libc::EBUSY,
            "ECANCELED" => libc::ECANCELED,
            "ECHILD" => libc::ECHILD,
            "ECHRNG" => libc::ECHRNG,
            "ECOMM" => libc::ECOMM,
            "ECONNABORTED" => libc::ECONNABORTED,
            "ECONNREFUSED" => libc::ECONNREFUSED,
            "ECONNRESET" => libc::ECONNRESET,
            "EDEADLK" => libc::EDEADLK,
            "EDEADLOCK" => libc::EDEADLOCK,
            "EDESTADDRREQ" => libc::EDESTADDRREQ,
            "EDOM" => libc::EDOM,
            "EDQUOT" => libc::EDQUOT,
            "EEXIST" => libc::EEXIST,
            "EFAULT" => libc::EFAULT,
            "EFBIG" => libc::EFBIG,
            "EHOSTDOWN" => libc::EHOSTDOWN,
            "EHOSTUNREACH" => libc::EHOSTUNREACH,
            "EHWPOISON" => libc::EHWPOISON,
            "EIDRM" => libc::EIDRM,
            "EILSEQ" => libc::EILSEQ,
            "EINPROGRESS" => libc::EINPROGRESS,
            "EINTR" => libc::EINTR,
            "EINVAL" => libc::EINVAL,
            "EIO" => libc::EIO,
            "EISCONN" => libc::EISCONN,
            "EISDIR" => libc::EISDIR,
            "EISNAM" => libc::EISNAM,
            "EKEYEXPIRED" => libc::EKEYEXPIRED,
            "EKEYREJECTED" => libc::EKEYREJECTED,
            "EKEYREVOKED" => libc::EKEYREVOKED,
            "EL2HLT" => libc::EL2HLT,
            "EL2NSYNC" => libc::EL2NSYNC,
            "EL3HLT" => libc::EL3HLT,
            "EL3RST" => libc::EL3RST,
            "ELIBACC" => libc::ELIBACC,
            "ELIBBAD" => libc::ELIBBAD,
            "ELIBMAX" => libc::ELIBMAX,
            "ELIBSCN" => libc::ELIBSCN,
            "ELIBEXEC" => libc::ELIBEXEC,
            "ELNRNG" => libc::ELNRNG,
            "ELOOP" => libc::ELOOP,
            "EMEDIUMTYPE" => libc::EMEDIUMTYPE,
            "EMFILE" => libc::EMFILE,
            "EMLINK" => libc::EMLINK,
            "EMSGSIZE" => libc::EMSGSIZE,
            "EMULTIHOP" => libc::EMULTIHOP,
            "ENAMETOOLONG" => libc::ENAMETOOLONG,
            "ENETDOWN" => libc::ENETDOWN,
            "ENETRESET" => libc::ENETRESET,
            "ENETUNREACH" => libc::ENETUNREACH,
            "ENFILE" => libc::ENFILE,
            "ENOANO" => libc::ENOANO,
            "ENOBUFS" => libc::ENOBUFS,
            "ENODATA" => libc::ENODATA,
            "ENODEV" => libc::ENODEV,
            "ENOENT" => libc::ENOENT,
            "ENOEXEC" => libc::ENOEXEC,
            "ENOKEY" => libc::ENOKEY,
            "ENOLCK" => libc::ENOLCK,
            "ENOLINK" => libc::ENOLINK,
            "ENOMEDIUM" => libc::ENOMEDIUM,
            "ENOMEM" => libc::ENOMEM,
            "ENOMSG" => libc::ENOMSG,
            "ENONET" => libc::ENONET,
            "ENOPKG" => libc::ENOPKG,
            "ENOPROTOOPT" => libc::ENOPROTOOPT,
            "ENOSPC" => libc::ENOSPC,
            "ENOSR" => libc::ENOSR,
            "ENOSTR" => libc::ENOSTR,
            "ENOSYS" => libc::ENOSYS,
            "ENOTBLK" => libc::ENOTBLK,
            "ENOTCONN" => libc::ENOTCONN,
            "ENOTDIR" => libc::ENOTDIR,
            "ENOTEMPTY" => libc::ENOTEMPTY,
            "ENOTRECOVERABLE" => libc::ENOTRECOVERABLE,
            "ENOTSOCK" => libc::ENOTSOCK,
            "ENOTSUP" => libc::ENOTSUP,
            "ENOTTY" => libc::ENOTTY,
            "ENOTUNIQ" => libc::ENOTUNIQ,
            "ENXIO" => libc::ENXIO,
            "EOPNOTSUPP" => libc::EOPNOTSUPP,
            "EOVERFLOW" => libc::EOVERFLOW,
            "EOWNERDEAD" => libc::EOWNERDEAD,
            "EPERM" => libc::EPERM,
            "EPFNOSUPPORT" => libc::EPFNOSUPPORT,
            "EPIPE" => libc::EPIPE,
            "EPROTO" => libc::EPROTO,
            "EPROTONOSUPPORT" => libc::EPROTONOSUPPORT,
            "EPROTOTYPE" => libc::EPROTOTYPE,
            "ERANGE" => libc::ERANGE,
            "EREMCHG" => libc::EREMCHG,
            "EREMOTE" => libc::EREMOTE,
            "EREMOTEIO" => libc::EREMOTEIO,
            "ERESTART" => libc::ERESTART,
            "ERFKILL" => libc::ERFKILL,
            "EROFS" => libc::EROFS,
            "ESHUTDOWN" => libc::ESHUTDOWN,
            "ESPIPE" => libc::ESPIPE,
            "ESOCKTNOSUPPORT" => libc::ESOCKTNOSUPPORT,
            "ESRCH" => libc::ESRCH,
            "ESTALE" => libc::ESTALE,
            "ESTRPIPE" => libc::ESTRPIPE,
            "ETIME" => libc::ETIME,
            "ETIMEDOUT" => libc::ETIMEDOUT,
            "ETOOMANYREFS" => libc::ETOOMANYREFS,
            "ETXTBSY" => libc::ETXTBSY,
            "EUCLEAN" => libc::EUCLEAN,
            "EUNATCH" => libc::EUNATCH,
            "EUSERS" => libc::EUSERS,
            "EWOULDBLOCK" => libc::EWOULDBLOCK,
            "EXDEV" => libc::EXDEV,
            "EXFULL" => libc::EXFULL,
            _ => return None,
        };
        Some(num)
    }

    fn libc_descripton(&self) -> String {
        let desc_ptr = self
            .number()
            .map(|num| unsafe { strerror(num) })
            .unwrap_or(std::ptr::null_mut());

        if desc_ptr.is_null() {
            "Unknown error".to_owned()
        } else {
            let desc = unsafe { CStr::from_ptr(desc_ptr) };
            desc.to_str().expect("UTF-8").to_owned()
        }
    }

    fn manpages_description(&self) -> String {
        match self.name.as_str() {
            "E2BIG" => "Argument list too long (POSIX.1-2001).",
            "EACCES" => "Permission denied (POSIX.1-2001).",
            "EADDRINUSE" => "Address already in use (POSIX.1-2001).",
            "EADDRNOTAVAIL" => "Address not available (POSIX.1-2001).",
            "EAFNOSUPPORT" => "Address family not supported (POSIX.1-2001).",
            "EAGAIN" => "Resource temporarily unavailable (may be the same value as EWOULDBLOCK) (POSIX.1-2001).",
            "EALREADY" => "Connection already in progress (POSIX.1-2001).",
            "EBADE" => "Invalid exchange.",
            "EBADF" => "Bad file descriptor (POSIX.1-2001).",
            "EBADFD" => "File descriptor in bad state.",
            "EBADMSG" => "Bad message (POSIX.1-2001).",
            "EBADR" => "Invalid request descriptor.",
            "EBADRQC" => "Invalid request code.",
            "EBADSLT" => "Invalid slot.",
            "EBUSY" => "Device or resource busy (POSIX.1-2001).",
            "ECANCELED" => "Operation canceled (POSIX.1-2001).",
            "ECHILD" => "No child processes (POSIX.1-2001).",
            "ECHRNG" => "Channel number out of range.",
            "ECOMM" => "Communication error on send.",
            "ECONNABORTED" => "Connection aborted (POSIX.1-2001).",
            "ECONNREFUSED" => "Connection refused (POSIX.1-2001).",
            "ECONNRESET" => "Connection reset (POSIX.1-2001).",
            "EDEADLK" => "Resource deadlock avoided (POSIX.1-2001).",
            "EDEADLOCK" => "On most architectures, a synonym for EDEADLK. On some architectures (e.g., Linux MIPS, PowerPC, SPARC), it is a separate error code \"File locking deadlock error\".",
            "EDESTADDRREQ" => "Destination address required (POSIX.1-2001).",
            "EDOM" => "Mathematics argument out of domain of function (POSIX.1, C99).",
            "EDQUOT" => "Disk quota exceeded (POSIX.1-2001).",
            "EEXIST" => "File exists (POSIX.1-2001).",
            "EFAULT" => "Bad address (POSIX.1-2001).",
            "EFBIG" => "File too large (POSIX.1-2001).",
            "EHOSTDOWN" => "Host is down.",
            "EHOSTUNREACH" => "Host is unreachable (POSIX.1-2001).",
            "EHWPOISON" => "Memory page has hardware error.",
            "EIDRM" => "Identifier removed (POSIX.1-2001).",
            "EILSEQ" => "Invalid or incomplete multibyte or wide character (POSIX.1, C99). The text shown here is the glibc error description; in POSIX.1, this error is described as \"Illegal byte sequence\".",
            "EINPROGRESS" => "Operation in progress (POSIX.1-2001).",
            "EINTR" => "Interrupted function call (POSIX.1-2001); see signal(7).",
            "EINVAL" => "Invalid argument (POSIX.1-2001).",
            "EIO" => "Input/output error (POSIX.1-2001).",
            "EISCONN" => "Socket is connected (POSIX.1-2001).",
            "EISDIR" => "Is a directory (POSIX.1-2001).",
            "EISNAM" => "Is a named type file.",
            "EKEYEXPIRED" => "Key has expired.",
            "EKEYREJECTED" => "Key was rejected by service.",
            "EKEYREVOKED" => "Key has been revoked.",
            "EL2HLT" => "Level 2 halted.",
            "EL2NSYNC" => "Level 2 not synchronized.",
            "EL3HLT" => "Level 3 halted.",
            "EL3RST" => "Level 3 reset.",
            "ELIBACC" => "Cannot access a needed shared library.",
            "ELIBBAD" => "Accessing a corrupted shared library.",
            "ELIBMAX" => "Attempting to link in too many shared libraries.",
            "ELIBSCN" => ".lib section in a.out corrupted",
            "ELIBEXEC" => "Cannot exec a shared library directly.",
            "ELNRNG" => "Link number out of range.",
            "ELOOP" => "Too many levels of symbolic links (POSIX.1-2001).",
            "EMEDIUMTYPE" => "Wrong medium type.",
            "EMFILE" => "Too many open files (POSIX.1-2001). Commonly caused by exceeding the RLIMIT_NOFILE resource limit described in getrlimit(2). Can also be caused by exceeding the limit specified in /proc/sys/fs/nr_open.",
            "EMLINK" => "Too many links (POSIX.1-2001).",
            "EMSGSIZE" => "Message too long (POSIX.1-2001).",
            "EMULTIHOP" => "Multihop attempted (POSIX.1-2001).",
            "ENAMETOOLONG" => "Filename too long (POSIX.1-2001).",
            "ENETDOWN" => "Network is down (POSIX.1-2001).",
            "ENETRESET" => "Connection aborted by network (POSIX.1-2001).",
            "ENETUNREACH" => "Network unreachable (POSIX.1-2001).",
            "ENFILE" => "Too many open files in system (POSIX.1-2001). On Linux, this is probably a result of encountering the /proc/sys/fs/file-max limit (see proc(5)).",
            "ENOANO" => "No anode.",
            "ENOBUFS" => "No buffer space available (POSIX.1 (XSI STREAMS option)).",
            "ENODATA" => "The named attribute does not exist, or the process has no access to this attribute; see xattr(7). In POSIX.1-2001 (XSI STREAMS option), this error was described as \"No message is available on the STREAM head read queue\".",
            "ENODEV" => "No such device (POSIX.1-2001).",
            "ENOENT" => "No such file or directory (POSIX.1-2001). Typically, this error results when a specified pathname does not exist, or one of the components in the directory prefix of a pathname does not exist, or the specified pathname is a dangling symbolic link.",
            "ENOEXEC" => "Exec format error (POSIX.1-2001).",
            "ENOKEY" => "Required key not available.",
            "ENOLCK" => "No locks available (POSIX.1-2001).",
            "ENOLINK" => "Link has been severed (POSIX.1-2001).",
            "ENOMEDIUM" => "No medium found.",
            "ENOMEM" => "Not enough space/cannot allocate memory (POSIX.1-2001).",
            "ENOMSG" => "No message of the desired type (POSIX.1-2001).",
            "ENONET" => "Machine is not on the network.",
            "ENOPKG" => "Package not installed.",
            "ENOPROTOOPT" => "Protocol not available (POSIX.1-2001).",
            "ENOSPC" => "No space left on device (POSIX.1-2001).",
            "ENOSR" => "No STREAM resources (POSIX.1 (XSI STREAMS option)).",
            "ENOSTR" => "Not a STREAM (POSIX.1 (XSI STREAMS option)).",
            "ENOSYS" => "Function not implemented (POSIX.1-2001).",
            "ENOTBLK" => "Block device required.",
            "ENOTCONN" => "The socket is not connected (POSIX.1-2001).",
            "ENOTDIR" => "Not a directory (POSIX.1-2001).",
            "ENOTEMPTY" => "Directory not empty (POSIX.1-2001).",
            "ENOTRECOVERABLE" => "State not recoverable (POSIX.1-2008).",
            "ENOTSOCK" => "Not a socket (POSIX.1-2001).",
            "ENOTSUP" => "Operation not supported (POSIX.1-2001).",
            "ENOTTY" => "Inappropriate I/O control operation (POSIX.1-2001).",
            "ENOTUNIQ" => "Name not unique on network.",
            "ENXIO" => "No such device or address (POSIX.1-2001).",
            "EOPNOTSUPP" => "Operation not supported on socket (POSIX.1-2001). (ENOTSUP and EOPNOTSUPP have the same value on Linux, but according to POSIX.1 these error values should be distinct.)",
            "EOVERFLOW" => "Value too large to be stored in data type (POSIX.1-2001).",
            "EOWNERDEAD" => "Owner died (POSIX.1-2008).",
            "EPERM" => "Operation not permitted (POSIX.1-2001).",
            "EPFNOSUPPORT" => "Protocol family not supported.",
            "EPIPE" => "Broken pipe (POSIX.1-2001).",
            "EPROTO" => "Protocol error (POSIX.1-2001).",
            "EPROTONOSUPPORT" => "Protocol not supported (POSIX.1-2001).",
            "EPROTOTYPE" => "Protocol wrong type for socket (POSIX.1-2001).",
            "ERANGE" => "Result too large (POSIX.1, C99).",
            "EREMCHG" => "Remote address changed.",
            "EREMOTE" => "Object is remote.",
            "EREMOTEIO" => "Remote I/O error.",
            "ERESTART" => "Interrupted system call should be restarted.",
            "ERFKILL" => "Operation not possible due to RF-kill.",
            "EROFS" => "Read-only filesystem (POSIX.1-2001).",
            "ESHUTDOWN" => "Cannot send after transport endpoint shutdown.",
            "ESPIPE" => "Invalid seek (POSIX.1-2001).",
            "ESOCKTNOSUPPORT" => "Socket type not supported.",
            "ESRCH" => "No such process (POSIX.1-2001).",
            "ESTALE" => "Stale file handle (POSIX.1-2001). This error can occur for NFS and for other filesystems.",
            "ESTRPIPE" => "Streams pipe error.",
            "ETIME" => "Timer expired (POSIX.1 (XSI STREAMS option)). (POSIX.1 says \"STREAM ioctl(2) timeout\".)",
            "ETIMEDOUT" => "Connection timed out (POSIX.1-2001).",
            "ETOOMANYREFS" => "Too many references: cannot splice.",
            "ETXTBSY" => "Text file busy (POSIX.1-2001).",
            "EUCLEAN" => "Structure needs cleaning.",
            "EUNATCH" => "Protocol driver not attached.",
            "EUSERS" => "Too many users.",
            "EWOULDBLOCK" => "Operation would block (may be same value as EAGAIN) (POSIX.1-2001).",
            "EXDEV" => "Improper link (POSIX.1-2001).",
            "EXFULL" => "Exchange full.",
            _ => "Unknown error",
        }
        .to_owned()
    }
}
