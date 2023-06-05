
#[derive(Debug)]
pub struct CharData<'a> {
    pub(crate) codepoint: u32,
    pub(crate) name: &'a str,
    pub(crate) category: Category,
    pub(crate) combining: CombiningClass,
    pub(crate) bidi: BidiCategory,
    pub(crate) decomp: Option<DecompMapping<'a>>,
    pub(crate) decimal_digit: Option<u8>,
    pub(crate) digit: Option<u8>,
    // FIXME: replace with exact fraction type?
    pub(crate) numeric: Option<&'a str>,
    pub(crate) mirrored: bool,
    pub(crate) old_name: Option<&'a str>,
    pub(crate) comment: Option<&'a str>,
    pub(crate) uppercase: Option<&'a str>,
    pub(crate) lowercase: Option<&'a str>,
    pub(crate) titlecase: Option<&'a str>,
}

impl<'a> CharData<'a> {
    #[inline]
    #[must_use]
    pub fn codepoint(&self) -> u32 {
        self.codepoint
    }

    #[inline]
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.name
    }

    #[inline]
    #[must_use]
    pub fn category(&self) -> Category {
        self.category
    }

    pub fn combining_class(&self) -> CombiningClass {
        self.combining
    }

    #[inline]
    #[must_use]
    pub fn bidi_category(&self) -> BidiCategory {
        self.bidi
    }

    #[inline]
    #[must_use]
    pub fn decomp_mapping(&self) -> Option<DecompMapping<'a>> {
        self.decomp
    }

    #[inline]
    #[must_use]
    pub fn decimal_digit_value(&self) -> Option<u8> {
        self.decimal_digit
    }

    #[inline]
    #[must_use]
    pub fn digit_value(&self) -> Option<u8> {
        self.digit
    }

    #[inline]
    #[must_use]
    pub fn numeric_value(&self) -> Option<&'a str> {
        self.numeric
    }

    #[inline]
    #[must_use]
    pub fn mirrored(&self) -> bool {
        self.mirrored
    }

    #[inline]
    #[must_use]
    pub fn unicode_1_name(&self) -> Option<&'a str> {
        self.old_name
    }

    #[inline]
    #[must_use]
    pub fn comment(&self) -> Option<&'a str> {
        self.comment
    }

    #[inline]
    #[must_use]
    pub fn uppercase(&self) -> Option<&'a str> {
        self.uppercase
    }

    #[inline]
    #[must_use]
    pub fn lowercase(&self) -> Option<&'a str> {
        self.lowercase
    }

    #[inline]
    #[must_use]
    pub fn titlecase(&self) -> Option<&'a str> {
        self.titlecase
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CombiningClass(pub u8);

