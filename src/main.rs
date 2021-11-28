//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
// https://github.com/reem/stainless
//

#[derive(Debug)]
struct Hero {
    name: String,
    energy: u16,
    strike: bool,
}
#[derive(Debug)]
struct Goblin {
    energy: u16,
    strike: bool,
}

impl Hero {
    fn jump(&self) {}
}

trait StrikeTrait {
    fn strike(&mut self);
}

impl StrikeTrait for Hero {
    fn strike(&mut self) {
        self.strike = true;
    }
}

impl StrikeTrait for Goblin {
    fn strike(&mut self) {
        self.strike = true;
    }
}

fn main() {
    let mut hero = Hero {
        name: "Dave".to_string(),
        energy: 100,
        strike: false,
    };
    let mut goblin = Goblin {
        energy: 99,
        strike: false,
    };
    println!("{:#?}", hero);
    hero.strike();
    println!("{:#?}", hero);
    println!("{:#?}", goblin);
    goblin.strike();
    println!("{:#?}", goblin);
}

//
