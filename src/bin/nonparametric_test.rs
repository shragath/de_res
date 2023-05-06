use std::{
    fs::{self, File},
    io::{LineWriter, Write},
    vec,
};

use savefile::load_file;
fn main() {
    let dynamic_system = "r";
    let sets = ["all", "p", "f", "r"]; //, "_p", "_f", "_r"];
    let chaotic_maps = [
        "de_henon",
        "de_lozi",
        "de_logistic",
        "de_sinusoidal",
        "de_rust",
    ]; // ["de_henon", "de_lozi", "de_logistic", "de_sinusoidal"];

    let load_path = format!("data/{}/data_de.bin", dynamic_system);
    let _ds: Box<Vec<Vec<Vec<Vec<f64>>>>> = Box::new(load_file(load_path, 0).unwrap());
    let load_path = format!("data/{}/data_de_rust.bin", dynamic_system);
    let ds_rust: Box<Vec<Vec<f64>>> = Box::new(load_file(load_path, 0).unwrap());
    // let ds_slice: Vec<_> = ds_rust{{{
    //     .iter()
    //     .map(|vecs_data| &vecs_data[0..gens_plot])
    //     .collect();}}}
    let _runs: usize = 30;
    let _gens: usize = 2000;
    let min_val = 3.045603382462501; // r
    // let min_val = 4.297251559609671; // rr
    let umbral = 2.0 * min_val;
    // Get min value{{{
    // let mins: &f64 = &_ds[0]
    //     .iter()
    //     .map(|c_map| {
    //         c_map
    //             .iter()
    //             .map(|run| run.clone().into_iter().reduce(f64::min).unwrap())
    //             .reduce(f64::min)
    //             .unwrap()
    //     })
    //     .reduce(f64::min)
    //     .unwrap();
    //     println!("Min: {}", mins);
// }}}

    for (i_set, set) in sets.iter().enumerate() {
        let mut indices: Vec<usize> = vec![0; _runs];
        // let mut values: Vec<f64> = vec![0.; _gens];
        let mut max_indices: Vec<usize> = vec![0; chaotic_maps.len()];
        for (i_map, _map) in chaotic_maps.iter().enumerate() {
            let set: &Vec<Vec<f64>>;
            match i_map {
                4.. => set = &ds_rust,
                _ => set = &_ds[i_set][i_map],
            }
            for (i_run, run) in set.iter().enumerate() {
                for (i_gen, gen) in run.iter().enumerate() {
                    if *gen <= umbral {
                        indices[i_run] = i_gen;
                        // values[i_run] = *gen;
                        break;
                    }
                }
            }
            max_indices[i_map] = indices.clone().into_iter().reduce(usize::max).unwrap();
        }

        let _gen = max_indices.clone().into_iter().reduce(usize::max).unwrap();
        println!("set: {}, max index: {}, {:?}", &set, &_gen, max_indices);

        if let Err(_e) = wilcoxon_test_data(
            &_ds[i_set],
            &ds_rust,
            _gen,
            chaotic_maps,
            set,
            dynamic_system,
        ) {
            panic!("Error at fn wilcoxon_test_data");
        };
    }
}

// generacion de archivos para pruebas no parametricas
fn wilcoxon_test_data(
    ds: &Vec<Vec<Vec<f64>>>,
    ds_rust: &Vec<Vec<f64>>,
    index: usize,
    chaotic_maps: [&str; 5],
    set: &str,
    dynamic_system: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = format!("data/wilcoxon/{}/{}", dynamic_system, set);

    for (i_map, map) in ds.iter().enumerate() {
        let _path = format!("{}/algoritmos", &root_dir);
        match std::fs::create_dir_all(&_path) {
            Err(e) => panic!("Error while creating dirs in fn wilcoxon_test_data: {}", e),
            _ => (),
        };
        let _file = File::create(format!("{}/{}.in", _path, chaotic_maps[i_map]))?;
        let mut _write = LineWriter::new(_file);

        // writes chaotic maps data
        for run in map.iter() {
            if let Err(_e) = _write.write_all(format!("{}\n", run[index]).as_bytes()) {
                eprintln!("Error while writing file: {}", _e);
                std::process::exit(1);
            };
            if let Err(_e) = _write.flush() {
                eprintln!("Error while writing file: {}", _e);
                std::process::exit(1);
            };
        }

        // writes rust data
        let _file = File::create(format!("{}/{}.in", _path, "de_rust"))?;
        let mut _write = LineWriter::new(_file);
        for run in ds_rust.iter() {
            if let Err(_e) = _write.write_all(format!("{}\n", run[index]).as_bytes()) {
                eprintln!("Error while writing file: {}", _e);
                std::process::exit(1);
            };
            if let Err(_e) = _write.flush() {
                eprintln!("Error while writing file: {}", _e);
                std::process::exit(1);
            };
        }
    }

    let file_name = format!("{}/wilcoxon.R", root_dir);
    let _file = File::create(&file_name)?;
    if let Err(_e) = fs::copy("data/wilcoxon.R", &file_name) {
        panic!("Error while copying wilcoxon.R file to set: {}", _e);
    };

    let working_dir = std::env::current_dir().unwrap();
    let dest_file = format!("{}/data/wilcoxon_res.tex", working_dir.display());
    let command = "Rscript";
    let path = std::fs::canonicalize(root_dir).unwrap();

    std::process::Command::new(command)
        .current_dir(&path)
        .arg("wilcoxon.R")
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("failed to execute process");

    let arg = format!("cat wilcoxon.tex >> {}", dest_file);
    std::process::Command::new("sh")
        .current_dir(path)
        .arg("-c")
        .arg(arg)
        .spawn()
        .expect("failed to execute command");

    Ok(())
}
