use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;

const TEST_CSV: &str = "test_data.csv";

fn create_test_csv() {
    let mut file = File::create(TEST_CSV).unwrap();
    write!(file, "ID,Name,Age,Grade,Subject,Score,Attendance
1,Alice Smith,18,12,Math,95,98%
2,Bob Johnson,17,11,Physics,88,95%
3,Charlie Brown,16,10,Chemistry,78,92%
4,Diana Lee,,12,Biology,92,97%
5,Eva Martinez,18,12,Math,91,99%
6,Frank Wilson,17,11,,85,93%
7,Grace Taylor,16,10,Physics,89,96%
8,Henry Davis,18,12,Chemistry,,90%
9,Ivy Chen,17,11,Biology,94,98%
10,Jack Thompson,16,10,Math,82,").unwrap();
}

#[test]
fn test_missing_values() {
    create_test_csv();

    let mut cmd = Command::cargo_bin("analyst").unwrap();
    let assert = cmd.arg("missing-values")
        .arg(TEST_CSV)
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Age: 1 missing values"))
        .stdout(predicate::str::contains("Subject: 1 missing values"))
        .stdout(predicate::str::contains("Score: 1 missing values"))
        .stdout(predicate::str::contains("Attendance: 1 missing values"));
}

#[test]
fn test_frequent_patterns() {
    create_test_csv();

    let mut cmd = Command::cargo_bin("analyst").unwrap();
    let assert = cmd.arg("frequent-patterns")
        .arg(TEST_CSV)
        .arg("--min-support")
        .arg("0.3")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Age:18,Grade:12 (support: 30.00%)"));
}

#[test]
fn test_column_stats() {
    create_test_csv();

    let mut cmd = Command::cargo_bin("analyst").unwrap();
    let assert = cmd.arg("column-stats")
        .arg(TEST_CSV)
        .arg("--column")
        .arg("Subject")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Math: 3 occurrences"))
        .stdout(predicate::str::contains("Physics: 2 occurrences"))
        .stdout(predicate::str::contains("Chemistry: 2 occurrences"))
        .stdout(predicate::str::contains("Biology: 2 occurrences"));
}

#[test]
fn test_extrema() {
    create_test_csv();

    let mut cmd = Command::cargo_bin("analyst").unwrap();
    let assert = cmd.arg("extrema")
        .arg(TEST_CSV)
        .arg("--column")
        .arg("Score")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Minimum value: 78"))
        .stdout(predicate::str::contains("Maximum value: 95"));
}

#[test]
fn test_nonexistent_file() {
    let mut cmd = Command::cargo_bin("analyst").unwrap();
    let assert = cmd.arg("missing-values")
        .arg("nonexistent.csv")
        .assert();

    assert
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn test_show() {
    create_test_csv();

    let mut cmd = Command::cargo_bin("analyst").unwrap();
    let assert = cmd.arg("show")
        .arg(TEST_CSV)
        .arg("--start")
        .arg("2")
        .arg("--end")
        .arg("4")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Bob Johnson"))
        .stdout(predicate::str::contains("Charlie Brown"))
        .stdout(predicate::str::contains("Diana Lee"))
        .stdout(predicate::str::contains("Physics"))
        .stdout(predicate::str::contains("Chemistry"))
        .stdout(predicate::str::contains("Biology"));
}