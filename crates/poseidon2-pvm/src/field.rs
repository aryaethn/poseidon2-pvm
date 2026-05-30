//! BN254 scalar field arithmetic (`Fr`), specialised for the PolkaVM (RISC-V 64-bit) target.
//!
//! Elements are stored as four 64-bit limbs in Montgomery form (`a * R mod r`, `R = 2^256`).
//! The whole reason this layer is hand-written is that the limb-wise multiply/add maps
//! directly onto native RISC-V 64-bit instructions, whereas a generic big-integer crate
//! emulates the same operations far less efficiently on PolkaVM.
//!
//! Milestone 1 implements every `todo!()` below and validates it against the `ark-bn254`
//! reference oracle in `tests/vectors.rs`.

/// BN254 scalar field modulus `r`, little-endian 64-bit limbs.
///
/// ```text
/// r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
///   = 0x30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001
/// ```
pub const MODULUS: [u64; 4] = [
    0x43E1_F593_F000_0001,
    0x2833_E848_79B9_7091,
    0xB850_45B6_8181_585D,
    0x3064_4E72_E131_A029,
];

/// Poseidon2 / Rescue use the power map `x^ALPHA`. For BN254, `gcd(5, r - 1) = 1`, so the
/// S-box is `x^5`.
pub const SBOX_ALPHA: u64 = 5;

/// A BN254 scalar field element in Montgomery form.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Fr(pub(crate) [u64; 4]);

// --- Montgomery constants, derived deterministically from `MODULUS` in Milestone 1. ---
// TODO(M1): replace these placeholders with the computed constants, and add a test that
// recomputes them from `MODULUS` so they can never silently drift.
impl Fr {
    /// `R mod r` — the Montgomery representation of 1.
    const R: [u64; 4] = [0, 0, 0, 0]; // TODO(M1): 2^256 mod r
    /// `R^2 mod r` — used to move integers into Montgomery form.
    #[allow(dead_code)]
    const R2: [u64; 4] = [0, 0, 0, 0]; // TODO(M1): 2^512 mod r
    /// `-r^{-1} mod 2^64` — the Montgomery reduction inverse (CIOS).
    #[allow(dead_code)]
    const INV: u64 = 0; // TODO(M1): -r^{-1} mod 2^64
}

impl Fr {
    /// The additive identity.
    pub const ZERO: Fr = Fr([0, 0, 0, 0]);

    /// The multiplicative identity (1 in Montgomery form).
    pub const ONE: Fr = Fr(Fr::R);

    /// Construct from a small integer, handling the Montgomery conversion.
    pub fn from_u64(_v: u64) -> Self {
        todo!("M1: from_u64 (multiply by R2 to enter Montgomery form)")
    }

    /// Decode a canonical little-endian 32-byte integer into Montgomery form.
    /// Returns `None` if the value is `>= r` (non-canonical encoding).
    pub fn from_bytes_le(_bytes: &[u8; 32]) -> Option<Self> {
        todo!("M1: canonical decode + range check + Montgomery conversion")
    }

    /// Encode to a canonical little-endian 32-byte integer (out of Montgomery form).
    pub fn to_bytes_le(&self) -> [u8; 32] {
        todo!("M1: Montgomery reduce to integer + serialise")
    }

    /// Whether this element is the additive identity.
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    /// Modular addition.
    pub fn add(self, _rhs: Self) -> Self {
        todo!("M1: limb add with conditional subtract of r")
    }

    /// Modular doubling.
    pub fn double(self) -> Self {
        todo!("M1: modular doubling")
    }

    /// Modular subtraction.
    pub fn sub(self, _rhs: Self) -> Self {
        todo!("M1: limb sub with conditional add of r")
    }

    /// Modular negation.
    pub fn neg(self) -> Self {
        todo!("M1: modular negation")
    }

    /// Montgomery multiplication (CIOS) over `u64` limbs.
    pub fn mul(self, _rhs: Self) -> Self {
        todo!("M1: CIOS Montgomery multiplication")
    }

    /// Dedicated squaring (or `self.mul(self)` as a first cut).
    pub fn square(self) -> Self {
        todo!("M1: squaring")
    }

    /// `self^exp` via square-and-multiply. Constant-time-in-the-exponent is NOT required:
    /// every exponent used by Poseidon2 / Rescue is public.
    pub fn pow(self, _exp: &[u64]) -> Self {
        todo!("M1: square-and-multiply")
    }

    /// The Poseidon2 / Rescue S-box `x^5`, expressed via the field operations so it is
    /// correct by construction once `square`/`mul` land.
    pub fn sbox(self) -> Self {
        let x2 = self.square();
        let x4 = x2.square();
        x4.mul(self)
    }

    /// Multiplicative inverse, or `None` for zero.
    pub fn inverse(self) -> Option<Self> {
        todo!("M1: inverse via Fermat's little theorem or a fixed addition chain")
    }
}