impl CombiningClass {
    pub fn name(self) -> Option<&'static str> {
        match self.0 {
            0 => Some("Not_Reordered"),
            1 => Some("Overlay"),
            6 => Some("Han_Reading"),
            7 => Some("Nukta"),
            8 => Some("Kana_Voicing"),
            9 => Some("Virama"),
            200 => Some("Attached_Below_Left"),
            202 => Some("Attached_Below"),
            214 => Some("Attached_Above"),
            216 => Some("Attached_Above_Right"),
            218 => Some("Below_Left"),
            220 => Some("Below"),
            222 => Some("Below_Right"),
            224 => Some("Left"),
            226 => Some("Right"),
            228 => Some("Above_Left"),
            230 => Some("Above"),
            232 => Some("Above_Right"),
            233 => Some("Double_Below"),
            234 => Some("Double_Above"),
            240 => Some("Iota_Subscript"),
            _ => None,
        }
    }

    pub fn is_combining(self) -> bool {
        self.0 != 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Category {
    Lu,
    Ll,
    Lt,
    Mn,
    Mc,
    Me,
    Nd,
    Nl,
    No,
    Zs,
    Zl,
    Zp,
    Cc,
    Cf,
    Cs,
    Co,
    Cn,
    Lm,
    Lo,
    Pc,
    Pd,
    Ps,
    Pe,
    Pi,
    Pf,
    Po,
    Sm,
    Sc,
    Sk,
    So,
}

impl Category {
    pub(crate) fn decode(encoded: u8) -> Option<Self> {
        match encoded {
            0 => Some(Self::Lu),
            1 => Some(Self::Ll),
            2 => Some(Self::Lt),
            3 => Some(Self::Mn),
            4 => Some(Self::Mc),
            5 => Some(Self::Me),
            6 => Some(Self::Nd),
            7 => Some(Self::Nl),
            8 => Some(Self::No),
            9 => Some(Self::Zs),
            10 => Some(Self::Zl),
            11 => Some(Self::Zp),
            12 => Some(Self::Cc),
            13 => Some(Self::Cf),
            14 => Some(Self::Cs),
            15 => Some(Self::Co),
            16 => Some(Self::Cn),
            17 => Some(Self::Lm),
            18 => Some(Self::Lo),
            19 => Some(Self::Pc),
            20 => Some(Self::Pd),
            21 => Some(Self::Ps),
            22 => Some(Self::Pe),
            23 => Some(Self::Pi),
            24 => Some(Self::Pf),
            25 => Some(Self::Po),
            26 => Some(Self::Sm),
            27 => Some(Self::Sc),
            28 => Some(Self::Sk),
            29 => Some(Self::So),
            _ => None,
        }
    }

    pub fn abbreviation(self) -> &'static str {
        match self {
            Self::Lu => "Lu",
            Self::Ll => "Ll",
            Self::Lt => "Lt",
            Self::Mn => "Mn",
            Self::Mc => "Mc",
            Self::Me => "Me",
            Self::Nd => "Nd",
            Self::Nl => "Nl",
            Self::No => "No",
            Self::Zs => "Zs",
            Self::Zl => "Zl",
            Self::Zp => "Zp",
            Self::Cc => "Cc",
            Self::Cf => "Cf",
            Self::Cs => "Cs",
            Self::Co => "Co",
            Self::Cn => "Cn",
            Self::Lm => "Lm",
            Self::Lo => "Lo",
            Self::Pc => "Pc",
            Self::Pd => "Pd",
            Self::Ps => "Ps",
            Self::Pe => "Pe",
            Self::Pi => "Pi",
            Self::Pf => "Pf",
            Self::Po => "Po",
            Self::Sm => "Sm",
            Self::Sc => "Sc",
            Self::Sk => "Sk",
            Self::So => "So",
        }
    }

    pub fn full_name(self) -> &'static str {
        match self {
            Self::Lu => "Letter, Uppercase",
            Self::Ll => "Letter, Lowercase",
            Self::Lt => "Letter, Titlecase",
            Self::Mn => "Mark, Non-Spacing",
            Self::Mc => "Mark, Spacing Combining",
            Self::Me => "Mark, Enclosing",
            Self::Nd => "Number, Decimal Digit",
            Self::Nl => "Number, Letter",
            Self::No => "Number, Other",
            Self::Zs => "Separator, Space",
            Self::Zl => "Separator, Line",
            Self::Zp => "Separator: Paragraph",
            Self::Cc => "Other, Control",
            Self::Cf => "Other, Format",
            Self::Cs => "Other, Surrogate",
            Self::Co => "Other, Private Use",
            Self::Cn => "Other, Not Assigned",
            Self::Lm => "Letter, Modifier",
            Self::Lo => "Letter, Other",
            Self::Pc => "Punctuation, Connector",
            Self::Pd => "Punctuation, Dash",
            Self::Ps => "Punctuation, Open",
            Self::Pe => "Punctuation, Close",
            Self::Pi => "Punctuation, Initial Quote",
            Self::Pf => "Punctuation, Final Quote",
            Self::Po => "Punctuation, Other",
            Self::Sm => "Symbol, Math",
            Self::Sc => "Symbol, Currency",
            Self::Sk => "Symbol, Modifier",
            Self::So => "Symbol, Other",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BidiCategory {
    L,
    R,
    Al,
    En,
    Es,
    Et,
    An,
    Cs,
    Nsm,
    Bn,
    B,
    S,
    Ws,
    On,
    Lre,
    Lro,
    Rle,
    Rlo,
    Pdf,
    Lri,
    Rli,
    Fsi,
    Pdi,
}

impl BidiCategory {
    pub(crate) fn decode(encoded: u8) -> Option<Self> {
        match encoded {
            0 => Some(Self::L),
            1 => Some(Self::R),
            2 => Some(Self::Al),
            3 => Some(Self::En),
            4 => Some(Self::Es),
            5 => Some(Self::Et),
            6 => Some(Self::An),
            7 => Some(Self::Cs),
            8 => Some(Self::Nsm),
            9 => Some(Self::Bn),
            10 => Some(Self::B),
            11 => Some(Self::S),
            12 => Some(Self::Ws),
            13 => Some(Self::On),
            14 => Some(Self::Lre),
            15 => Some(Self::Lro),
            16 => Some(Self::Rle),
            17 => Some(Self::Rlo),
            18 => Some(Self::Pdf),
            19 => Some(Self::Lri),
            20 => Some(Self::Rli),
            21 => Some(Self::Fsi),
            22 => Some(Self::Pdi),
            _ => None,
        }
    }

    pub fn abbreviation(self) -> &'static str {
        match self {
            Self::L => "L",
            Self::R => "R",
            Self::Al => "AL",
            Self::En => "EN",
            Self::Es => "ES",
            Self::Et => "ET",
            Self::An => "AN",
            Self::Cs => "CS",
            Self::Nsm => "NSM",
            Self::Bn => "BN",
            Self::B => "B",
            Self::S => "S",
            Self::Ws => "WS",
            Self::On => "ON",
            Self::Lre => "LRE",
            Self::Lro => "LRO",
            Self::Rle => "RLE",
            Self::Rlo => "RLO",
            Self::Pdf => "PDF",
            Self::Lri => "LRI",
            Self::Rli => "RLI",
            Self::Fsi => "FSI",
            Self::Pdi => "PDI",            
        }
    }

    pub fn full_name(self) -> &'static str {
        match self {
            Self::L => "Left_To_Right",
            Self::R => "Right_To_Left",
            Self::Al => "Arabic_Letter",
            Self::En => "European_Number",
            Self::Es => "European_Separator",
            Self::Et => "European_Terminator",
            Self::An => "Arabic_Number",
            Self::Cs => "Common_Separator",
            Self::Nsm => "Nonspacing_Mark",
            Self::Bn => "Boundary_Neutral",
            Self::B => "Paragraph_Separator",
            Self::S => "Segment_Separator",
            Self::Ws => "White_Space",
            Self::On => "Other_Neutral",
            Self::Lre => "Left_To_Right_Embedding",
            Self::Lro => "Left_To_Right_Override",
            Self::Rle => "Right_To_Left_Embedding",
            Self::Rlo => "Right_To_Left_Override",
            Self::Pdf => "Pop_Directional_Format",
            Self::Lri => "Left_To_Right_Isolate",
            Self::Rli => "Right_To_Left_Isolate",
            Self::Fsi => "First_Strong_Isolate",
            Self::Pdi => "Pop_Directional_Isolate",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DecompMapping<'a> {
    kind: Option<DecompKind>,
    value: &'a str,
}

impl<'a> DecompMapping<'a> {
    pub(crate) fn new(kind: Option<DecompKind>, value: &'a str) -> Self {
        Self { kind, value }
    }

    #[inline]
    #[must_use]
    pub fn kind(self) -> Option<DecompKind> {
        self.kind
    }

    #[inline]
    #[must_use]
    pub fn value(self) -> &'a str {
        self.value
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DecompKind {
    Nobreak,
    Compat,
    Super,
    Fraction,
    Sub,
    Font,
    Circle,
    Wide,
    Vertical,
    Square,
    Isolated,
    Final,
    Initial,
    Medial,
    Small,
    Narrow,
}

impl DecompKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Nobreak => "noBreak",
            Self::Compat => "compat",
            Self::Super => "super",
            Self::Fraction => "fraction",
            Self::Sub => "sub",
            Self::Font => "font",
            Self::Circle => "circle",
            Self::Wide => "wide",
            Self::Vertical => "vertical",
            Self::Square => "square",
            Self::Isolated => "isolated",
            Self::Final => "final",
            Self::Initial => "initial",
            Self::Medial => "medial",
            Self::Small => "small",
            Self::Narrow => "narrow",            
        }
    }
}

