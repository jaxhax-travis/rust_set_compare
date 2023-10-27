use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Takes two data set files and compares them and output stats
struct Args {
    /// Text file with the first (or previous) data
    file_one: std::path::PathBuf,

    /// Text file with the second (or next) data
    file_two: std::path::PathBuf,

    /// List out new and missing records from the second set.
    #[arg(short, long, action)]
    difference: bool,

    /// Show records that are in either set, but not both.
    #[arg(short, long, action)]
    symmetric: bool,

    /// Show all the common records in both sets.
    #[arg(short, long, action)]
    common: bool,

    /// Show all unique values across both sets.
    #[arg(short, long, action)]
    union: bool,

    /// Show all of the set lists (VERBOSE)
    #[arg(short, long, action)]
    all: bool
}

// Color code constants
const RED: &str = "\x1b[31m";
const GRN: &str = "\x1b[32m";
const BLU: &str = "\x1b[34;1m";
const YLW: &str = "\x1b[33;1m";
const CLR: &str = "\x1b[m";

/************************  Print Functions   ************************/
/// Simple function to print a key value pair in a formatted manner
fn print_kv_info(key: &str, val: String) {
    println!(" {}[*]{} {}: {}", BLU, CLR, key, val);
}

/// Print each record in a vector of strings.
fn print_records(dataset: &[&String]) {
    let records = sort_target_vector(dataset);
    for record in records {
        println!("\t{}", record);
    }
}

// Print a header and count, and the record set if print_flag is true.
fn print_results(header: &str, dataset: &[&String], print_flag: bool) {
    print_kv_info(format!("{}{}{}", YLW, header, CLR).as_str(), dataset.len().to_string());
    if print_flag {
        print_records(&dataset);
    }
}

/// Print and error message and exit with status code 1.
fn print_fatal(msg: &str) {
    println!(" {}[!] ERROR:{} {}", RED, CLR, msg);
    process::exit(1);
}

/************************  File Functions   ************************/
/// Read each line of a file into a String hashset.
fn read_file_into_hashset(filepath: std::path::PathBuf) -> HashSet<String> {
    let mut dataset: HashSet<String> = HashSet::new();

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap().trim().to_string().len() > 0 {
            dataset.insert(line.unwrap().trim().to_string());
        }
    }

    return dataset
}

/************************  Sort Functions   ************************/
/// Check if string value is an IP address.
fn is_ip(val: &String) -> bool {
    // Split the string by octals
    let parts: Vec<String> = val.split(".").map(|s| s.to_string()).collect();

    // Check that the split yielded 4 parts.
    if parts.len() != 4 { 
        return false
    }

    // Check that the 4 parts are 0-255.
    for part in parts {
        if part.parse::<u8>().is_ok() == false {
            return false
        }
    }

    return true
}

/// Convert an IP address string to a decimal number
fn ip_to_u32(val: &str) -> u32 {
    if is_ip(&val.to_string()) == false {
        return 0
    }

    let parts: Vec<String> = val.split(".").map(|s| s.to_string()).collect();
    let mut ip_decimal: u32 = 0;
    for (idx, part) in parts.iter().enumerate() {
        ip_decimal += part.parse::<u32>().unwrap() << (8 * (3-idx));
    }
    return ip_decimal
}

/// Sort a vector by IP address first and everything else afterwards.
fn sort_target_vector<'a>(dataset: &[&'a String]) -> Vec<&'a String> {
    let mut ips: Vec<&String> = Vec::new();
    let mut hostnames: Vec<&String> = Vec::new();

    // Split up the IP Addresses and Hostnames.
    for record in dataset {
        if is_ip(record) {
            ips.push(record);
        } else {
            hostnames.push(record);
        }
    }

    // Sort the sets
    ips.sort_by(|a, b| ip_to_u32(a).cmp(&ip_to_u32(b)));
    hostnames.sort();

    // Join them hostnames to the ips
    ips.append(&mut hostnames); 
    return ips
}

fn main() {
    // Print the Banner.
    println!("\n\t{}---===[ Compare Sets ]===---{}\n", YLW, CLR);

    // Parse the Arguments
    let args = Args::parse();

    // Print the files being processed
    print_kv_info("First file", format!("{}{}{}", GRN, args.file_one.display().to_string(), CLR));
    print_kv_info("Second file", format!("{}{}{}", GRN, args.file_two.display().to_string(), CLR));

    // Load the files in to hashsets.
    let set_one = read_file_into_hashset(args.file_one);
    let set_two = read_file_into_hashset(args.file_two);
    let set_one_len = set_one.len() as i64;
    let set_two_len = set_two.len() as i64;

    // Handle an empty set.
    if set_one_len == 0 || set_two_len == 0 {
        print_fatal("One of the two sets is empty. Exiting!!\n");
    }

    // Print the Set Counts
    print_kv_info("First Set Unique Records", set_one.len().to_string());
    print_kv_info("Second Set Unique Records", set_two.len().to_string());
    if (set_two_len - set_one_len) < 0 {
        print_kv_info("Difference", format!("{}{}{}", RED, set_two_len - set_one_len, CLR));
    } else {
        print_kv_info("Difference", ((set_two.len() - set_one.len()) as i64).to_string());
    }
    println!("");

    // Get the differences.
    let mut union_set = set_one.union(&set_two).collect::<Vec<&String>>();
    let mut intersection_set = set_one.intersection(&set_two).collect::<Vec<&String>>();
    let mut symmetric_set = set_one.symmetric_difference(&set_two).collect::<Vec<&String>>();
    let mut set_one_diff = set_one.difference(&set_two).collect::<Vec<&String>>();
    let mut set_two_diff = set_two.difference(&set_one).collect::<Vec<&String>>();

    // Sort the sets.
    union_set.sort();
    intersection_set.sort();
    symmetric_set.sort();
    set_one_diff.sort();
    set_two_diff.sort();

    // Output the results.
    print_results("Total Unique Records in Both Sets", &union_set, args.union || args.all);
    print_results("Common Records Found in Both Sets", &intersection_set, args.common || args.all);
    print_results("Symmetrical Differences between the Two Sets", &symmetric_set, args.symmetric || args.all);
    print_results("New Records in Second Set", &set_two_diff, args.difference || args.all);
    print_results("Missing Records in Second Set", &set_one_diff, args.difference || args.all);

    println!("\n {}[+] Done!{}\n", GRN, CLR);
}
