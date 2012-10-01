#[link(name = "rustrepl", vers = "0.2", author = "dbp")];

extern mod std;

use std::tempfile;
use io::WriterUtil;

struct AbstrSession {mut view_items: ~[~str], mut definitions: ~[~str], mut stmt: ~str}

fn write_session(s: AbstrSession, path: path::Path) {
    if os::path_exists(&path) {
        os::remove_file(&path);
    }
    let w = result::get(&(io::file_writer(&path, ~[io::Create])));
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

fn check_session(s: AbstrSession, tmppath: path::Path) -> bool {
    let path = tmppath.push("sess.rs");
    write_session(s, copy path);
    let out = run::program_output(~"rustc", ~[path.to_str()]);
    match out.status {
        0 => { return true; }
        _ => {
            let fst = copy str::split_char(out.err, '\n')[0];
            let err_indx = option::get(&(str::find_str(fst, ~"error:")));
            io::println(str::slice(fst, err_indx, str::len(fst)));
            return false;
        }
    }
}

fn handle_command(c: char, rest: ~str, tmppath: path::Path) {
    match c {
        'w' => {
            let new_path = path::from_str(str::trim(rest));
            // pretty print the output
            let out = run::program_output(~"rustc", 
                                          ~[~"--pretty", ~"normal", 
                                            tmppath.push("sess.rs").to_str()]);
            let out_path = tmppath.push("out.rs");
            if os::path_exists(&out_path) {
                os::remove_file(&out_path);
            }
            let w = result::get(&(io::file_writer(&out_path, ~[io::Create])));
            w.write_str(out.out);
            if !os::copy_file(&out_path, &new_path) {
                io::println(fmt!("could not write to %s.", new_path.to_str()));

            }
        }
        'l' => {
            io::println(~"not implemented yet.");
        }
        'h' => {
            io::println("about:");
            io::println("this is a very simple repl for rust. type in \
                         expressions to evaluate,");
            io::println("let statements to make local definitions, import \
                         and use statements, and ");
            io::println("fn statements to define functions. don't add \
                         trailing semis!");
            io::println("commands:");
            io::println(":w filename.rs - write current session to file.");
            io::println(":l filename.rs - load session from file - will erase \
                         current session. (not yet implemented)");
            io::println(":h - this message.");
        }
        _ => {
            io::println("unknown command. :h for help.");
        }
    }
}

fn main() {
    // set up working directory
    let p = tempfile::mkdtemp(&(os::tmpdir()), ~"repl");
    if option::is_none(&p) {
        fail ~"could not create temporary directory";
    }
    let tmppath = option::unwrap(p);

    io::println("repl for rust, 0.2.");
    io::println("don't use trailing semicolons. :h for help, Ctrl-D to quit.");

    let mut session = AbstrSession {view_items: ~[], definitions: ~[], stmt: ~""};
    loop {
        let stdin = io::stdin();

        io::print("rust> ");
        let raw_input = (stdin as io::ReaderUtil).read_line();
        if str::is_empty(raw_input) {
            if stdin.eof() {
                io::println("");
                break;
            }
            loop;
        }
        let input = str::trim(raw_input);

        if input[0] == ':' as u8 {
            handle_command(input[1] as char, if str::len(input) > 3 {
                    str::slice(input, 3, str::len(input))
                } else {~""}, copy tmppath);
        } else {
            let view_pop;
            let def_pop;
            let stmt_pop;
        
            let mut run = false; // should we run, ie, has stmt changed
        
            if str::starts_with(input, ~"use ") || 
               str::starts_with(input, ~"extern mod ") {
                session.view_items.push(input);
                view_pop = true;
                def_pop = false;
                stmt_pop = ~"";
            } else if str::starts_with(input, ~"fn ") || 
                      str::starts_with(input, ~"let ") {
                session.definitions.push(input);
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
        
            if check_session(copy session, copy tmppath) {
                if run {
                    let sess = tmppath.push("sess");
                    let res = run::program_output(sess.to_str(), ~[]);
                    io::print(res.out);
                }
            } else {
                if view_pop {
                    session.view_items.pop();
                }
                if def_pop {
                    session.definitions.pop();
                }
                session.stmt = stmt_pop;
            }
        }

    }
    // clean up tmp stuff. we want recursive, so call out to system.
    remove_tmpdir(tmppath);
}

#[cfg(unix)]
fn remove_tmpdir(tmppath: Path) {
    run::program_output(~"rm", ~[~"-R", tmppath.to_str()]);
}
#[cfg(windows)]
fn remove_tmpdir(tmppath: Path) {
    run::program_output(~"rd", ~[~"/S", ~"/Q", tmppath.to_str()]);
}