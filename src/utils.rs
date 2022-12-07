use anyhow::{anyhow, Result};
use evdev::Key;

pub fn get_key_from_str(s: &str) -> Result<Key> {
    match s {
        "KEY_RESERVED" => Ok(Key::KEY_RESERVED),
        "KEY_ESC" => Ok(Key::KEY_ESC),
        "KEY_1" => Ok(Key::KEY_1),
        "KEY_2" => Ok(Key::KEY_2),
        "KEY_3" => Ok(Key::KEY_3),
        "KEY_4" => Ok(Key::KEY_4),
        "KEY_5" => Ok(Key::KEY_5),
        "KEY_6" => Ok(Key::KEY_6),
        "KEY_7" => Ok(Key::KEY_7),
        "KEY_8" => Ok(Key::KEY_8),
        "KEY_9" => Ok(Key::KEY_9),
        "KEY_0" => Ok(Key::KEY_0),
        "KEY_MINUS" => Ok(Key::KEY_MINUS),
        "KEY_EQUAL" => Ok(Key::KEY_EQUAL),
        "KEY_BACKSPACE" => Ok(Key::KEY_BACKSPACE),
        "KEY_TAB" => Ok(Key::KEY_TAB),
        "KEY_Q" => Ok(Key::KEY_Q),
        "KEY_W" => Ok(Key::KEY_W),
        "KEY_E" => Ok(Key::KEY_E),
        "KEY_R" => Ok(Key::KEY_R),
        "KEY_T" => Ok(Key::KEY_T),
        "KEY_Y" => Ok(Key::KEY_Y),
        "KEY_U" => Ok(Key::KEY_U),
        "KEY_I" => Ok(Key::KEY_I),
        "KEY_O" => Ok(Key::KEY_O),
        "KEY_P" => Ok(Key::KEY_P),
        "KEY_LEFTBRACE" => Ok(Key::KEY_LEFTBRACE),
        "KEY_RIGHTBRACE" => Ok(Key::KEY_RIGHTBRACE),
        "KEY_ENTER" => Ok(Key::KEY_ENTER),
        "KEY_LEFTCTRL" => Ok(Key::KEY_LEFTCTRL),
        "KEY_A" => Ok(Key::KEY_A),
        "KEY_S" => Ok(Key::KEY_S),
        "KEY_D" => Ok(Key::KEY_D),
        "KEY_F" => Ok(Key::KEY_F),
        "KEY_G" => Ok(Key::KEY_G),
        "KEY_H" => Ok(Key::KEY_H),
        "KEY_J" => Ok(Key::KEY_J),
        "KEY_K" => Ok(Key::KEY_K),
        "KEY_L" => Ok(Key::KEY_L),
        "KEY_SEMICOLON" => Ok(Key::KEY_SEMICOLON),
        "KEY_APOSTROPHE" => Ok(Key::KEY_APOSTROPHE),
        "KEY_GRAVE" => Ok(Key::KEY_GRAVE),
        "KEY_LEFTSHIFT" => Ok(Key::KEY_LEFTSHIFT),
        "KEY_BACKSLASH" => Ok(Key::KEY_BACKSLASH),
        "KEY_Z" => Ok(Key::KEY_Z),
        "KEY_X" => Ok(Key::KEY_X),
        "KEY_C" => Ok(Key::KEY_C),
        "KEY_V" => Ok(Key::KEY_V),
        "KEY_B" => Ok(Key::KEY_B),
        "KEY_N" => Ok(Key::KEY_N),
        "KEY_M" => Ok(Key::KEY_M),
        "KEY_COMMA" => Ok(Key::KEY_COMMA),
        "KEY_DOT" => Ok(Key::KEY_DOT),
        "KEY_SLASH" => Ok(Key::KEY_SLASH),
        "KEY_RIGHTSHIFT" => Ok(Key::KEY_RIGHTSHIFT),
        "KEY_KPASTERISK" => Ok(Key::KEY_KPASTERISK),
        "KEY_LEFTALT" => Ok(Key::KEY_LEFTALT),
        "KEY_SPACE" => Ok(Key::KEY_SPACE),
        "KEY_CAPSLOCK" => Ok(Key::KEY_CAPSLOCK),
        "KEY_F1" => Ok(Key::KEY_F1),
        "KEY_F2" => Ok(Key::KEY_F2),
        "KEY_F3" => Ok(Key::KEY_F3),
        "KEY_F4" => Ok(Key::KEY_F4),
        "KEY_F5" => Ok(Key::KEY_F5),
        "KEY_F6" => Ok(Key::KEY_F6),
        "KEY_F7" => Ok(Key::KEY_F7),
        "KEY_F8" => Ok(Key::KEY_F8),
        "KEY_F9" => Ok(Key::KEY_F9),
        "KEY_F10" => Ok(Key::KEY_F10),
        "KEY_NUMLOCK" => Ok(Key::KEY_NUMLOCK),
        "KEY_SCROLLLOCK" => Ok(Key::KEY_SCROLLLOCK),
        "KEY_KP7" => Ok(Key::KEY_KP7),
        "KEY_KP8" => Ok(Key::KEY_KP8),
        "KEY_KP9" => Ok(Key::KEY_KP9),
        "KEY_KPMINUS" => Ok(Key::KEY_KPMINUS),
        "KEY_KP4" => Ok(Key::KEY_KP4),
        "KEY_KP5" => Ok(Key::KEY_KP5),
        "KEY_KP6" => Ok(Key::KEY_KP6),
        "KEY_KPPLUS" => Ok(Key::KEY_KPPLUS),
        "KEY_KP1" => Ok(Key::KEY_KP1),
        "KEY_KP2" => Ok(Key::KEY_KP2),
        "KEY_KP3" => Ok(Key::KEY_KP3),
        "KEY_KP0" => Ok(Key::KEY_KP0),
        "KEY_KPDOT" => Ok(Key::KEY_KPDOT),
        "KEY_ZENKAKUHANKAKU" => Ok(Key::KEY_ZENKAKUHANKAKU),
        "KEY_102ND" => Ok(Key::KEY_102ND),
        "KEY_F11" => Ok(Key::KEY_F11),
        "KEY_F12" => Ok(Key::KEY_F12),
        "KEY_RO" => Ok(Key::KEY_RO),
        "KEY_KATAKANA" => Ok(Key::KEY_KATAKANA),
        "KEY_HIRAGANA" => Ok(Key::KEY_HIRAGANA),
        "KEY_HENKAN" => Ok(Key::KEY_HENKAN),
        "KEY_KATAKANAHIRAGANA" => Ok(Key::KEY_KATAKANAHIRAGANA),
        "KEY_MUHENKAN" => Ok(Key::KEY_MUHENKAN),
        "KEY_KPJPCOMMA" => Ok(Key::KEY_KPJPCOMMA),
        "KEY_KPENTER" => Ok(Key::KEY_KPENTER),
        "KEY_RIGHTCTRL" => Ok(Key::KEY_RIGHTCTRL),
        "KEY_KPSLASH" => Ok(Key::KEY_KPSLASH),
        "KEY_SYSRQ" => Ok(Key::KEY_SYSRQ),
        "KEY_RIGHTALT" => Ok(Key::KEY_RIGHTALT),
        "KEY_LINEFEED" => Ok(Key::KEY_LINEFEED),
        "KEY_HOME" => Ok(Key::KEY_HOME),
        "KEY_UP" => Ok(Key::KEY_UP),
        "KEY_PAGEUP" => Ok(Key::KEY_PAGEUP),
        "KEY_LEFT" => Ok(Key::KEY_LEFT),
        "KEY_RIGHT" => Ok(Key::KEY_RIGHT),
        "KEY_END" => Ok(Key::KEY_END),
        "KEY_DOWN" => Ok(Key::KEY_DOWN),
        "KEY_PAGEDOWN" => Ok(Key::KEY_PAGEDOWN),
        "KEY_INSERT" => Ok(Key::KEY_INSERT),
        "KEY_DELETE" => Ok(Key::KEY_DELETE),
        "KEY_MACRO" => Ok(Key::KEY_MACRO),
        "KEY_MUTE" => Ok(Key::KEY_MUTE),
        "KEY_VOLUMEDOWN" => Ok(Key::KEY_VOLUMEDOWN),
        "KEY_VOLUMEUP" => Ok(Key::KEY_VOLUMEUP),
        "KEY_POWER" => Ok(Key::KEY_POWER),
        "KEY_KPEQUAL" => Ok(Key::KEY_KPEQUAL),
        "KEY_KPPLUSMINUS" => Ok(Key::KEY_KPPLUSMINUS),
        "KEY_PAUSE" => Ok(Key::KEY_PAUSE),
        "KEY_SCALE" => Ok(Key::KEY_SCALE),
        "KEY_KPCOMMA" => Ok(Key::KEY_KPCOMMA),
        "KEY_HANGEUL" => Ok(Key::KEY_HANGEUL),
        "KEY_HANJA" => Ok(Key::KEY_HANJA),
        "KEY_YEN" => Ok(Key::KEY_YEN),
        "KEY_LEFTMETA" => Ok(Key::KEY_LEFTMETA),
        "KEY_RIGHTMETA" => Ok(Key::KEY_RIGHTMETA),
        "KEY_COMPOSE" => Ok(Key::KEY_COMPOSE),
        "KEY_STOP" => Ok(Key::KEY_STOP),
        "KEY_AGAIN" => Ok(Key::KEY_AGAIN),
        "KEY_PROPS" => Ok(Key::KEY_PROPS),
        "KEY_UNDO" => Ok(Key::KEY_UNDO),
        "KEY_FRONT" => Ok(Key::KEY_FRONT),
        "KEY_COPY" => Ok(Key::KEY_COPY),
        "KEY_OPEN" => Ok(Key::KEY_OPEN),
        "KEY_PASTE" => Ok(Key::KEY_PASTE),
        "KEY_FIND" => Ok(Key::KEY_FIND),
        "KEY_CUT" => Ok(Key::KEY_CUT),
        "KEY_HELP" => Ok(Key::KEY_HELP),
        "KEY_MENU" => Ok(Key::KEY_MENU),
        "KEY_CALC" => Ok(Key::KEY_CALC),
        "KEY_SETUP" => Ok(Key::KEY_SETUP),
        "KEY_SLEEP" => Ok(Key::KEY_SLEEP),
        "KEY_WAKEUP" => Ok(Key::KEY_WAKEUP),
        "KEY_FILE" => Ok(Key::KEY_FILE),
        "KEY_SENDFILE" => Ok(Key::KEY_SENDFILE),
        "KEY_DELETEFILE" => Ok(Key::KEY_DELETEFILE),
        "KEY_XFER" => Ok(Key::KEY_XFER),
        "KEY_PROG1" => Ok(Key::KEY_PROG1),
        "KEY_PROG2" => Ok(Key::KEY_PROG2),
        "KEY_WWW" => Ok(Key::KEY_WWW),
        "KEY_MSDOS" => Ok(Key::KEY_MSDOS),
        "KEY_COFFEE" => Ok(Key::KEY_COFFEE),
        "KEY_DIRECTION" => Ok(Key::KEY_DIRECTION),
        "KEY_ROTATE_DISPLAY" => Ok(Key::KEY_ROTATE_DISPLAY),
        "KEY_CYCLEWINDOWS" => Ok(Key::KEY_CYCLEWINDOWS),
        "KEY_MAIL" => Ok(Key::KEY_MAIL),
        "KEY_BOOKMARKS" => Ok(Key::KEY_BOOKMARKS),
        "KEY_COMPUTER" => Ok(Key::KEY_COMPUTER),
        "KEY_BACK" => Ok(Key::KEY_BACK),
        "KEY_FORWARD" => Ok(Key::KEY_FORWARD),
        "KEY_CLOSECD" => Ok(Key::KEY_CLOSECD),
        "KEY_EJECTCD" => Ok(Key::KEY_EJECTCD),
        "KEY_EJECTCLOSECD" => Ok(Key::KEY_EJECTCLOSECD),
        "KEY_NEXTSONG" => Ok(Key::KEY_NEXTSONG),
        "KEY_PLAYPAUSE" => Ok(Key::KEY_PLAYPAUSE),
        "KEY_PREVIOUSSONG" => Ok(Key::KEY_PREVIOUSSONG),
        "KEY_STOPCD" => Ok(Key::KEY_STOPCD),
        "KEY_RECORD" => Ok(Key::KEY_RECORD),
        "KEY_REWIND" => Ok(Key::KEY_REWIND),
        "KEY_PHONE" => Ok(Key::KEY_PHONE),
        "KEY_ISO" => Ok(Key::KEY_ISO),
        "KEY_CONFIG" => Ok(Key::KEY_CONFIG),
        "KEY_HOMEPAGE" => Ok(Key::KEY_HOMEPAGE),
        "KEY_REFRESH" => Ok(Key::KEY_REFRESH),
        "KEY_EXIT" => Ok(Key::KEY_EXIT),
        "KEY_MOVE" => Ok(Key::KEY_MOVE),
        "KEY_EDIT" => Ok(Key::KEY_EDIT),
        "KEY_SCROLLUP" => Ok(Key::KEY_SCROLLUP),
        "KEY_SCROLLDOWN" => Ok(Key::KEY_SCROLLDOWN),
        "KEY_KPLEFTPAREN" => Ok(Key::KEY_KPLEFTPAREN),
        "KEY_KPRIGHTPAREN" => Ok(Key::KEY_KPRIGHTPAREN),
        "KEY_NEW" => Ok(Key::KEY_NEW),
        "KEY_REDO" => Ok(Key::KEY_REDO),
        "KEY_F13" => Ok(Key::KEY_F13),
        "KEY_F14" => Ok(Key::KEY_F14),
        "KEY_F15" => Ok(Key::KEY_F15),
        "KEY_F16" => Ok(Key::KEY_F16),
        "KEY_F17" => Ok(Key::KEY_F17),
        "KEY_F18" => Ok(Key::KEY_F18),
        "KEY_F19" => Ok(Key::KEY_F19),
        "KEY_F20" => Ok(Key::KEY_F20),
        "KEY_F21" => Ok(Key::KEY_F21),
        "KEY_F22" => Ok(Key::KEY_F22),
        "KEY_F23" => Ok(Key::KEY_F23),
        "KEY_F24" => Ok(Key::KEY_F24),
        "KEY_PLAYCD" => Ok(Key::KEY_PLAYCD),
        "KEY_PAUSECD" => Ok(Key::KEY_PAUSECD),
        "KEY_PROG3" => Ok(Key::KEY_PROG3),
        "KEY_PROG4" => Ok(Key::KEY_PROG4),
        "KEY_DASHBOARD" => Ok(Key::KEY_DASHBOARD),
        "KEY_SUSPEND" => Ok(Key::KEY_SUSPEND),
        "KEY_CLOSE" => Ok(Key::KEY_CLOSE),
        "KEY_PLAY" => Ok(Key::KEY_PLAY),
        "KEY_FASTFORWARD" => Ok(Key::KEY_FASTFORWARD),
        "KEY_BASSBOOST" => Ok(Key::KEY_BASSBOOST),
        "KEY_PRINT" => Ok(Key::KEY_PRINT),
        "KEY_HP" => Ok(Key::KEY_HP),
        "KEY_CAMERA" => Ok(Key::KEY_CAMERA),
        "KEY_SOUND" => Ok(Key::KEY_SOUND),
        "KEY_QUESTION" => Ok(Key::KEY_QUESTION),
        "KEY_EMAIL" => Ok(Key::KEY_EMAIL),
        "KEY_CHAT" => Ok(Key::KEY_CHAT),
        "KEY_SEARCH" => Ok(Key::KEY_SEARCH),
        "KEY_CONNECT" => Ok(Key::KEY_CONNECT),
        "KEY_FINANCE" => Ok(Key::KEY_FINANCE),
        "KEY_SPORT" => Ok(Key::KEY_SPORT),
        "KEY_SHOP" => Ok(Key::KEY_SHOP),
        "KEY_ALTERASE" => Ok(Key::KEY_ALTERASE),
        "KEY_CANCEL" => Ok(Key::KEY_CANCEL),
        "KEY_BRIGHTNESSDOWN" => Ok(Key::KEY_BRIGHTNESSDOWN),
        "KEY_BRIGHTNESSUP" => Ok(Key::KEY_BRIGHTNESSUP),
        "KEY_MEDIA" => Ok(Key::KEY_MEDIA),
        "KEY_SWITCHVIDEOMODE" => Ok(Key::KEY_SWITCHVIDEOMODE),
        "KEY_KBDILLUMTOGGLE" => Ok(Key::KEY_KBDILLUMTOGGLE),
        "KEY_KBDILLUMDOWN" => Ok(Key::KEY_KBDILLUMDOWN),
        "KEY_KBDILLUMUP" => Ok(Key::KEY_KBDILLUMUP),
        "KEY_SEND" => Ok(Key::KEY_SEND),
        "KEY_REPLY" => Ok(Key::KEY_REPLY),
        "KEY_FORWARDMAIL" => Ok(Key::KEY_FORWARDMAIL),
        "KEY_SAVE" => Ok(Key::KEY_SAVE),
        "KEY_DOCUMENTS" => Ok(Key::KEY_DOCUMENTS),
        "KEY_BATTERY" => Ok(Key::KEY_BATTERY),
        "KEY_BLUETOOTH" => Ok(Key::KEY_BLUETOOTH),
        "KEY_WLAN" => Ok(Key::KEY_WLAN),
        "KEY_UWB" => Ok(Key::KEY_UWB),
        "KEY_UNKNOWN" => Ok(Key::KEY_UNKNOWN),
        "KEY_VIDEO_NEXT" => Ok(Key::KEY_VIDEO_NEXT),
        "KEY_VIDEO_PREV" => Ok(Key::KEY_VIDEO_PREV),
        "KEY_BRIGHTNESS_CYCLE" => Ok(Key::KEY_BRIGHTNESS_CYCLE),
        "KEY_BRIGHTNESS_AUTO" => Ok(Key::KEY_BRIGHTNESS_AUTO),
        "KEY_DISPLAY_OFF" => Ok(Key::KEY_DISPLAY_OFF),
        "KEY_WWAN" => Ok(Key::KEY_WWAN),
        "KEY_RFKILL" => Ok(Key::KEY_RFKILL),
        "KEY_MICMUTE" => Ok(Key::KEY_MICMUTE),
        "BTN_0" => Ok(Key::BTN_0),
        "BTN_1" => Ok(Key::BTN_1),
        "BTN_2" => Ok(Key::BTN_2),
        "BTN_3" => Ok(Key::BTN_3),
        "BTN_4" => Ok(Key::BTN_4),
        "BTN_5" => Ok(Key::BTN_5),
        "BTN_6" => Ok(Key::BTN_6),
        "BTN_7" => Ok(Key::BTN_7),
        "BTN_8" => Ok(Key::BTN_8),
        "BTN_9" => Ok(Key::BTN_9),
        "BTN_LEFT" => Ok(Key::BTN_LEFT),
        "BTN_RIGHT" => Ok(Key::BTN_RIGHT),
        "BTN_MIDDLE" => Ok(Key::BTN_MIDDLE),
        "BTN_SIDE" => Ok(Key::BTN_SIDE),
        "BTN_EXTRA" => Ok(Key::BTN_EXTRA),
        "BTN_FORWARD" => Ok(Key::BTN_FORWARD),
        "BTN_BACK" => Ok(Key::BTN_BACK),
        "BTN_TASK" => Ok(Key::BTN_TASK),
        "BTN_TRIGGER" => Ok(Key::BTN_TRIGGER),
        "BTN_THUMB" => Ok(Key::BTN_THUMB),
        "BTN_THUMB2" => Ok(Key::BTN_THUMB2),
        "BTN_TOP" => Ok(Key::BTN_TOP),
        "BTN_TOP2" => Ok(Key::BTN_TOP2),
        "BTN_PINKIE" => Ok(Key::BTN_PINKIE),
        "BTN_BASE" => Ok(Key::BTN_BASE),
        "BTN_BASE2" => Ok(Key::BTN_BASE2),
        "BTN_BASE3" => Ok(Key::BTN_BASE3),
        "BTN_BASE4" => Ok(Key::BTN_BASE4),
        "BTN_BASE5" => Ok(Key::BTN_BASE5),
        "BTN_BASE6" => Ok(Key::BTN_BASE6),
        "BTN_DEAD" => Ok(Key::BTN_DEAD),
        "BTN_SOUTH" => Ok(Key::BTN_SOUTH),
        "BTN_EAST" => Ok(Key::BTN_EAST),
        "BTN_C" => Ok(Key::BTN_C),
        "BTN_NORTH" => Ok(Key::BTN_NORTH),
        "BTN_WEST" => Ok(Key::BTN_WEST),
        "BTN_Z" => Ok(Key::BTN_Z),
        "BTN_TL" => Ok(Key::BTN_TL),
        "BTN_TR" => Ok(Key::BTN_TR),
        "BTN_TL2" => Ok(Key::BTN_TL2),
        "BTN_TR2" => Ok(Key::BTN_TR2),
        "BTN_SELECT" => Ok(Key::BTN_SELECT),
        "BTN_START" => Ok(Key::BTN_START),
        "BTN_MODE" => Ok(Key::BTN_MODE),
        "BTN_THUMBL" => Ok(Key::BTN_THUMBL),
        "BTN_THUMBR" => Ok(Key::BTN_THUMBR),
        "BTN_TOOL_PEN" => Ok(Key::BTN_TOOL_PEN),
        "BTN_TOOL_RUBBER" => Ok(Key::BTN_TOOL_RUBBER),
        "BTN_TOOL_BRUSH" => Ok(Key::BTN_TOOL_BRUSH),
        "BTN_TOOL_PENCIL" => Ok(Key::BTN_TOOL_PENCIL),
        "BTN_TOOL_AIRBRUSH" => Ok(Key::BTN_TOOL_AIRBRUSH),
        "BTN_TOOL_FINGER" => Ok(Key::BTN_TOOL_FINGER),
        "BTN_TOOL_MOUSE" => Ok(Key::BTN_TOOL_MOUSE),
        "BTN_TOOL_LENS" => Ok(Key::BTN_TOOL_LENS),
        "BTN_TOOL_QUINTTAP" => Ok(Key::BTN_TOOL_QUINTTAP),
        "BTN_TOUCH" => Ok(Key::BTN_TOUCH),
        "BTN_STYLUS" => Ok(Key::BTN_STYLUS),
        "BTN_STYLUS2" => Ok(Key::BTN_STYLUS2),
        "BTN_TOOL_DOUBLETAP" => Ok(Key::BTN_TOOL_DOUBLETAP),
        "BTN_TOOL_TRIPLETAP" => Ok(Key::BTN_TOOL_TRIPLETAP),
        "BTN_TOOL_QUADTAP" => Ok(Key::BTN_TOOL_QUADTAP),
        "BTN_GEAR_DOWN" => Ok(Key::BTN_GEAR_DOWN),
        "BTN_GEAR_UP" => Ok(Key::BTN_GEAR_UP),
        "KEY_OK" => Ok(Key::KEY_OK),
        "KEY_SELECT" => Ok(Key::KEY_SELECT),
        "KEY_GOTO" => Ok(Key::KEY_GOTO),
        "KEY_CLEAR" => Ok(Key::KEY_CLEAR),
        "KEY_POWER2" => Ok(Key::KEY_POWER2),
        "KEY_OPTION" => Ok(Key::KEY_OPTION),
        "KEY_INFO" => Ok(Key::KEY_INFO),
        "KEY_TIME" => Ok(Key::KEY_TIME),
        "KEY_VENDOR" => Ok(Key::KEY_VENDOR),
        "KEY_ARCHIVE" => Ok(Key::KEY_ARCHIVE),
        "KEY_PROGRAM" => Ok(Key::KEY_PROGRAM),
        "KEY_CHANNEL" => Ok(Key::KEY_CHANNEL),
        "KEY_FAVORITES" => Ok(Key::KEY_FAVORITES),
        "KEY_EPG" => Ok(Key::KEY_EPG),
        "KEY_PVR" => Ok(Key::KEY_PVR),
        "KEY_MHP" => Ok(Key::KEY_MHP),
        "KEY_LANGUAGE" => Ok(Key::KEY_LANGUAGE),
        "KEY_TITLE" => Ok(Key::KEY_TITLE),
        "KEY_SUBTITLE" => Ok(Key::KEY_SUBTITLE),
        "KEY_ANGLE" => Ok(Key::KEY_ANGLE),
        "KEY_ZOOM" => Ok(Key::KEY_ZOOM),
        "KEY_FULL_SCREEN" => Ok(Key::KEY_FULL_SCREEN),
        "KEY_MODE" => Ok(Key::KEY_MODE),
        "KEY_KEYBOARD" => Ok(Key::KEY_KEYBOARD),
        "KEY_SCREEN" => Ok(Key::KEY_SCREEN),
        "KEY_PC" => Ok(Key::KEY_PC),
        "KEY_TV" => Ok(Key::KEY_TV),
        "KEY_TV2" => Ok(Key::KEY_TV2),
        "KEY_VCR" => Ok(Key::KEY_VCR),
        "KEY_VCR2" => Ok(Key::KEY_VCR2),
        "KEY_SAT" => Ok(Key::KEY_SAT),
        "KEY_SAT2" => Ok(Key::KEY_SAT2),
        "KEY_CD" => Ok(Key::KEY_CD),
        "KEY_TAPE" => Ok(Key::KEY_TAPE),
        "KEY_RADIO" => Ok(Key::KEY_RADIO),
        "KEY_TUNER" => Ok(Key::KEY_TUNER),
        "KEY_PLAYER" => Ok(Key::KEY_PLAYER),
        "KEY_TEXT" => Ok(Key::KEY_TEXT),
        "KEY_DVD" => Ok(Key::KEY_DVD),
        "KEY_AUX" => Ok(Key::KEY_AUX),
        "KEY_MP3" => Ok(Key::KEY_MP3),
        "KEY_AUDIO" => Ok(Key::KEY_AUDIO),
        "KEY_VIDEO" => Ok(Key::KEY_VIDEO),
        "KEY_DIRECTORY" => Ok(Key::KEY_DIRECTORY),
        "KEY_LIST" => Ok(Key::KEY_LIST),
        "KEY_MEMO" => Ok(Key::KEY_MEMO),
        "KEY_CALENDAR" => Ok(Key::KEY_CALENDAR),
        "KEY_RED" => Ok(Key::KEY_RED),
        "KEY_GREEN" => Ok(Key::KEY_GREEN),
        "KEY_YELLOW" => Ok(Key::KEY_YELLOW),
        "KEY_BLUE" => Ok(Key::KEY_BLUE),
        "KEY_CHANNELUP" => Ok(Key::KEY_CHANNELUP),
        "KEY_CHANNELDOWN" => Ok(Key::KEY_CHANNELDOWN),
        "KEY_FIRST" => Ok(Key::KEY_FIRST),
        "KEY_LAST" => Ok(Key::KEY_LAST),
        "KEY_AB" => Ok(Key::KEY_AB),
        "KEY_NEXT" => Ok(Key::KEY_NEXT),
        "KEY_RESTART" => Ok(Key::KEY_RESTART),
        "KEY_SLOW" => Ok(Key::KEY_SLOW),
        "KEY_SHUFFLE" => Ok(Key::KEY_SHUFFLE),
        "KEY_BREAK" => Ok(Key::KEY_BREAK),
        "KEY_PREVIOUS" => Ok(Key::KEY_PREVIOUS),
        "KEY_DIGITS" => Ok(Key::KEY_DIGITS),
        "KEY_TEEN" => Ok(Key::KEY_TEEN),
        "KEY_TWEN" => Ok(Key::KEY_TWEN),
        "KEY_VIDEOPHONE" => Ok(Key::KEY_VIDEOPHONE),
        "KEY_GAMES" => Ok(Key::KEY_GAMES),
        "KEY_ZOOMIN" => Ok(Key::KEY_ZOOMIN),
        "KEY_ZOOMOUT" => Ok(Key::KEY_ZOOMOUT),
        "KEY_ZOOMRESET" => Ok(Key::KEY_ZOOMRESET),
        "KEY_WORDPROCESSOR" => Ok(Key::KEY_WORDPROCESSOR),
        "KEY_EDITOR" => Ok(Key::KEY_EDITOR),
        "KEY_SPREADSHEET" => Ok(Key::KEY_SPREADSHEET),
        "KEY_GRAPHICSEDITOR" => Ok(Key::KEY_GRAPHICSEDITOR),
        "KEY_PRESENTATION" => Ok(Key::KEY_PRESENTATION),
        "KEY_DATABASE" => Ok(Key::KEY_DATABASE),
        "KEY_NEWS" => Ok(Key::KEY_NEWS),
        "KEY_VOICEMAIL" => Ok(Key::KEY_VOICEMAIL),
        "KEY_ADDRESSBOOK" => Ok(Key::KEY_ADDRESSBOOK),
        "KEY_MESSENGER" => Ok(Key::KEY_MESSENGER),
        "KEY_DISPLAYTOGGLE" => Ok(Key::KEY_DISPLAYTOGGLE),
        "KEY_SPELLCHECK" => Ok(Key::KEY_SPELLCHECK),
        "KEY_LOGOFF" => Ok(Key::KEY_LOGOFF),
        "KEY_DOLLAR" => Ok(Key::KEY_DOLLAR),
        "KEY_EURO" => Ok(Key::KEY_EURO),
        "KEY_FRAMEBACK" => Ok(Key::KEY_FRAMEBACK),
        "KEY_FRAMEFORWARD" => Ok(Key::KEY_FRAMEFORWARD),
        "KEY_CONTEXT_MENU" => Ok(Key::KEY_CONTEXT_MENU),
        "KEY_MEDIA_REPEAT" => Ok(Key::KEY_MEDIA_REPEAT),
        "KEY_10CHANNELSUP" => Ok(Key::KEY_10CHANNELSUP),
        "KEY_10CHANNELSDOWN" => Ok(Key::KEY_10CHANNELSDOWN),
        "KEY_IMAGES" => Ok(Key::KEY_IMAGES),
        "KEY_DEL_EOL" => Ok(Key::KEY_DEL_EOL),
        "KEY_DEL_EOS" => Ok(Key::KEY_DEL_EOS),
        "KEY_INS_LINE" => Ok(Key::KEY_INS_LINE),
        "KEY_DEL_LINE" => Ok(Key::KEY_DEL_LINE),
        "KEY_FN" => Ok(Key::KEY_FN),
        "KEY_FN_ESC" => Ok(Key::KEY_FN_ESC),
        "KEY_FN_F1" => Ok(Key::KEY_FN_F1),
        "KEY_FN_F2" => Ok(Key::KEY_FN_F2),
        "KEY_FN_F3" => Ok(Key::KEY_FN_F3),
        "KEY_FN_F4" => Ok(Key::KEY_FN_F4),
        "KEY_FN_F5" => Ok(Key::KEY_FN_F5),
        "KEY_FN_F6" => Ok(Key::KEY_FN_F6),
        "KEY_FN_F7" => Ok(Key::KEY_FN_F7),
        "KEY_FN_F8" => Ok(Key::KEY_FN_F8),
        "KEY_FN_F9" => Ok(Key::KEY_FN_F9),
        "KEY_FN_F10" => Ok(Key::KEY_FN_F10),
        "KEY_FN_F11" => Ok(Key::KEY_FN_F11),
        "KEY_FN_F12" => Ok(Key::KEY_FN_F12),
        "KEY_FN_1" => Ok(Key::KEY_FN_1),
        "KEY_FN_2" => Ok(Key::KEY_FN_2),
        "KEY_FN_D" => Ok(Key::KEY_FN_D),
        "KEY_FN_E" => Ok(Key::KEY_FN_E),
        "KEY_FN_F" => Ok(Key::KEY_FN_F),
        "KEY_FN_S" => Ok(Key::KEY_FN_S),
        "KEY_FN_B" => Ok(Key::KEY_FN_B),
        "KEY_BRL_DOT1" => Ok(Key::KEY_BRL_DOT1),
        "KEY_BRL_DOT2" => Ok(Key::KEY_BRL_DOT2),
        "KEY_BRL_DOT3" => Ok(Key::KEY_BRL_DOT3),
        "KEY_BRL_DOT4" => Ok(Key::KEY_BRL_DOT4),
        "KEY_BRL_DOT5" => Ok(Key::KEY_BRL_DOT5),
        "KEY_BRL_DOT6" => Ok(Key::KEY_BRL_DOT6),
        "KEY_BRL_DOT7" => Ok(Key::KEY_BRL_DOT7),
        "KEY_BRL_DOT8" => Ok(Key::KEY_BRL_DOT8),
        "KEY_BRL_DOT9" => Ok(Key::KEY_BRL_DOT9),
        "KEY_BRL_DOT10" => Ok(Key::KEY_BRL_DOT10),
        "KEY_NUMERIC_0" => Ok(Key::KEY_NUMERIC_0),
        "KEY_NUMERIC_1" => Ok(Key::KEY_NUMERIC_1),
        "KEY_NUMERIC_2" => Ok(Key::KEY_NUMERIC_2),
        "KEY_NUMERIC_3" => Ok(Key::KEY_NUMERIC_3),
        "KEY_NUMERIC_4" => Ok(Key::KEY_NUMERIC_4),
        "KEY_NUMERIC_5" => Ok(Key::KEY_NUMERIC_5),
        "KEY_NUMERIC_6" => Ok(Key::KEY_NUMERIC_6),
        "KEY_NUMERIC_7" => Ok(Key::KEY_NUMERIC_7),
        "KEY_NUMERIC_8" => Ok(Key::KEY_NUMERIC_8),
        "KEY_NUMERIC_9" => Ok(Key::KEY_NUMERIC_9),
        "KEY_NUMERIC_STAR" => Ok(Key::KEY_NUMERIC_STAR),
        "KEY_NUMERIC_POUND" => Ok(Key::KEY_NUMERIC_POUND),
        "KEY_NUMERIC_A" => Ok(Key::KEY_NUMERIC_A),
        "KEY_NUMERIC_B" => Ok(Key::KEY_NUMERIC_B),
        "KEY_NUMERIC_C" => Ok(Key::KEY_NUMERIC_C),
        "KEY_NUMERIC_D" => Ok(Key::KEY_NUMERIC_D),
        "KEY_CAMERA_FOCUS" => Ok(Key::KEY_CAMERA_FOCUS),
        "KEY_WPS_BUTTON" => Ok(Key::KEY_WPS_BUTTON),
        "KEY_TOUCHPAD_TOGGLE" => Ok(Key::KEY_TOUCHPAD_TOGGLE),
        "KEY_TOUCHPAD_ON" => Ok(Key::KEY_TOUCHPAD_ON),
        "KEY_TOUCHPAD_OFF" => Ok(Key::KEY_TOUCHPAD_OFF),
        "KEY_CAMERA_ZOOMIN" => Ok(Key::KEY_CAMERA_ZOOMIN),
        "KEY_CAMERA_ZOOMOUT" => Ok(Key::KEY_CAMERA_ZOOMOUT),
        "KEY_CAMERA_UP" => Ok(Key::KEY_CAMERA_UP),
        "KEY_CAMERA_DOWN" => Ok(Key::KEY_CAMERA_DOWN),
        "KEY_CAMERA_LEFT" => Ok(Key::KEY_CAMERA_LEFT),
        "KEY_CAMERA_RIGHT" => Ok(Key::KEY_CAMERA_RIGHT),
        "KEY_ATTENDANT_ON" => Ok(Key::KEY_ATTENDANT_ON),
        "KEY_ATTENDANT_OFF" => Ok(Key::KEY_ATTENDANT_OFF),
        "KEY_ATTENDANT_TOGGLE" => Ok(Key::KEY_ATTENDANT_TOGGLE),
        "KEY_LIGHTS_TOGGLE" => Ok(Key::KEY_LIGHTS_TOGGLE),
        "BTN_DPAD_UP" => Ok(Key::BTN_DPAD_UP),
        "BTN_DPAD_DOWN" => Ok(Key::BTN_DPAD_DOWN),
        "BTN_DPAD_LEFT" => Ok(Key::BTN_DPAD_LEFT),
        "BTN_DPAD_RIGHT" => Ok(Key::BTN_DPAD_RIGHT),
        "KEY_ALS_TOGGLE" => Ok(Key::KEY_ALS_TOGGLE),
        "KEY_BUTTONCONFIG" => Ok(Key::KEY_BUTTONCONFIG),
        "KEY_TASKMANAGER" => Ok(Key::KEY_TASKMANAGER),
        "KEY_JOURNAL" => Ok(Key::KEY_JOURNAL),
        "KEY_CONTROLPANEL" => Ok(Key::KEY_CONTROLPANEL),
        "KEY_APPSELECT" => Ok(Key::KEY_APPSELECT),
        "KEY_SCREENSAVER" => Ok(Key::KEY_SCREENSAVER),
        "KEY_VOICECOMMAND" => Ok(Key::KEY_VOICECOMMAND),
        "KEY_ASSISTANT" => Ok(Key::KEY_ASSISTANT),
        "KEY_KBD_LAYOUT_NEXT" => Ok(Key::KEY_KBD_LAYOUT_NEXT),
        "KEY_BRIGHTNESS_MIN" => Ok(Key::KEY_BRIGHTNESS_MIN),
        "KEY_BRIGHTNESS_MAX" => Ok(Key::KEY_BRIGHTNESS_MAX),
        "KEY_KBDINPUTASSIST_PREV" => Ok(Key::KEY_KBDINPUTASSIST_PREV),
        "KEY_KBDINPUTASSIST_NEXT" => Ok(Key::KEY_KBDINPUTASSIST_NEXT),
        "KEY_KBDINPUTASSIST_PREVGROUP" => Ok(Key::KEY_KBDINPUTASSIST_PREVGROUP),
        "KEY_KBDINPUTASSIST_NEXTGROUP" => Ok(Key::KEY_KBDINPUTASSIST_NEXTGROUP),
        "KEY_KBDINPUTASSIST_ACCEPT" => Ok(Key::KEY_KBDINPUTASSIST_ACCEPT),
        "KEY_KBDINPUTASSIST_CANCEL" => Ok(Key::KEY_KBDINPUTASSIST_CANCEL),
        "KEY_RIGHT_UP" => Ok(Key::KEY_RIGHT_UP),
        "KEY_RIGHT_DOWN" => Ok(Key::KEY_RIGHT_DOWN),
        "KEY_LEFT_UP" => Ok(Key::KEY_LEFT_UP),
        "KEY_LEFT_DOWN" => Ok(Key::KEY_LEFT_DOWN),
        "KEY_ROOT_MENU" => Ok(Key::KEY_ROOT_MENU),
        "KEY_MEDIA_TOP_MENU" => Ok(Key::KEY_MEDIA_TOP_MENU),
        "KEY_NUMERIC_11" => Ok(Key::KEY_NUMERIC_11),
        "KEY_NUMERIC_12" => Ok(Key::KEY_NUMERIC_12),
        "KEY_AUDIO_DESC" => Ok(Key::KEY_AUDIO_DESC),
        "KEY_3D_MODE" => Ok(Key::KEY_3D_MODE),
        "KEY_NEXT_FAVORITE" => Ok(Key::KEY_NEXT_FAVORITE),
        "KEY_STOP_RECORD" => Ok(Key::KEY_STOP_RECORD),
        "KEY_PAUSE_RECORD" => Ok(Key::KEY_PAUSE_RECORD),
        "KEY_VOD" => Ok(Key::KEY_VOD),
        "KEY_UNMUTE" => Ok(Key::KEY_UNMUTE),
        "KEY_FASTREVERSE" => Ok(Key::KEY_FASTREVERSE),
        "KEY_SLOWREVERSE" => Ok(Key::KEY_SLOWREVERSE),
        "KEY_DATA" => Ok(Key::KEY_DATA),
        "KEY_ONSCREEN_KEYBOARD" => Ok(Key::KEY_ONSCREEN_KEYBOARD),
        "KEY_PRIVACY_SCREEN_TOGGLE" => Ok(Key::KEY_PRIVACY_SCREEN_TOGGLE),
        "KEY_SELECTIVE_SCREENSHOT" => Ok(Key::KEY_SELECTIVE_SCREENSHOT),
        "BTN_TRIGGER_HAPPY1" => Ok(Key::BTN_TRIGGER_HAPPY1),
        "BTN_TRIGGER_HAPPY2" => Ok(Key::BTN_TRIGGER_HAPPY2),
        "BTN_TRIGGER_HAPPY3" => Ok(Key::BTN_TRIGGER_HAPPY3),
        "BTN_TRIGGER_HAPPY4" => Ok(Key::BTN_TRIGGER_HAPPY4),
        "BTN_TRIGGER_HAPPY5" => Ok(Key::BTN_TRIGGER_HAPPY5),
        "BTN_TRIGGER_HAPPY6" => Ok(Key::BTN_TRIGGER_HAPPY6),
        "BTN_TRIGGER_HAPPY7" => Ok(Key::BTN_TRIGGER_HAPPY7),
        "BTN_TRIGGER_HAPPY8" => Ok(Key::BTN_TRIGGER_HAPPY8),
        "BTN_TRIGGER_HAPPY9" => Ok(Key::BTN_TRIGGER_HAPPY9),
        "BTN_TRIGGER_HAPPY10" => Ok(Key::BTN_TRIGGER_HAPPY10),
        "BTN_TRIGGER_HAPPY11" => Ok(Key::BTN_TRIGGER_HAPPY11),
        "BTN_TRIGGER_HAPPY12" => Ok(Key::BTN_TRIGGER_HAPPY12),
        "BTN_TRIGGER_HAPPY13" => Ok(Key::BTN_TRIGGER_HAPPY13),
        "BTN_TRIGGER_HAPPY14" => Ok(Key::BTN_TRIGGER_HAPPY14),
        "BTN_TRIGGER_HAPPY15" => Ok(Key::BTN_TRIGGER_HAPPY15),
        "BTN_TRIGGER_HAPPY16" => Ok(Key::BTN_TRIGGER_HAPPY16),
        "BTN_TRIGGER_HAPPY17" => Ok(Key::BTN_TRIGGER_HAPPY17),
        "BTN_TRIGGER_HAPPY18" => Ok(Key::BTN_TRIGGER_HAPPY18),
        "BTN_TRIGGER_HAPPY19" => Ok(Key::BTN_TRIGGER_HAPPY19),
        "BTN_TRIGGER_HAPPY20" => Ok(Key::BTN_TRIGGER_HAPPY20),
        "BTN_TRIGGER_HAPPY21" => Ok(Key::BTN_TRIGGER_HAPPY21),
        "BTN_TRIGGER_HAPPY22" => Ok(Key::BTN_TRIGGER_HAPPY22),
        "BTN_TRIGGER_HAPPY23" => Ok(Key::BTN_TRIGGER_HAPPY23),
        "BTN_TRIGGER_HAPPY24" => Ok(Key::BTN_TRIGGER_HAPPY24),
        "BTN_TRIGGER_HAPPY25" => Ok(Key::BTN_TRIGGER_HAPPY25),
        "BTN_TRIGGER_HAPPY26" => Ok(Key::BTN_TRIGGER_HAPPY26),
        "BTN_TRIGGER_HAPPY27" => Ok(Key::BTN_TRIGGER_HAPPY27),
        "BTN_TRIGGER_HAPPY28" => Ok(Key::BTN_TRIGGER_HAPPY28),
        "BTN_TRIGGER_HAPPY29" => Ok(Key::BTN_TRIGGER_HAPPY29),
        "BTN_TRIGGER_HAPPY30" => Ok(Key::BTN_TRIGGER_HAPPY30),
        "BTN_TRIGGER_HAPPY31" => Ok(Key::BTN_TRIGGER_HAPPY31),
        "BTN_TRIGGER_HAPPY32" => Ok(Key::BTN_TRIGGER_HAPPY32),
        "BTN_TRIGGER_HAPPY33" => Ok(Key::BTN_TRIGGER_HAPPY33),
        "BTN_TRIGGER_HAPPY34" => Ok(Key::BTN_TRIGGER_HAPPY34),
        "BTN_TRIGGER_HAPPY35" => Ok(Key::BTN_TRIGGER_HAPPY35),
        "BTN_TRIGGER_HAPPY36" => Ok(Key::BTN_TRIGGER_HAPPY36),
        "BTN_TRIGGER_HAPPY37" => Ok(Key::BTN_TRIGGER_HAPPY37),
        "BTN_TRIGGER_HAPPY38" => Ok(Key::BTN_TRIGGER_HAPPY38),
        "BTN_TRIGGER_HAPPY39" => Ok(Key::BTN_TRIGGER_HAPPY39),
        "BTN_TRIGGER_HAPPY40" => Ok(Key::BTN_TRIGGER_HAPPY40),

        _ => Err(anyhow!("Could not find {} in Supported key list", s)),
    }
}