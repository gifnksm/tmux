pub(crate) type Code = libc::c_ulong;

/// Special key codes.
pub(crate) mod code {
    use super::Code;

    //  Focus events.
    pub(crate) const FOCUS_IN: Code = 68719476736;
    pub(crate) const FOCUS_OUT: Code = 68719476737;

    // "Any" key, used if not found in key table.
    pub(crate) const ANY: Code = 68719476738;

    // Paste brackets.
    pub(crate) const PASTE_START: Code = 68719476739;
    pub(crate) const PASTE_END: Code = 68719476740;

    // Mouse keys.
    pub(crate) const MOUSE: Code = 68719476741; // unclassified mouse event
    pub(crate) const DRAGGING: Code = 68719476742; // dragging in progress
    pub(crate) const DOUBLECLICK: Code = 68719476743; // double click complete
    pub(crate) const MOUSEMOVE_PANE: Code = 68719476744;
    pub(crate) const MOUSEMOVE_STATUS: Code = 68719476745;
    pub(crate) const MOUSEMOVE_STATUS_LEFT: Code = 68719476746;
    pub(crate) const MOUSEMOVE_STATUS_RIGHT: Code = 68719476747;
    pub(crate) const MOUSEMOVE_STATUS_DEFAULT: Code = 68719476748;
    pub(crate) const MOUSEMOVE_BORDER: Code = 68719476749;
    pub(crate) const MOUSEDOWN1_PANE: Code = 68719476750;
    pub(crate) const MOUSEDOWN1_STATUS: Code = 68719476751;
    pub(crate) const MOUSEDOWN1_STATUS_LEFT: Code = 68719476752;
    pub(crate) const MOUSEDOWN1_STATUS_RIGHT: Code = 68719476753;
    pub(crate) const MOUSEDOWN1_STATUS_DEFAULT: Code = 68719476754;
    pub(crate) const MOUSEDOWN1_BORDER: Code = 68719476755;
    pub(crate) const MOUSEDOWN2_PANE: Code = 68719476756;
    pub(crate) const MOUSEDOWN2_STATUS: Code = 68719476757;
    pub(crate) const MOUSEDOWN2_STATUS_LEFT: Code = 68719476758;
    pub(crate) const MOUSEDOWN2_STATUS_RIGHT: Code = 68719476759;
    pub(crate) const MOUSEDOWN2_STATUS_DEFAULT: Code = 68719476760;
    pub(crate) const MOUSEDOWN2_BORDER: Code = 68719476761;
    pub(crate) const MOUSEDOWN3_PANE: Code = 68719476762;
    pub(crate) const MOUSEDOWN3_STATUS: Code = 68719476763;
    pub(crate) const MOUSEDOWN3_STATUS_LEFT: Code = 68719476764;
    pub(crate) const MOUSEDOWN3_STATUS_RIGHT: Code = 68719476765;
    pub(crate) const MOUSEDOWN3_STATUS_DEFAULT: Code = 68719476766;
    pub(crate) const MOUSEDOWN3_BORDER: Code = 68719476767;
    pub(crate) const MOUSEUP1_PANE: Code = 68719476768;
    pub(crate) const MOUSEUP1_STATUS: Code = 68719476769;
    pub(crate) const MOUSEUP1_STATUS_LEFT: Code = 68719476770;
    pub(crate) const MOUSEUP1_STATUS_RIGHT: Code = 68719476771;
    pub(crate) const MOUSEUP1_STATUS_DEFAULT: Code = 68719476772;
    pub(crate) const MOUSEUP1_BORDER: Code = 68719476773;
    pub(crate) const MOUSEUP2_PANE: Code = 68719476774;
    pub(crate) const MOUSEUP2_STATUS: Code = 68719476775;
    pub(crate) const MOUSEUP2_STATUS_LEFT: Code = 68719476776;
    pub(crate) const MOUSEUP2_STATUS_RIGHT: Code = 68719476777;
    pub(crate) const MOUSEUP2_STATUS_DEFAULT: Code = 68719476778;
    pub(crate) const MOUSEUP2_BORDER: Code = 68719476779;
    pub(crate) const MOUSEUP3_PANE: Code = 68719476780;
    pub(crate) const MOUSEUP3_STATUS: Code = 68719476781;
    pub(crate) const MOUSEUP3_STATUS_LEFT: Code = 68719476782;
    pub(crate) const MOUSEUP3_STATUS_RIGHT: Code = 68719476783;
    pub(crate) const MOUSEUP3_STATUS_DEFAULT: Code = 68719476784;
    pub(crate) const MOUSEUP3_BORDER: Code = 68719476785;
    pub(crate) const MOUSEDRAG1_PANE: Code = 68719476786;
    pub(crate) const MOUSEDRAG1_STATUS: Code = 68719476787;
    pub(crate) const MOUSEDRAG1_STATUS_LEFT: Code = 68719476788;
    pub(crate) const MOUSEDRAG1_STATUS_RIGHT: Code = 68719476789;
    pub(crate) const MOUSEDRAG1_STATUS_DEFAULT: Code = 68719476790;
    pub(crate) const MOUSEDRAG1_BORDER: Code = 68719476791;
    pub(crate) const MOUSEDRAG2_PANE: Code = 68719476792;
    pub(crate) const MOUSEDRAG2_STATUS: Code = 68719476793;
    pub(crate) const MOUSEDRAG2_STATUS_LEFT: Code = 68719476794;
    pub(crate) const MOUSEDRAG2_STATUS_RIGHT: Code = 68719476795;
    pub(crate) const MOUSEDRAG2_STATUS_DEFAULT: Code = 68719476796;
    pub(crate) const MOUSEDRAG2_BORDER: Code = 68719476797;
    pub(crate) const MOUSEDRAG3_PANE: Code = 68719476798;
    pub(crate) const MOUSEDRAG3_STATUS: Code = 68719476799;
    pub(crate) const MOUSEDRAG3_STATUS_LEFT: Code = 68719476800;
    pub(crate) const MOUSEDRAG3_STATUS_RIGHT: Code = 68719476801;
    pub(crate) const MOUSEDRAG3_STATUS_DEFAULT: Code = 68719476802;
    pub(crate) const MOUSEDRAG3_BORDER: Code = 68719476803;
    pub(crate) const MOUSEDRAGEND1_PANE: Code = 68719476804;
    pub(crate) const MOUSEDRAGEND1_STATUS: Code = 68719476805;
    pub(crate) const MOUSEDRAGEND1_STATUS_LEFT: Code = 68719476806;
    pub(crate) const MOUSEDRAGEND1_STATUS_RIGHT: Code = 68719476807;
    pub(crate) const MOUSEDRAGEND1_STATUS_DEFAULT: Code = 68719476808;
    pub(crate) const MOUSEDRAGEND1_BORDER: Code = 68719476809;
    pub(crate) const MOUSEDRAGEND2_PANE: Code = 68719476810;
    pub(crate) const MOUSEDRAGEND2_STATUS: Code = 68719476811;
    pub(crate) const MOUSEDRAGEND2_STATUS_LEFT: Code = 68719476812;
    pub(crate) const MOUSEDRAGEND2_STATUS_RIGHT: Code = 68719476813;
    pub(crate) const MOUSEDRAGEND2_STATUS_DEFAULT: Code = 68719476814;
    pub(crate) const MOUSEDRAGEND2_BORDER: Code = 68719476815;
    pub(crate) const MOUSEDRAGEND3_PANE: Code = 68719476816;
    pub(crate) const MOUSEDRAGEND3_STATUS: Code = 68719476817;
    pub(crate) const MOUSEDRAGEND3_STATUS_LEFT: Code = 68719476818;
    pub(crate) const MOUSEDRAGEND3_STATUS_RIGHT: Code = 68719476819;
    pub(crate) const MOUSEDRAGEND3_STATUS_DEFAULT: Code = 68719476820;
    pub(crate) const MOUSEDRAGEND3_BORDER: Code = 68719476821;
    pub(crate) const WHEELUP_PANE: Code = 68719476822;
    pub(crate) const WHEELUP_STATUS: Code = 68719476823;
    pub(crate) const WHEELUP_STATUS_LEFT: Code = 68719476824;
    pub(crate) const WHEELUP_STATUS_RIGHT: Code = 68719476825;
    pub(crate) const WHEELUP_STATUS_DEFAULT: Code = 68719476826;
    pub(crate) const WHEELUP_BORDER: Code = 68719476827;
    pub(crate) const WHEELDOWN_PANE: Code = 68719476828;
    pub(crate) const WHEELDOWN_STATUS: Code = 68719476829;
    pub(crate) const WHEELDOWN_STATUS_LEFT: Code = 68719476830;
    pub(crate) const WHEELDOWN_STATUS_RIGHT: Code = 68719476831;
    pub(crate) const WHEELDOWN_STATUS_DEFAULT: Code = 68719476832;
    pub(crate) const WHEELDOWN_BORDER: Code = 68719476833;
    pub(crate) const SECONDCLICK1_PANE: Code = 68719476834;
    pub(crate) const SECONDCLICK1_STATUS: Code = 68719476835;
    pub(crate) const SECONDCLICK1_STATUS_LEFT: Code = 68719476836;
    pub(crate) const SECONDCLICK1_STATUS_RIGHT: Code = 68719476837;
    pub(crate) const SECONDCLICK1_STATUS_DEFAULT: Code = 68719476838;
    pub(crate) const SECONDCLICK1_BORDER: Code = 68719476839;
    pub(crate) const SECONDCLICK2_PANE: Code = 68719476840;
    pub(crate) const SECONDCLICK2_STATUS: Code = 68719476841;
    pub(crate) const SECONDCLICK2_STATUS_LEFT: Code = 68719476842;
    pub(crate) const SECONDCLICK2_STATUS_RIGHT: Code = 68719476843;
    pub(crate) const SECONDCLICK2_STATUS_DEFAULT: Code = 68719476844;
    pub(crate) const SECONDCLICK2_BORDER: Code = 68719476845;
    pub(crate) const SECONDCLICK3_PANE: Code = 68719476846;
    pub(crate) const SECONDCLICK3_STATUS: Code = 68719476847;
    pub(crate) const SECONDCLICK3_STATUS_LEFT: Code = 68719476848;
    pub(crate) const SECONDCLICK3_STATUS_RIGHT: Code = 68719476849;
    pub(crate) const SECONDCLICK3_STATUS_DEFAULT: Code = 68719476850;
    pub(crate) const SECONDCLICK3_BORDER: Code = 68719476851;
    pub(crate) const DOUBLECLICK1_PANE: Code = 68719476852;
    pub(crate) const DOUBLECLICK1_STATUS: Code = 68719476853;
    pub(crate) const DOUBLECLICK1_STATUS_LEFT: Code = 68719476854;
    pub(crate) const DOUBLECLICK1_STATUS_RIGHT: Code = 68719476855;
    pub(crate) const DOUBLECLICK1_STATUS_DEFAULT: Code = 68719476856;
    pub(crate) const DOUBLECLICK1_BORDER: Code = 68719476857;
    pub(crate) const DOUBLECLICK2_PANE: Code = 68719476858;
    pub(crate) const DOUBLECLICK2_STATUS: Code = 68719476859;
    pub(crate) const DOUBLECLICK2_STATUS_LEFT: Code = 68719476860;
    pub(crate) const DOUBLECLICK2_STATUS_RIGHT: Code = 68719476861;
    pub(crate) const DOUBLECLICK2_STATUS_DEFAULT: Code = 68719476862;
    pub(crate) const DOUBLECLICK2_BORDER: Code = 68719476863;
    pub(crate) const DOUBLECLICK3_PANE: Code = 68719476864;
    pub(crate) const DOUBLECLICK3_STATUS: Code = 68719476865;
    pub(crate) const DOUBLECLICK3_STATUS_LEFT: Code = 68719476866;
    pub(crate) const DOUBLECLICK3_STATUS_RIGHT: Code = 68719476867;
    pub(crate) const DOUBLECLICK3_STATUS_DEFAULT: Code = 68719476868;
    pub(crate) const DOUBLECLICK3_BORDER: Code = 68719476869;
    pub(crate) const TRIPLECLICK1_PANE: Code = 68719476870;
    pub(crate) const TRIPLECLICK1_STATUS: Code = 68719476871;
    pub(crate) const TRIPLECLICK1_STATUS_LEFT: Code = 68719476872;
    pub(crate) const TRIPLECLICK1_STATUS_RIGHT: Code = 68719476873;
    pub(crate) const TRIPLECLICK1_STATUS_DEFAULT: Code = 68719476874;
    pub(crate) const TRIPLECLICK1_BORDER: Code = 68719476875;
    pub(crate) const TRIPLECLICK2_PANE: Code = 68719476876;
    pub(crate) const TRIPLECLICK2_STATUS: Code = 68719476877;
    pub(crate) const TRIPLECLICK2_STATUS_LEFT: Code = 68719476878;
    pub(crate) const TRIPLECLICK2_STATUS_RIGHT: Code = 68719476879;
    pub(crate) const TRIPLECLICK2_STATUS_DEFAULT: Code = 68719476880;
    pub(crate) const TRIPLECLICK2_BORDER: Code = 68719476881;
    pub(crate) const TRIPLECLICK3_PANE: Code = 68719476882;
    pub(crate) const TRIPLECLICK3_STATUS: Code = 68719476883;
    pub(crate) const TRIPLECLICK3_STATUS_LEFT: Code = 68719476884;
    pub(crate) const TRIPLECLICK3_STATUS_RIGHT: Code = 68719476885;
    pub(crate) const TRIPLECLICK3_STATUS_DEFAULT: Code = 68719476886;
    pub(crate) const TRIPLECLICK3_BORDER: Code = 68719476887;

