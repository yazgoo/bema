extern crate bema;

use plotters::prelude::*;
use bema::*;
use indoc::indoc;
use compile_time_run::run_command;
use image::ImageBuffer;
use image::DynamicImage;

fn image_from_plot(f: &dyn Fn(&mut [u8], u32, u32) -> Result<(), Box<dyn std::error::Error>>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    const PLOT_WIDTH : usize = 640;
    const PLOT_HEIGHT : usize = 480;
    let mut plot :[u8;PLOT_WIDTH*PLOT_HEIGHT*3] = [0;PLOT_WIDTH*PLOT_HEIGHT*3];
    let _ = f(&mut plot, PLOT_WIDTH as u32, PLOT_HEIGHT as u32)?;
    let img = DynamicImage::ImageRgb8(ImageBuffer::from_raw(PLOT_WIDTH as u32, PLOT_HEIGHT as u32, plot.to_vec()).unwrap());
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
    Ok(bytes)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    slides(|b| {

        b.slide("a slide with just text", |s| {
            s.text("text in the first slide")
            .text("")
            .t("t is an alias for text")
            .text("")
            .text("multi line text is also supported:")
            .text(indoc! {"

            • this is very usefull
            • for lists
            • while keeping them
            • centered

                "})
        })

        .slide("code", |s| {
            s.t("").t("helloworld.c").t("")
            .code("c", indoc! {r#"
                #include <stdio.h>
                int main() {
                   printf("Hello, World!");
                   return 0;
                }
                "#})
        })

        .slide("diagram - requires dot (graphviz) at compile time", |s| {
            s.image(run_command!("sh", "-c", r##"echo '
            digraph X {
                rankdir = "LR"
                bgcolor= "transparent"
                node [ style=filled,fill="#65b2ff", fontname = "helvetica", shape = "rectangle" ]
                edge [ color="#65b2ff" , fontname = "helvetica", fontcolor="#65b2ff"]
                graph [ fontname = "helvetica", color="#3f6190", fontcolor="#3f6190", nodesep="0" ];
                a -> b -> c
                b -> d
                dpi=500
            }
            ' | dot -Tpng"##).to_vec(), ".png", Some(500))
        })

        .slide("plotting (using plotters crate)", |s| {
            s.image(image_from_plot(
                    &(|plot: &mut [u8], width: u32, height: u32| -> Result<(), Box<dyn std::error::Error>>  {
                        let root =
                            BitMapBackend::with_buffer(plot, (width as u32, height as u32)).into_drawing_area();

                        root.fill(&WHITE)?;

                        let mut chart = ChartBuilder::on(&root)
                            .x_label_area_size(35)
                            .y_label_area_size(40)
                            .margin(5)
                            .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

                        chart
                            .configure_mesh()
                            .disable_x_mesh()
                            .bold_line_style(&WHITE.mix(0.3))
                            .y_desc("Count")
                            .x_desc("Bucket")
                            .axis_desc_style(("sans-serif", 15))
                            .draw()?;

                        let data = [
                            0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
                        ];

                        chart.draw_series(
                            Histogram::vertical(&chart)
                            .style(RED.mix(0.5).filled())
                            .data(data.iter().map(|x: &u32| (*x, 1))),
                        )?;

                        Ok(())
                    })).unwrap(), ".png", None)
                .t("a plot")
        })

        .slide("image and code aligned vertically", |s| {
            s.rows(2)
                .image(include_bytes!("capybara.jpg").to_vec(), ".jpg", Some(500))
                .code("rs", indoc! {r#"
                    // main function
                    fn main() {

                        // Print to the console
                        println!("Hello World!");
                    }
                "#})
        })

    }).run()?;

    Ok(())
}
