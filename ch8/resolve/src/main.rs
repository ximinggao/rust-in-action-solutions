use clap::{Arg, Command};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

fn main() {
    let command = Command::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(Arg::new("dns-server").short('s').default_value("1.1.1.1"))
        .arg(Arg::new("domain-name").required(true))
        .get_matches();

    let raw_domain_name: &String = command.get_one("domain-name").unwrap();
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
}
