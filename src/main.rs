use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::table;

use raytracing::scenes::SceneType;

fn seperated<T>(num: T) -> String
where
    T: std::fmt::Display,
{
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The scene to render
    #[arg(short = 't', long, value_enum)]
    scene_type: SceneType,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let settings = raytracing::Settings::from_scene_type(args.scene_type);

    let nr_pixels = settings.image.image_width * settings.image.image_height;
    // Set up progress bar
    let progress = ProgressBar::new(nr_pixels as u64).with_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40} {percent}%").unwrap(),
    );

    println!(
        "Rendering {}x{} image...",
        settings.image.image_width, settings.image.image_height
    );

    let now = std::time::Instant::now();

    // Render
    let image = raytracing::render_image_from_settings(settings, &progress);

    let render_time = now.elapsed();
    let single_core_render_time = render_time.mul_f32(num_cpus::get() as f32);

    image.write_ppm("img.ppm")?;
    progress.finish();

    println!("Done!");

    // Print stats
    let nr_initial_rays = image.width * image.height * settings.renderer.samples_per_pixel;
    let table = table!(
        ["Nr. of threads", num_cpus::get()],
        [
            "Total render time",
            format!("{}ms", seperated(render_time.as_millis()))
        ],
        [
            "Total single core render time",
            format!("{:.2}s", single_core_render_time.as_secs_f32())
        ],
        [
            "Time per initial ray",
            format!(
                "{:.2}ns",
                render_time.as_nanos() as f32 / nr_initial_rays as f32
            )
        ],
        [
            "Single core time per initial ray",
            format!(
                "{:.2}ns",
                single_core_render_time.as_nanos() as f32 / nr_initial_rays as f32
            )
        ]
    );
    table.printstd();

    Ok(())
}
