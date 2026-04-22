mod kitchen {
    fn secret_recipe() -> &'static str { "42 spices" }
    pub fn menu() -> &'static str { "Today's special" }

    pub mod staff {
        pub fn cook() -> String {
            format!("Cooking with {}", super::secret_recipe())
        }
    }
}

fn main() {
    println!("{}", kitchen::menu());             // Line A -> Compiles
    // println!("{}", kitchen::secret_recipe());     // Line B -> Error because secret_recipe() is private function
    println!("{}", kitchen::staff::cook());       // Line C -> Compiles
}