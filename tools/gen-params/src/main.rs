//! `gen-params` — deterministic Poseidon2 / Rescue-Prime parameter generation.
//!
//! This is the Milestone 1 **reproducibility** deliverable. It derives every round
//! constant (Grain LFSR, seeded exactly as the Poseidon/Poseidon2 specification
//! prescribes) and the external / internal matrices from public, documented inputs, then
//! emits the Rust source consumed by `crates/poseidon2-pvm/src/params/`. Anyone can
//! re-run it and diff the output to verify the checked-in constants: no magic numbers.

fn main() {
    eprintln!("gen-params: Poseidon2 / Rescue parameter generator (Milestone 1).");
    eprintln!();
    eprintln!("Planned interface:");
    eprintln!("  gen-params poseidon2 --field bn254 --width <t> --alpha 5 --security 128");
    eprintln!("  gen-params rescue    --field bn254 --width <t> --alpha 5 --security 128");
    eprintln!();
    eprintln!("Each run reports the chosen R_F / R_P, the Grain LFSR seed string, and a");
    eprintln!("provenance header, then writes src/params/<field>_t<width>.rs.");

    // TODO(M1):
    //   1. Compute R_F, R_P for (field, width, alpha, security) from the design rationale.
    //   2. Generate round constants via the Grain LFSR (documented seed string).
    //   3. Generate the external matrix and the internal-matrix diagonal per Poseidon2.
    //   4. Emit Rust source plus a provenance comment block (seed, date, tool version).
    todo!("M1: implement deterministic parameter generation");
}
