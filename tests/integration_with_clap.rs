
#[cfg(test)]
mod test_integration_with_clap {
    extern crate contactopensource;

    extern crate assert_cmd;

    use std::process::Command;
    //use assert_cmd::prelude::*;

    ////
    //
    // Basics
    //
    ////

    #[test]
    fn bare() {
        let output = Command::new("./target/debug/contactopensource").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, "");
    }

    #[test]
    fn help() {
        let output = Command::new("./target/debug/contactopensource").arg("--help").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert!(s.contains("ContactOpenSource 1.0.0"));
        assert!(s.contains("USAGE"));
    }

    #[test]
    fn version() {
        let output = Command::new("./target/debug/contactopensource").arg("--version").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert!(s.contains("ContactOpenSource 1.0.0"));
    }

    ////
    //
    // Output
    //
    ////

    #[test]
    fn output_text() {
        let output = Command::new("./target/debug/contactopensource").arg("--output-text").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, ""); //TODO
    }

    #[test]
    fn output_json() {
        let output = Command::new("./target/debug/contactopensource").arg("--output-json").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, ""); //TODO
    }

    #[test]
    fn output_html() {
        let output = Command::new("./target/debug/contactopensource").arg("--output-html").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, ""); //TODO
    }

    #[test]
    fn output_xml() {
        let output = Command::new("./target/debug/contactopensource").arg("--output-xml").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, ""); //TODO
    }

    ////
    //
    // Subcommands for administration
    //
    ////

    #[test]
    fn subcommand_db() {
        let output = Command::new("./target/debug/contactopensource").arg("db").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        let v: Vec<&str> =  s.split_whitespace().collect();
        assert_eq!(v[0], "db"); // TODO replace with anything more useful
    }

    #[test]
    fn subcommand_debug() {
        let output = Command::new("./target/debug/contactopensource").arg("debug").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        let v: Vec<&str> =  s.split_whitespace().collect();
        assert_eq!(v[0], "debug"); // TODO replace with anything more useful
    }

    #[test]
    fn subcommand_sql() {
        let output = Command::new("./target/debug/contactopensource").arg("sql").output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        let v: Vec<&str> =  s.split_whitespace().collect();
        assert_eq!(v[0], "sql"); // TODO replace with anything more useful
    }

    /////
    //
    // Subcommands for summaries
    //
    ////

    #[test]
    fn subcommand_count() {
        let output = Command::new("./target/debug/contactopensource").args(&["count", "table", "arcs"]).output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, "count:0\n"); //TODO
    }

    #[test]
    fn subcommand_list() {
        let output = Command::new("./target/debug/contactopensource").args(&["list", "table", "arcs"]).output().expect("failure");
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, ""); //TODO
    }

    /////
    //
    // Subcommands for CRUD
    //
    ////

    // #[test]
    // fn subcommand_create() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["create", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_create_with_conflicting_record() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["create", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_read() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["read", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_read_without_existing_record() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["read", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_update() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["update", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_update_without_existing_record() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["update", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_delete() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["delete", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    // #[test]
    // fn subcommand_delete_without_existing_record() {
    //     let id = t::id::fab();
    //     let output = Command::new("./target/debug/contactopensource").args(&["delete", "arcs", "--id", id.to_string().as_str()]).output().expect("failure");
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     assert_eq!(s, ""); //TODO
    // }

    /////
    //
    // TODO assert_cmd
    //
    ////

    // #[test]
    // fn example() {
    //     Command::cargo_bin("bin_fixture")
    //         .unwrap()
    //         .assert()
    //         .success();
    // }

}
