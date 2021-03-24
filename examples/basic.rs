extern crate bema;

use bema::*;

fn main() {

    slides(|| {

        slide("a slide with just text", || {
            text("text in the first slide");
            text("");
            t("t is an alias for text");

        });

        slide("code", || {
            code("rb", r#"
              def example_code(a, b)
                p a
              end
                "#);
        });

        slide("diagram (require dot (graphviz))", || {
            diagram(r#"
            diagram 'digraph {
              a b
              dpi = 55
            }
        "#);
        });

        slide("image", || {
            image("");
        });

    });

}
