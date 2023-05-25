use std::path::Path;
use std::process::Command;

use rstest::rstest;
use rustemo_compiler::output_cmp;

#[rstest]
#[case("pmsm iron losses.tse")]
#[case("active filter (switching).tse")]
#[case("ieee 33 bus system.tse")]
#[case("shipboard power.tse")]
#[case("high impedance fault.tse")]
#[case("battery generic server and client.tse")]
#[case("dynamic table.tse")]
#[case("wind farm microgrid.tse")]
#[case("foxbms inspired architecture.tse")]
#[case("terrestrial microgrid.tse")]
#[case("terrestrial microgrid gen.tse")]
fn test_models(#[case] model: &str) {
    assert!(Path::new(&format!("src/tests/models/{model}")).exists());
    let result = Command::new("typhoondsl-outliner")
        .arg("-j")
        .arg("-p")
        .arg(format!("src/tests/models/{model}"))
        .output()
        .expect("Failed to start outliner. Is it installed?");
    let target = std::path::PathBuf::from(model);
    let target_stem = target.file_stem().unwrap().to_string_lossy();
    output_cmp!(
        &format!("src/tests/models/{target_stem}.json"),
        String::from_utf8_lossy(&result.stdout).as_ref()
    );
}

#[rstest]
#[case("User Library.tlib")]
#[case("motor_lib.tlib")]
fn test_libraries(#[case] library: &str) {
    assert!(Path::new(&format!("src/tests/libraries/{library}")).exists());
    let result = Command::new("typhoondsl-outliner")
        .arg("-j")
        .arg("-p")
        .arg(format!("src/tests/libraries/{library}"))
        .output()
        .expect("Failed to start outliner. Is it installed?");
    let target = std::path::PathBuf::from(library);
    let target_stem = target.file_stem().unwrap().to_string_lossy();
    output_cmp!(
        &format!("src/tests/libraries/{target_stem}.json"),
        String::from_utf8_lossy(&result.stdout).as_ref()
    );
}
