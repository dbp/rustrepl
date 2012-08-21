use std;

import std::tempfile;
import io::WriterUtil;

type abstr_session = {view_items: ~[~str], definitions: ~[~str], stmt: ~str};

fn write_session(s: abstr_session, path: ~str) {
    if os::path_exists(path) {
        os::remove_file(path);
    }
    let w = result::get(io::file_writer(path, ~[io::Create]));
    for vec::each(s.view_items) |i| {
        w.write_line(i+~";");
    }
    w.write_line(~"fn main() {");
    for vec::each(s.definitions) |d| {
        w.write_line(d+~";");
    }
    if s.stmt != ~"" {
        w.write_line(~"io::println(fmt!(\"%?\","+s.stmt+~"));");
    }
    w.write_line(~"\n}\n")
}

fn check_session(s: abstr_session, path: ~str) -> bool {
    let path = fmt!("%s/sess.rs", path);
    write_session(s, path);
    let out = run::program_output(~"rustc", ~[path]);
    match out.status {
        0 => { return true; }
        _ => {
            let fst = copy str::split_char(out.err, '\n')[0];
            let err_indx = option::get(str::find_str(fst, ~"error:"));
            io::println(str::slice(fst, err_indx, str::len(fst)));
            return false;
        }
    }
}

fn handle_command(c: char, rest: ~str, path: ~str) {
    match c {
        'w' => {
            let new_path = str::trim(rest);
            // pretty print the output
            let out = run::program_output(~"rustc", ~[~"--pretty", ~"normal", 
                                                      fmt!("%s/sess.rs", path)]);
            let out_path = fmt!("%s/out.rs", path);
            if os::path_exists(out_path) {
                os::remove_file(out_path);
            }
            let w = result::get(io::file_writer(out_path, ~[io::Create]));
            w.write_str(out.out);
            if !os::copy_file(out_path, new_path) {
                io::println(fmt!("could not write to %s.", path));

            }
        }
        'l' => {
            io::println(~"not implemented yet.");
        }
        'h' => {
            io::println("about:");
            io::println("this is a very simple repl for rust. type in expressions to evaluate,");
            io::println("let statements to make local definitions, import and use statements, and ");
            io::println("fn statements to define functions. don't add trailing semis!");
            io::println("commands:");
            io::println(":w filename.rs - write current session to file.");
            io::println(":l filename.rs - load session from file - will erase current session. (not yet implemented)");
            io::println(":h - this message.")
        }
        _ => {
            io::println("unknown command. :h for help.");
        }
    }
}

fn main() {
    // set up working directory
    let p = tempfile::mkdtemp(~"/tmp/", ~"repl");
    if option::is_none(p) {
        fail ~"could not create temporary directory";
    }
    let path = option::unwrap(p);

    io::println("repl for rust, 0.1. *nix only, for now (b/c of tmpdirs). don't add semis.");
    io::println(":h for help");

    let mut session = {view_items: ~[], definitions: ~[], stmt: ~""};
    loop {
        let stdin = io::stdin();

        io::print("rust> ");
        let raw_input = stdin.read_line();
        if str::is_empty(raw_input) {
            if stdin.eof() {
                io::println("");
                break;
            }
            again;
        }
        let input = str::trim(raw_input);

        if input[0] == ':' as u8 {
            handle_command(input[1] as char, if str::len(input) > 3 {
                    str::slice(input, 3, str::len(input))
                } else {~""}, path);
        } else {
            let view_pop;
            let def_pop;
            let stmt_pop;
        
            let mut run = false; // should we run, ie, has stmt changed
        
            if str::starts_with(input, ~"use ") || str::starts_with(input, ~"import ") {
                vec::push(session.view_items, input);
                view_pop = true;
                def_pop = false;
                stmt_pop = ~"";
            } else if str::starts_with(input, ~"fn ") || str::starts_with(input, ~"let ") {
                vec::push(session.definitions, input);
                view_pop = false;
                def_pop = true;
                stmt_pop = ~"";
            } else {
                view_pop = false;
                def_pop = false;
                stmt_pop = copy session.stmt;
                run = true;
                session.stmt = input;
            }
        
            if check_session(session, path) {
                if run {
                    let res = run::program_output(fmt!("%s/sess", path), ~[]);
                    io::print(res.out);
                }
            } else {
                if view_pop {
                    vec::pop(session.view_items);
                }
                if def_pop {
                    vec::pop(session.definitions);
                }
                session.stmt = stmt_pop;
            }
        }

    }
}