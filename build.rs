fn main() {
  static_vcruntime::metabuild();
  // Merges empty `.rdata` and `.pdata` into .text section saving a few bytes in data
  // directories portion  of PE header.
  println!("cargo:rustc-link-arg-bins=/MERGE:.rdata=.text");
  println!("cargo:rustc-link-arg-bins=/MERGE:.pdata=.text");
  // Removes `IMAGE_DEBUG_DIRECTORY` from PE.
  println!("cargo:rustc-link-arg-bins=/EMITPOGOPHASEINFO");
  println!("cargo:rustc-link-arg-bins=/DEBUG:NONE");
}