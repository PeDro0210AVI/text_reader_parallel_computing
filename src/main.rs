use std::{fs, time::{Duration, Instant}};
mod in_sequence;
mod parallel;

const RUNS: usize = 5;

fn main() {
    fs::create_dir_all("results").expect("no se pudo crear el directorio results/");

    let mut seq_times: Vec<Duration> = Vec::with_capacity(RUNS);
    let mut par2_times: Vec<Duration> = Vec::with_capacity(RUNS);
    let mut par4_times: Vec<Duration> = Vec::with_capacity(RUNS);

    for run in 1..=RUNS {
        eprintln!("=== Ejecucion {run}/{RUNS} ===");

        eprintln!("-- secuencial --");
        let sequence_instant = Instant::now();
        in_sequence::in_sequence();
        seq_times.push(sequence_instant.elapsed());

        eprintln!("-- paralela 2 hilos --");
        let parallel_2_instant = Instant::now();
        parallel::parallel(2);
        par2_times.push(parallel_2_instant.elapsed());

        eprintln!("-- paralela 4 hilos --");
        let parallel_4_instant = Instant::now();
        parallel::parallel(4);
        par4_times.push(parallel_4_instant.elapsed());
    }

    let avg_seq = average(&seq_times);
    let avg_par2 = average(&par2_times);
    let avg_par4 = average(&par4_times);

    write_times_csv(&seq_times, &par2_times, &par4_times, avg_seq, avg_par2, avg_par4);
    write_speedup_csv(&seq_times, &par2_times, &par4_times, avg_seq, avg_par2, avg_par4);
    write_efficiency_csv(&seq_times, &par2_times, &par4_times, avg_seq, avg_par2, avg_par4);

    eprintln!("CSVs generados en results/: tiempos.csv, speedup.csv, efficiency.csv");
}

fn average(durations: &[Duration]) -> f64 {
    durations.iter().map(Duration::as_secs_f64).sum::<f64>() / durations.len() as f64
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

fn write_times_csv(
    seq: &[Duration],
    par2: &[Duration],
    par4: &[Duration],
    avg_seq: f64,
    avg_par2: f64,
    avg_par4: f64,
) {
    let mut csv = String::from("NUM.,TIEMPO SECUENCIAL (ms),TIEMPO PARALELA 2 HILOS (ms),TIEMPO PARALELA 4 HILOS (ms)\n");

    for i in 0..seq.len() {
        csv.push_str(&format!(
            "{},{:.3},{:.3},{:.3}\n",
            i + 1,
            ms(seq[i]),
            ms(par2[i]),
            ms(par4[i])
        ));
    }

    csv.push_str(&format!(
        "Promedio,{:.3},{:.3},{:.3}\n",
        avg_seq * 1000.0,
        avg_par2 * 1000.0,
        avg_par4 * 1000.0
    ));

    fs::write("results/tiempos.csv", csv).expect("no se pudo escribir tiempos.csv");
}

fn write_speedup_csv(
    seq: &[Duration],
    par2: &[Duration],
    par4: &[Duration],
    avg_seq: f64,
    avg_par2: f64,
    avg_par4: f64,
) {
    let mut csv = String::from("NUM.,SPEEDUP 2 HILOS,SPEEDUP 4 HILOS\n");

    for i in 0..seq.len() {
        let speedup_2 = seq[i].as_secs_f64() / par2[i].as_secs_f64();
        let speedup_4 = seq[i].as_secs_f64() / par4[i].as_secs_f64();
        csv.push_str(&format!("{},{:.4},{:.4}\n", i + 1, speedup_2, speedup_4));
    }

    csv.push_str(&format!(
        "Promedio,{:.4},{:.4}\n",
        avg_seq / avg_par2,
        avg_seq / avg_par4
    ));

    fs::write("results/speedup.csv", csv).expect("no se pudo escribir speedup.csv");
}

fn write_efficiency_csv(
    seq: &[Duration],
    par2: &[Duration],
    par4: &[Duration],
    avg_seq: f64,
    avg_par2: f64,
    avg_par4: f64,
) {
    let mut csv = String::from("NUM.,EFFICIENCY 2 HILOS,EFFICIENCY 4 HILOS\n");

    for i in 0..seq.len() {
        let speedup_2 = seq[i].as_secs_f64() / par2[i].as_secs_f64();
        let speedup_4 = seq[i].as_secs_f64() / par4[i].as_secs_f64();
        csv.push_str(&format!(
            "{},{:.4},{:.4}\n",
            i + 1,
            speedup_2 / 2.0,
            speedup_4 / 4.0
        ));
    }

    csv.push_str(&format!(
        "Promedio,{:.4},{:.4}\n",
        (avg_seq / avg_par2) / 2.0,
        (avg_seq / avg_par4) / 4.0
    ));

    fs::write("results/efficiency.csv", csv).expect("no se pudo escribir efficiency.csv");
}
