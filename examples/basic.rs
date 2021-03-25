extern crate bema;

use bema::*;

fn main() {

    slides(|mut b| {

        b.slide("a slide with just text", |mut s| {
            s.text("text in the first slide");
            s.text("");
            s.t("t is an alias for text");
            s
        });

        b.slide("code", |mut s| {
            s.code("rb", r#"
              def examplescode(a, b)
                p a
              end
                "#);
            s
        });

        b.slide("diagram (require dot (graphviz))", |mut s| {
            s.diagram(r#"
            diagram 'digraph {
              a b
              dpi = 55
            }
        "#);
            s
        });

        b.slide("image", |mut s| {
            s.image("");
            s
        });

    });

}