    // Backspace key.
    pub(crate) const BSPACE: Code = 68719476888;

    // Function keys.
    pub(crate) const F1: Code = 68719476889;
    pub(crate) const F2: Code = 68719476890;
    pub(crate) const F3: Code = 68719476891;
    pub(crate) const F4: Code = 68719476892;
    pub(crate) const F5: Code = 68719476893;
    pub(crate) const F6: Code = 68719476894;
    pub(crate) const F7: Code = 68719476895;
    pub(crate) const F8: Code = 68719476896;
    pub(crate) const F9: Code = 68719476897;
    pub(crate) const F10: Code = 68719476898;
    pub(crate) const F11: Code = 68719476899;
    pub(crate) const F12: Code = 68719476900;
    pub(crate) const IC: Code = 68719476901;
    pub(crate) const DC: Code = 68719476902;
    pub(crate) const HOME: Code = 68719476903;
    pub(crate) const END: Code = 68719476904;
    pub(crate) const NPAGE: Code = 68719476905;
    pub(crate) const PPAGE: Code = 68719476906;
    pub(crate) const BTAB: Code = 68719476907;

    // Arrow keys.
    pub(crate) const UP: Code = 68719476908;
    pub(crate) const DOWN: Code = 68719476909;
    pub(crate) const LEFT: Code = 68719476910;
    pub(crate) const RIGHT: Code = 68719476911;

    // Numeric keypad.
    pub(crate) const KP_SLASH: Code = 68719476912;
    pub(crate) const KP_STAR: Code = 68719476913;
    pub(crate) const KP_MINUS: Code = 68719476914;
    pub(crate) const KP_SEVEN: Code = 68719476915;
    pub(crate) const KP_EIGHT: Code = 68719476916;
    pub(crate) const KP_NINE: Code = 68719476917;
    pub(crate) const KP_PLUS: Code = 68719476918;
    pub(crate) const KP_FOUR: Code = 68719476919;
    pub(crate) const KP_FIVE: Code = 68719476920;
    pub(crate) const KP_SIX: Code = 68719476921;
    pub(crate) const KP_ONE: Code = 68719476922;
    pub(crate) const KP_TWO: Code = 68719476923;
    pub(crate) const KP_THREE: Code = 68719476924;
    pub(crate) const KP_ENTER: Code = 68719476925;
    pub(crate) const KP_ZERO: Code = 68719476926;
    pub(crate) const KP_PERIOD: Code = 68719476927;
}
