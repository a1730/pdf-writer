use super::*;

/// Writer for the _annotations array_ in a [_page_](Page).
///
/// This struct is created by [`Page::annotations`].
pub struct Annotations<'a> {
    array: Array<&'a mut PdfWriter>,
}

impl<'a> Annotations<'a> {
    pub(crate) fn start(obj: Obj<&'a mut PdfWriter>) -> Self {
        Self { array: obj.array() }
    }

    /// Start writing a new annotation dictionary.
    pub fn push(&mut self) -> Annotation<'_> {
        Annotation::new(self.obj())
    }
}

deref!('a, Annotations<'a> => Array<&'a mut PdfWriter>, array);

/// Writer for an _annotation dictionary_.
///
/// This struct is created by [`Annotations::push`].
pub struct Annotation<'a> {
    dict: Dict<&'a mut PdfWriter>,
}

impl<'a> Annotation<'a> {
    pub(crate) fn new(obj: Obj<&'a mut PdfWriter>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Annot"));
        Self { dict }
    }

    /// Write the `/Subtype` attribute to tell the viewer the type of this
    /// particular annotation.
    pub fn subtype(&mut self, kind: AnnotationType) -> &mut Self {
        self.pair(Name(b"Subtype"), kind.to_name());
        self
    }

    /// Write the `/Rect` attribute. This is the location and dimensions of the
    /// annotation on the page.
    pub fn rect(&mut self, rect: Rect) -> &mut Self {
        self.pair(Name(b"Rect"), rect);
        self
    }

    /// Write the `/Contents` attribute. This is the content or alt-text,
    /// depending on the [`AnnotationType`].
    pub fn contents(&mut self, text: TextStr) -> &mut Self {
        self.pair(Name(b"Contents"), text);
        self
    }

    /// Write the `/NM` attribute. This uniquely identifies the anotation on the
    /// page. (1.3+)
    pub fn name(&mut self, text: TextStr) -> &mut Self {
        self.pair(Name(b"NM"), text);
        self
    }

    /// Write the `/M` attribute, specifying the date the annotation was last
    /// modified. (1.1+)
    pub fn modified(&mut self, date: Date) -> &mut Self {
        self.pair(Name(b"M"), date);
        self
    }

    /// Write the `/F` attribute.
    pub fn flags(&mut self, flags: AnnotationFlags) -> &mut Self {
        self.pair(Name(b"F"), flags.bits() as i32);
        self
    }

    /// Write the `/Border` attribute. This describes the look of the border
    /// around the annotation, including width and horizontal and vertical
    /// border radii. The function may also receive a dash pattern which
    /// specifies the lengths and gaps of the border segments on a dashed
    /// border. Although all PDF versions accept `/Border`, this feature
    /// specifically is only available in PDF 1.1 or later.
    pub fn border(
        &mut self,
        h_radius: f32,
        v_radius: f32,
        width: f32,
        dash_pattern: Option<impl IntoIterator<Item = f32>>,
    ) -> &mut Self {
        let mut array = self.key(Name(b"Border")).array();
        array.item(h_radius);
        array.item(v_radius);
        array.item(width);

        if let Some(pattern) = dash_pattern {
            array.obj().array().typed().items(pattern);
        }

        array.finish();
        self
    }

