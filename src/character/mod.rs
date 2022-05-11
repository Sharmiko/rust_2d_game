pub mod character {
    pub struct Character {
        name: String,
        health: i8,
        pub idle: Vec<String>
    }
    
    
    impl Character {
    
        pub fn new() -> Self {
            Self {
                name: "Punk".to_string(),
                health: 10,
                idle: vec![
                    "assets/chars/punk/processed/Punk_idle_0.png".to_string(), 
                    "assets/chars/punk/processed/Punk_idle_1.png".to_string(),
                    "assets/chars/punk/processed/Punk_idle_2.png".to_string(),
                    "assets/chars/punk/processed/Punk_idle_3.png".to_string()
                ]
            }
        }
    }
        
}