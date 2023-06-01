use std::fs;
use std::process;

#[test]
fn test_typst_lua() {

    let cmd = process::Command::new("lua")
        .arg("tests/main.lua")
        .output();

    let t1 = fs::read("tests/test1_result.pdf").expect("Couldn't read tests/test1_result.pdf");
    let t1_proof = fs::read("tests/test1_proof.pdf").expect("Couldn't read tests/test1_result.pdf");
    let t2 = fs::read("tests/test2_result.pdf").expect("Couldn't read tests/test2_result.pdf");
    let t2_proof = fs::read("tests/test2_proof.pdf").expect("Couldn't read tests/test2_result.pdf");
    //let t3 = fs::read("tests/test3_result.pdf").expect("Couldn't read tests/test3_result.pdf");
    //let t3_proof = fs::read("tests/test3_proof.pdf").expect("Couldn't read tests/test3_result.pdf");

    assert_eq!(t1, t1_proof, "test1_result.pdf is different from test1_proof.pdf!");
    assert_eq!(t2, t2_proof, "test2_result.pdf is different from test2_proof.pdf!");
    //assert_eq!(t3, t3_proof, "test3_result.pdf is different from test3_proof.pdf!");

    let cmd = process::Command::new("rm")
    .arg("test1_result.pdf")
    .arg("test2_result.pdf")
    //.arg("test3_result.pdf")
    .output();

}
