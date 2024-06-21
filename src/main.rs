use clap::Parser;
use rcli::{process_name, Opts, SubCommand};

// fn example() -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     for result in rdr.deserialize() {
//         // Notice that we need to provide a type hint for automatic
//         // deserialization.
//         let record: Record = result?;
//         println!("{:?}", record);
//     }
//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv) => process_name(&csv.input, &csv.output)?,
    }
    Ok(())
}
