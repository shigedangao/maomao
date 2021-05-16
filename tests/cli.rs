use std::error::Error;
use assert_cmd::Command;

#[test]
fn expect_to_generate_yaml_from_example() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd.arg("generate").arg("-p").arg("examples");
    cmd.assert().success();

    Ok(())
}

#[test]
fn expect_to_generate_yaml_output_to_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd
        .arg("generate")
        .arg("-p")
        .arg("examples")
        .arg("-o")
        .arg("output.yaml")
        .arg("-m");
    cmd.assert().success();

    Ok(())
}

#[test]
fn expect_to_generate_yaml_output_to_folder() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd
        .arg("generate")
        .arg("-p")
        .arg("examples")
        .arg("-o")
        .arg("./output");
    cmd.assert().success();

    Ok(())
}

#[test]
fn expect_to_generate_custom_crd() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd
        .arg("generate")
        .arg("-p")
        .arg("examples/crd")
        .arg("-o")
        .arg("crd.yaml")
        .arg("-m");
    cmd.assert().success();

    Ok(())
}

#[test]
fn expect_to_generate_yaml_even_folder_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd
        .arg("generate")
        .arg("-p")
        .arg("examples")
        .arg("-o")
        .arg("./bar");
    cmd.assert().success();

    Ok(())
}

#[test]
fn expect_to_return_error_toml_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd
        .arg("generate")
        .arg("-p")
        .arg("foo");
    let output = cmd.output()?.stdout;
    let output_str = String::from_utf8(output)?;

    assert!(output_str.contains("No such file or directory (os error 2)"));

    Ok(())
}

#[test]
fn expect_to_run_diff_run() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("maomao")?;

    cmd
        .arg("diff")
        .arg("-p")
        .arg("examples/diff");
    cmd.assert().success();

    Ok(())
}