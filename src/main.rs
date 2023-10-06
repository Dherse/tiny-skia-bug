use sk::Pixmap;
use tiny_skia as sk;

const STROKE: &[u8] = include_bytes!("stroke.png");
const FILL: &[u8] = include_bytes!("fill.png");

fn main() {
    let pixel_per_pt: f32 = 2.0;
    let size = 100.0 * pixel_per_pt;

    // Load the stroke and fill (instead of generating it)
    let stroke = sk::Pixmap::decode_png(STROKE).unwrap();
    let fill = sk::Pixmap::decode_png(FILL).unwrap();

    // Setup the canvas
    let mut canvas = sk::Pixmap::new(size.round() as u32, size.round() as u32).unwrap();
    canvas.fill(sk::Color::WHITE);

    // The shape's transform (just centering it)
    let ts = sk::Transform::from_translate(25.0, 25.0);

    // Setup the shape (a simple square for this example)
    let (w, h) = (50.0, 50.0);
    let rect = sk::Rect::from_xywh(0.0, 0.0, w, h).unwrap();
    let path = sk::PathBuilder::from_rect(rect);

    // We will fill the path with the gradient
    // The scale here is because the stroke is 30px wide
    let paint = sk_paint(&fill, sk::Transform::identity());
    let fill_rule = sk::FillRule::default();
    canvas.fill_path(&path, &paint, fill_rule, ts, None);

    // Create the stroke itself
    let rect_stroke = sk::Stroke {
        width: 15.0,
        miter_limit: 4.0,
        line_cap: sk::LineCap::Butt,
        line_join: sk::LineJoin::Miter,
        dash: None,
    };

    let paint = sk_paint(&stroke, sk::Transform::identity());
    canvas.stroke_path(&path, &paint, &rect_stroke, ts, None);

    canvas.save_png("output.png").unwrap();

}

fn sk_paint(stroke: &Pixmap, fill_transform: sk::Transform) -> sk::Paint {
    let mut sk_paint: sk::Paint<'_> = sk::Paint::default();
    sk_paint.anti_alias = false;
    sk_paint.shader = sk::Pattern::new(
        stroke.as_ref(),
        sk::SpreadMode::Pad,
        sk::FilterQuality::Nearest,
        1.0,
        fill_transform
            .pre_scale(1.0 / 2.0, 1.0 / 2.0),
    );

    sk_paint
}
