from enum import Enum
from struct import pack
from typing import Optional

# FIXME:
# Rather than creating many duplicated entries for "first / last" blocks, include a series of
# rules in the encoded file about what regions are duplicated / what ranges of codepoints need
# to be offset when using them to index into the table.

class StringTableIndex:
    def __init__(self, bs: bytes):
        if len(bs) != 3:
            raise ValueError('string table index must be 3 bytes')
        self.__bs = bs

    @classmethod
    def from_int(cls, index: int):
        # Ensure that the index is less than 0xffffff, because 0xffffff is the "invalid index"
        # sentinel value.
        if index >= 0xffffff:
            raise ValueError('non-nil string table indices must be less than 0xffffff')
        return cls(index.to_bytes(length=3, byteorder='little', signed=False))

    @classmethod
    def invalid(cls):
        return cls(b'\xff\xff\xff')

    def to_bytes(self):
        return self.__bs

class StringTable:
    def __init__(self):
        self.__buf = bytearray()
        self.__map = {}

    def push(self, s: str) -> StringTableIndex:
        if s in self.__map:
            return self.__map[s]
        
        insert_pos = len(self.__buf)
        s_bytes = s.encode(encoding='utf-8')
        s_len_bytes = len(s_bytes).to_bytes(length=1, byteorder='little', signed=False)
        self.__buf.extend(s_len_bytes)
        self.__buf.extend(s_bytes)
        self.__map[s] = insert_pos
        return StringTableIndex.from_int(insert_pos)

    def to_bytes(self) -> bytes:
        return bytes(self.__buf)

class Category(Enum):
    LU = 0
    LL = 1
    LT = 2
    MN = 3
    MC = 4
    ME = 5
    ND = 6
    NL = 7
    NO = 8
    ZS = 9
    ZL = 10
    ZP = 11
    CC = 12
    CF = 13
    CS = 14
    CO = 15
    CN = 16
    LM = 17
    LO = 18
    PC = 19
    PD = 20
    PS = 21
    PE = 22
    PI = 23
    PF = 24
    PO = 25
    SM = 26
    SC = 27
    SK = 28
    SO = 29

class Bidi(Enum):
    L = 0
    R = 1
    AL = 2
    EN = 3
    ES = 4
    ET = 5
    AN = 6
    CS = 7
    NSM = 8
    BN = 9
    B = 10
    S = 11
    WS = 12
    ON = 13
    LRE = 14
    LRO = 15
    RLE = 16
    RLO = 17
    PDF = 18
    LRI = 19
    RLI = 20
    FSI = 21
    PDI = 22

class DecompKind(Enum):
    NONE = 0
    ANONYMOUS = 1
    NOBREAK = 2
    COMPAT = 3
    SUPER = 4
    FRACTION = 5
    SUB = 6
    FONT = 7
    CIRCLE = 8
    WIDE = 9
    VERTICAL = 10
    SQUARE = 11
    ISOLATED = 12
    FINAL = 13
    INITIAL = 14
    MEDIAL = 15
    SMALL = 16
    NARROW = 17

class GroupKind(Enum):
    NO_VALUE = 0
    HAS_VALUE = 1

class Group:
    def __init__(self, kind: GroupKind, start: int, end: int):
        self.__kind = kind
        self.__start = start
        self.__end = end

    def __str__(self):
        return 'Group({}, {:x}, {:x})'.format(self.__kind, self.__start, self.__end)

    def kind(self) -> GroupKind:
        return self.__kind

    def start(self) -> int:
        return self.__start

    def end(self) -> int:
        return self.__end

def parse_codepoint_string(cp_str: str) -> str:
    return ''.join([chr(int(cp, 16)) for cp in cp_str.split()])

def encode_char_data(
    code: int,
    name: StringTableIndex,
    category: Category,
    combining: int,
    bidi: Bidi,
    decomp_kind: DecompKind,
    decomp: StringTableIndex,
    decimal_digit: Optional[int],
    digit: Optional[int],
    numeric_value: StringTableIndex,
    mirrored: bool,
    old_name: StringTableIndex,
    comment: StringTableIndex,
    uppercase: StringTableIndex,
    lowercase: StringTableIndex,
    titlecase: StringTableIndex
) -> bytes:
    # TODO: use a single "flags" byte to store:
    # - mirrored
    # - decomp_kind (5 bits needed)

    encoded = bytearray()

    # Pack the category, bidirectional category, decomposition kind and mirrored boolean into two
    # bytes.
    flags = 0
    flags |= int(category) & 0x1f
    flags |= (int(bidi) & 0x1f) << 5
    flags |= (int(decomp_kind) & 0x1f) << 10
    flags |= int(mirrored) << 15
    encoded.extend(flags.to_bytes(length=2, byteorder='little'))

    # 4 bits decimal digit, 4 bits digit (max is 9). Both need bit patterns for "none"

    assert len(encoded) == 28

    return bytes(encoded)

