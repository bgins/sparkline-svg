wit_bindgen::generate!({
    world: "sparkline",
    exports: {
        world: Component,
    }
});

use csscolorparser;
use svg::{
    node::{
        element::{
            path::{Command, Data, Parameters},
            Description, Path, Title,
        },
        Text,
    },
    Document,
};

pub struct Component;

impl Guest for Component {
    fn generate(
        data: Vec<u32>,
        title: String,
        description: String,
        width: u32,
        height: u32,
        line_color: String,
        fill_color: String,
    ) -> String {
        // Check line and fill color values
        csscolorparser::parse(line_color.as_str())
            .expect(format!("Could not parse line color value {line_color}").as_str());
        csscolorparser::parse(fill_color.as_str())
            .expect(format!("Could not parse fill color value {fill_color}").as_str());

        // max_y is used to invert inputs to start at the bottom left
        // and to set the height of the view box.
        let max_y = data.iter().fold(0, |acc, n| acc.max(*n));

        // Build up line data
        let mut line_data = Data::new().move_to((0, max_y - data[0]));
        for (index, y) in data.iter().enumerate() {
            let y = max_y - y;

            line_data.append(Command::Line(
                svg::node::element::path::Position::Absolute,
                Parameters::from(vec![index as f32, y as f32]),
            ));
        }

        // Build up fill data
        let fill_data = line_data
            .clone()
            .line_to((data.len(), max_y))
            .line_to((0, max_y))
            .close();

        // Generate line and fill paths
        let line = Path::new()
            .set("fill", "transparent")
            .set("stroke", line_color)
            .set("stroke-width", 4)
            .set("vector-effect", "non-scaling-stroke")
            .set("d", line_data);
        let fill = Path::new()
            .set("fill", fill_color)
            .set("stroke", "transparent")
            .set("d", fill_data);

        // Title and description are included for accessibility
        let title = Title::new().add(Text::new(title));
        let description = Description::new().add(Text::new(description));

        // Generate SVG document
        let document = Document::new()
            .set("width", format!("{width}px"))
            .set("height", format!("{height}px"))
            .set("viewBox", (0, 0, data.len() - 1, max_y))
            .set("preserveAspectRatio", "none")
            .set("role", "img")
            .add(title)
            .add(description)
            .add(line)
            .add(fill);

        document.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_svg() {
        let result = Component::generate(
            vec![1, 0, 5, 4, 8, 10, 15, 10, 5, 4],
            "mySVG".to_string(),
            "It's a sparkline".to_string(),
            500,
            180,
            "#2e4374".to_string(),
            "#7c81ad".to_string(),
        );

        assert_eq!(
            result,
            r##"<svg height="180px" preserveAspectRatio="none" role="img" viewBox="0 0 9 15" width="500px" xmlns="http://www.w3.org/2000/svg">
<title>
mySVG
</title>
<desc>
It's a sparkline
</desc>
<path d="M0,14 L0,14 L1,15 L2,10 L3,11 L4,7 L5,5 L6,0 L7,5 L8,10 L9,11" fill="transparent" stroke="#2e4374" stroke-width="4" vector-effect="non-scaling-stroke"/>
<path d="M0,14 L0,14 L1,15 L2,10 L3,11 L4,7 L5,5 L6,0 L7,5 L8,10 L9,11 L10,15 L0,15 z" fill="#7c81ad" stroke="transparent"/>
</svg>"##
        )
    }
}
