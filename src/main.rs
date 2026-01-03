use std::fs;
use printpdf::*;
use crate::compiler::parser::Parser;
use crate::compiler::pdf::{PdfData, PdfRenderer};

mod compiler;
mod util;

fn main() {
    let src = r#"hello world :3"#;
    let mut parser = Parser::new(src);
    let ast = parser.parse();
    // println!("{:#?}", ast);

    let metadata = PdfData {
        name: "pdf",
        path: "./output.pdf",
        width: Mm(inches!(8.5)),
        height: Mm(inches!(11)),
    };

    let mut renderer = PdfRenderer::new(ast, metadata);
    let warnings = renderer.render();
    println!("{:#?}", warnings);
    // let line = Line {
    //     // Quadratic shape. The "false" determines if the next (following)
    //     // point is a bezier handle (for curves)
    //     // If you want holes, simply reorder the winding of the points to be
    //     // counterclockwise instead of clockwise.
    //     points: vec![
    //         LinePoint {
    //             p: Point::new(Mm(100.0), Mm(100.0)),
    //             bezier: false,
    //         },
    //         LinePoint {
    //             p: Point::new(Mm(100.0), Mm(200.0)),
    //             bezier: false,
    //         },
    //         LinePoint {
    //             p: Point::new(Mm(300.0), Mm(200.0)),
    //             bezier: false,
    //         },
    //         LinePoint {
    //             p: Point::new(Mm(300.0), Mm(100.0)),
    //             bezier: false,
    //         },
    //     ],
    //     is_closed: true,
    // };
    //
    // // Triangle shape
    // let polygon = Polygon {
    //     rings: vec![
    //         PolygonRing {
    //             points: vec![
    //                 LinePoint {
    //                     p: Point::new(Mm(150.0), Mm(150.0)),
    //                     bezier: false,
    //                 },
    //                 LinePoint {
    //                     p: Point::new(Mm(150.0), Mm(250.0)),
    //                     bezier: false,
    //                 },
    //                 LinePoint {
    //                     p: Point::new(Mm(350.0), Mm(250.0)),
    //                     bezier: false,
    //                 },
    //             ],
    //
    //         }
    //     ],
    //     mode: PaintMode::FillStroke,
    //     winding_order: WindingOrder::NonZero,
    // };
    //
    // // Graphics config
    // let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    // let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
    // let mut dash_pattern = LineDashPattern::default();
    // dash_pattern.dash_1 = Some(20);
    //
    // let extgstate = ExtendedGraphicsState::default();
    //
    // let page1_contents = vec![
    //     // add line1 (square)
    //     Op::SetOutlineColor { col: Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None)) },
    //     Op::SetOutlineThickness { pt: Pt(10.0) },
    //     Op::DrawLine { line },
    //
    //     // add line2 (triangle)
    //     Op::SaveGraphicsState,
    //     Op::LoadGraphicsState { gs: doc.add_graphics_state(extgstate) },
    //     Op::SetLineDashPattern { dash: dash_pattern },
    //     Op::SetLineJoinStyle { join: LineJoinStyle::Round },
    //     Op::SetLineCapStyle { cap: LineCapStyle::Round },
    //     Op::SetFillColor { col: fill_color },
    //     Op::SetOutlineThickness { pt: Pt(15.0) },
    //     Op::SetOutlineColor { col: outline_color },
    //     Op::DrawPolygon { polygon },
    //     Op::RestoreGraphicsState,
    // ];
    //
    // let page1 = PdfPage::new(Mm(215.9), Mm(279.4), page1_contents);
}