    /// Start writing the `/BS` attribute. These are some more elaborate border
    /// settings taking precedence over `/B` for some annotation types. (1.2+)
    pub fn border_style(&mut self) -> BorderStyle<'_> {
        BorderStyle::new(self.key(Name(b"BS")))
    }

    /// Write the `/C` attribute forcing a transparent color. This sets the
    /// annotations background color and its popup title bar color. (1.1+)
    pub fn color_transparent(&mut self) -> &mut Self {
        self.key(Name(b"C")).array().typed::<f32>();
        self
    }

    /// Write the `/C` attribute using a grayscale color. This sets the
    /// annotations background color and its popup title bar color. (1.1+)
    pub fn color_gray(&mut self, gray: f32) -> &mut Self {
        self.key(Name(b"C")).array().typed().item(gray);
        self
    }

    /// Write the `/C` attribute using an RGB color. This sets the annotations
    /// background color and its popup title bar color. (1.1+)
    pub fn color_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.key(Name(b"C")).array().typed().items([r, g, b]);
        self
    }

    /// Write the `/C` attribute using a CMYK color. This sets the annotations
    /// background color and its popup title bar color. (1.1+)
    pub fn color_cmyk(&mut self, c: f32, m: f32, y: f32, k: f32) -> &mut Self {
        self.key(Name(b"C")).array().typed().items([c, m, y, k]);
        self
    }

    /// Start writing the `/A` dictionary. Only permissible for the subtype
    /// `Link`.
    pub fn action(&mut self) -> Action<'_> {
        Action::new(self.key(Name(b"A")))
    }

    /// Write the `/H` attribute to set what effect is used to convey that the
    /// user is pressing a link annotation. Only permissible for the subtype
    /// `Link`. (1.2+)
    pub fn highlight(&mut self, effect: HighlightEffect) -> &mut Self {
        self.pair(Name(b"H"), effect.to_name());
        self
    }

    /// Write the `/T` attribute. This is in the title bar of markup annotations
    /// and should be the name of the annotation author. (1.1+)
    pub fn author(&mut self, text: TextStr) -> &mut Self {
        self.pair(Name(b"T"), text);
        self
    }

    /// Write the `/Subj` attribute. This is the subject of the annotation.
    /// (1.5+)
    pub fn subject(&mut self, text: TextStr) -> &mut Self {
        self.pair(Name(b"Subj"), text);
        self
    }

    /// Write the `/QuadPoints` attribute, specifying the region in which the
    /// link should be activated. (1.6+)
    pub fn quad_points(
        &mut self,
        coordinates: impl IntoIterator<Item = f32>,
    ) -> &mut Self {
        self.key(Name(b"QuadPoints")).array().typed().items(coordinates);
        self
    }

    /// Write the `/LL` attribute. This defines the start and end point of a
    /// line annotation
    pub fn line_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        self.key(Name(b"LL")).array().typed().items([x1, y1, x2, y2]);
        self
    }

    /// Start writing the `/FS` attribute, setting which file to reference.
    pub fn file(&mut self) -> FileSpec<'_> {
        FileSpec::new(self.key(Name(b"FS")))
    }

    /// Write the `/Name` attribute with one of the predefined icon names. Refer
    /// to the specification to see which names are allowed for which annotation
    /// types.
    pub fn icon_predefined(&mut self, icon: AnnotationIcon) -> &mut Self {
        self.pair(Name(b"Name"), icon.to_name());
        self
    }

    /// Write the `/Name` attribute with a custom icon name.
    pub fn icon_custom(&mut self, name: Name) -> &mut Self {
        self.pair(Name(b"Name"), name);
        self
    }
}

deref!('a, Annotation<'a> => Dict<&'a mut PdfWriter>, dict);

/// Kind of the annotation to produce.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnnotationType {
    /// Inline text.
    Text,
    /// A link.
    Link,
    /// A line. (1.3+)
    Line,
    /// A square. (1.3+)
    Square,
    /// A circle. (1.3+)
    Circle,
    /// Highlighting the text on the page. (1.3+)
    Highlight,
    /// Underline the text on the page. (1.3+)
    Underline,
    /// Squiggly underline of the text on the page. (1.4+)
    Squiggly,
    /// Strike out the text on the page. (1.3+)
    StrikeOut,
    /// A reference to another file. (1.3+)
    FileAttachment,
}

impl AnnotationType {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::Text => Name(b"Text"),
            Self::Link => Name(b"Link"),
            Self::Line => Name(b"Line"),
            Self::Square => Name(b"Square"),
            Self::Circle => Name(b"Circle"),
            Self::Highlight => Name(b"Highlight"),
            Self::Underline => Name(b"Underline"),
            Self::Squiggly => Name(b"Squiggly"),
            Self::StrikeOut => Name(b"StrikeOut"),
            Self::FileAttachment => Name(b"FileAttachment"),
        }
    }
}

/// Possible icons for an annotation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnnotationIcon {
    /// Speech bubble. For use with text annotations.
    Comment,
    /// For use with text annotations.
    Key,
    /// Sticky note. For use with text annotations.
    Note,
    /// Question mark or manual. For use with text annotations.
    Help,
    /// For use with text annotations.
    NewParagraph,
    /// For use with text annotations.
    Paragraph,
    /// A plus or similar. For use with text annotations.
    Insert,
    /// Chart. For use with file attachment annotations.
    Graph,
    /// For use with file attachment annotations.
    PushPin,
    /// For use with file attachment annotations.
    Paperclip,
    /// For use with file attachment annotations.
    Tag,
}

