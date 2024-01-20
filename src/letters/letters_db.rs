use super::{Forms, LettersType};

pub const TATWEEL: char = '\u{0640}';
pub const ZWJ: char = '\u{200D}';

/// Arabic letters
pub const LETTERS_ARABIC: [LettersType; 78] = [
    // ARABIC LETTER HAMZA
    ('\u{0621}', Forms::new('\u{FE80}', '\0', '\0', '\0')),
    // ARABIC LETTER ALEF WITH MADDA ABOVE
    ('\u{0622}', Forms::new('\u{FE81}', '\0', '\0', '\u{FE82}')),
    // ARABIC LETTER ALEF WITH HAMZA ABOVE
    ('\u{0623}', Forms::new('\u{FE83}', '\0', '\0', '\u{FE84}')),
    // ARABIC LETTER WAW WITH HAMZA ABOVE
    ('\u{0624}', Forms::new('\u{FE85}', '\0', '\0', '\u{FE86}')),
    // ARABIC LETTER ALEF WITH HAMZA BELOW
    ('\u{0625}', Forms::new('\u{FE87}', '\0', '\0', '\u{FE88}')),
    // ARABIC LETTER YEH WITH HAMZA ABOVE
    (
        '\u{0626}',
        Forms::new('\u{FE89}', '\u{FE8B}', '\u{FE8C}', '\u{FE8A}'),
    ),
    // ARABIC LETTER ALEF
    ('\u{0627}', Forms::new('\u{FE8D}', '\0', '\0', '\u{FE8E}')),
    // ARABIC LETTER BEH
    (
        '\u{0628}',
        Forms::new('\u{FE8F}', '\u{FE91}', '\u{FE92}', '\u{FE90}'),
    ),
    // ARABIC LETTER TEH MARBUTA
    ('\u{0629}', Forms::new('\u{FE93}', '\0', '\0', '\u{FE94}')),
    // ARABIC LETTER TEH
    (
        '\u{062A}',
        Forms::new('\u{FE95}', '\u{FE97}', '\u{FE98}', '\u{FE96}'),
    ),
    // ARABIC LETTER THEH
    (
        '\u{062B}',
        Forms::new('\u{FE99}', '\u{FE9B}', '\u{FE9C}', '\u{FE9A}'),
    ),
    // ARABIC LETTER JEEM
    (
        '\u{062C}',
        Forms::new('\u{FE9D}', '\u{FE9F}', '\u{FEA0}', '\u{FE9E}'),
    ),
    // ARABIC LETTER HAH
    (
        '\u{062D}',
        Forms::new('\u{FEA1}', '\u{FEA3}', '\u{FEA4}', '\u{FEA2}'),
    ),
    // ARABIC LETTER KHAH
    (
        '\u{062E}',
        Forms::new('\u{FEA5}', '\u{FEA7}', '\u{FEA8}', '\u{FEA6}'),
    ),
    // ARABIC LETTER DAL
    ('\u{062F}', Forms::new('\u{FEA9}', '\0', '\0', '\u{FEAA}')),
    // ARABIC LETTER THAL
    ('\u{0630}', Forms::new('\u{FEAB}', '\0', '\0', '\u{FEAC}')),
    // ARABIC LETTER REH
    ('\u{0631}', Forms::new('\u{FEAD}', '\0', '\0', '\u{FEAE}')),
    // ARABIC LETTER ZAIN
    ('\u{0632}', Forms::new('\u{FEAF}', '\0', '\0', '\u{FEB0}')),
    // ARABIC LETTER SEEN
    (
        '\u{0633}',
        Forms::new('\u{FEB1}', '\u{FEB3}', '\u{FEB4}', '\u{FEB2}'),
    ),
    // ARABIC LETTER SHEEN
    (
        '\u{0634}',
        Forms::new('\u{FEB5}', '\u{FEB7}', '\u{FEB8}', '\u{FEB6}'),
    ),
    // ARABIC LETTER SAD
    (
        '\u{0635}',
        Forms::new('\u{FEB9}', '\u{FEBB}', '\u{FEBC}', '\u{FEBA}'),
    ),
    // ARABIC LETTER DAD
    (
        '\u{0636}',
        Forms::new('\u{FEBD}', '\u{FEBF}', '\u{FEC0}', '\u{FEBE}'),
    ),
    // ARABIC LETTER TAH
    (
        '\u{0637}',
        Forms::new('\u{FEC1}', '\u{FEC3}', '\u{FEC4}', '\u{FEC2}'),
    ),
    // ARABIC LETTER ZAH
    (
        '\u{0638}',
        Forms::new('\u{FEC5}', '\u{FEC7}', '\u{FEC8}', '\u{FEC6}'),
    ),
    // ARABIC LETTER AIN
    (
        '\u{0639}',
        Forms::new('\u{FEC9}', '\u{FECB}', '\u{FECC}', '\u{FECA}'),
    ),
    // ARABIC LETTER GHAIN
    (
        '\u{063A}',
        Forms::new('\u{FECD}', '\u{FECF}', '\u{FED0}', '\u{FECE}'),
    ),
    // ARABIC TATWEEL
    (
        '\u{0640}',
        Forms::new('\u{0640}', '\u{0640}', '\u{0640}', '\u{0640}'),
    ),
    // ARABIC LETTER FEH
    (
        '\u{0641}',
        Forms::new('\u{FED1}', '\u{FED3}', '\u{FED4}', '\u{FED2}'),
    ),
    // ARABIC LETTER QAF
    (
        '\u{0642}',
        Forms::new('\u{FED5}', '\u{FED7}', '\u{FED8}', '\u{FED6}'),
    ),
    // ARABIC LETTER KAF
    (
        '\u{0643}',
        Forms::new('\u{FED9}', '\u{FEDB}', '\u{FEDC}', '\u{FEDA}'),
    ),
    // ARABIC LETTER LAM
    (
        '\u{0644}',
        Forms::new('\u{FEDD}', '\u{FEDF}', '\u{FEE0}', '\u{FEDE}'),
    ),
    // ARABIC LETTER MEEM
    (
        '\u{0645}',
        Forms::new('\u{FEE1}', '\u{FEE3}', '\u{FEE4}', '\u{FEE2}'),
    ),
    // ARABIC LETTER NOON
    (
        '\u{0646}',
        Forms::new('\u{FEE5}', '\u{FEE7}', '\u{FEE8}', '\u{FEE6}'),
    ),
    // ARABIC LETTER HEH
    (
        '\u{0647}',
        Forms::new('\u{FEE9}', '\u{FEEB}', '\u{FEEC}', '\u{FEEA}'),
    ),
    // ARABIC LETTER WAW
    ('\u{0648}', Forms::new('\u{FEED}', '\0', '\0', '\u{FEEE}')),
    // ARABIC LETTER (UIGHUR KAZAKH KIRGHIZ)? ALEF MAKSURA
    (
        '\u{0649}',
        Forms::new('\u{FEEF}', '\u{FBE8}', '\u{FBE9}', '\u{FEF0}'),
    ),
    // ARABIC LETTER YEH
    (
        '\u{064A}',
        Forms::new('\u{FEF1}', '\u{FEF3}', '\u{FEF4}', '\u{FEF2}'),
    ),
    // ARABIC LETTER ALEF WASLA
    ('\u{0671}', Forms::new('\u{FB50}', '\0', '\0', '\u{FB51}')),
    // ARABIC LETTER U WITH HAMZA ABOVE
    ('\u{0677}', Forms::new('\u{FBDD}', '\0', '\0', '\0')),
    // ARABIC LETTER TTEH
    (
        '\u{0679}',
        Forms::new('\u{FB66}', '\u{FB68}', '\u{FB69}', '\u{FB67}'),
    ),
    // ARABIC LETTER TTEHEH
    (
        '\u{067A}',
        Forms::new('\u{FB5E}', '\u{FB60}', '\u{FB61}', '\u{FB5F}'),
    ),
    // ARABIC LETTER BEEH
    (
        '\u{067B}',
        Forms::new('\u{FB52}', '\u{FB54}', '\u{FB55}', '\u{FB53}'),
    ),
    // ARABIC LETTER PEH
    (
        '\u{067E}',
        Forms::new('\u{FB56}', '\u{FB58}', '\u{FB59}', '\u{FB57}'),
    ),
    // ARABIC LETTER TEHEH
    (
        '\u{067F}',
        Forms::new('\u{FB62}', '\u{FB64}', '\u{FB65}', '\u{FB63}'),
    ),
    // ARABIC LETTER BEHEH
    (
        '\u{0680}',
        Forms::new('\u{FB5A}', '\u{FB5C}', '\u{FB5D}', '\u{FB5B}'),
    ),
    // ARABIC LETTER NYEH
    (
        '\u{0683}',
        Forms::new('\u{FB76}', '\u{FB78}', '\u{FB79}', '\u{FB77}'),
    ),
    // ARABIC LETTER DYEH
    (
        '\u{0684}',
        Forms::new('\u{FB72}', '\u{FB74}', '\u{FB75}', '\u{FB73}'),
    ),
    // ARABIC LETTER TCHEH
    (
        '\u{0686}',
        Forms::new('\u{FB7A}', '\u{FB7C}', '\u{FB7D}', '\u{FB7B}'),
    ),
    // ARABIC LETTER TCHEHEH
    (
        '\u{0687}',
        Forms::new('\u{FB7E}', '\u{FB80}', '\u{FB81}', '\u{FB7F}'),
    ),
    // ARABIC LETTER DDAL
    ('\u{0688}', Forms::new('\u{FB88}', '\0', '\0', '\u{FB89}')),
    // ARABIC LETTER DAHAL
    ('\u{068C}', Forms::new('\u{FB84}', '\0', '\0', '\u{FB85}')),
    // ARABIC LETTER DDAHAL
    ('\u{068D}', Forms::new('\u{FB82}', '\0', '\0', '\u{FB83}')),
    // ARABIC LETTER DUL
    ('\u{068E}', Forms::new('\u{FB86}', '\0', '\0', '\u{FB87}')),
    // ARABIC LETTER RREH
    ('\u{0691}', Forms::new('\u{FB8C}', '\0', '\0', '\u{FB8D}')),
    // ARABIC LETTER JEH
    ('\u{0698}', Forms::new('\u{FB8A}', '\0', '\0', '\u{FB8B}')),
    // ARABIC LETTER VEH
    (
        '\u{06A4}',
        Forms::new('\u{FB6A}', '\u{FB6C}', '\u{FB6D}', '\u{FB6B}'),
    ),
    // ARABIC LETTER PEHEH
    (
        '\u{06A6}',
        Forms::new('\u{FB6E}', '\u{FB70}', '\u{FB71}', '\u{FB6F}'),
    ),
    // ARABIC LETTER KEHEH
    (
        '\u{06A9}',
        Forms::new('\u{FB8E}', '\u{FB90}', '\u{FB91}', '\u{FB8F}'),
    ),
    // ARABIC LETTER NG
    (
        '\u{06AD}',
        Forms::new('\u{FBD3}', '\u{FBD5}', '\u{FBD6}', '\u{FBD4}'),
    ),
    // ARABIC LETTER GAF
    (
        '\u{06AF}',
        Forms::new('\u{FB92}', '\u{FB94}', '\u{FB95}', '\u{FB93}'),
    ),
    // ARABIC LETTER NGOEH
    (
        '\u{06B1}',
        Forms::new('\u{FB9A}', '\u{FB9C}', '\u{FB9D}', '\u{FB9B}'),
    ),
    // ARABIC LETTER GUEH
    (
        '\u{06B3}',
        Forms::new('\u{FB96}', '\u{FB98}', '\u{FB99}', '\u{FB97}'),
    ),
    // ARABIC LETTER NOON GHUNNA
    ('\u{06BA}', Forms::new('\u{FB9E}', '\0', '\0', '\u{FB9F}')),
    // ARABIC LETTER RNOON
    (
        '\u{06BB}',
        Forms::new('\u{FBA0}', '\u{FBA2}', '\u{FBA3}', '\u{FBA1}'),
    ),
    // ARABIC LETTER HEH DOACHASHMEE
    (
        '\u{06BE}',
        Forms::new('\u{FBAA}', '\u{FBAC}', '\u{FBAD}', '\u{FBAB}'),
    ),
    // ARABIC LETTER HEH WITH YEH ABOVE
    ('\u{06C0}', Forms::new('\u{FBA4}', '\0', '\0', '\u{FBA5}')),
    // ARABIC LETTER HEH GOAL
    (
        '\u{06C1}',
        Forms::new('\u{FBA6}', '\u{FBA8}', '\u{FBA9}', '\u{FBA7}'),
    ),
    // ARABIC LETTER KIRGHIZ OE
    ('\u{06C5}', Forms::new('\u{FBE0}', '\0', '\0', '\u{FBE1}')),
    // ARABIC LETTER OE
    ('\u{06C6}', Forms::new('\u{FBD9}', '\0', '\0', '\u{FBDA}')),
    // ARABIC LETTER U
    ('\u{06C7}', Forms::new('\u{FBD7}', '\0', '\0', '\u{FBD8}')),
    // ARABIC LETTER YU
    ('\u{06C8}', Forms::new('\u{FBDB}', '\0', '\0', '\u{FBDC}')),
    // ARABIC LETTER KIRGHIZ YU
    ('\u{06C9}', Forms::new('\u{FBE2}', '\0', '\0', '\u{FBE3}')),
    // ARABIC LETTER VE
    ('\u{06CB}', Forms::new('\u{FBDE}', '\0', '\0', '\u{FBDF}')),
    // ARABIC LETTER FARSI YEH
    (
        '\u{06CC}',
        Forms::new('\u{FBFC}', '\u{FBFE}', '\u{FBFF}', '\u{FBFD}'),
    ),
    // ARABIC LETTER E
    (
        '\u{06D0}',
        Forms::new('\u{FBE4}', '\u{FBE6}', '\u{FBE7}', '\u{FBE5}'),
    ),
    // ARABIC LETTER YEH BARREE
    ('\u{06D2}', Forms::new('\u{FBAE}', '\0', '\0', '\u{FBAF}')),
    // ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
    ('\u{06D3}', Forms::new('\u{FBB0}', '\0', '\0', '\u{FBB1}')),
    // ZWJ
    (
        '\u{200D}',
        Forms::new('\u{200D}', '\u{200D}', '\u{200D}', '\u{200D}'),
    ),
];

