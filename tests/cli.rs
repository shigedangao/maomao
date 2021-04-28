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