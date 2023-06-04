
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
