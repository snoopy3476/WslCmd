pub fn manage_mode(orig_bin_basename: &String, cmd_bin_basename: &String, args: &Vec<String>) {
    print!(
        concat!("original_bin:\t{}\n", "cmdline_bin:\t{}\n"),
        orig_bin_basename, cmd_bin_basename
    );
    for (i, arg) in args.iter().enumerate() {
        println!("arg[{}]: \t{:?}", i, arg);
    }

    print_help(cmd_bin_basename);
}

fn print_help(bin_name: &String) {
    print!(
        concat!(
            "usage: {0} <operation> [<arg1> <arg2> ...]\n",
            "\n",
            "  <operation>\n",
            "\n",
            "    - Link new commands\n",
            "\n",
            "        {0} new <command-name-1> <command-name-2> ...\n",
            "        {0} add <command-name-1> <command-name-2> ...\n",
            "        {0} ln <command-name-1> <command-name-2> ...\n",
            "\n",
            "    - Unlink existing commands\n",
            "\n",
            "        {0} del <command-name-1> <command-name-2> ...\n",
            "        {0} rm <command-name-1> <command-name-2> ...\n",
            "\n",
            "    - List linked commands\n",
            "\n",
            "        {0} list\n",
            "\n"
        ),
        bin_name
    );
}
