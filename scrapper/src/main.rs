fn main() {

    let bytes = std::fs::read("file.pdf").unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    assert!(out.contains("This is a small demonstration"));

}
