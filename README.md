# rust_set_compare
A simple rust project that takes two text files of line items and helps compare the unique differences in them with union, intersection, symmetric_difference, etc. Since this reads each line into a hashset, it will deduplicate any duplicate records. This can be useful for finding what lines are new or missing between two list files, or what they share in common.

Since I mostly use this to sort list of IP addresses and hostnames, this tool sorts the output by IP addresses first then hostnames. IP addresses should be sorted in the correct order. Hostnames are sorted in the normally.

# Getting Started

## Installation

### Binary Release (Windows)
A pre-compiled version of the application in a zip file can be found in the `release` tab on this repository.

### Build From Source
The application can be built using cargo build --release. The resulting binary will be found in target/release/.

# Help Screen

```
        ---===[ Compare Sets ]===---

Compare two data set text files and print stats about them.

Usage: rust_set_compare [OPTIONS] <FILE_ONE> <FILE_TWO>

Arguments:
  <FILE_ONE>  Text file with the first (or previous) data
  <FILE_TWO>  Text file with the second (or next) data

Options:
  -d, --difference  List out new and missing records from the second set
  -s, --symmetric   Show records that are in either set, but not both
  -c, --common      Show all the common records in both sets
  -u, --union       Show all unique values across both sets
  -a, --all         Show all of the set lists (VERBOSE)
  -h, --help        Print help
  -V, --version     Print version
```


# Example
## Sample Files
Consider having these two files -> `test1.txt` and `test2.txt` with the following contents:
```
$ cat test1.txt
192.168.1.1
192.168.1.5
192.168.1.6
192.168.1.7
192.168.1.8
192.168.1.9
192.168.1.10
192.168.1.11
192.168.1.12
192.168.1.13
www.example.com
10.1.1.1
10.1.1.2
10.1.1.3
10.1.1.4
10.1.1.5
10.1.1.6

$ cat test2.txt
192.168.1.8
192.168.1.9
192.168.1.10
192.168.1.11
192.168.1.12
192.168.1.13
192.168.1.14
192.168.1.15
192.168.1.16
192.168.1.17
192.168.1.18
192.168.1.19
```

## General Stats
You can see there are some common values, and differences. This tool if given these files without any other switches would just provide general stats about the files.

```
$ ./rust_set_compare test1.txt test2.txt

        ---===[ Compare Sets ]===---

 [*] First file: /tmp/test1.txt
 [*] Second file: /tmp/test2.txt
 [*] First Set Unique Records: 17
 [*] Second Set Unique Records: 12
 [*] Difference: -5

 [*] Total Unique Records in Both Sets: 23
 [*] Common Records Found in Both Sets: 6
 [*] Symmetrical Differences between the Two Sets: 17
 [*] New Records in Second Set: 6
 [*] Missing Records in Second Set: 11

 [+] Done!
```

## Finding Common Records
Just add the `--common` `-c` switch!  This will still show the stats, but the stat about the common records will dump a list of them.

```
$ ./rust_set_compare test1.txt test2.txt --common
        ---===[ Compare Sets ]===---

 [*] First file: /tmp/test1.txt
 [*] Second file: /tmp/test2.txt
 [*] First Set Unique Records: 17
 [*] Second Set Unique Records: 12
 [*] Difference: -5

 [*] Total Unique Records in Both Sets: 23
 [*] Common Records Found in Both Sets: 6
        192.168.1.8
        192.168.1.9
        192.168.1.10
        192.168.1.11
        192.168.1.12
        192.168.1.13
 [*] Symmetrical Differences between the Two Sets: 17
 [*] New Records in Second Set: 6
 [*] Missing Records in Second Set: 11

 [+] Done!
```

## Finding the Differences
There are really two methods here.  It depends on if you want the new and missing broken out, or just a listing of both together.


### `--difference`
`--difference` will provide two lists under the `New Records in Second Set` and `Missing Records in Second Set` headers to show whats different about the second file.

```
$ ./rust_set_compare test1.txt test2.txt --difference

        ---===[ Compare Sets ]===---

 [*] First file: /tmp/test1.txt
 [*] Second file: /tmp/test2.txt
 [*] First Set Unique Records: 17
 [*] Second Set Unique Records: 12
 [*] Difference: -5

 [*] Total Unique Records in Both Sets: 23
 [*] Common Records Found in Both Sets: 6
 [*] Symmetrical Differences between the Two Sets: 17
 [*] New Records in Second Set: 6
        192.168.1.14
        192.168.1.15
        192.168.1.16
        192.168.1.17
        192.168.1.18
        192.168.1.19
 [*] Missing Records in Second Set: 11
        10.1.1.1
        10.1.1.2
        10.1.1.3
        10.1.1.4
        10.1.1.5
        10.1.1.6
        192.168.1.1
        192.168.1.5
        192.168.1.6
        192.168.1.7
        www.example.com

 [+] Done!
```


### `--symmetric`
`--symmetric` will show a list that combines the records that exist in one set, but not the other, across both sets.

```
$ ./rust_set_compare test1.txt test2.txt --symmetric

        ---===[ Compare Sets ]===---

 [*] First file: /tmp/test1.txt
 [*] Second file: /tmp/test2.txt
 [*] First Set Unique Records: 17
 [*] Second Set Unique Records: 12
 [*] Difference: -5

 [*] Total Unique Records in Both Sets: 23
 [*] Common Records Found in Both Sets: 6
 [*] Symmetrical Differences between the Two Sets: 17
        10.1.1.1
        10.1.1.2
        10.1.1.3
        10.1.1.4
        10.1.1.5
        10.1.1.6
        192.168.1.1
        192.168.1.5
        192.168.1.6
        192.168.1.7
        192.168.1.14
        192.168.1.15
        192.168.1.16
        192.168.1.17
        192.168.1.18
        192.168.1.19
        www.example.com
 [*] New Records in Second Set: 6
 [*] Missing Records in Second Set: 11

 [+] Done!
```


## Get a list of all unique records in both sets
To get a list of all the unique records in both sets, use the `--union` switch.

The difference between the `--union` switch and the `--common` switch is that `--common` shows records shared in both sets, where `--union` is all records, regardless of if they are in one set or both sets.

```
$ ./rust_set_compare test1.txt test2.txt --union

        ---===[ Compare Sets ]===---

 [*] First file: /tmp/test1.txt
 [*] Second file: /tmp/test2.txt
 [*] First Set Unique Records: 17
 [*] Second Set Unique Records: 12
 [*] Difference: -5

 [*] Total Unique Records in Both Sets: 23
        10.1.1.1
        10.1.1.2
        10.1.1.3
        10.1.1.4
        10.1.1.5
        10.1.1.6
        192.168.1.1
        192.168.1.5
        192.168.1.6
        192.168.1.7
        192.168.1.8
        192.168.1.9
        192.168.1.10
        192.168.1.11
        192.168.1.12
        192.168.1.13
        192.168.1.14
        192.168.1.15
        192.168.1.16
        192.168.1.17
        192.168.1.18
        192.168.1.19
        www.example.com
 [*] Common Records Found in Both Sets: 6
 [*] Symmetrical Differences between the Two Sets: 17
 [*] New Records in Second Set: 6
 [*] Missing Records in Second Set: 11

 [+] Done!
```


## `--all` Switch
This --all is a switch that will effectively combine the following four switches:

`--difference --symmetric --common --union`

With that said, this output can be kind of long. So the example here in the read me will be omitted.