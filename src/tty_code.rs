pub(crate) type Code = libc::c_uint;

/// Termcap codes.
pub(crate) mod code {
    use super::Code;

    pub(crate) const ACSC: Code = 0;
    pub(crate) const AM: Code = 1;
    pub(crate) const AX: Code = 2;
    pub(crate) const BCE: Code = 3;
    pub(crate) const BEL: Code = 4;
    pub(crate) const BLINK: Code = 5;
    pub(crate) const BOLD: Code = 6;
    pub(crate) const CIVIS: Code = 7;
    pub(crate) const CLEAR: Code = 8;
    pub(crate) const CLMG: Code = 9;
    pub(crate) const CMG: Code = 10;
    pub(crate) const CNORM: Code = 11;
    pub(crate) const COLORS: Code = 12;
    pub(crate) const CR: Code = 13;
    pub(crate) const CS: Code = 14;
    pub(crate) const CSR: Code = 15;
    pub(crate) const CUB: Code = 16;
    pub(crate) const CUB1: Code = 17;
    pub(crate) const CUD: Code = 18;
    pub(crate) const CUD1: Code = 19;
    pub(crate) const CUF: Code = 20;
    pub(crate) const CUF1: Code = 21;
    pub(crate) const CUP: Code = 22;
    pub(crate) const CUU: Code = 23;
    pub(crate) const CUU1: Code = 24;
    pub(crate) const CVVIS: Code = 25;
    pub(crate) const DCH: Code = 26;
    pub(crate) const DCH1: Code = 27;
    pub(crate) const DIM: Code = 28;
    pub(crate) const DL: Code = 29;
    pub(crate) const DL1: Code = 30;
    pub(crate) const DSBP: Code = 31;
    pub(crate) const DSEKS: Code = 32;
    pub(crate) const DSFCS: Code = 33;
    pub(crate) const DSMG: Code = 34;
    pub(crate) const E3: Code = 35;
    pub(crate) const ECH: Code = 36;
    pub(crate) const ED: Code = 37;
    pub(crate) const EL: Code = 38;
    pub(crate) const EL1: Code = 39;
    pub(crate) const ENACS: Code = 40;
    pub(crate) const ENBP: Code = 41;
    pub(crate) const ENEKS: Code = 42;
    pub(crate) const ENFCS: Code = 43;
    pub(crate) const ENMG: Code = 44;
    pub(crate) const FSL: Code = 45;
    pub(crate) const HOME: Code = 46;
    pub(crate) const HPA: Code = 47;
    pub(crate) const ICH: Code = 48;
    pub(crate) const ICH1: Code = 49;
    pub(crate) const IL: Code = 50;
    pub(crate) const IL1: Code = 51;
    pub(crate) const INDN: Code = 52;
    pub(crate) const INVIS: Code = 53;
    pub(crate) const KCBT: Code = 54;
    pub(crate) const KCUB1: Code = 55;
    pub(crate) const KCUD1: Code = 56;
    pub(crate) const KCUF1: Code = 57;
    pub(crate) const KCUU1: Code = 58;
    pub(crate) const KDC2: Code = 59;
    pub(crate) const KDC3: Code = 60;
    pub(crate) const KDC4: Code = 61;
    pub(crate) const KDC5: Code = 62;
    pub(crate) const KDC6: Code = 63;
    pub(crate) const KDC7: Code = 64;
    pub(crate) const KDCH1: Code = 65;
    pub(crate) const KDN2: Code = 66;
    pub(crate) const KDN3: Code = 67;
    pub(crate) const KDN4: Code = 68;
    pub(crate) const KDN5: Code = 69;
    pub(crate) const KDN6: Code = 70;
    pub(crate) const KDN7: Code = 71;
    pub(crate) const KEND: Code = 72;
    pub(crate) const KEND2: Code = 73;
    pub(crate) const KEND3: Code = 74;
    pub(crate) const KEND4: Code = 75;
    pub(crate) const KEND5: Code = 76;
    pub(crate) const KEND6: Code = 77;
    pub(crate) const KEND7: Code = 78;
    pub(crate) const KF1: Code = 79;
    pub(crate) const KF10: Code = 80;
    pub(crate) const KF11: Code = 81;
    pub(crate) const KF12: Code = 82;
    pub(crate) const KF13: Code = 83;
    pub(crate) const KF14: Code = 84;
    pub(crate) const KF15: Code = 85;
    pub(crate) const KF16: Code = 86;
    pub(crate) const KF17: Code = 87;
    pub(crate) const KF18: Code = 88;
    pub(crate) const KF19: Code = 89;
    pub(crate) const KF2: Code = 90;
    pub(crate) const KF20: Code = 91;
    pub(crate) const KF21: Code = 92;
    pub(crate) const KF22: Code = 93;
    pub(crate) const KF23: Code = 94;
    pub(crate) const KF24: Code = 95;
    pub(crate) const KF25: Code = 96;
    pub(crate) const KF26: Code = 97;
    pub(crate) const KF27: Code = 98;
    pub(crate) const KF28: Code = 99;
    pub(crate) const KF29: Code = 100;
    pub(crate) const KF3: Code = 101;
    pub(crate) const KF30: Code = 102;
    pub(crate) const KF31: Code = 103;
    pub(crate) const KF32: Code = 104;
    pub(crate) const KF33: Code = 105;
    pub(crate) const KF34: Code = 106;
    pub(crate) const KF35: Code = 107;
    pub(crate) const KF36: Code = 108;
    pub(crate) const KF37: Code = 109;
    pub(crate) const KF38: Code = 110;
    pub(crate) const KF39: Code = 111;
    pub(crate) const KF4: Code = 112;
    pub(crate) const KF40: Code = 113;
    pub(crate) const KF41: Code = 114;
    pub(crate) const KF42: Code = 115;
    pub(crate) const KF43: Code = 116;
    pub(crate) const KF44: Code = 117;
    pub(crate) const KF45: Code = 118;
    pub(crate) const KF46: Code = 119;
    pub(crate) const KF47: Code = 120;
    pub(crate) const KF48: Code = 121;
    pub(crate) const KF49: Code = 122;
    pub(crate) const KF5: Code = 123;
    pub(crate) const KF50: Code = 124;
    pub(crate) const KF51: Code = 125;
    pub(crate) const KF52: Code = 126;
    pub(crate) const KF53: Code = 127;
    pub(crate) const KF54: Code = 128;
    pub(crate) const KF55: Code = 129;
    pub(crate) const KF56: Code = 130;
    pub(crate) const KF57: Code = 131;
    pub(crate) const KF58: Code = 132;
    pub(crate) const KF59: Code = 133;
    pub(crate) const KF6: Code = 134;
    pub(crate) const KF60: Code = 135;
    pub(crate) const KF61: Code = 136;
    pub(crate) const KF62: Code = 137;
    pub(crate) const KF63: Code = 138;
    pub(crate) const KF7: Code = 139;
    pub(crate) const KF8: Code = 140;
    pub(crate) const KF9: Code = 141;
    pub(crate) const KHOM2: Code = 142;
    pub(crate) const KHOM3: Code = 143;
    pub(crate) const KHOM4: Code = 144;
    pub(crate) const KHOM5: Code = 145;
    pub(crate) const KHOM6: Code = 146;
    pub(crate) const KHOM7: Code = 147;
    pub(crate) const KHOME: Code = 148;
    pub(crate) const KIC2: Code = 149;
    pub(crate) const KIC3: Code = 150;
    pub(crate) const KIC4: Code = 151;
    pub(crate) const KIC5: Code = 152;
    pub(crate) const KIC6: Code = 153;
    pub(crate) const KIC7: Code = 154;
    pub(crate) const KICH1: Code = 155;
    pub(crate) const KIND: Code = 156;
    pub(crate) const KLFT2: Code = 157;
    pub(crate) const KLFT3: Code = 158;
    pub(crate) const KLFT4: Code = 159;
    pub(crate) const KLFT5: Code = 160;
    pub(crate) const KLFT6: Code = 161;
    pub(crate) const KLFT7: Code = 162;
    pub(crate) const KMOUS: Code = 163;
    pub(crate) const KNP: Code = 164;
    pub(crate) const KNXT2: Code = 165;
    pub(crate) const KNXT3: Code = 166;
    pub(crate) const KNXT4: Code = 167;
    pub(crate) const KNXT5: Code = 168;
    pub(crate) const KNXT6: Code = 169;
    pub(crate) const KNXT7: Code = 170;
    pub(crate) const KPP: Code = 171;
    pub(crate) const KPRV2: Code = 172;
    pub(crate) const KPRV3: Code = 173;
    pub(crate) const KPRV4: Code = 174;
    pub(crate) const KPRV5: Code = 175;
    pub(crate) const KPRV6: Code = 176;
    pub(crate) const KPRV7: Code = 177;
    pub(crate) const KRI: Code = 178;
    pub(crate) const KRIT2: Code = 179;
    pub(crate) const KRIT3: Code = 180;
    pub(crate) const KRIT4: Code = 181;
    pub(crate) const KRIT5: Code = 182;
    pub(crate) const KRIT6: Code = 183;
    pub(crate) const KRIT7: Code = 184;
    pub(crate) const KUP2: Code = 185;
    pub(crate) const KUP3: Code = 186;
    pub(crate) const KUP4: Code = 187;
    pub(crate) const KUP5: Code = 188;
    pub(crate) const KUP6: Code = 189;
    pub(crate) const KUP7: Code = 190;
    pub(crate) const MS: Code = 191;
    pub(crate) const OL: Code = 192;
    pub(crate) const OP: Code = 193;
    pub(crate) const REV: Code = 194;
    pub(crate) const RGB: Code = 195;
    pub(crate) const RI: Code = 196;
    pub(crate) const RIN: Code = 197;
    pub(crate) const RMACS: Code = 198;
    pub(crate) const RMCUP: Code = 199;
    pub(crate) const RMKX: Code = 200;
    pub(crate) const SE: Code = 201;
    pub(crate) const SETAB: Code = 202;
    pub(crate) const SETAF: Code = 203;
    pub(crate) const SETAL: Code = 204;
    pub(crate) const SETRGBB: Code = 205;
    pub(crate) const SETRGBF: Code = 206;
    pub(crate) const SETULC: Code = 207;
    pub(crate) const SGR0: Code = 208;
    pub(crate) const SITM: Code = 209;
    pub(crate) const SMACS: Code = 210;
    pub(crate) const SMCUP: Code = 211;
    pub(crate) const SMKX: Code = 212;
    pub(crate) const SMOL: Code = 213;
    pub(crate) const SMSO: Code = 214;
    pub(crate) const SMUL: Code = 215;
    pub(crate) const SMULX: Code = 216;
    pub(crate) const SMXX: Code = 217;
    pub(crate) const SS: Code = 218;
    pub(crate) const SYNC: Code = 219;
    pub(crate) const TC: Code = 220;
    pub(crate) const TSL: Code = 221;
    pub(crate) const U8: Code = 222;
    pub(crate) const VPA: Code = 223;
    pub(crate) const XT: Code = 224;
}
