//!
//! TPCOptions
//! A simple tool for managing command line options and trace/log/debug
//! instrumentation for the _T_wo _P_hase _C_ommit project. Exports a
//! constructor for a struct that represents command line options for the
//! project, uses the clap crate to collect command line options, and use the
//! log and stderrlog crates to initialize the application to use trace!(),
//! debug!(), info!() etc macros from the log crate.
//!
extern crate log;
extern crate stderrlog;
extern crate clap;
use clap::{Arg, App};

extern crate ctrlc;
#[derive(Clone, Debug)]
pub struct TPCOptions {
    pub send_success_probability: f64,        // Probability that a message send succeeds
    pub operation_success_probability: f64,   // Probability that an operation succeeds
    pub num_clients: u32,                     // Number of concurrent clients issuing requests
    pub num_requests: u32,                    // Number of requests issued per client
    pub num_participants: u32,                // Number of participants in 2PC protocol (not including coordinator)
    pub verbosity: usize,                     // Integer verbosity level. experiment with 0 (default) to 5 (fire-hose of output)
    pub mode: String,                         // One of "run", "client", "particpant", "check"
    pub log_path: String,                     // Directory for client, participant, and coordinator logs
    pub ipc_path: String,                     // Path to IPC socket for setting up communication with the coordinator
    pub num: u32,                             // Participant / Client number for naming the log files
}

impl TPCOptions {

    ///
    /// new()
    /// Returns a new options structure containing CLI options / defaults
    ///
    pub fn new() -> TPCOptions {

        // Default values for CLI options
        let default_send_success_probability = "1.0";
        let default_operation_success_probability = "1.0";
        let default_num_participants = "3";
        let default_num_clients = "3";
        let default_num_requests = "15";
        let default_verbosity = "0";
        let default_mode = "run";
        let default_log_path = "./logs/";
        let default_ipc_path = "none";
        let default_num = "0";

        // Set-Up clap
        let matches = App::new("concurrency-2pc")
            .version("0.1.0")
            .author("Rust Student <student@cs.utexas.edu>")
            .about("2PC exercise written in Rust")
            .arg(Arg::with_name("send_success_probability")
                    .short("S")
                    .required(false)
                    .takes_value(true)
                    .help("Probability participants successfully send messages"))
            .arg(Arg::with_name("operation_success_probability")
                    .short("s")
                    .required(false)
                    .takes_value(true)
                    .help("Probability participants successfully execute requests"))
            .arg(Arg::with_name("num_clients")
                    .short("c")
                    .required(false)
                    .takes_value(true)
                    .help("Number of clients making requests"))
            .arg(Arg::with_name("num_participants")
                    .short("p")
                    .required(false)
                    .takes_value(true)
                    .help("Number of participants in protocol"))
            .arg(Arg::with_name("num_requests")
                    .short("r")
                    .required(false)
                    .takes_value(true)
                    .help("Number of requests made per client"))
            .arg(Arg::with_name("verbosity")
                    .short("v")
                    .required(false)
                    .takes_value(true)
                    .help("Output verbosity: 0->No Output, 5->Output Everything"))
            .arg(Arg::with_name("log_path")
                    .short("l")
                    .required(false)
                    .takes_value(true)
                    .help("Specifies path to directory where logs are stored"))
            .arg(Arg::with_name("mode")
                    .short("m")
                    .required(false)
                    .takes_value(true)
                    .help("Mode: \"run\" starts 2PC, \"client\" starts a client process, \"participant\" starts a participant process, \"check\" checks logs produced by previous run"))
            .arg(Arg::with_name("ipc_path")
                    .long("ipc_path")
                    .required(false)
                    .takes_value(true)
                    .help("Path for IPC socket for communication"))
            .arg(Arg::with_name("num")
                    .long("num")
                    .required(false)
                    .takes_value(true)
                    .help("Participant / Client number for naming the log files. Ranges from 0 to num_clients - 1 or num_participants - 1"))
            .get_matches();

        // Parse CLI options and take default values if none given
        let mode = matches.value_of("mode").unwrap_or(default_mode);
        let operation_success_probability = matches.value_of("operation_success_probability").unwrap_or(default_operation_success_probability).parse::<f64>().unwrap();
        let send_success_probability = matches.value_of("send_success_probability").unwrap_or(default_send_success_probability).parse::<f64>().unwrap();
        let num_clients = matches.value_of("num_clients").unwrap_or(default_num_clients).parse::<u32>().unwrap();
        let num_participants = matches.value_of("num_participants").unwrap_or(default_num_participants).parse::<u32>().unwrap();
        let num_requests = matches.value_of("num_requests").unwrap_or(default_num_requests).parse::<u32>().unwrap();
        let verbosity = matches.value_of("verbosity").unwrap_or(default_verbosity).parse::<usize>().unwrap();
        let log_path = matches.value_of("log_path").unwrap_or(default_log_path);
        let ipc_path = matches.value_of("ipc_path").unwrap_or(default_ipc_path);
        let num = matches.value_of("num").unwrap_or(default_num).parse::<u32>().unwrap();

        // IPC path is necessary for client / participant to communicate with the coordinator
        match mode.as_ref() {
            "run" => {},
            "client" => {
                if ipc_path == default_ipc_path {
                    panic!("No ipc_path specified for client mode");
                }
            },
            "participant" => {
                if ipc_path == default_ipc_path {
                    panic!("No ipc_path specified for participant mode");
                }
            },
            "check" => {},
            _ => panic!("unknown execution mode requested!"),
        }

        TPCOptions {
            send_success_probability: send_success_probability,
            operation_success_probability: operation_success_probability,
            num_clients: num_clients,
            num_participants: num_participants,
            num_requests: num_requests,
            verbosity: verbosity,
            mode: mode.to_string(),
            log_path: log_path.to_string(),
            ipc_path: ipc_path.to_string(),
            num: num,
        }
    }

    ///
    /// Convert this struct to a vector of CLI options
    ///
    /// HINT: This can be useful for passing arguments to children processes
    ///       using the std::process::Command's args(..) function
    pub fn as_vec(&self) -> Vec<String> {
        vec![
            format!("-S{}", self.send_success_probability),
            format!("-s{}", self.operation_success_probability),
            format!("-c{}", self.num_clients),
            format!("-r{}", self.num_requests),
            format!("-p{}", self.num_participants),
            format!("-v{}", self.verbosity),
            format!("-m{}", self.mode),
            format!("-l{}", self.log_path),
            format!("--ipc_path={}", self.ipc_path),
            format!("--num={}", self.num),
        ]
    }
}
