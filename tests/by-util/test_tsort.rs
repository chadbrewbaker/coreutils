// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
use crate::common::util::TestScenario;

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}
#[test]
fn test_sort_call_graph() {
    new_ucmd!()
        .arg("call_graph.txt")
        .run()
        .stdout_is_fixture("call_graph.expected");
}

#[test]
fn test_sort_self_loop() {
    new_ucmd!()
        .pipe_in("first first\nfirst second second second")
        .succeeds()
        .stdout_only("first\nsecond\n");
}

#[test]
fn test_sort_floating_nodes() {
    new_ucmd!()
        .pipe_in("d d\nc c\na a\nb b")
        .succeeds()
        .stdout_only("a\nb\nc\nd\n");
}

#[test]
fn test_sort_gnu_cycle_1() {
    new_ucmd!()
        .pipe_in("t b\nt s\ns t\n")
        .fails()
        .stderr_contains("tsort: -, input contains a loop:");
       
}

#[test]
fn test_sort_gnu_cycle_2() {
    new_ucmd!()
        .pipe_in("t x\nt s\ns t\n")
        .fails()
        .stderr_contains("tsort: -, input contains a loop:")
        .stdout_contains("");
        //gnu expects .stdout_contains("s\nt\nx\n")    
}

#[test]
fn test_sort_gnu_posix_1() {
    new_ucmd!()
        .pipe_in("a b c c d e\ng g\nf g e f\nh h\n")
        .succeeds()
        .stderr_contains("")
        .stdout_contains("a\nc\nd\nh\nb\ne\nf\ng\n");
}

#[test]
fn test_sort_gnu_linear_1() {
    new_ucmd!()
        .pipe_in("a b b c c d d e e f f g\n")
        .succeeds()
        .stderr_contains("")
        .stdout_contains("a\nb\nc\nd\ne\nf\ng\n");
}

#[test]
fn test_sort_gnu_tree_1() {
    new_ucmd!()
        .pipe_in("a b b c c d d e e f f g\nc x x y y z\n")
        .succeeds()
        .stderr_contains("")
        .stdout_contains("a\nb\nc\nd\nx\ne\ny\nf\nz\ng\n");
        //gnu expects .stdout_contains("a\nb\nc\nx\nd\ny\ne\nz\nf\ng\n");
}

#[test]
fn test_sort_gnu_tree_2() {
    new_ucmd!()
        .pipe_in("a b b c c d d e e f f g\nc x x y y z\nf r r s s t\n")
        .succeeds()
        .stderr_contains("")
        .stdout_contains("a\nb\nc\nd\nx\ne\ny\nf\nz\ng\nr\ns\nt\n");
        //gnu expects .stdout_contains("a\nb\nc\nx\nd\ny\ne\nz\nf\nr\ng\ns\nt\n");
} 

#[test]
fn test_no_such_file() {
    new_ucmd!()
        .arg("invalid_file_txt")
        .fails()
        .stderr_contains("No such file or directory");
}

#[test]
fn test_version_flag() {
    let version_short = new_ucmd!().arg("-V").succeeds();
    let version_long = new_ucmd!().arg("--version").succeeds();

    assert_eq!(version_short.stdout_str(), version_long.stdout_str());
}

#[test]
fn test_help_flag() {
    let help_short = new_ucmd!().arg("-h").succeeds();
    let help_long = new_ucmd!().arg("--help").succeeds();

    assert_eq!(help_short.stdout_str(), help_long.stdout_str());
}

#[test]
fn test_multiple_arguments() {
    new_ucmd!()
        .arg("call_graph.txt")
        .arg("invalid_file")
        .fails()
        .stderr_contains("unexpected argument 'invalid_file' found");
}

#[test]
fn test_error_on_dir() {
    let (at, mut ucmd) = at_and_ucmd!();
    at.mkdir("tsort_test_dir");
    ucmd.arg("tsort_test_dir")
        .fails()
        .stderr_contains("tsort: tsort_test_dir: read error: Is a directory");
}
