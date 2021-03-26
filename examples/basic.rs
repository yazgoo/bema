extern crate bema;

use bema::*;
use indoc::indoc;
use compile_time_run::run_command;

fn main() {

    slides(|b| {

        b.slide("a slide with just text", |s| {
            s.text("text in the first slide")
            .text("")
            .t("t is an alias for text")
        })

        .slide("code", |s| {
            s.t("helloworld.rb").t("")
            .code("rb", indoc! {r#"
              puts 'Hello, world!'
              "#})
            .t("").t("helloworld.c").t("")
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
                dpi=200
            }
            ' | dot -Tpng"##), ".png")
        })

        .slide("image", |s| {
            s.image(include_bytes!("capybara.jpg"), ".jpg")
        })

    }).run().unwrap();


}
