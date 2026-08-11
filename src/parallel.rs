use std::{
    collections::{HashMap, VecDeque},
    env,
    fs::File,
    io::{self, Read, Write},
    sync::{Arc, Mutex},
    thread,
};

type WordDb = HashMap<String, u32>;

pub fn parallel(num_thread: usize) {
    let path = read_file_path();
    let content = open_valid_file(path);

    let db = count_words_parallel(&content, num_thread);

    println!("{db:?}");
}

fn read_file_path() -> String {
    env::args()
        .nth(1)
        .unwrap_or_else(|| "data/test1.txt".to_string())
}

fn open_valid_file(mut path: String) -> String {
    loop {
        match File::open(&path) {
            Ok(mut src_file) => {
                let mut content = String::new();
                let read_ok = src_file.read_to_string(&mut content).is_ok();

                if read_ok && !content.trim().is_empty() {
                    return content;
                }

                path = ask_for_another_path("Archivo vacio o invalido.");
            }
            Err(_) => {
                path = ask_for_another_path("No se pudo abrir el archivo.");
            }
        }
    }
}

fn ask_for_another_path(reason: &str) -> String {
    print!("{reason} Ingresa otra ruta (.txt o .pdf): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn count_words_parallel(content: &str, thread_count: usize) -> WordDb {
    let available_num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let num_threads = if thread_count > available_num_threads {
        available_num_threads
    } else {
        thread_count
    };

    let chunks = split_into_chunks(content, num_threads);

    let pending_chunks = Arc::new(Mutex::new(VecDeque::from(chunks)));
    let db: Arc<Mutex<WordDb>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = Vec::with_capacity(num_threads);

    // FORK: iniciar ejecucion paralela de los m hilos, cada uno mapeado a un core
    for _ in 0..num_threads {
        let pending_chunks = Arc::clone(&pending_chunks);
        let db = Arc::clone(&db);

        handles.push(thread::spawn(move || {
            while let Some(chunk) = take_next_chunk(&pending_chunks) {
                process_chunk(&chunk, &db);
            }
        }));
    }

    // JOIN: esperar a que todos los hilos terminen antes de consolidar la DB
    for handle in handles {
        handle.join().expect("un hilo hizo panic");
    }

    Arc::try_unwrap(db).unwrap().into_inner().unwrap()
}

fn take_next_chunk(pending_chunks: &Arc<Mutex<VecDeque<String>>>) -> Option<String> {
    pending_chunks.lock().unwrap().pop_front()
}

fn process_chunk(chunk: &str, db: &Arc<Mutex<WordDb>>) {
    let word = &mut String::new();

    for c in chunk.chars() {
        if c.is_whitespace() || c.is_ascii_punctuation() {
            close_word(word, db);
            continue;
        }

        word.push(c);
    }

    close_word(word, db);
}

fn close_word(word: &mut String, db: &Arc<Mutex<WordDb>>) {
    if word.is_empty() {
        return;
    }

    let mut db = db.lock().unwrap();
    db.entry(word.clone())
        .and_modify(|count| *count += 1)
        .or_insert(1);

    word.clear();
}

fn split_into_chunks(content: &str, n: usize) -> Vec<String> {
    if content.is_empty() {
        return Vec::new();
    }

    let bytes = content.len();
    let target_size = bytes.div_ceil(n).max(1);

    let mut chunks = Vec::with_capacity(n);
    let mut start = 0;

    while start < bytes {
        let mut end = (start + target_size).min(bytes);

        while end < bytes && !content.is_char_boundary(end) {
            end += 1;
        }
        while end < bytes && !content.as_bytes()[end].is_ascii_whitespace() {
            end += 1;
        }

        chunks.push(content[start..end].to_string());
        start = end;
    }

    chunks
}