/// Arabic letters version 2
pub const LETTERS_ARABIC_V2: [LettersType; 80] = [
    // ARABIC LETTER HAMZA
    ('\u{0621}', Forms::new('\u{FE80}', '\0', '\0', '\0')),
    // ARABIC LETTER ALEF WITH MADDA ABOVE
    ('\u{0622}', Forms::new('\u{0622}', '\0', '\0', '\u{FE82}')),
    // ARABIC LETTER ALEF WITH HAMZA ABOVE
    ('\u{0623}', Forms::new('\u{0623}', '\0', '\0', '\u{FE84}')),
    // ARABIC LETTER WAW WITH HAMZA ABOVE
    ('\u{0624}', Forms::new('\u{0624}', '\0', '\0', '\u{FE86}')),
    // ARABIC LETTER ALEF WITH HAMZA BELOW
    ('\u{0625}', Forms::new('\u{0625}', '\0', '\0', '\u{FE88}')),
    // ARABIC LETTER YEH WITH HAMZA ABOVE
    (
        '\u{0626}',
        Forms::new('\u{0626}', '\u{FE8B}', '\u{FE8C}', '\u{FE8A}'),
    ),
    // ARABIC LETTER ALEF
    ('\u{0627}', Forms::new('\u{0627}', '\0', '\0', '\u{FE8E}')),
    // ARABIC LETTER BEH
    (
        '\u{0628}',
        Forms::new('\u{0628}', '\u{FE91}', '\u{FE92}', '\u{FE90}'),
    ),
    // ARABIC LETTER TEH MARBUTA
    ('\u{0629}', Forms::new('\u{0629}', '\0', '\0', '\u{FE94}')),
    // ARABIC LETTER TEH
    (
        '\u{062A}',
        Forms::new('\u{062A}', '\u{FE97}', '\u{FE98}', '\u{FE96}'),
    ),
    // ARABIC LETTER THEH
    (
        '\u{062B}',
        Forms::new('\u{062B}', '\u{FE9B}', '\u{FE9C}', '\u{FE9A}'),
    ),
    // ARABIC LETTER JEEM
    (
        '\u{062C}',
        Forms::new('\u{062C}', '\u{FE9F}', '\u{FEA0}', '\u{FE9E}'),
    ),
    // ARABIC LETTER HAH
    (
        '\u{062D}',
        Forms::new('\u{FEA1}', '\u{FEA3}', '\u{FEA4}', '\u{FEA2}'),
    ),
    // ARABIC LETTER KHAH
    (
        '\u{062E}',
        Forms::new('\u{062E}', '\u{FEA7}', '\u{FEA8}', '\u{FEA6}'),
    ),
    // ARABIC LETTER DAL
    ('\u{062F}', Forms::new('\u{062F}', '\0', '\0', '\u{FEAA}')),
    // ARABIC LETTER THAL
    ('\u{0630}', Forms::new('\u{0630}', '\0', '\0', '\u{FEAC}')),
    // ARABIC LETTER REH
    ('\u{0631}', Forms::new('\u{0631}', '\0', '\0', '\u{FEAE}')),
    // ARABIC LETTER ZAIN
    ('\u{0632}', Forms::new('\u{0632}', '\0', '\0', '\u{FEB0}')),
    // ARABIC LETTER SEEN
    (
        '\u{0633}',
        Forms::new('\u{0633}', '\u{FEB3}', '\u{FEB4}', '\u{FEB2}'),
    ),
    // ARABIC LETTER SHEEN
    (
        '\u{0634}',
        Forms::new('\u{0634}', '\u{FEB7}', '\u{FEB8}', '\u{FEB6}'),
    ),
    // ARABIC LETTER SAD
    (
        '\u{0635}',
        Forms::new('\u{0635}', '\u{FEBB}', '\u{FEBC}', '\u{FEBA}'),
    ),
    // ARABIC LETTER DAD
    (
        '\u{0636}',
        Forms::new('\u{0636}', '\u{FEBF}', '\u{FEC0}', '\u{FEBE}'),
    ),
    // ARABIC LETTER TAH
    (
        '\u{0637}',
        Forms::new('\u{0637}', '\u{FEC3}', '\u{FEC4}', '\u{FEC2}'),
    ),
    // ARABIC LETTER ZAH
    (
        '\u{0638}',
        Forms::new('\u{0638}', '\u{FEC7}', '\u{FEC8}', '\u{FEC6}'),
    ),
    // ARABIC LETTER AIN
    (
        '\u{0639}',
        Forms::new('\u{0639}', '\u{FECB}', '\u{FECC}', '\u{FECA}'),
    ),
    // ARABIC LETTER GHAIN
    (
        '\u{063A}',
        Forms::new('\u{063A}', '\u{FECF}', '\u{FED0}', '\u{FECE}'),
    ),
    // ARABIC TATWEEL
    (
        '\u{0640}',
        Forms::new('\u{0640}', '\u{0640}', '\u{0640}', '\u{0640}'),
    ),
    // ARABIC LETTER FEH
    (
        '\u{0641}',
        Forms::new('\u{0641}', '\u{FED3}', '\u{FED4}', '\u{FED2}'),
    ),
    // ARABIC LETTER QAF
    (
        '\u{0642}',
        Forms::new('\u{0642}', '\u{FED7}', '\u{FED8}', '\u{FED6}'),
    ),
    // ARABIC LETTER KAF
    (
        '\u{0643}',
        Forms::new('\u{0643}', '\u{FEDB}', '\u{FEDC}', '\u{FEDA}'),
    ),
    // ARABIC LETTER LAM
    (
        '\u{0644}',
        Forms::new('\u{0644}', '\u{FEDF}', '\u{FEE0}', '\u{FEDE}'),
    ),
    // ARABIC LETTER MEEM
    (
        '\u{0645}',
        Forms::new('\u{0645}', '\u{FEE3}', '\u{FEE4}', '\u{FEE2}'),
    ),
    // ARABIC LETTER NOON
    (
        '\u{0646}',
        Forms::new('\u{0646}', '\u{FEE7}', '\u{FEE8}', '\u{FEE6}'),
    ),
    // ARABIC LETTER HEH
    (
        '\u{0647}',
        Forms::new('\u{0647}', '\u{FEEB}', '\u{FEEC}', '\u{FEEA}'),
    ),
    // ARABIC LETTER WAW
    ('\u{0648}', Forms::new('\u{0648}', '\0', '\0', '\u{FEEE}')),
    // ARABIC LETTER (UIGHUR KAZAKH KIRGHIZ)? ALEF MAKSURA
    (
        '\u{0649}',
        Forms::new('\u{0649}', '\u{FBE8}', '\u{FBE9}', '\u{FEF0}'),
    ),
    // ARABIC LETTER YEH
    (
        '\u{064A}',
        Forms::new('\u{064A}', '\u{FEF3}', '\u{FEF4}', '\u{FEF2}'),
    ),
    // ARABIC LETTER ALEF WASLA
    ('\u{0671}', Forms::new('\u{0671}', '\0', '\0', '\u{FB51}')),
    // ARABIC LETTER U WITH HAMZA ABOVE
    ('\u{0677}', Forms::new('\u{0677}', '\0', '\0', '\0')),
    // ARABIC LETTER TTEH
    (
        '\u{0679}',
        Forms::new('\u{0679}', '\u{FB68}', '\u{FB69}', '\u{FB67}'),
    ),
    // ARABIC LETTER TTEHEH
    (
        '\u{067A}',
        Forms::new('\u{067A}', '\u{FB60}', '\u{FB61}', '\u{FB5F}'),
    ),
    // ARABIC LETTER BEEH
    (
        '\u{067B}',
        Forms::new('\u{067B}', '\u{FB54}', '\u{FB55}', '\u{FB53}'),
    ),
    // ARABIC LETTER PEH
    (
        '\u{067E}',
        Forms::new('\u{067E}', '\u{FB58}', '\u{FB59}', '\u{FB57}'),
    ),
    // ARABIC LETTER TEHEH
    (
        '\u{067F}',
        Forms::new('\u{067F}', '\u{FB64}', '\u{FB65}', '\u{FB63}'),
    ),
    // ARABIC LETTER BEHEH
    (
        '\u{0680}',
        Forms::new('\u{0680}', '\u{FB5C}', '\u{FB5D}', '\u{FB5B}'),
    ),
    // ARABIC LETTER NYEH
    (
        '\u{0683}',
        Forms::new('\u{0683}', '\u{FB78}', '\u{FB79}', '\u{FB77}'),
    ),
    // ARABIC LETTER DYEH
    (
        '\u{0684}',
        Forms::new('\u{0684}', '\u{FB74}', '\u{FB75}', '\u{FB73}'),
    ),
    // ARABIC LETTER TCHEH
    (
        '\u{0686}',
        Forms::new('\u{0686}', '\u{FB7C}', '\u{FB7D}', '\u{FB7B}'),
    ),
    // ARABIC LETTER TCHEHEH
    (
        '\u{0687}',
        Forms::new('\u{0687}', '\u{FB80}', '\u{FB81}', '\u{FB7F}'),
    ),
    // ARABIC LETTER DDAL
    ('\u{0688}', Forms::new('\u{0688}', '\0', '\0', '\u{FB89}')),
    // ARABIC LETTER DAHAL
    ('\u{068C}', Forms::new('\u{068C}', '\0', '\0', '\u{FB85}')),
    // ARABIC LETTER DDAHAL
    ('\u{068D}', Forms::new('\u{068D}', '\0', '\0', '\u{FB83}')),
    // ARABIC LETTER DUL
    ('\u{068E}', Forms::new('\u{068E}', '\0', '\0', '\u{FB87}')),
    // ARABIC LETTER RREH
    ('\u{0691}', Forms::new('\u{0691}', '\0', '\0', '\u{FB8D}')),
    // ARABIC LETTER JEH
    ('\u{0698}', Forms::new('\u{0698}', '\0', '\0', '\u{FB8B}')),
    // ARABIC LETTER VEH
    (
        '\u{06A4}',
        Forms::new('\u{06A4}', '\u{FB6C}', '\u{FB6D}', '\u{FB6B}'),
    ),
    // ARABIC LETTER PEHEH
    (
        '\u{06A6}',
        Forms::new('\u{06A6}', '\u{FB70}', '\u{FB71}', '\u{FB6F}'),
    ),
    // ARABIC LETTER KEHEH
    (
        '\u{06A9}',
        Forms::new('\u{06A9}', '\u{FB90}', '\u{FB91}', '\u{FB8F}'),
    ),
    // ARABIC LETTER NG
    (
        '\u{06AD}',
        Forms::new('\u{06AD}', '\u{FBD5}', '\u{FBD6}', '\u{FBD4}'),
    ),
    // ARABIC LETTER GAF
    (
        '\u{06AF}',
        Forms::new('\u{06AF}', '\u{FB94}', '\u{FB95}', '\u{FB93}'),
    ),
    // ARABIC LETTER NGOEH
    (
        '\u{06B1}',
        Forms::new('\u{06B1}', '\u{FB9C}', '\u{FB9D}', '\u{FB9B}'),
    ),
    // ARABIC LETTER GUEH
    (
        '\u{06B3}',
        Forms::new('\u{06B3}', '\u{FB98}', '\u{FB99}', '\u{FB97}'),
    ),
    // ARABIC LETTER NOON GHUNNA
    ('\u{06BA}', Forms::new('\u{06BA}', '\0', '\0', '\u{FB9F}')),
    // ARABIC LETTER RNOON
    (
        '\u{06BB}',
        Forms::new('\u{06BB}', '\u{FBA2}', '\u{FBA3}', '\u{FBA1}'),
    ),
    // ARABIC LETTER HEH DOACHASHMEE
    (
        '\u{06BE}',
        Forms::new('\u{06BE}', '\u{FBAC}', '\u{FBAD}', '\u{FBAB}'),
    ),
    // ARABIC LETTER HEH WITH YEH ABOVE
    ('\u{06C0}', Forms::new('\u{06C0}', '\0', '\0', '\u{FBA5}')),
    // ARABIC LETTER HEH GOAL
    (
        '\u{06C1}',
        Forms::new('\u{06C1}', '\u{FBA8}', '\u{FBA9}', '\u{FBA7}'),
    ),
    // ARABIC LETTER KIRGHIZ OE
    ('\u{06C5}', Forms::new('\u{06C5}', '\0', '\0', '\u{FBE1}')),
    // ARABIC LETTER OE
    ('\u{06C6}', Forms::new('\u{06C6}', '\0', '\0', '\u{FBDA}')),
    // ARABIC LETTER U
    ('\u{06C7}', Forms::new('\u{06C7}', '\0', '\0', '\u{FBD8}')),
    // ARABIC LETTER YU
    ('\u{06C8}', Forms::new('\u{06C8}', '\0', '\0', '\u{FBDC}')),
    // ARABIC LETTER KIRGHIZ YU
    ('\u{06C9}', Forms::new('\u{06C9}', '\0', '\0', '\u{FBE3}')),
    // ARABIC LETTER VE
    ('\u{06CB}', Forms::new('\u{06CB}', '\0', '\0', '\u{FBDF}')),
    // ARABIC LETTER FARSI YEH
    (
        '\u{06CC}',
        Forms::new('\u{06CC}', '\u{FBFE}', '\u{FBFF}', '\u{FBFD}'),
    ),
    // ARABIC LETTER E
    (
        '\u{06D0}',
        Forms::new('\u{06D0}', '\u{FBE6}', '\u{FBE7}', '\u{FBE5}'),
    ),
    // ARABIC LETTER YEH BARREE
    ('\u{06D2}', Forms::new('\u{06D2}', '\0', '\0', '\u{FBAF}')),
    // ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
    ('\u{06D3}', Forms::new('\u{06D3}', '\0', '\0', '\u{FBB1}')),
    // Kurdish letter YEAH
    (
        '\u{06ce}',
        Forms::new('\u{E004}', '\u{E005}', '\u{E006}', '\u{E004}'),
    ),
    // Kurdish letter Hamza same as arabic Teh without the point
    ('\u{06d5}', Forms::new('\u{06d5}', '\0', '\0', '\u{E000}')),
    // ZWJ
    (
        '\u{200D}',
        Forms::new('\u{200D}', '\u{200D}', '\u{200D}', '\u{200D}'),
    ),
];

