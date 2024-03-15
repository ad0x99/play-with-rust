#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // The unwrap_or_else method takes one argument is a closure without any arguments that returns a value T, in this case is ShirtColor
        // If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some. If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical pipes). The body of the closure calls self.most_stocked(). We’re defining the closure here, and the implementation of unwrap_or_else will evaluate the closure later if the result is needed.
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn run_inventory() {
    // The store contains 1 red and 2 blue shirts
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // The user 1 get a giveaway of red shirt
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    // The user 2 get a giveaway of random shirt
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
