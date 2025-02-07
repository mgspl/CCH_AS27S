use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CoolOS {
    generation: u32,
}

impl CoolOS {
    pub fn update(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn rollback(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

fn main() {
    let mut gen_history = Vec::<String>::new();

    let mut installed_os = CoolOS { generation: 0 };

    installed_os.generation = 1;
    gen_history.push(installed_os.update());

    installed_os.generation = 2;
    gen_history.push(installed_os.update());

    installed_os.generation = 3;
    gen_history.push(installed_os.update());

    println!("Histórico de gerações do Cool_OS:");
    for gen in gen_history.iter() {
        println!("{}", gen);
    }

    let installed_os = CoolOS::rollback(&gen_history.pop().unwrap());
    println!(
        "O Cool_OS foi restaurado para a geração: {}",
        installed_os.generation
    );

    let installed_os = CoolOS::rollback(&gen_history.pop().unwrap());
    println!(
        "O Cool_OS foi restaurado para a geração: {}",
        installed_os.generation
    );
}
