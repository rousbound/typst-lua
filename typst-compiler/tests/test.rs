#[cfg(test)]
mod tests {

    use std::fs;
    use typst_compiler::compile;

    pub fn is_pdf(bytes: &[u8]) -> bool {
        let pdf_signature: &[u8] = b"%PDF-";
        bytes.starts_with(pdf_signature)
    }

    #[test]
    fn test_api() {
        match compile("tests/test.typ".into(), &None) {
            Ok(data) => {
                if is_pdf(&data) {
                    fs::write("tests/test.pdf", data).unwrap();
                } else {
                    panic!("Test failed: Output is not a PDF");
                }
            }
            Err(e) => {
                panic!("Test failed: Compiler error: {}", e);
            }
        }
    }
    #[test]
    fn test_error_msg() {
        match compile("tests/test_error.typ".into(), &None) {
            Ok(data) => {
                // if is_pdf(&data) {
                //     fs::write("tests/test.pdf", data).unwrap();
                // } else {
                //     panic!("Test failed: Output is not a PDF");
                // }
            }
            Err(e) => {
                println!("Test ok: Compiler error: {}", e);
            }
        }
    }

    //#[test]
    //fn test_api_with_data() {
    //let my_value: Value = json!({
    //"name": "John",
    //"age": 30,
    //"is_student": true,
    //"hobbies": ["reading", "coding", "hiking"],
    //"address": {
    //"street": "123 Main St",
    //"city": "Anytown",
    //"state": "CA",
    //"zip": "12345"
    //}
    //});

    ////let mut compiler: Compiler = Compiler::new(".".into());
    //match compile("tests/test_with_json.typ".into(), &my_value) {
    //Ok(data) => {
    //if is_pdf(&data) {
    //fs::write("tests/test.pdf", data).unwrap();
    //} else {
    //panic!("Test failed: Output is not a PDF");
    //}
    //},
    //Err(e) => {
    //panic!("Test failed: Compiler error: {}", e);
    //}
    //}
    //}
}
