use std::fs;
use std::time::Instant;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use savefile::save_file;

fn main() {
    let start = Instant::now();

    let path = "data/rr/";
    let sets = ["all", "_p", "_f", "_r"]; // ["all", "_p", "_f", "_r"];
    let chaotic_maps = [
        // "de_rust",
        "de_henon",
        "de_lozi",
        "de_logistic",
        "de_sinusoidal",
    ]; // ["de_henon", "de_lozi", "de_logistic", "de_sinusoidal"];
    let _runs = 30;
    let _gens = 2000;
    // let mins: Vec<f64> = vec![];
    let save = true;

    let mut _data: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![]; chaotic_maps.len()]; sets.len()];

    for set in 0..sets.len() {
        println!("Reading set {}", sets[set]);
        for map in 0..chaotic_maps.len() {
            // Use Rayon for parallel execution
            let data_run: Vec<Vec<f64>> = (0.._runs)
                .into_par_iter()
                .map(|run| {
                    // read 2000 csv files
                    (0.._gens)
                        .into_iter()
                        .map(|gen| {
                            let file_path = format!(
                                "{}/{}/{}/run_{}/pop_{}.csv",
                                path, sets[set], chaotic_maps[map], run, gen
                            );
                            let ds = fs::read_to_string(file_path).expect("File not found");
                            // read line and extract objective function result from line
                            let row: Vec<f64> = ds
                                .lines()
                                .map(|line| {
                                    let v = line
                                        .split(",")
                                        .collect::<Vec<&str>>()
                                        .pop()
                                        .unwrap()
                                        .trim()
                                        .parse::<f64>();

                                    match v {
                                        Ok(val) => val,
                                        Err(e) => panic!("error: {}", e),
                                    }
                                })
                                .collect();

                            let min = row.clone().into_iter().reduce(f64::min).unwrap();
                            min
                        })
                        .collect::<Vec<f64>>()
                })
                .collect();

            // for run in 0.._runs {{{{
            //     for gen in 0.._gens {
            //         let file_path = format!(
            //             "{}/{}/{}/run_{}/pop_{}.csv",
            //             path, sets[set], chaotic_maps[map], run, gen
            //         );
            //         let ds = fs::read_to_string(file_path).expect("File not found");
            //         let row: Vec<f64> = ds
            //             .lines()
            //             .map(|line| {
            //                 let v = line
            //                     .split(",")
            //                     .collect::<Vec<&str>>()
            //                     .pop()
            //                     .unwrap()
            //                     .trim()
            //                     .parse::<f64>();
            //
            //                 match v {
            //                     Ok(val) => val,
            //                     Err(e) => panic!("error: {}", e),
            //                 }
            //             })
            //             .collect();
            //         let best = row.clone().into_iter().reduce(f64::min).unwrap();
            //         // data_all[run][gen] = best;
            //         _data[set][map][run][gen] = best;
            //         // println!("len: {}, \n {:?}", row.len(), row);
            //         // println!("{}", best);
            //     }
            // }}}}
            _data[set][map] = data_run;
            println!("Data loaded {}", chaotic_maps[map]);
        }
    }

    println!(
        "Sets: {}, maps: {}, runs: {}, gens: {} \n",
        sets.len(),
        chaotic_maps.len(),
        _runs,
        _gens
    );
    // println!("{}", &_data[0][0].len());

    if save {
        save_file("data/rr/data_de.bin", 0, &_data).unwrap();
    }

    let duration = start.elapsed();
    println!("Finished in: {:?}", duration);
}
