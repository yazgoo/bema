extern crate bema;

use bema::*;
use indoc::indoc;

fn main() {

    slides(|b| {

        b.slide("a slide with just text", |s| {
            s.text("text in the first slide")
            .text("")
            .t("t is an alias for text")
        })

        .slide("code", |s| {
            s.code("rb", indoc! {r#"
              def examplescode(a, b)
                p a
              end
                "#})
        })

        .slide("diagram (require dot (graphviz))", |s| {
            s.diagram(indoc! {r#"
            diagram 'digraph {
              a b
              dpi = 55
            }
        "#})
        })

        .slide("image", |s| {
            s.image("")
        })

    }).run()

}
