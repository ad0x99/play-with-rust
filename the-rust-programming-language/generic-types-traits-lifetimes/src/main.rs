mod generic_data_types;
mod lifetimes;
mod traits;

fn main() {
    println!("========Generic Data Types========");
    generic_data_types::run_generic_data_types();

    println!("========Traits========");
    traits::run_traits();

    println!("========Lifetimes========");
    lifetimes::run_lifetimes()
}
