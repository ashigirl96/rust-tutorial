use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};


struct Table {
    // mutexは並列処理を制御するための機構
    // →内容へ同時アクセスできるのは1スレッドに限定
    // これがforkに求められる性質
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

// implブロックは構造体に関する定義ができる.
impl Philosopher {
    // new()は`self`を取らないので, 関連関数となる.
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    // Vec<T>は可変長の配列型
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Michel Foucault", 3, 4),
        Philosopher { name: "Emma Goldman".to_string(), left: 0, right: 4 },
    ];

    // `_` はplaceholderで、何らかの型のベクトルですと言ってる
    // into_iter()は各々の所有権を持つイテレータを生成する
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();


        // spawnは新しいスレッド上でクロージャーを実行する
        // クロージャは特別なアノテーション, `move`を必要とし, キャプチャする値の所有権が
        // クロージャ内へと移動する
        thread::spawn(move || {
            p.eat(&table);
        })  // spawnの末尾にセミコロンを置かないことで, 式としている, 正しい戻り値を返すため
    }).collect();  // spawnの戻り値(各スレッドへのハンドル)をcollectしてる

    for h in handles {
        // join()がループ
        // 各スレッド実行が完了するまで実行をブロックする
        h.join().unwrap();
    }

    // for文
//    for p in &philosophers {
//        p.eat();
//    }
}
