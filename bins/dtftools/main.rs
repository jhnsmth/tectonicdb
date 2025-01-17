extern crate itertools;
extern crate clap;
extern crate byteorder;
extern crate tdb_core;
#[macro_use]
extern crate indoc;


mod dtfnumpy;
mod dtfcheck;
mod dtfcat;
mod dtfsplit;
mod dtfconcat;
mod dtfrepair;
use clap::{Arg, App};

fn main() {
    let matches = App::new("dtftools")
        .version(env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT"))
        .author("Ricky Han <tectonic@rickyhan.com>")
        .about("tools for dtf files")
        .subcommand_required(true)
        .subcommand(clap::SubCommand::with_name("cat")
            .about(indoc!("
                Print dtf files to plaintext
                Examples:
                # filter by symbol and epoch under given folder and output csv
                dtftools cat --folder ./test/zrx --symbol bnc_zrx_btc --min 1514764800000 --max 1514851200000 --csv > out
                # count number of updates across files
                dtftools cat --folder ./test/zrx -m
                # same as above but rebin into minute candle
                dtftools cat --folder ./test/zrx --symbol bnc_zrx_btc --min 1514764800000 --max 1514851200000 --csv --timebars > out
                # hour candle
                dtftools cat --folder ./test/zrx --symbol bnc_zrx_btc --min 1514764800000 --max 1514851200000 --csv --timebars -minutes 60 > out
                # read metadata of file
                dtftools cat -m test.dtf
                # convert to csv
                dtftools cat test.dtf --csv
                "))
            .arg(
                Arg::with_name("input")
                    .value_name("INPUT")
                    .help("file to read")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("output file")
                .required(false)
                .takes_value(true),
            )
            .arg(
                Arg::with_name("symbol")
                .long("symbol")
                .value_name("SYMBOL")
                .help("symbol too lookup")
                .required(false)
                .takes_value(true),
            )
            .arg(
                Arg::with_name("min")
                .long("min")
                .value_name("MIN")
                .help("minimum value to filter for")
                .required(false)
                .takes_value(true)
            )
            .arg(
                Arg::with_name("max")
                .long("max")
                .value_name("MAX")
                .help("maximum value to filter for")
                .required(false)
                .takes_value(true)
            )
            .arg(
                Arg::with_name("folder")
                .long("folder")
                .conflicts_with("input")
                .requires("symbol")
                .value_name("FOLDER")
                .help("folder to search")
                .required(false)
                .takes_value(true)
            )
            .arg(
                Arg::with_name("meta")
                    .short('m')
                    .long("show_metadata")
                    .help("read only the metadata"),
            )

            .arg(Arg::with_name("csv")
                .long("csv")
                .conflicts_with("json")
                .help("set output format as csv"))
            .arg(Arg::with_name("json")
                .long("json")
                .conflicts_with("csv")
                .help("set output format as json"))


            .arg(Arg::with_name("timebars")
                .short('t')
                .long("timebars")
                .help("output rebinned time-candles"))
            .arg(Arg::with_name("aligned")
                .short('a')
                .long("aligned")
                .requires("timebars")
                .help(indoc!("
                    align with minute mark (\"snap to grid\")
                    --|------|------|------|-->
                    |
                    ^ discard up to this point
                ")))
            .arg(Arg::with_name("minutes")
                .short('g')
                .long("minutes")
                .required(false)
                .requires("timebars")
                .value_name("MINUTES")
                .help("granularity in minute. e.g. -m 60 # hour candle")
                .takes_value(true))
        )

        .subcommand(clap::SubCommand::with_name("check")
            .about(indoc!("
                Check dtf file for defect
                Examples:
                dtftools check 1.dtf -c
                "))
            .arg(
                Arg::with_name("threshold")
                    .short('t')
                    .long("threshold")
                    .help("gap threshold in seconds")
                    .default_value("60")
                    .value_name("THRESHOLD")
                    .required(false)
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("input")
                    .value_name("INPUT")
                    .help("file to read")
                    .required(true)
                    .takes_value(true)
            ))

        .subcommand(clap::SubCommand::with_name("numpy")
            .about(indoc!("
                Convert dtf files to .npz format
                Examples:
                dtftools numpy 1.dtf -c
                "))
            .arg(
                Arg::with_name("compressed")
                    .short('c')
                    .long("compressed")
                    .help("use Deflated compression")
            )
            .arg(
                Arg::with_name("input")
                    .value_name("INPUT")
                    .help("file to read")
                    .required(true)
                    .takes_value(true),
            ))

        .subcommand(clap::SubCommand::with_name("concat")
                .about(indoc!("
                    Concatenates two DTF files into a single output file.
                    Examples:
                    dtfconcat file1.dtf file2.dtf output.dtf
                    "))
                .arg(
                    Arg::with_name("input1")
                        .value_name("INPUT1")
                        .help("First file to read")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                )
                .arg(
                    Arg::with_name("input2")
                        .value_name("INPUT2")
                        .help("Second file to read")
                        .required(true)
                        .takes_value(true)
                        .index(2)
                )
                .arg(
                    Arg::with_name("output")
                        .value_name("OUTPUT")
                        .help("Output file")
                        .required(true)
                        .takes_value(true)
                        .index(3)
                ))

        .subcommand(clap::SubCommand::with_name("split")
            .about(indoc!("
                Splits big dtf files into smaller ones
                Examples:
                dtftools split test.dtf -f test-{}.dtf
                "))
            .arg(
                Arg::with_name("input")
                    .value_name("INPUT")
                    .help("file to read")
                    .required(true)
                    .takes_value(true))
            .arg(
                Arg::with_name("BATCH")
                    .short('b')
                    .long("batch_size")
                    .value_name("BATCH_SIZE")
                    .help("Specify the number of batches to read")
                    .required(true)
                    .takes_value(true),
            ))
        .subcommand(clap::SubCommand::with_name("repair")
            .about(indoc!("
                Examples:
                dtftools repair test.dtf -o test-repaired.dtf
                "))
            .arg(
                Arg::with_name("input")
                    .value_name("INPUT")
                    .help("file to read")
                    .required(true)
                    .takes_value(true))
            .arg(
                Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("output file")
                .required(false)
                .takes_value(true),
            ))
    .get_matches();

    match matches.subcommand() {
       Some(("cat", sub_matches)) => dtfcat::run(sub_matches),
       Some(("check", sub_matches)) => dtfcheck::run(sub_matches),
       Some(("numpy", sub_matches)) => dtfnumpy::run(sub_matches),
       Some(("split", sub_matches)) => dtfsplit::run(sub_matches),
       Some(("concat", sub_matches)) => dtfconcat::run(sub_matches),
       Some(("repair", sub_matches)) => dtfrepair::run(sub_matches),
        _ => unreachable!(),
    }
}
