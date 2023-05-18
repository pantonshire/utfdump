use std::fmt;

#[derive(Clone, Debug)]
pub struct CharData<'a> {
    name: &'a str,
    category: Category,
    combining_class: CombiningClass,
}

impl<'a> CharData<'a> {
    pub fn from_row(row: &'a str) -> Option<(u32, Self)> {
        let mut fields = [""; 15];
        for (i, field) in row.splitn(15, ';').enumerate() {
            fields[i] = field;
        }

        let codepoint = u32::from_str_radix(fields[0], 16).ok()?;
        let name = fields[1];
        let category = Category::from_abbr(fields[2])?;
        let ccc = CombiningClass(u8::from_str_radix(fields[3], 10).ok()?);

        Some((codepoint, Self::from_parts(name, category, ccc)))
    }

    pub fn from_parts(name: &'a str, category: Category, combining_class: CombiningClass) -> Self {
        Self { name, category, combining_class }
    }

    pub fn with_name<'b>(self, name: &'a str) -> CharData<'b>
    where
        'a: 'b,
    {
        Self { name, ..self }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn combining_class(&self) -> CombiningClass {
        self.combining_class
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
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

    pub fn byte_repr(self) -> u8 {
        self as u8
    }

    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "Lu" => Some(Self::Lu),
            "Ll" => Some(Self::Ll),
            "Lt" => Some(Self::Lt),
            "Mn" => Some(Self::Mn),
            "Mc" => Some(Self::Mc),
            "Me" => Some(Self::Me),
            "Nd" => Some(Self::Nd),
            "Nl" => Some(Self::Nl),
            "No" => Some(Self::No),
            "Zs" => Some(Self::Zs),
            "Zl" => Some(Self::Zl),
            "Zp" => Some(Self::Zp),
            "Cc" => Some(Self::Cc),
            "Cf" => Some(Self::Cf),
            "Cs" => Some(Self::Cs),
            "Co" => Some(Self::Co),
            "Cn" => Some(Self::Cn),
            "Lm" => Some(Self::Lm),
            "Lo" => Some(Self::Lo),
            "Pc" => Some(Self::Pc),
            "Pd" => Some(Self::Pd),
            "Ps" => Some(Self::Ps),
            "Pe" => Some(Self::Pe),
            "Pi" => Some(Self::Pi),
            "Pf" => Some(Self::Pf),
            "Po" => Some(Self::Po),
            "Sm" => Some(Self::Sm),
            "Sc" => Some(Self::Sc),
            "Sk" => Some(Self::Sk),
            "So" => Some(Self::So),
            _ => None,
        }
    }

    pub fn abbr(self) -> &'static str {
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

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbr())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl fmt::Display for CombiningClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "Ccc{}", self.0),
        }
    }
}

