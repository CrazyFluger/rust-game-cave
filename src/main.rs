use std::io;
use std::cell::RefCell;

struct Cave<'a> {
    west: usize,
    east: usize,
    north: usize,
    south: usize,
    chests: RefCell<Vec<&'a Chest>>,
    description: String,
}

struct Chest {
    description: String,
}

struct Player<'a> {
    cave: usize,
    chest: Option<&'a Chest>,
}

impl<'a> Player<'a> {
    fn info(&mut self) {
        match &self.chest {
            None => println!("Вы ничего не несёте."),
            chest => println!("Вы несёте сундук \"{}\"", chest.unwrap().description),
        }
    }
    fn take(&mut self, cave: &'a Cave<'a>) {
        match &self.chest {
            None => {
                let chest: Option<&Chest> = cave.chests.borrow_mut().pop();
                match chest {
                    None => println!("В пещере пусто."),
                    chest => {
                        self.chest = chest;
                        println!("Вы взяли сундук \"{}\"", chest.unwrap().description);
                    }
                }
            },
            in_hands => {
                println!("Вы уже несёте сундук \"{}\"", in_hands.unwrap().description);
            },
        }
    }
    fn put(&mut self, cave: &'a Cave<'a>) {
        match &self.chest {
            None => {
                println!("У вас в руках ничего нет!");
            },
            chest => {
                let chest: &'a Chest = chest.unwrap();
                cave.chests.borrow_mut().push(chest);
                println!("Вы положили сундук с \"{}\"", chest.description);
                self.chest = None;
            },
        }
    }
    fn east(&mut self, cave: &Cave) {
        match &cave.east {
            99 => println!("На западе нет прохода!"),
            index => {
                println!(">>> переход в {}", index);
                self.cave = *index
            }
        }
    }
    fn west(&mut self, cave: &Cave) {
        match &cave.west {
            99 => println!("На востоке нет прохода!"),
            index => {
                println!(">>> переход в {}", index);
                self.cave = *index
            }
        }
    }
    fn north(&mut self, cave: &Cave) {
        match &cave.north {
            99 => println!("На севере нет прохода!"),
            index => {
                println!(">>> переход в {}", index);
                self.cave = *index
            }
        }
    }
    fn south(&mut self, cave: &Cave) {
        match &cave.south {
            99 => println!("На юге нет прохода!"),
            index => {
                println!(">>> переход в {}", index);
                self.cave = *index
            }
        }
    }
}

fn main() {
    let chests: [Chest; 9] = [
        /* 0 */Chest { description: String::from("Первый ящик") },
        /* 1 */Chest { description: String::from("Второй ящик") },
        /* 2 */Chest { description: String::from("Третий ящик") },
        /* 3 */Chest { description: String::from("Четвёртый ящик") },
        /* 4 */Chest { description: String::from("Пятый ящик") },
        /* 5 */Chest { description: String::from("Шестой ящик") },
        /* 6 */Chest { description: String::from("Седьмой ящик") },
        /* 7 */Chest { description: String::from("Восьмой ящик") },
        /* 8 */Chest { description: String::from("Девятый ящик") },
    ];

    //     N
    //  -------
    //  |0 1|2|
    //  |-- | |
    // W|3 4 5|E
    //  | ----|
    //  |6 7 8|
    //  -------
    //     S
    let caves: [Cave; 9] = [
        /* 0 */Cave { west: 99, east:  1, north: 99, south: 99, chests: RefCell::new(vec![&chests[0]]), description: String::from("Тёмной и первой пещере") },
        /* 1 */Cave { west:  0, east: 99, north: 99, south:  4, chests: RefCell::new(vec![&chests[1]]), description: String::from("Тёмной и второй пещере") },
        /* 2 */Cave { west: 99, east: 99, north: 99, south:  5, chests: RefCell::new(vec![&chests[2]]), description: String::from("Тёмной и третьей пещере") },
        /* 3 */Cave { west: 99, east:  4, north: 99, south:  6, chests: RefCell::new(vec![&chests[3]]), description: String::from("Тёмной и четвёртой пещере") },
        /* 4 */Cave { west:  3, east:  5, north:  1, south: 99, chests: RefCell::new(vec![&chests[4]]), description: String::from("Тёмной и пятой пещере") },
        /* 5 */Cave { west:  4, east: 99, north:  2, south: 99, chests: RefCell::new(vec![&chests[5]]), description: String::from("Тёмной и шестой пещере") },
        /* 6 */Cave { west: 99, east:  7, north:  3, south: 99, chests: RefCell::new(vec![&chests[6]]), description: String::from("Тёмной и седьмой пещере") },
        /* 7 */Cave { west:  6, east:  8, north: 99, south: 99, chests: RefCell::new(vec![&chests[7]]), description: String::from("Тёмной и восьмой пещере") },
        /* 8 */Cave { west:  7, east: 99, north: 99, south: 99, chests: RefCell::new(vec![&chests[8]]), description: String::from("Тёмной и девятой пещере") },
    ];

    let mut player = Player {
        cave: 1,
        chest: None,
    };

    let mut steps: u8 = 50;

    println!("Добро пожаловать в пещеры Rust!");
    println!("Для помощи наберите 'h' или 'help'!");

    loop {
        println!("================================");
        match &steps {
            0 => {
                println!("У вас кончились ходы.");
                println!("Вы проиграли!");
                break;
            }
            steps => println!("У вас осталось {} шагов", steps),
        }
        player.info();

        let cave = &caves[player.cave];
        println!("Вы находитесь в {}", cave.description);

        let chests_ref = cave.chests.borrow();
        let count = chests_ref.iter().len();
        match &count {
            0 => println!("В пещере пусто"),
            9 => {
                println!("Поздравляем, Вы собрали все ящики в одной пещере!");
                break;
            }
            _n => {
                println!("В пещере находятся коробки:");
                for (i, chest) in chests_ref.iter().enumerate() {
                    println!("{}. {}", i + 1, chest.description);
                }
            }
        }
        drop(chests_ref);
        println!("Что вы собираетесь делать?");

        let mut cmd: String = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Не получилось прочитать строку!");

        let cmd = cmd.trim().to_lowercase();

        match cmd.as_str() {
            "t" | "take" => {
                player.take(&cave);
                steps -= 1;
            },
            "p" | "put" => {
                player.put(&cave);
                steps -= 1;
            },
            "e" | "east" => {
                player.east(&cave);
                steps -= 1;
            },
            "w" | "west" => {
                player.west(&cave);
                steps -= 1;

            },
            "n" | "north" => {
                player.north(&cave);
                steps -= 1;
            },
            "s" | "south" => {
                player.south(&cave);
                steps -= 1;
            },
            "h" | "help" => {
                println!("======================= HELP ============================");
                println!("Ваша задача собрать все ящики в одной пещере за 50 шагов.");
                println!("За один раз Вы можете переносить только один ящик.");
                println!("Вы можете использовать следующие команды:");
                println!("'t' или 'take'  - взять ящик");
                println!("'p' или 'put'   - поставить ящик");
                println!("'е' или 'east'  - идти на восток");
                println!("'w' или 'west'  - идти на запад");
                println!("'n' или 'north' - идти на север");
                println!("'s' или 'south' - идти на юг");
                println!("'h' или 'help'  - вызвать эту справку.");
                println!("'exit'          - выход из игры.");
            },
            "exit"  => {
                println!("Вы покинули игру");
                break;
            },
            v => println!("Неизвестная команда \"{}\"", v),
        }
        println!();
    }

    println!("Игра окончена!");
}