/// Kurdish letters
pub const LETTERS_KURDISH: [LettersType; 80] = [
    // ARABIC LETTER HAMZA
    ('\u{0621}', Forms::new('\u{FE80}', '\0', '\0', '\0')),
    // ARABIC LETTER ALEF WITH MADDA ABOVE
    ('\u{0622}', Forms::new('\u{0622}', '\0', '\0', '\u{FE82}')),
    // ARABIC LETTER ALEF WITH HAMZA ABOVE
    ('\u{0623}', Forms::new('\u{0623}', '\0', '\0', '\u{FE84}')),
    // ARABIC LETTER WAW WITH HAMZA ABOVE
    ('\u{0624}', Forms::new('\u{0624}', '\0', '\0', '\u{FE86}')),
    // ARABIC LETTER ALEF WITH HAMZA BELOW
    ('\u{0625}', Forms::new('\u{0625}', '\0', '\0', '\u{FE88}')),
    // ARABIC LETTER YEH WITH HAMZA ABOVE
    (
        '\u{0626}',
        Forms::new('\u{0626}', '\u{FE8B}', '\u{FE8C}', '\u{FE8A}'),
    ),
    // ARABIC LETTER ALEF
    ('\u{0627}', Forms::new('\u{0627}', '\0', '\0', '\u{FE8E}')),
    // ARABIC LETTER BEH
    (
        '\u{0628}',
        Forms::new('\u{0628}', '\u{FE91}', '\u{FE92}', '\u{FE90}'),
    ),
    // ARABIC LETTER TEH MARBUTA
    ('\u{0629}', Forms::new('\u{0629}', '\0', '\0', '\u{FE94}')),
    // ARABIC LETTER TEH
    (
        '\u{062A}',
        Forms::new('\u{062A}', '\u{FE97}', '\u{FE98}', '\u{FE96}'),
    ),
    // ARABIC LETTER THEH
    (
        '\u{062B}',
        Forms::new('\u{062B}', '\u{FE9B}', '\u{FE9C}', '\u{FE9A}'),
    ),
    // ARABIC LETTER JEEM
    (
        '\u{062C}',
        Forms::new('\u{062C}', '\u{FE9F}', '\u{FEA0}', '\u{FE9E}'),
    ),
    // ARABIC LETTER HAH
    (
        '\u{062D}',
        Forms::new('\u{FEA1}', '\u{FEA3}', '\u{FEA4}', '\u{FEA2}'),
    ),
    // ARABIC LETTER KHAH
    (
        '\u{062E}',
        Forms::new('\u{062E}', '\u{FEA7}', '\u{FEA8}', '\u{FEA6}'),
    ),
    // ARABIC LETTER DAL
    ('\u{062F}', Forms::new('\u{062F}', '\0', '\0', '\u{FEAA}')),
    // ARABIC LETTER THAL
    ('\u{0630}', Forms::new('\u{0630}', '\0', '\0', '\u{FEAC}')),
    // ARABIC LETTER REH
    ('\u{0631}', Forms::new('\u{0631}', '\0', '\0', '\u{FEAE}')),
    // ARABIC LETTER ZAIN
    ('\u{0632}', Forms::new('\u{0632}', '\0', '\0', '\u{FEB0}')),
    // ARABIC LETTER SEEN
    (
        '\u{0633}',
        Forms::new('\u{0633}', '\u{FEB3}', '\u{FEB4}', '\u{FEB2}'),
    ),
    // ARABIC LETTER SHEEN
    (
        '\u{0634}',
        Forms::new('\u{0634}', '\u{FEB7}', '\u{FEB8}', '\u{FEB6}'),
    ),
    // ARABIC LETTER SAD
    (
        '\u{0635}',
        Forms::new('\u{0635}', '\u{FEBB}', '\u{FEBC}', '\u{FEBA}'),
    ),
    // ARABIC LETTER DAD
    (
        '\u{0636}',
        Forms::new('\u{0636}', '\u{FEBF}', '\u{FEC0}', '\u{FEBE}'),
    ),
    // ARABIC LETTER TAH
    (
        '\u{0637}',
        Forms::new('\u{0637}', '\u{FEC3}', '\u{FEC4}', '\u{FEC2}'),
    ),
    // ARABIC LETTER ZAH
    (
        '\u{0638}',
        Forms::new('\u{0638}', '\u{FEC7}', '\u{FEC8}', '\u{FEC6}'),
    ),
    // ARABIC LETTER AIN
    (
        '\u{0639}',
        Forms::new('\u{0639}', '\u{FECB}', '\u{FECC}', '\u{FECA}'),
    ),
    // ARABIC LETTER GHAIN
    (
        '\u{063A}',
        Forms::new('\u{063A}', '\u{FECF}', '\u{FED0}', '\u{FECE}'),
    ),
    // ARABIC TATWEEL
    (
        '\u{0640}',
        Forms::new('\u{0640}', '\u{0640}', '\u{0640}', '\u{0640}'),
    ),
    // ARABIC LETTER FEH
    (
        '\u{0641}',
        Forms::new('\u{0641}', '\u{FED3}', '\u{FED4}', '\u{FED2}'),
    ),
    // ARABIC LETTER QAF
    (
        '\u{0642}',
        Forms::new('\u{0642}', '\u{FED7}', '\u{FED8}', '\u{FED6}'),
    ),
    // ARABIC LETTER KAF
    (
        '\u{0643}',
        Forms::new('\u{0643}', '\u{FEDB}', '\u{FEDC}', '\u{FEDA}'),
    ),
    // ARABIC LETTER LAM
    (
        '\u{0644}',
        Forms::new('\u{0644}', '\u{FEDF}', '\u{FEE0}', '\u{FEDE}'),
    ),
    // ARABIC LETTER MEEM
    (
        '\u{0645}',
        Forms::new('\u{0645}', '\u{FEE3}', '\u{FEE4}', '\u{FEE2}'),
    ),
    // ARABIC LETTER NOON
    (
        '\u{0646}',
        Forms::new('\u{0646}', '\u{FEE7}', '\u{FEE8}', '\u{FEE6}'),
    ),
    // ARABIC LETTER HEH
    (
        '\u{0647}',
        Forms::new('\u{FBAB}', '\u{FBAB}', '\u{FBAB}', '\u{FBAB}'),
    ),
    // ARABIC LETTER WAW
    ('\u{0648}', Forms::new('\u{0648}', '\0', '\0', '\u{FEEE}')),
    // ARABIC LETTER (UIGHUR KAZAKH KIRGHIZ)? ALEF MAKSURA
    (
        '\u{0649}',
        Forms::new('\u{0649}', '\u{FBE8}', '\u{FBE9}', '\u{FEF0}'),
    ),
    // ARABIC LETTER YEH
    (
        '\u{064A}',
        Forms::new('\u{064A}', '\u{FEF3}', '\u{FEF4}', '\u{FEF2}'),
    ),
    // ARABIC LETTER ALEF WASLA
    ('\u{0671}', Forms::new('\u{0671}', '\0', '\0', '\u{FB51}')),
    // ARABIC LETTER U WITH HAMZA ABOVE
    ('\u{0677}', Forms::new('\u{0677}', '\0', '\0', '\0')),
    // ARABIC LETTER TTEH
    (
        '\u{0679}',
        Forms::new('\u{0679}', '\u{FB68}', '\u{FB69}', '\u{FB67}'),
    ),
    // ARABIC LETTER TTEHEH
    (
        '\u{067A}',
        Forms::new('\u{067A}', '\u{FB60}', '\u{FB61}', '\u{FB5F}'),
    ),
    // ARABIC LETTER BEEH
    (
        '\u{067B}',
        Forms::new('\u{067B}', '\u{FB54}', '\u{FB55}', '\u{FB53}'),
    ),
    // ARABIC LETTER PEH
    (
        '\u{067E}',
        Forms::new('\u{067E}', '\u{FB58}', '\u{FB59}', '\u{FB57}'),
    ),
    // ARABIC LETTER TEHEH
    (
        '\u{067F}',
        Forms::new('\u{067F}', '\u{FB64}', '\u{FB65}', '\u{FB63}'),
    ),
    // ARABIC LETTER BEHEH
    (
        '\u{0680}',
        Forms::new('\u{0680}', '\u{FB5C}', '\u{FB5D}', '\u{FB5B}'),
    ),
    // ARABIC LETTER NYEH
    (
        '\u{0683}',
        Forms::new('\u{0683}', '\u{FB78}', '\u{FB79}', '\u{FB77}'),
    ),
    // ARABIC LETTER DYEH
    (
        '\u{0684}',
        Forms::new('\u{0684}', '\u{FB74}', '\u{FB75}', '\u{FB73}'),
    ),
    // ARABIC LETTER TCHEH
    (
        '\u{0686}',
        Forms::new('\u{0686}', '\u{FB7C}', '\u{FB7D}', '\u{FB7B}'),
    ),
    // ARABIC LETTER TCHEHEH
    (
        '\u{0687}',
        Forms::new('\u{0687}', '\u{FB80}', '\u{FB81}', '\u{FB7F}'),
    ),
    // ARABIC LETTER DDAL
    ('\u{0688}', Forms::new('\u{0688}', '\0', '\0', '\u{FB89}')),
    // ARABIC LETTER DAHAL
    ('\u{068C}', Forms::new('\u{068C}', '\0', '\0', '\u{FB85}')),
    // ARABIC LETTER DDAHAL
    ('\u{068D}', Forms::new('\u{068D}', '\0', '\0', '\u{FB83}')),
    // ARABIC LETTER DUL
    ('\u{068E}', Forms::new('\u{068E}', '\0', '\0', '\u{FB87}')),
    // ARABIC LETTER RREH
    ('\u{0691}', Forms::new('\u{0691}', '\0', '\0', '\u{FB8D}')),
    // ARABIC LETTER JEH
    ('\u{0698}', Forms::new('\u{0698}', '\0', '\0', '\u{FB8B}')),
    // ARABIC LETTER VEH
    (
        '\u{06A4}',
        Forms::new('\u{06A4}', '\u{FB6C}', '\u{FB6D}', '\u{FB6B}'),
    ),
    // ARABIC LETTER PEHEH
    (
        '\u{06A6}',
        Forms::new('\u{06A6}', '\u{FB70}', '\u{FB71}', '\u{FB6F}'),
    ),
    // ARABIC LETTER KEHEH
    (
        '\u{06A9}',
        Forms::new('\u{06A9}', '\u{FB90}', '\u{FB91}', '\u{FB8F}'),
    ),
    // ARABIC LETTER NG
    (
        '\u{06AD}',
        Forms::new('\u{06AD}', '\u{FBD5}', '\u{FBD6}', '\u{FBD4}'),
    ),
    // ARABIC LETTER GAF
    (
        '\u{06AF}',
        Forms::new('\u{06AF}', '\u{FB94}', '\u{FB95}', '\u{FB93}'),
    ),
    // ARABIC LETTER NGOEH
    (
        '\u{06B1}',
        Forms::new('\u{06B1}', '\u{FB9C}', '\u{FB9D}', '\u{FB9B}'),
    ),
    // ARABIC LETTER GUEH
    (
        '\u{06B3}',
        Forms::new('\u{06B3}', '\u{FB98}', '\u{FB99}', '\u{FB97}'),
    ),
    // ARABIC LETTER NOON GHUNNA
    ('\u{06BA}', Forms::new('\u{06BA}', '\0', '\0', '\u{FB9F}')),
    // ARABIC LETTER RNOON
    (
        '\u{06BB}',
        Forms::new('\u{06BB}', '\u{FBA2}', '\u{FBA3}', '\u{FBA1}'),
    ),
    // ARABIC LETTER HEH DOACHASHMEE
    (
        '\u{06BE}',
        Forms::new('\u{06BE}', '\u{FBAC}', '\u{FBAD}', '\u{FBAB}'),
    ),
    // ARABIC LETTER HEH WITH YEH ABOVE
    ('\u{06C0}', Forms::new('\u{06C0}', '\0', '\0', '\u{FBA5}')),
    // ARABIC LETTER HEH GOAL
    (
        '\u{06C1}',
        Forms::new('\u{06C1}', '\u{FBA8}', '\u{FBA9}', '\u{FBA7}'),
    ),
    // ARABIC LETTER KIRGHIZ OE
    ('\u{06C5}', Forms::new('\u{06C5}', '\0', '\0', '\u{FBE1}')),
    // ARABIC LETTER OE
    ('\u{06C6}', Forms::new('\u{06C6}', '\0', '\0', '\u{FBDA}')),
    // ARABIC LETTER U
    ('\u{06C7}', Forms::new('\u{06C7}', '\0', '\0', '\u{FBD8}')),
    // ARABIC LETTER YU
    ('\u{06C8}', Forms::new('\u{06C8}', '\0', '\0', '\u{FBDC}')),
    // ARABIC LETTER KIRGHIZ YU
    ('\u{06C9}', Forms::new('\u{06C9}', '\0', '\0', '\u{FBE3}')),
    // ARABIC LETTER VE
    ('\u{06CB}', Forms::new('\u{06CB}', '\0', '\0', '\u{FBDF}')),
    // ARABIC LETTER FARSI YEH
    (
        '\u{06CC}',
        Forms::new('\u{06CC}', '\u{FBFE}', '\u{FBFF}', '\u{FBFD}'),
    ),
    // ARABIC LETTER E
    (
        '\u{06D0}',
        Forms::new('\u{06D0}', '\u{FBE6}', '\u{FBE7}', '\u{FBE5}'),
    ),
    // ARABIC LETTER YEH BARREE
    ('\u{06D2}', Forms::new('\u{06D2}', '\0', '\0', '\u{FBAF}')),
    // ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
    ('\u{06D3}', Forms::new('\u{06D3}', '\0', '\0', '\u{FBB1}')),
    // Kurdish letter YEAH
    (
        '\u{06ce}',
        Forms::new('\u{E004}', '\u{E005}', '\u{E006}', '\u{E004}'),
    ),
    // Kurdish letter Hamza same as arabic Teh without the point
    ('\u{06d5}', Forms::new('\u{06d5}', '\0', '\0', '\u{E000}')),
    // ZWJ
    (
        '\u{200D}',
        Forms::new('\u{200D}', '\u{200D}', '\u{200D}', '\u{200D}'),
    ),
];
