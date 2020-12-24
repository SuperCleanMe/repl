pub fn exit(code: i32) -> ! {
    println!("Goodbye...");
    std::process::exit(code);
}