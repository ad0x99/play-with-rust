mod destructuring;
mod extra_conditionals_and_match_guards;
mod ignore_values_in_pattern;
mod pattern_syntax;

fn main() {
    println!("============Pattern Syntax============");
    pattern_syntax::run_pattern_syntax();

    println!("============Pattern Destructuring============");
    destructuring::run_destructuring();

    println!("============Ignore Values in Pattern============");
    ignore_values_in_pattern::run_ignore_values();

    println!("============Extra conditionals & Match guards============");
    extra_conditionals_and_match_guards::run_extra_conditionals_and_match_guards();
}
