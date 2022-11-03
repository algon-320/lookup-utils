#![cfg(all(
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "aarch64"
    )
))]

use box_drawing_table::{ansi_term::Style, Align, Border, Cell, Column, Row, Table};
use clap::Parser;
use libc::{c_int, strsignal};
use std::ffi::CStr;

// NOTE: based on Linux man-pages 6.01

/// A simple utility to look up Linux signals
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    /// signal number (e.g. "2"),
    /// signal name (e.g. "SIGINT"),
    /// or shell status code (e.g. "130") if "-s" option is specified
    query: Vec<String>,

    #[clap(long, default_value_t = false)]
    /// Disable pretty-printing
    simple: bool,

    #[clap(short, long, default_value_t = false)]
    /// List all signals
    list: bool,

    #[clap(short, long, default_value_t = false)]
    /// Interpret numbers as status code instead of signal number
    status: bool,

    #[clap(long, default_value_t = false)]
    /// Display the description using strsignal(3)
    libc: bool,
}

fn main() {
    let args = Args::parse();

    let mut rows = Vec::new();

    let queries = if args.list { list() } else { args.query };
    for q in queries {
        let sigdesc = if let Ok(mut num) = q.parse::<c_int>() {
            if args.status {
                num -= 128;
            }
            SignalDesc::from_number(num).ok_or(num)
        } else {
            Ok(SignalDesc::from_name(q.clone()))
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
                description = "Unknown signal".to_owned();
            }
        }

        if args.simple {
            println!("{} {} {}", name, number, description);
        } else {
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
        create_table(rows);
    }
}