impl AnnotationIcon {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::Comment => Name(b"Comment"),
            Self::Key => Name(b"Key"),
            Self::Note => Name(b"Note"),
            Self::Help => Name(b"Help"),
            Self::NewParagraph => Name(b"NewParagraph"),
            Self::Paragraph => Name(b"Paragraph"),
            Self::Insert => Name(b"Insert"),
            Self::Graph => Name(b"Graph"),
            Self::PushPin => Name(b"PushPin"),
            Self::Paperclip => Name(b"Paperclip"),
            Self::Tag => Name(b"Tag"),
        }
    }
}

bitflags::bitflags! {
    /// Bitflags describing various characteristics of annotations.
    pub struct AnnotationFlags: u32 {
        /// This will hide the annotation if the viewer does not recognize its
        /// subtype. Otherwise, it will be rendered as specified in its apprearance
        /// stream.
        const INVISIBLE = 1 << 0;
        /// This hides the annotation from view and disallows interaction. (1.2+)
        const HIDDEN = 1 << 1;
        /// Print the annotation. If not set, it will be always hidden on print.
        /// (1.2+)
        const PRINT = 1 << 2;
        /// Do not zoom the annotation appearance if the document is zoomed in.
        /// (1.3+)
        const NO_ZOOM = 1 << 3;
        /// Do not rotate the annotation appearance if the document is zoomed in.
        /// (1.3+)
        const NO_ROTATE = 1 << 4;
        /// Do not view the annotation on screen. It may still show on print.
        /// (1.3+)
        const NO_VIEW = 1 << 5;
        /// Do not allow interactions. (1.3+)
        const READ_ONLY = 1 << 6;
        /// Do not allow the user to delete or reposition the annotation. Contents
        /// may still be changed. (1.4+)
        const LOCKED = 1 << 7;
        /// Invert the interpretation of the `no_view` flag for certain events.
        /// (1.5+)
        const TOGGLE_NO_VIEW = 1 << 8;
        /// Do not allow content changes. (1.7+)
        const LOCKED_CONTENTS = 1 << 9;
    }
}

/// Writer for an _action dictionary_.
///
/// This struct is created by [`Annotation::action`].
pub struct Action<'a> {
    dict: Dict<&'a mut PdfWriter>,
}

impl<'a> Action<'a> {
    pub(crate) fn new(obj: Obj<&'a mut PdfWriter>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Action"));
        Self { dict }
    }

    /// Write the `/S` attribute to set the action type.
    pub fn action_type(&mut self, kind: ActionType) -> &mut Self {
        self.pair(Name(b"S"), kind.to_name());
        self
    }

    /// Start writing the `/D` attribute to set the destination of this
    /// GoTo-type action.
    pub fn dest_direct(&mut self, page: Ref) -> Destination<'_> {
        Destination::start(self.key(Name(b"D")), page)
    }

    /// Write the `/D` attribute to set the destination of this GoTo-type action
    /// to a named destination.
    pub fn dest_named(&mut self, name: Name) -> &mut Self {
        self.pair(Name(b"D"), name);
        self
    }

    /// Start writing the `/F` attribute, setting which file to go to or which
    /// application to launch.
    pub fn file(&mut self) -> FileSpec<'_> {
        FileSpec::new(self.key(Name(b"F")))
    }

    /// Write the `/NewWindow` attribute to set whether this remote GoTo action
    /// should open the referenced destination in another window.
    pub fn new_window(&mut self, new: bool) -> &mut Self {
        self.pair(Name(b"NewWindow"), new);
        self
    }

    /// Write the `/URI` attribute to set where this link action goes.
    pub fn uri(&mut self, uri: Str) -> &mut Self {
        self.pair(Name(b"URI"), uri);
        self
    }

    /// Write the `/IsMap` attribute to set if the click position of the user's
    /// cursor inside the link rectangle should be appended to the referenced
    /// URI as a query parameter.
    pub fn is_map(&mut self, map: bool) -> &mut Self {
        self.pair(Name(b"IsMap"), map);
        self
    }
}

deref!('a, Action<'a> => Dict<&'a mut PdfWriter>, dict);

/// What kind of action to perform when clicking a link annotation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ActionType {
    /// Go to a destination in the document.
    GoTo,
    /// Go to a destination in another document.
    RemoteGoTo,
    /// Launch an application.
    Launch,
    /// Open a URI.
    Uri,
}

