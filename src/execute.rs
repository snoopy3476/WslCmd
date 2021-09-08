pub fn execute_mode(orig_bin_basename: &String, cmd_bin_basename: &String, args: &Vec<String>) {
    print!(
        concat!("original_bin:\t{}\n", "cmdline_bin:\t{}\n"),
        orig_bin_basename, cmd_bin_basename
    );
    for (i, arg) in args.iter().enumerate() {
        println!("arg[{}]: \t{:?}", i, arg);
    }
}