fn create_table(rows: Vec<Row>) {
    let mut table = Table::new(vec![
        Border::Double.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
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
        "SIGABRT",
        "SIGALRM",
        "SIGBUS",
        "SIGCHLD",
        "SIGCLD",
        "SIGCONT",
        "SIGEMT",
        "SIGFPE",
        "SIGHUP",
        "SIGILL",
        "SIGINFO",
        "SIGINT",
        "SIGIO",
        "SIGIOT",
        "SIGKILL",
        "SIGLOST",
        "SIGPIPE",
        "SIGPOLL",
        "SIGPROF",
        "SIGPWR",
        "SIGQUIT",
        "SIGSEGV",
        "SIGSTKFLT",
        "SIGSTOP",
        "SIGTSTP",
        "SIGSYS",
        "SIGTERM",
        "SIGTRAP",
        "SIGTTIN",
        "SIGTTOU",
        "SIGUNUSED",
        "SIGURG",
        "SIGUSR1",
        "SIGUSR2",
        "SIGVTALRM",
        "SIGXCPU",
        "SIGXFSZ",
        "SIGWINCH",
    ]
    .iter()
    .map(|name| name.to_string())
    .collect()
}

struct SignalDesc {
    name: String,
}

impl SignalDesc {
    fn from_name(name: String) -> Self {
        Self { name }
    }

    fn from_number(signum: c_int) -> Option<Self> {
        let name = match signum {
            libc::SIGABRT => "SIGABRT",
            libc::SIGALRM => "SIGALRM",
            libc::SIGBUS => "SIGBUS",
            libc::SIGCHLD => "SIGCHLD",
            libc::SIGCONT => "SIGCONT",
            libc::SIGFPE => "SIGFPE",
            libc::SIGHUP => "SIGHUP",
            libc::SIGILL => "SIGILL",
            libc::SIGINT => "SIGINT",
            libc::SIGIO => "SIGIO",
            libc::SIGKILL => "SIGKILL",
            libc::SIGPIPE => "SIGPIPE",
            libc::SIGPROF => "SIGPROF",
            libc::SIGPWR => "SIGPWR",
            libc::SIGQUIT => "SIGQUIT",
            libc::SIGSEGV => "SIGSEGV",
            libc::SIGSTKFLT => "SIGSTKFLT",
            libc::SIGSTOP => "SIGSTOP",
            libc::SIGTSTP => "SIGTSTP",
            libc::SIGSYS => "SIGSYS",
            libc::SIGTERM => "SIGTERM",
            libc::SIGTRAP => "SIGTRAP",
            libc::SIGTTIN => "SIGTTIN",
            libc::SIGTTOU => "SIGTTOU",
            libc::SIGURG => "SIGURG",
            libc::SIGUSR1 => "SIGUSR1",
            libc::SIGUSR2 => "SIGUSR2",
            libc::SIGVTALRM => "SIGVTALRM",
            libc::SIGXCPU => "SIGXCPU",
            libc::SIGXFSZ => "SIGXFSZ",
            libc::SIGWINCH => "SIGWINCH",
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
            "SIGABRT" => libc::SIGABRT,
            "SIGALRM" => libc::SIGALRM,
            "SIGBUS" => libc::SIGBUS,
            "SIGCHLD" => libc::SIGCHLD,
            "SIGCLD" => libc::SIGCHLD, // !
            "SIGCONT" => libc::SIGCONT,
            "SIGEMT" => return None, // !
            "SIGFPE" => libc::SIGFPE,
            "SIGHUP" => libc::SIGHUP,
            "SIGILL" => libc::SIGILL,
            "SIGINFO" => libc::SIGPWR, // !
            "SIGINT" => libc::SIGINT,
            "SIGIO" => libc::SIGIO,
            "SIGIOT" => libc::SIGIOT,
            "SIGKILL" => libc::SIGKILL,
            "SIGLOST" => return None, // !
            "SIGPIPE" => libc::SIGPIPE,
            "SIGPOLL" => libc::SIGPOLL,
            "SIGPROF" => libc::SIGPROF,
            "SIGPWR" => libc::SIGPWR,
            "SIGQUIT" => libc::SIGQUIT,
            "SIGSEGV" => libc::SIGSEGV,
            "SIGSTKFLT" => libc::SIGSTKFLT,
            "SIGSTOP" => libc::SIGSTOP,
            "SIGTSTP" => libc::SIGTSTP,
            "SIGSYS" => libc::SIGSYS,
            "SIGTERM" => libc::SIGTERM,
            "SIGTRAP" => libc::SIGTRAP,
            "SIGTTIN" => libc::SIGTTIN,
            "SIGTTOU" => libc::SIGTTOU,
            "SIGUNUSED" => libc::SIGSYS, // !
            "SIGURG" => libc::SIGURG,
            "SIGUSR1" => libc::SIGUSR1,
            "SIGUSR2" => libc::SIGUSR2,
            "SIGVTALRM" => libc::SIGVTALRM,
            "SIGXCPU" => libc::SIGXCPU,
            "SIGXFSZ" => libc::SIGXFSZ,
            "SIGWINCH" => libc::SIGWINCH,
            _ => return None,
        };
        Some(num)
    }

    fn libc_descripton(&self) -> String {
        let desc_ptr = self
            .number()
            .map(|num| unsafe { strsignal(num) })
            .unwrap_or(std::ptr::null_mut());

        if desc_ptr.is_null() {
            "Unknown signal".to_owned()
        } else {
            let desc = unsafe { CStr::from_ptr(desc_ptr) };
            desc.to_str().expect("UTF-8").to_owned()
        }
    }

    fn manpages_description(&self) -> String {
        match self.name.as_str() {
            "SIGABRT" => "Abort signal from abort(3)",
            "SIGALRM" => "Timer signal from alarm(2)",
            "SIGBUS" => "Bus error (bad memory access)",
            "SIGCHLD" => "Child stopped or terminated",
            "SIGCLD" => "A synonym for SIGCHLD",
            "SIGCONT" => "Continue if stopped",
            "SIGEMT" => "Emulator trap",
            "SIGFPE" => "Floating-point exception",
            "SIGHUP" => "Hangup detected on controlling terminal or death of controlling process",
            "SIGILL" => "Illegal Instruction",
            "SIGINFO" => "A synonym for SIGPWR",
            "SIGINT" => "Interrupt from keyboard",
            "SIGIO" => "I/O now possible (4.2BSD)",
            "SIGIOT" => "IOT trap. A synonym for SIGABRT",
            "SIGKILL" => "Kill signal",
            "SIGLOST" => "File lock lost (unused)",
            "SIGPIPE" => "Broken pipe: write to pipe with no readers; see pipe(7)",
            "SIGPOLL" => "Pollable event (Sys V); synonym for SIGIO",
            "SIGPROF" => "Profiling timer expired",
            "SIGPWR" => "Power failure (System V)",
            "SIGQUIT" => "Quit from keyboard",
            "SIGSEGV" => "Invalid memory reference",
            "SIGSTKFLT" => "Stack fault on coprocessor (unused)",
            "SIGSTOP" => "Stop process",
            "SIGTSTP" => "Stop typed at terminal",
            "SIGSYS" => "Bad system call (SVr4); see also seccomp(2)",
            "SIGTERM" => "Termination signal",
            "SIGTRAP" => "Trace/breakpoint trap",
            "SIGTTIN" => "Terminal input for background process",
            "SIGTTOU" => "Terminal output for background process",
            "SIGUNUSED" => "Synonymous with SIGSYS",
            "SIGURG" => "Urgent condition on socket (4.2BSD)",
            "SIGUSR1" => "User-defined signal 1",
            "SIGUSR2" => "User-defined signal 2",
            "SIGVTALRM" => "Virtual alarm clock (4.2BSD)",
            "SIGXCPU" => "CPU time limit exceeded (4.2BSD); see setrlimit(2)",
            "SIGXFSZ" => "File size limit exceeded (4.2BSD); see setrlimit(2)",
            "SIGWINCH" => "Window resize signal (4.3BSD, Sun)",
            _ => "Unknown signal",
        }
        .to_owned()
    }
}
