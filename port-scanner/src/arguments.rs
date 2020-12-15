use std::net::IpAddr;
use std::str::FromStr;

pub struct Argument {
    flag: String,
    pub(crate) ipaddr: IpAddr,
    pub(crate) threads: u16,
}

impl Argument {
    pub fn new(args: &[String]) -> Result<Argument, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too much arguments");
        }
        let next = args[1].clone();
        match IpAddr::from_str(&next) {
            Ok(ipaddr) => Ok(Argument {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            }),
            _ => {
                if next.contains("-h") || next.contains("--help") && args.len() == 2 {
                    Err("help")
                } else if next.contains("-t") == true {
                    let threads = match args[2].parse::<u16>() {
                        Ok(v) => v,
                        Err(_) => return Err("fail to parse threads number"),
                    };
                    let ipaddr = match IpAddr::from_str(args[3].as_str()) {
                        Ok(v) => v,
                        Err(_) => return Err("fail to parse ipaddr"),
                    };

                    return Ok(Argument {
                        ipaddr,
                        threads,
                        flag: next,
                    });
                } else {
                    Err("please use -h")
                }
            }
        }
    }
}

#[test]
fn arguments_test() {
    let arguments = vec![String::from("program"), String::from("1.1.1.1")];
    match Argument::new(arguments.as_slice()) {
        Ok(v) => {
            assert_eq!(v.flag, String::new());
            assert_eq!(v.threads, 4);
            assert_eq!(v.ipaddr.to_string(), "1.1.1.1".to_owned());
        }
        _ => assert!(true),
    }

    let arguments = vec![
        String::from("program"),
        String::from("-t"),
        String::from("16"),
        String::from("1.1.1.1"),
    ];
    match Argument::new(arguments.as_slice()) {
        Ok(v) => {
            assert_eq!(v.flag, String::from("-t"));
            assert_eq!(v.threads, 16);
            assert_eq!(v.ipaddr.to_string(), "1.1.1.1".to_owned());
        }
        _ => assert!(true),
    }

    let arguments = vec![String::from("program"), String::from("-h")];
    match Argument::new(arguments.as_slice()) {
        Ok(_) => {
            assert!(false);
        }
        Err(v) => assert_eq!(v, "help"),
    }

    let arguments = vec![String::from("program")];
    match Argument::new(arguments.as_slice()) {
        Ok(_) => {
            assert!(false);
        }
        Err(v) => assert_eq!(v, "not enough arguments"),
    }
    let arguments = vec![
        String::from("program"),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    match Argument::new(arguments.as_slice()) {
        Ok(_) => {
            assert!(false);
        }
        Err(v) => assert_eq!(v, "too much arguments"),
    }
}
