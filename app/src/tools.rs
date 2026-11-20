#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Select,
    Pen,
    Line,
    Rectangle,
    Ellipse,
    Text,
    Brush,
    Eraser,
    Eyedropper,
    ColorPicker,
    Hand,
    Zoom,
    Slice,
    Hotspot,
    Symbol,
}

impl Tool {
    pub fn name(&self) -> &str {
        match self {
            Tool::Select => "Select",
            Tool::Pen => "Pen",
            Tool::Line => "Line",
            Tool::Rectangle => "Rectangle",
            Tool::Ellipse => "Ellipse",
            Tool::Text => "Text",
            Tool::Brush => "Brush",
            Tool::Eraser => "Eraser",
            Tool::Eyedropper => "Eyedropper",
            Tool::ColorPicker => "Color Picker",
            Tool::Hand => "Hand",
            Tool::Zoom => "Zoom",
            Tool::Slice => "Slice",
            Tool::Hotspot => "Hotspot",
            Tool::Symbol => "Symbol",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tool::Select => "L",
            Tool::Pen => "P",
            Tool::Line => "|",
            Tool::Rectangle => "R",
            Tool::Ellipse => "O",
            Tool::Text => "T",
            Tool::Brush => "B",
            Tool::Eraser => "E",
            Tool::Eyedropper => "I",
            Tool::ColorPicker => "C",
            Tool::Hand => "H",
            Tool::Zoom => "Z",
            Tool::Slice => "S",
            Tool::Hotspot => "Hs",
            Tool::Symbol => "Sy",
        }
    }
}
