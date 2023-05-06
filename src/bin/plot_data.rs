use hypermelon::prelude::*;
use poloto::build;
use savefile::load_file;

fn main() {
    let dynamic_system = "rr";
    let load_path = format!("data/{}/data_de.bin", dynamic_system);
    let _ds: Box<Vec<Vec<Vec<Vec<f64>>>>> = Box::new(load_file(load_path, 0).unwrap());
    let sets = ["all", "p", "f", "r"]; //, "_p", "_f", "_r"];
    let chaotic_maps = ["de_henon", "de_lozi", "de_logistic", "de_sinusoidal"]; // ["de_henon", "de_lozi", "de_logistic", "de_sinusoidal"];
    let dir = format!("data/{}/plots", dynamic_system);

    let gens_plot: usize = 1000;

    for (i_s, set) in sets.iter().enumerate() {
        println!("Plotting {}", set);
        for (i_cm, map) in chaotic_maps.iter().enumerate() {
            let ds_conv = &_ds[i_s][i_cm];
            let ds_slice: Vec<_> = ds_conv
                .into_iter()
                .map(|vecs_data| &vecs_data[0..gens_plot])
                .collect();

            let path = format!("{}/{}/", dir, map);
            match std::fs::create_dir(&path) {
                _ => (),
            }

            let _file_name = format!("{}_{}.svg", map, set);
            let path = format!("{}/{}", path, _file_name);
            let file = std::fs::File::create(&path).unwrap();

            // plot data
            let header = format!("{} {}", map, set);
            match graph_data(ds_slice, file, header) {
                Ok(_) => println!("{}", map),
                _ => panic!("Error while plotting data"),
            };
        }
        println!();
    }


    let load_path = format!("data/{}/data_de_rust.bin", dynamic_system);
    let ds_rust: Box<Vec<Vec<f64>>> = Box::new(load_file(load_path, 0).unwrap());
    let ds_slice: Vec<_> = ds_rust
        .iter()
        .map(|vecs_data| &vecs_data[0..gens_plot])
        .collect();

    let file_path = format!("data/{}/plots/de_rust.svg", dynamic_system);
    let file = std::fs::File::create(file_path).unwrap();
    match graph_data(ds_slice, file, String::from("de_rust all")) {
        Ok(_) => println!("\n de_rust"),
        _ => panic!("Error while plotting data"),
    };
}

fn graph_data(ds: Vec<&[f64]>, file: std::fs::File, header_label: String) -> Result<(), &str> {
    let svg = poloto::header().with_viewbox_width(1200.0);

    let style =
        poloto::render::Theme::light().append(".poloto_line{stroke-dasharray:2;stroke-width:2;}");
    let style = style.append(".poloto_imgs.poloto_ticks{stroke-width:4;}");

    let plots = poloto::plots!(ds.iter().map(|vec_data| {
        build::plot("").line(vec_data.iter().enumerate().map(|(i_d, v)| (i_d as f64, v)))
    }));

    poloto::frame()
        .with_tick_lines([true, true])
        .with_viewbox(svg.get_viewbox())
        .build()
        .data(poloto::plots!(build::origin(), plots))
        .build_and_label((header_label, "Gens", "Objective function"))
        .append_to(svg.append(style))
        .render_io_write(file)
        .unwrap();

    Ok(())

    // let command = format!("eog {} > /dev/null", path);{{{
    // std::process::Command::new("sh")
    //     .args(["-c", &command])
    //     .spawn()
    //     .expect("failed to execute process");}}}
}
