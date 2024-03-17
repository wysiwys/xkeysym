use keysym_generator::generate;


fn main() {

    generate("src/automatically_generated.rs").expect("Generation failed");

}