with open('unicode_data_latest.txt', 'r') as fd:
    input_data = fd.read()

char_data_table = bytearray()
string_table = StringTable()
# FIXME: calculate cumulative offset required for codepoints after each group
groups = []
in_group = False
prev_code = None

uniq_vals = {}

for row in input_data.splitlines():    
    is_group_start = False

    [
        cell_code,
        cell_name,
        cell_category,
        cell_combining,
        cell_bidi,
        cell_decomp,
        cell_decimal_digit,
        cell_digit,
        cell_numeric,
        cell_mirrored,
        cell_old_name,
        cell_comment,
        cell_uppercase,
        cell_lowercase,
        cell_titlecase
    ] = [cell.strip() for cell in row.split(';')]

    uniq_vals[cell_numeric] = True

    code = int(cell_code, 16)

    assert prev_code is None or prev_code < code

    # If the previous row was the start of a group, this row should be the end of the group. We now
    # know both the start and end codepoints of the group, so we can append it to the groups list.
    if in_group:
        assert cell_name.startswith('<') and cell_name.endswith(', Last>')
        groups.append(Group(GroupKind.HAS_VALUE, prev_code, code))

    # If there is a gap between the previous codepoint and this codepoint, add a "no value" group
    # to the list of groups to indicate the gap.
    elif prev_code is not None and code > prev_code + 1:
        groups.append(Group(GroupKind.NO_VALUE, prev_code + 1, code - 1))
    
    prev_code = code

    # If we are at the end of a group, continue the loop without creating a new character data
    # entry, since the entire group uses the entry created for the start of the group.
    if in_group:
        in_group = False
        continue

    if cell_name.startswith('<') and cell_name.endswith(', First>'):
        name = cell_name.removeprefix('<').removesuffix(', First>')
        in_group = True
    else:
        name = string_table.push(cell_name)
    
    category = Category[cell_category.upper()]
    combining = int(cell_combining)
    bidi = Bidi[cell_bidi.upper()]

    if cell_decomp:
        # If the decomposition string starts with an angle bracket, extract the decomposition kind
        # from between the angle brackets.
        if cell_decomp.startswith('<'):
            [
                decomp_kind_str,
                decomp_str
            ] = [s.strip() for s in cell_decomp.removeprefix('<').split('>', 1)]
            decomp_kind = DecompKind[decomp_kind_str.upper()]
        else:
            decomp_kind = DecompKind.ANONYMOUS
            decomp_str = cell_decomp
        # The decomposition is a series of ASCII-encoded codepoints separated by spaces, so split
        # the decomposition string by whitespace and convert each of the encoded codepoints to
        # actual characters.
        decomp = string_table.push(parse_codepoint_string(decomp_str))
    else:
        decomp_kind = DecompKind.NONE
        decomp = StringTableIndex.invalid()

    if cell_decimal_digit:
        decimal_digit = int(cell_decimal_digit, 10)
    else:
        decimal_digit = None

    if cell_digit:
        digit = int(cell_digit, 10)
    else:
        digit = None

    if cell_numeric:
        numeric_value = string_table.push(cell_numeric)
    else:
        numeric_value = StringTableIndex.invalid()

    mirrored = cell_mirrored == 'Y'

    if cell_old_name:
        old_name = string_table.push(cell_old_name)
    else:
        old_name = StringTableIndex.invalid()

    if cell_comment:
        comment = string_table.push(cell_comment)
    else:
        comment = StringTableIndex.invalid()

    if cell_uppercase:
        uppercase = string_table.push(parse_codepoint_string(cell_uppercase))
    else:
        uppercase = StringTableIndex.invalid()

    if cell_lowercase:
        lowercase = string_table.push(parse_codepoint_string(cell_lowercase))
    else:
        lowercase = StringTableIndex.invalid()

    if cell_titlecase:
        titlecase = string_table.push(parse_codepoint_string(cell_titlecase))
    else:
        titlecase = StringTableIndex.invalid()

    encoded = encode_char_data(
        code,
        name,
        category,
        combining,
        bidi,
        decomp_kind,
        decomp,
        decimal_digit,
        digit,
        numeric_value,
        mirrored,
        old_name,
        comment,
        uppercase,
        lowercase,
        titlecase
    )

print(len(string_table.to_bytes()))

# for k in uniq_vals.keys():
#     print(k)

# for group in groups:
#     print(group)
