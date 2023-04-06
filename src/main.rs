use rust_img_ops::median::Median;
use rust_img_ops::img_ops::ImageOp;
fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "median" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let median = Median {};
            let in_img = image::open(infile).expect("Failed to open INFILE.").into_luma8();
            let out_img = median.apply(&in_img);
            out_img.save(outfile).expect("Failed writing OUTFILE.");
        }

        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    print_help(Median {});

    std::process::exit(-1);
}

trait HelpStr {
    const HELP_STR: &'static str;
}

fn print_help<T: HelpStr>(_: T) {
    println!("{}", T::HELP_STR);
}

impl HelpStr for Median {
    const HELP_STR: &'static str = "median INFILE OUTFILE";
}
