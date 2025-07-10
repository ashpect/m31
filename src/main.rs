use ark_ff::{Fp64, MontBackend, MontConfig, PrimeField};

#[derive(MontConfig)]
#[modulus = "2147483647"]
#[generator = "3"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "1"]
pub struct M31FrConfig;
pub type M31Fr = Fp64<MontBackend<M31FrConfig, 1>>; 

pub struct FieldElement<F: PrimeField>(F);
pub type M31FieldElement = FieldElement<M31Fr>;

fn main() {
    let p1 = FieldElement(M31Fr::from(123u32));
    let p2 = FieldElement(M31Fr::from(456u32));
    let p3 = FieldElement(M31Fr::from(789u32));

    let big_val = 3_000_000_000u64; // bigger than p=2147483647
    let wrapped_big_val = FieldElement(M31Fr::from(big_val));

    let near_p_val = 2_147_483_646u64; // just below p
    let wrapped_near_p_val = FieldElement(M31Fr::from(near_p_val));

    // For arithmetic operations, use clone and operator overloads
    let sum = FieldElement(wrapped_big_val.0.clone() + wrapped_near_p_val.0.clone());

    println!("Point 1: {}", p1.0);
    println!("Point 2: {}", p2.0);
    println!("Point 3: {}", p3.0);
    println!("Sum: {}", sum.0);
    println!("Wrapped big val: {}", wrapped_big_val.0);
}