impl ActionType {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::GoTo => Name(b"GoTo"),
            Self::RemoteGoTo => Name(b"GoToR"),
            Self::Launch => Name(b"Launch"),
            Self::Uri => Name(b"URI"),
        }
    }
}

/// Highlighting effect applied when a user holds the mouse button over some
/// annotations.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HighlightEffect {
    /// No effect.
    None,
    /// Invert the colors inside of the annotation rect.
    Invert,
    /// Invert the colors on the annotation border.
    Outline,
    /// Make the annotation rect's area appear depressed.
    Push,
}

impl HighlightEffect {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::None => Name(b"N"),
            Self::Invert => Name(b"I"),
            Self::Outline => Name(b"O"),
            Self::Push => Name(b"P"),
        }
    }
}

/// Writer for a _file specification dictionary_.
///
/// This struct is created by [`Annotation::file`] and [`Action::file`].
pub struct FileSpec<'a> {
    dict: Dict<&'a mut PdfWriter>,
}

impl<'a> FileSpec<'a> {
    pub(crate) fn new(obj: Obj<&'a mut PdfWriter>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Filespec"));
        Self { dict }
    }

    /// Write the `/FS` attribute to set the file system this entry relates to.
    /// If you set the `system` argument to `Name(b"URL")`, this becomes an URL
    /// specification.
    pub fn file_system(&mut self, system: Name) -> &mut Self {
        self.pair(Name(b"FS"), system);
        self
    }

    /// Write the `/F` attribute to set the file path. Directories are indicated
    /// by `/`, independent of the platform.
    pub fn file(&mut self, path: Str) -> &mut Self {
        self.pair(Name(b"F"), path);
        self
    }

    /// Write the `/UF` attribute to set a Unicode-compatible path. Directories
    /// are indicated by `/`, independent of the platform. (1.7+)
    pub fn unic_file(&mut self, path: TextStr) -> &mut Self {
        self.pair(Name(b"UF"), path);
        self
    }

    /// Write the `/V` attribute to indicate whether to cache the file.
    pub fn volatile(&mut self, no_cache: bool) -> &mut Self {
        self.pair(Name(b"V"), no_cache);
        self
    }

    /// Write the `/Desc` attribute to set a file description. (1.6+)
    pub fn description(&mut self, desc: TextStr) -> &mut Self {
        self.pair(Name(b"Desc"), desc);
        self
    }
}

deref!('a, FileSpec<'a> => Dict<&'a mut PdfWriter>, dict);

/// Writer for an _border style dictionary_.
///
/// This struct is created by [`Annotation::border_style`].
pub struct BorderStyle<'a> {
    dict: Dict<&'a mut PdfWriter>,
}

impl<'a> BorderStyle<'a> {
    pub(crate) fn new(obj: Obj<&'a mut PdfWriter>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Border"));
        Self { dict }
    }

    /// Write the `/W` attribute. This is the width of the border in points.
    pub fn width(&mut self, points: f32) -> &mut Self {
        self.pair(Name(b"W"), points);
        self
    }

    /// Write the `/S` attribute.
    pub fn style(&mut self, style: BorderType) -> &mut Self {
        self.pair(Name(b"S"), style.to_name());
        self
    }

    /// Write the `/D` attribute to set the repeating lengths of dashes and gaps
    /// inbetween.
    pub fn dashes(&mut self, dash_pattern: impl IntoIterator<Item = f32>) -> &mut Self {
        self.key(Name(b"D")).array().typed().items(dash_pattern);
        self
    }
}

deref!('a, BorderStyle<'a> => Dict<&'a mut PdfWriter>, dict);

/// The kind of line to draw on the border.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BorderType {
    /// A solid line.
    Solid,
    /// A dashed line, dash pattern may be specified further elsewhere.
    Dashed,
    /// A line with a 3D effect.
    Beveled,
    /// A line that makes the rectangle appear depressed.
    Inset,
    /// A single line at the bottom of the border rectangle.
    Underline,
}

impl BorderType {
    fn to_name(self) -> Name<'static> {
        match self {
            Self::Solid => Name(b"S"),
            Self::Dashed => Name(b"D"),
            Self::Beveled => Name(b"B"),
            Self::Inset => Name(b"I"),
            Self::Underline => Name(b"U"),
        }
    }
}
