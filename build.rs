use io::ProgramMetadata;

fn main() {
    // gear_wasm_builder::build();
    // gasm::build();
    gear_wasm_builder::build_with_metadata::<ProgramMetadata>();
}