pub(crate) enum OptionalDecompKind {
    None,
    Anon,
    Named(DecompKind),
}

impl OptionalDecompKind {
    pub(crate) fn decode(encoded: u8) -> Option<Self> {
        match encoded {
            0 => Some(Self::None),
            1 => Some(Self::Anon),
            2 => Some(Self::Named(DecompKind::Nobreak)),
            3 => Some(Self::Named(DecompKind::Compat)),
            4 => Some(Self::Named(DecompKind::Super)),
            5 => Some(Self::Named(DecompKind::Fraction)),
            6 => Some(Self::Named(DecompKind::Sub)),
            7 => Some(Self::Named(DecompKind::Font)),
            8 => Some(Self::Named(DecompKind::Circle)),
            9 => Some(Self::Named(DecompKind::Wide)),
            10 => Some(Self::Named(DecompKind::Vertical)),
            11 => Some(Self::Named(DecompKind::Square)),
            12 => Some(Self::Named(DecompKind::Isolated)),
            13 => Some(Self::Named(DecompKind::Final)),
            14 => Some(Self::Named(DecompKind::Initial)),
            15 => Some(Self::Named(DecompKind::Medial)),
            16 => Some(Self::Named(DecompKind::Small)),
            17 => Some(Self::Named(DecompKind::Narrow)),
            _ => None,
        }
    }
}
