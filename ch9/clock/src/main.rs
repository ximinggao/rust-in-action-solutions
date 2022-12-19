use chrono::{DateTime, Local, TimeZone};
use clap::{Parser, Subcommand, ValueEnum};

#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use std::mem::zeroed;

        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the current time in the specified standard
    Get {
        #[arg(short, long, value_enum, default_value_t = Standard::RFC3339)]
        standard: Standard,
    },

    /// Set the clock with the datetime
    Set {
        #[arg(short, long, value_enum, default_value_t = Standard::RFC3339)]
        standard: Standard,

        datetime: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Standard {
    RFC3339,
    RFC2822,
    Timestamp,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Get { standard }) => {
            let now = Clock::get();
            match standard {
                Standard::RFC3339 => println!("{}", now.to_rfc3339()),
                Standard::RFC2822 => println!("{}", now.to_rfc2822()),
                Standard::Timestamp => println!("{}", now.timestamp()),
            }
        }
        Some(Commands::Set { standard, datetime }) => {
            let parser = match standard {
                Standard::RFC2822 => DateTime::parse_from_rfc2822,
                Standard::RFC3339 => DateTime::parse_from_rfc3339,
                _ => unimplemented!(),
            };

            let err_msg = format!("Unable to parse {} according to {:?}", datetime, standard);
            let t = parser(&datetime).expect(&err_msg);
            Clock::set(t);

            let maybe_error = std::io::Error::last_os_error();
            let os_error_code = maybe_error.raw_os_error();
            match os_error_code {
                Some(0) => {
                    println!("Clock set to {}", datetime);
                }
                Some(_) => {
                    eprintln!("Unable to set the time: {:?}", maybe_error);
                }
                None => (),
            }
        }
        None => {
            println!("Use the default command Get");
        }
    }
}
