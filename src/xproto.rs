/*
 * This file generated automatically from xproto.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(unused_unsafe)];
use core;
use core::libc::*;
use ll::base::*;
use base;
use base::*;
use ll;
use ll::xproto::*;
use core::option::Option;
use core::iterator::Iterator;

pub type Char2b = base::Struct<char2b>;

pub type Char2bIterator = char2b_iterator;

pub type WindowIterator = window_iterator;

pub type PixmapIterator = pixmap_iterator;

pub type CursorIterator = cursor_iterator;

pub type FontIterator = font_iterator;

pub type GcontextIterator = gcontext_iterator;

pub type ColormapIterator = colormap_iterator;

pub type AtomIterator = atom_iterator;

pub type DrawableIterator = drawable_iterator;

pub type FontableIterator = fontable_iterator;

pub type VisualidIterator = visualid_iterator;

pub type TimestampIterator = timestamp_iterator;

pub type KeysymIterator = keysym_iterator;

pub type KeycodeIterator = keycode_iterator;

pub type ButtonIterator = button_iterator;

pub type PointIterator = point_iterator;

pub type RectangleIterator = rectangle_iterator;

pub type ArcIterator = arc_iterator;

pub type FormatIterator = format_iterator;


pub type visual_class = c_uint;//{
    pub static XCB_VISUAL_CLASS_STATIC_GRAY : visual_class = 0;
    pub static XCB_VISUAL_CLASS_GRAY_SCALE : visual_class = 1;
    pub static XCB_VISUAL_CLASS_STATIC_COLOR : visual_class = 2;
    pub static XCB_VISUAL_CLASS_PSEUDO_COLOR : visual_class = 3;
    pub static XCB_VISUAL_CLASS_TRUE_COLOR : visual_class = 4;
    pub static XCB_VISUAL_CLASS_DIRECT_COLOR : visual_class = 5;
//}
pub type Visualtype = base::Struct<visualtype>;

pub type VisualtypeIterator = visualtype_iterator;

pub type DepthIterator = depth_iterator;


pub type event_mask = c_uint;//{
    pub static XCB_EVENT_MASK_NO_EVENT : event_mask = 0;
    pub static XCB_EVENT_MASK_KEY_PRESS : event_mask = 1;
    pub static XCB_EVENT_MASK_KEY_RELEASE : event_mask = 2;
    pub static XCB_EVENT_MASK_BUTTON_PRESS : event_mask = 4;
    pub static XCB_EVENT_MASK_BUTTON_RELEASE : event_mask = 8;
    pub static XCB_EVENT_MASK_ENTER_WINDOW : event_mask = 16;
    pub static XCB_EVENT_MASK_LEAVE_WINDOW : event_mask = 32;
    pub static XCB_EVENT_MASK_POINTER_MOTION : event_mask = 64;
    pub static XCB_EVENT_MASK_POINTER_MOTION_HINT : event_mask = 128;
    pub static XCB_EVENT_MASK_BUTTON_1_MOTION : event_mask = 256;
    pub static XCB_EVENT_MASK_BUTTON_2_MOTION : event_mask = 512;
    pub static XCB_EVENT_MASK_BUTTON_3_MOTION : event_mask = 1024;
    pub static XCB_EVENT_MASK_BUTTON_4_MOTION : event_mask = 2048;
    pub static XCB_EVENT_MASK_BUTTON_5_MOTION : event_mask = 4096;
    pub static XCB_EVENT_MASK_BUTTON_MOTION : event_mask = 8192;
    pub static XCB_EVENT_MASK_KEYMAP_STATE : event_mask = 16384;
    pub static XCB_EVENT_MASK_EXPOSURE : event_mask = 32768;
    pub static XCB_EVENT_MASK_VISIBILITY_CHANGE : event_mask = 65536;
    pub static XCB_EVENT_MASK_STRUCTURE_NOTIFY : event_mask = 131072;
    pub static XCB_EVENT_MASK_RESIZE_REDIRECT : event_mask = 262144;
    pub static XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY : event_mask = 524288;
    pub static XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT : event_mask = 1048576;
    pub static XCB_EVENT_MASK_FOCUS_CHANGE : event_mask = 2097152;
    pub static XCB_EVENT_MASK_PROPERTY_CHANGE : event_mask = 4194304;
    pub static XCB_EVENT_MASK_COLOR_MAP_CHANGE : event_mask = 8388608;
    pub static XCB_EVENT_MASK_OWNER_GRAB_BUTTON : event_mask = 16777216;
//}

pub type backing_store = c_uint;//{
    pub static XCB_BACKING_STORE_NOT_USEFUL : backing_store = 0;
    pub static XCB_BACKING_STORE_WHEN_MAPPED : backing_store = 1;
    pub static XCB_BACKING_STORE_ALWAYS : backing_store = 2;
//}
pub type ScreenIterator = screen_iterator;

pub type SetupRequestIterator = setup_request_iterator;

pub type SetupFailedIterator = setup_failed_iterator;

pub type SetupAuthenticateIterator = setup_authenticate_iterator;


pub type image_order = c_uint;//{
    pub static XCB_IMAGE_ORDER_LSB_FIRST : image_order = 0;
    pub static XCB_IMAGE_ORDER_MSB_FIRST : image_order = 1;
//}
pub type SetupIterator = setup_iterator;


pub type mod_mask = c_uint;//{
    pub static XCB_MOD_MASK_SHIFT : mod_mask = 1;
    pub static XCB_MOD_MASK_LOCK : mod_mask = 2;
    pub static XCB_MOD_MASK_CONTROL : mod_mask = 4;
    pub static XCB_MOD_MASK_1 : mod_mask = 8;
    pub static XCB_MOD_MASK_2 : mod_mask = 16;
    pub static XCB_MOD_MASK_3 : mod_mask = 32;
    pub static XCB_MOD_MASK_4 : mod_mask = 64;
    pub static XCB_MOD_MASK_5 : mod_mask = 128;
    pub static XCB_MOD_MASK_ANY : mod_mask = 32768;
//}

pub type key_but_mask = c_uint;//{
    pub static XCB_KEY_BUT_MASK_SHIFT : key_but_mask = 1;
    pub static XCB_KEY_BUT_MASK_LOCK : key_but_mask = 2;
    pub static XCB_KEY_BUT_MASK_CONTROL : key_but_mask = 4;
    pub static XCB_KEY_BUT_MASK_MOD_1 : key_but_mask = 8;
    pub static XCB_KEY_BUT_MASK_MOD_2 : key_but_mask = 16;
    pub static XCB_KEY_BUT_MASK_MOD_3 : key_but_mask = 32;
    pub static XCB_KEY_BUT_MASK_MOD_4 : key_but_mask = 64;
    pub static XCB_KEY_BUT_MASK_MOD_5 : key_but_mask = 128;
    pub static XCB_KEY_BUT_MASK_BUTTON_1 : key_but_mask = 256;
    pub static XCB_KEY_BUT_MASK_BUTTON_2 : key_but_mask = 512;
    pub static XCB_KEY_BUT_MASK_BUTTON_3 : key_but_mask = 1024;
    pub static XCB_KEY_BUT_MASK_BUTTON_4 : key_but_mask = 2048;
    pub static XCB_KEY_BUT_MASK_BUTTON_5 : key_but_mask = 4096;
//}

pub type window_enum = c_uint;//{
    pub static XCB_WINDOW_NONE : window_enum = 0;
//}
/** Opcode for xcb_key_press. */
pub static XCB_KEY_PRESS : u8 = 2;
pub type KeyPressEvent = base::Event<key_press_event>;
/** Opcode for xcb_key_release. */
pub static XCB_KEY_RELEASE : u8 = 3;
pub type KeyReleaseEvent = base::Event<key_release_event>;

pub type button_mask = c_uint;//{
    pub static XCB_BUTTON_MASK_1 : button_mask = 256;
    pub static XCB_BUTTON_MASK_2 : button_mask = 512;
    pub static XCB_BUTTON_MASK_3 : button_mask = 1024;
    pub static XCB_BUTTON_MASK_4 : button_mask = 2048;
    pub static XCB_BUTTON_MASK_5 : button_mask = 4096;
    pub static XCB_BUTTON_MASK_ANY : button_mask = 32768;
//}
/** Opcode for xcb_button_press. */
pub static XCB_BUTTON_PRESS : u8 = 4;
pub type ButtonPressEvent = base::Event<button_press_event>;
/** Opcode for xcb_button_release. */
pub static XCB_BUTTON_RELEASE : u8 = 5;
pub type ButtonReleaseEvent = base::Event<button_release_event>;

pub type motion = c_uint;//{
    pub static XCB_MOTION_NORMAL : motion = 0;
    pub static XCB_MOTION_HINT : motion = 1;
//}
/** Opcode for xcb_motion_notify. */
pub static XCB_MOTION_NOTIFY : u8 = 6;
pub type MotionNotifyEvent = base::Event<motion_notify_event>;

pub type notify_detail = c_uint;//{
    pub static XCB_NOTIFY_DETAIL_ANCESTOR : notify_detail = 0;
    pub static XCB_NOTIFY_DETAIL_VIRTUAL : notify_detail = 1;
    pub static XCB_NOTIFY_DETAIL_INFERIOR : notify_detail = 2;
    pub static XCB_NOTIFY_DETAIL_NONLINEAR : notify_detail = 3;
    pub static XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL : notify_detail = 4;
    pub static XCB_NOTIFY_DETAIL_POINTER : notify_detail = 5;
    pub static XCB_NOTIFY_DETAIL_POINTER_ROOT : notify_detail = 6;
    pub static XCB_NOTIFY_DETAIL_NONE : notify_detail = 7;
//}

pub type notify_mode = c_uint;//{
    pub static XCB_NOTIFY_MODE_NORMAL : notify_mode = 0;
    pub static XCB_NOTIFY_MODE_GRAB : notify_mode = 1;
    pub static XCB_NOTIFY_MODE_UNGRAB : notify_mode = 2;
    pub static XCB_NOTIFY_MODE_WHILE_GRABBED : notify_mode = 3;
//}
/** Opcode for xcb_enter_notify. */
pub static XCB_ENTER_NOTIFY : u8 = 7;
pub type EnterNotifyEvent = base::Event<enter_notify_event>;
/** Opcode for xcb_leave_notify. */
pub static XCB_LEAVE_NOTIFY : u8 = 8;
pub type LeaveNotifyEvent = base::Event<leave_notify_event>;
/** Opcode for xcb_focus_in. */
pub static XCB_FOCUS_IN : u8 = 9;
pub type FocusInEvent = base::Event<focus_in_event>;
/** Opcode for xcb_focus_out. */
pub static XCB_FOCUS_OUT : u8 = 10;
pub type FocusOutEvent = base::Event<focus_out_event>;
/** Opcode for xcb_keymap_notify. */
pub static XCB_KEYMAP_NOTIFY : u8 = 11;
pub type KeymapNotifyEvent = base::Event<keymap_notify_event>;
/** Opcode for xcb_expose. */
pub static XCB_EXPOSE : u8 = 12;
pub type ExposeEvent = base::Event<expose_event>;
/** Opcode for xcb_graphics_exposure. */
pub static XCB_GRAPHICS_EXPOSURE : u8 = 13;
pub type GraphicsExposureEvent = base::Event<graphics_exposure_event>;
/** Opcode for xcb_no_exposure. */
pub static XCB_NO_EXPOSURE : u8 = 14;
pub type NoExposureEvent = base::Event<no_exposure_event>;

pub type visibility = c_uint;//{
    pub static XCB_VISIBILITY_UNOBSCURED : visibility = 0;
    pub static XCB_VISIBILITY_PARTIALLY_OBSCURED : visibility = 1;
    pub static XCB_VISIBILITY_FULLY_OBSCURED : visibility = 2;
//}
/** Opcode for xcb_visibility_notify. */
pub static XCB_VISIBILITY_NOTIFY : u8 = 15;
pub type VisibilityNotifyEvent = base::Event<visibility_notify_event>;
/** Opcode for xcb_create_notify. */
pub static XCB_CREATE_NOTIFY : u8 = 16;
pub type CreateNotifyEvent = base::Event<create_notify_event>;
/** Opcode for xcb_destroy_notify. */
pub static XCB_DESTROY_NOTIFY : u8 = 17;
pub type DestroyNotifyEvent = base::Event<destroy_notify_event>;
/** Opcode for xcb_unmap_notify. */
pub static XCB_UNMAP_NOTIFY : u8 = 18;
pub type UnmapNotifyEvent = base::Event<unmap_notify_event>;
/** Opcode for xcb_map_notify. */
pub static XCB_MAP_NOTIFY : u8 = 19;
pub type MapNotifyEvent = base::Event<map_notify_event>;
/** Opcode for xcb_map_request. */
pub static XCB_MAP_REQUEST : u8 = 20;
pub type MapRequestEvent = base::Event<map_request_event>;
/** Opcode for xcb_reparent_notify. */
pub static XCB_REPARENT_NOTIFY : u8 = 21;
pub type ReparentNotifyEvent = base::Event<reparent_notify_event>;
/** Opcode for xcb_configure_notify. */
pub static XCB_CONFIGURE_NOTIFY : u8 = 22;
pub type ConfigureNotifyEvent = base::Event<configure_notify_event>;
/** Opcode for xcb_configure_request. */
pub static XCB_CONFIGURE_REQUEST : u8 = 23;
pub type ConfigureRequestEvent = base::Event<configure_request_event>;
/** Opcode for xcb_gravity_notify. */
pub static XCB_GRAVITY_NOTIFY : u8 = 24;
pub type GravityNotifyEvent = base::Event<gravity_notify_event>;
/** Opcode for xcb_resize_request. */
pub static XCB_RESIZE_REQUEST : u8 = 25;
pub type ResizeRequestEvent = base::Event<resize_request_event>;

pub type place = c_uint;//{
    
/** The window is now on top of all siblings. */
    pub static XCB_PLACE_ON_TOP : place = 0;
    
/** The window is now below all siblings. */
    pub static XCB_PLACE_ON_BOTTOM : place = 1;
//}
/** Opcode for xcb_circulate_notify. */
pub static XCB_CIRCULATE_NOTIFY : u8 = 26;
pub type CirculateNotifyEvent = base::Event<circulate_notify_event>;
/** Opcode for xcb_circulate_request. */
pub static XCB_CIRCULATE_REQUEST : u8 = 27;
pub type CirculateRequestEvent = base::Event<circulate_request_event>;

pub type property = c_uint;//{
    pub static XCB_PROPERTY_NEW_VALUE : property = 0;
    pub static XCB_PROPERTY_DELETE : property = 1;
//}
/** Opcode for xcb_property_notify. */
pub static XCB_PROPERTY_NOTIFY : u8 = 28;
pub type PropertyNotifyEvent = base::Event<property_notify_event>;
/** Opcode for xcb_selection_clear. */
pub static XCB_SELECTION_CLEAR : u8 = 29;
pub type SelectionClearEvent = base::Event<selection_clear_event>;

pub type time = c_uint;//{
    pub static XCB_TIME_CURRENT_TIME : time = 0;
//}

pub type atom_enum = c_uint;//{
    pub static XCB_ATOM_NONE : atom_enum = 0;
    pub static XCB_ATOM_ANY : atom_enum = 0;
    pub static XCB_ATOM_PRIMARY : atom_enum = 1;
    pub static XCB_ATOM_SECONDARY : atom_enum = 2;
    pub static XCB_ATOM_ARC : atom_enum = 3;
    pub static XCB_ATOM_ATOM : atom_enum = 4;
    pub static XCB_ATOM_BITMAP : atom_enum = 5;
    pub static XCB_ATOM_CARDINAL : atom_enum = 6;
    pub static XCB_ATOM_COLORMAP : atom_enum = 7;
    pub static XCB_ATOM_CURSOR : atom_enum = 8;
    pub static XCB_ATOM_CUT_BUFFER0 : atom_enum = 9;
    pub static XCB_ATOM_CUT_BUFFER1 : atom_enum = 10;
    pub static XCB_ATOM_CUT_BUFFER2 : atom_enum = 11;
    pub static XCB_ATOM_CUT_BUFFER3 : atom_enum = 12;
    pub static XCB_ATOM_CUT_BUFFER4 : atom_enum = 13;
    pub static XCB_ATOM_CUT_BUFFER5 : atom_enum = 14;
    pub static XCB_ATOM_CUT_BUFFER6 : atom_enum = 15;
    pub static XCB_ATOM_CUT_BUFFER7 : atom_enum = 16;
    pub static XCB_ATOM_DRAWABLE : atom_enum = 17;
    pub static XCB_ATOM_FONT : atom_enum = 18;
    pub static XCB_ATOM_INTEGER : atom_enum = 19;
    pub static XCB_ATOM_PIXMAP : atom_enum = 20;
    pub static XCB_ATOM_POINT : atom_enum = 21;
    pub static XCB_ATOM_RECTANGLE : atom_enum = 22;
    pub static XCB_ATOM_RESOURCE_MANAGER : atom_enum = 23;
    pub static XCB_ATOM_RGB_COLOR_MAP : atom_enum = 24;
    pub static XCB_ATOM_RGB_BEST_MAP : atom_enum = 25;
    pub static XCB_ATOM_RGB_BLUE_MAP : atom_enum = 26;
    pub static XCB_ATOM_RGB_DEFAULT_MAP : atom_enum = 27;
    pub static XCB_ATOM_RGB_GRAY_MAP : atom_enum = 28;
    pub static XCB_ATOM_RGB_GREEN_MAP : atom_enum = 29;
    pub static XCB_ATOM_RGB_RED_MAP : atom_enum = 30;
    pub static XCB_ATOM_STRING : atom_enum = 31;
    pub static XCB_ATOM_VISUALID : atom_enum = 32;
    pub static XCB_ATOM_WINDOW : atom_enum = 33;
    pub static XCB_ATOM_WM_COMMAND : atom_enum = 34;
    pub static XCB_ATOM_WM_HINTS : atom_enum = 35;
    pub static XCB_ATOM_WM_CLIENT_MACHINE : atom_enum = 36;
    pub static XCB_ATOM_WM_ICON_NAME : atom_enum = 37;
    pub static XCB_ATOM_WM_ICON_SIZE : atom_enum = 38;
    pub static XCB_ATOM_WM_NAME : atom_enum = 39;
    pub static XCB_ATOM_WM_NORMAL_HINTS : atom_enum = 40;
    pub static XCB_ATOM_WM_SIZE_HINTS : atom_enum = 41;
    pub static XCB_ATOM_WM_ZOOM_HINTS : atom_enum = 42;
    pub static XCB_ATOM_MIN_SPACE : atom_enum = 43;
    pub static XCB_ATOM_NORM_SPACE : atom_enum = 44;
    pub static XCB_ATOM_MAX_SPACE : atom_enum = 45;
    pub static XCB_ATOM_END_SPACE : atom_enum = 46;
    pub static XCB_ATOM_SUPERSCRIPT_X : atom_enum = 47;
    pub static XCB_ATOM_SUPERSCRIPT_Y : atom_enum = 48;
    pub static XCB_ATOM_SUBSCRIPT_X : atom_enum = 49;
    pub static XCB_ATOM_SUBSCRIPT_Y : atom_enum = 50;
    pub static XCB_ATOM_UNDERLINE_POSITION : atom_enum = 51;
    pub static XCB_ATOM_UNDERLINE_THICKNESS : atom_enum = 52;
    pub static XCB_ATOM_STRIKEOUT_ASCENT : atom_enum = 53;
    pub static XCB_ATOM_STRIKEOUT_DESCENT : atom_enum = 54;
    pub static XCB_ATOM_ITALIC_ANGLE : atom_enum = 55;
    pub static XCB_ATOM_X_HEIGHT : atom_enum = 56;
    pub static XCB_ATOM_QUAD_WIDTH : atom_enum = 57;
    pub static XCB_ATOM_WEIGHT : atom_enum = 58;
    pub static XCB_ATOM_POINT_SIZE : atom_enum = 59;
    pub static XCB_ATOM_RESOLUTION : atom_enum = 60;
    pub static XCB_ATOM_COPYRIGHT : atom_enum = 61;
    pub static XCB_ATOM_NOTICE : atom_enum = 62;
    pub static XCB_ATOM_FONT_NAME : atom_enum = 63;
    pub static XCB_ATOM_FAMILY_NAME : atom_enum = 64;
    pub static XCB_ATOM_FULL_NAME : atom_enum = 65;
    pub static XCB_ATOM_CAP_HEIGHT : atom_enum = 66;
    pub static XCB_ATOM_WM_CLASS : atom_enum = 67;
    pub static XCB_ATOM_WM_TRANSIENT_FOR : atom_enum = 68;
//}
/** Opcode for xcb_selection_request. */
pub static XCB_SELECTION_REQUEST : u8 = 30;
pub type SelectionRequestEvent = base::Event<selection_request_event>;
/** Opcode for xcb_selection_notify. */
pub static XCB_SELECTION_NOTIFY : u8 = 31;
pub type SelectionNotifyEvent = base::Event<selection_notify_event>;

pub type colormap_state = c_uint;//{
    
/** The colormap was uninstalled. */
    pub static XCB_COLORMAP_STATE_UNINSTALLED : colormap_state = 0;
    
/** The colormap was installed. */
    pub static XCB_COLORMAP_STATE_INSTALLED : colormap_state = 1;
//}

pub type colormap_enum = c_uint;//{
    pub static XCB_COLORMAP_NONE : colormap_enum = 0;
//}
/** Opcode for xcb_colormap_notify. */
pub static XCB_COLORMAP_NOTIFY : u8 = 32;
pub type ColormapNotifyEvent = base::Event<colormap_notify_event>;
pub type ClientMessageDataIterator = client_message_data_iterator;

/** Opcode for xcb_client_message. */
pub static XCB_CLIENT_MESSAGE : u8 = 33;
pub type ClientMessageEvent = base::Event<client_message_event>;

pub type mapping = c_uint;//{
    pub static XCB_MAPPING_MODIFIER : mapping = 0;
    pub static XCB_MAPPING_KEYBOARD : mapping = 1;
    pub static XCB_MAPPING_POINTER : mapping = 2;
//}
/** Opcode for xcb_mapping_notify. */
pub static XCB_MAPPING_NOTIFY : u8 = 34;
pub type MappingNotifyEvent = base::Event<mapping_notify_event>;
/** Opcode for xcb_request. */
pub static XCB_REQUEST : u8 = 1;
pub type RequestError = base::Error<request_error>;
/** Opcode for xcb_value. */
pub static XCB_VALUE : u8 = 2;
pub type ValueError = base::Error<value_error>;
/** Opcode for xcb_window. */
pub static XCB_WINDOW : u8 = 3;
pub type WindowError = base::Error<window_error>;
/** Opcode for xcb_pixmap. */
pub static XCB_PIXMAP : u8 = 4;
pub type PixmapError = base::Error<pixmap_error>;
/** Opcode for xcb_atom. */
pub static XCB_ATOM : u8 = 5;
pub type AtomError = base::Error<atom_error>;
/** Opcode for xcb_cursor. */
pub static XCB_CURSOR : u8 = 6;
pub type CursorError = base::Error<cursor_error>;
/** Opcode for xcb_font. */
pub static XCB_FONT : u8 = 7;
pub type FontError = base::Error<font_error>;
/** Opcode for xcb_match. */
pub static XCB_MATCH : u8 = 8;
pub type MatchError = base::Error<match_error>;
/** Opcode for xcb_drawable. */
pub static XCB_DRAWABLE : u8 = 9;
pub type DrawableError = base::Error<drawable_error>;
/** Opcode for xcb_access. */
pub static XCB_ACCESS : u8 = 10;
pub type AccessError = base::Error<access_error>;
/** Opcode for xcb_alloc. */
pub static XCB_ALLOC : u8 = 11;
pub type AllocError = base::Error<alloc_error>;
/** Opcode for xcb_colormap. */
pub static XCB_COLORMAP : u8 = 12;
pub type ColormapError = base::Error<colormap_error>;
/** Opcode for xcb_g_context. */
pub static XCB_G_CONTEXT : u8 = 13;
pub type GContextError = base::Error<g_context_error>;
/** Opcode for xcb_id_choice. */
pub static XCB_ID_CHOICE : u8 = 14;
pub type IdChoiceError = base::Error<id_choice_error>;
/** Opcode for xcb_name. */
pub static XCB_NAME : u8 = 15;
pub type NameError = base::Error<name_error>;
/** Opcode for xcb_length. */
pub static XCB_LENGTH : u8 = 16;
pub type LengthError = base::Error<length_error>;
/** Opcode for xcb_implementation. */
pub static XCB_IMPLEMENTATION : u8 = 17;
pub type ImplementationError = base::Error<implementation_error>;

pub type window_class = c_uint;//{
    pub static XCB_WINDOW_CLASS_COPY_FROM_PARENT : window_class = 0;
    pub static XCB_WINDOW_CLASS_INPUT_OUTPUT : window_class = 1;
    pub static XCB_WINDOW_CLASS_INPUT_ONLY : window_class = 2;
//}

pub type cw = c_uint;//{
    
/** Overrides the default background-pixmap. The background pixmap and window must
have the same root and same depth. Any size pixmap can be used, although some
sizes may be faster than others.

If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
The server may fill the contents with the previous screen contents or with
contents of its own choosing.

If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
used, but the window must have the same depth as the parent (or a Match error
results).   The parent's background is tracked, and the current version is
used each time the window background is required. */
    pub static XCB_CW_BACK_PIXMAP : cw = 1;
    
/** Overrides `BackPixmap`. A pixmap of undefined size filled with the specified
background pixel is used for the background. Range-checking is not performed,
the background pixel is truncated to the appropriate number of bits. */
    pub static XCB_CW_BACK_PIXEL : cw = 2;
    
/** Overrides the default border-pixmap. The border pixmap and window must have the
same root and the same depth. Any size pixmap can be used, although some sizes
may be faster than others.

The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is
copied (subsequent changes to the parent's border attribute do not affect the
child), but the window must have the same depth as the parent. */
    pub static XCB_CW_BORDER_PIXMAP : cw = 4;
    
/** Overrides `BorderPixmap`. A pixmap of undefined size filled with the specified
border pixel is used for the border. Range checking is not performed on the
border-pixel value, it is truncated to the appropriate number of bits. */
    pub static XCB_CW_BORDER_PIXEL : cw = 8;
    
/** Defines which region of the window should be retained if the window is resized. */
    pub static XCB_CW_BIT_GRAVITY : cw = 16;
    
/** Defines how the window should be repositioned if the parent is resized (see
`ConfigureWindow`). */
    pub static XCB_CW_WIN_GRAVITY : cw = 32;
    
/** A backing-store of `WhenMapped` advises the server that maintaining contents of
obscured regions when the window is mapped would be beneficial. A backing-store
of `Always` advises the server that maintaining contents even when the window
is unmapped would be beneficial. In this case, the server may generate an
exposure event when the window is created. A value of `NotUseful` advises the
server that maintaining contents is unnecessary, although a server may still
choose to maintain contents while the window is mapped. Note that if the server
maintains contents, then the server should maintain complete contents not just
the region within the parent boundaries, even if the window is larger than its
parent. While the server maintains contents, exposure events will not normally
be generated, but the server may stop maintaining contents at any time. */
    pub static XCB_CW_BACKING_STORE : cw = 64;
    
/** The backing-planes indicates (with bits set to 1) which bit planes of the
window hold dynamic data that must be preserved in backing-stores and during
save-unders. */
    pub static XCB_CW_BACKING_PLANES : cw = 128;
    
/** The backing-pixel specifies what value to use in planes not covered by
backing-planes. The server is free to save only the specified bit planes in the
backing-store or save-under and regenerate the remaining planes with the
specified pixel value. Any bits beyond the specified depth of the window in
these values are simply ignored. */
    pub static XCB_CW_BACKING_PIXEL : cw = 256;
    
/** The override-redirect specifies whether map and configure requests on this
window should override a SubstructureRedirect on the parent, typically to
inform a window manager not to tamper with the window. */
    pub static XCB_CW_OVERRIDE_REDIRECT : cw = 512;
    
/** If 1, the server is advised that when this window is mapped, saving the
contents of windows it obscures would be beneficial. */
    pub static XCB_CW_SAVE_UNDER : cw = 1024;
    
/** The event-mask defines which events the client is interested in for this window
(or for some event types, inferiors of the window). */
    pub static XCB_CW_EVENT_MASK : cw = 2048;
    
/** The do-not-propagate-mask defines which events should not be propagated to
ancestor windows when no client has the event type selected in this window. */
    pub static XCB_CW_DONT_PROPAGATE : cw = 4096;
    
/** The colormap specifies the colormap that best reflects the true colors of the window. Servers
capable of supporting multiple hardware colormaps may use this information, and window man-
agers may use it for InstallColormap requests. The colormap must have the same visual type
and root as the window (or a Match error results). If CopyFromParent is specified, the parent's
colormap is copied (subsequent changes to the parent's colormap attribute do not affect the child).
However, the window must have the same visual type as the parent (or a Match error results),
and the parent must not have a colormap of None (or a Match error results). For an explanation
of None, see FreeColormap request. The colormap is copied by sharing the colormap object
between the child and the parent, not by making a complete copy of the colormap contents. */
    pub static XCB_CW_COLORMAP : cw = 8192;
    
/** If a cursor is specified, it will be used whenever the pointer is in the window. If None is speci-
fied, the parent's cursor will be used when the pointer is in the window, and any change in the
parent's cursor will cause an immediate change in the displayed cursor. */
    pub static XCB_CW_CURSOR : cw = 16384;
//}

pub type back_pixmap = c_uint;//{
    pub static XCB_BACK_PIXMAP_NONE : back_pixmap = 0;
    pub static XCB_BACK_PIXMAP_PARENT_RELATIVE : back_pixmap = 1;
//}

pub type gravity = c_uint;//{
    pub static XCB_GRAVITY_BIT_FORGET : gravity = 0;
    pub static XCB_GRAVITY_WIN_UNMAP : gravity = 0;
    pub static XCB_GRAVITY_NORTH_WEST : gravity = 1;
    pub static XCB_GRAVITY_NORTH : gravity = 2;
    pub static XCB_GRAVITY_NORTH_EAST : gravity = 3;
    pub static XCB_GRAVITY_WEST : gravity = 4;
    pub static XCB_GRAVITY_CENTER : gravity = 5;
    pub static XCB_GRAVITY_EAST : gravity = 6;
    pub static XCB_GRAVITY_SOUTH_WEST : gravity = 7;
    pub static XCB_GRAVITY_SOUTH : gravity = 8;
    pub static XCB_GRAVITY_SOUTH_EAST : gravity = 9;
    pub static XCB_GRAVITY_STATIC : gravity = 10;
//}
/** Opcode for xcb_create_window. */
pub static XCB_CREATE_WINDOW : u8 = 1;
/** Opcode for xcb_change_window_attributes. */
pub static XCB_CHANGE_WINDOW_ATTRIBUTES : u8 = 2;

pub type map_state = c_uint;//{
    pub static XCB_MAP_STATE_UNMAPPED : map_state = 0;
    pub static XCB_MAP_STATE_UNVIEWABLE : map_state = 1;
    pub static XCB_MAP_STATE_VIEWABLE : map_state = 2;
//}
pub type GetWindowAttributesCookie<'self> = base::Cookie<'self, get_window_attributes_cookie>;

/** Opcode for xcb_get_window_attributes. */
pub static XCB_GET_WINDOW_ATTRIBUTES : u8 = 3;
pub type GetWindowAttributesReply = base::Reply<get_window_attributes_reply>;
/** Opcode for xcb_destroy_window. */
pub static XCB_DESTROY_WINDOW : u8 = 4;
/** Opcode for xcb_destroy_subwindows. */
pub static XCB_DESTROY_SUBWINDOWS : u8 = 5;

pub type set_mode = c_uint;//{
    pub static XCB_SET_MODE_INSERT : set_mode = 0;
    pub static XCB_SET_MODE_DELETE : set_mode = 1;
//}
/** Opcode for xcb_change_save_set. */
pub static XCB_CHANGE_SAVE_SET : u8 = 6;
/** Opcode for xcb_reparent_window. */
pub static XCB_REPARENT_WINDOW : u8 = 7;
/** Opcode for xcb_map_window. */
pub static XCB_MAP_WINDOW : u8 = 8;
/** Opcode for xcb_map_subwindows. */
pub static XCB_MAP_SUBWINDOWS : u8 = 9;
/** Opcode for xcb_unmap_window. */
pub static XCB_UNMAP_WINDOW : u8 = 10;
/** Opcode for xcb_unmap_subwindows. */
pub static XCB_UNMAP_SUBWINDOWS : u8 = 11;

pub type config_window = c_uint;//{
    pub static XCB_CONFIG_WINDOW_X : config_window = 1;
    pub static XCB_CONFIG_WINDOW_Y : config_window = 2;
    pub static XCB_CONFIG_WINDOW_WIDTH : config_window = 4;
    pub static XCB_CONFIG_WINDOW_HEIGHT : config_window = 8;
    pub static XCB_CONFIG_WINDOW_BORDER_WIDTH : config_window = 16;
    pub static XCB_CONFIG_WINDOW_SIBLING : config_window = 32;
    pub static XCB_CONFIG_WINDOW_STACK_MODE : config_window = 64;
//}

pub type stack_mode = c_uint;//{
    pub static XCB_STACK_MODE_ABOVE : stack_mode = 0;
    pub static XCB_STACK_MODE_BELOW : stack_mode = 1;
    pub static XCB_STACK_MODE_TOP_IF : stack_mode = 2;
    pub static XCB_STACK_MODE_BOTTOM_IF : stack_mode = 3;
    pub static XCB_STACK_MODE_OPPOSITE : stack_mode = 4;
//}
/** Opcode for xcb_configure_window. */
pub static XCB_CONFIGURE_WINDOW : u8 = 12;

pub type circulate = c_uint;//{
    pub static XCB_CIRCULATE_RAISE_LOWEST : circulate = 0;
    pub static XCB_CIRCULATE_LOWER_HIGHEST : circulate = 1;
//}
/** Opcode for xcb_circulate_window. */
pub static XCB_CIRCULATE_WINDOW : u8 = 13;
pub type GetGeometryCookie<'self> = base::Cookie<'self, get_geometry_cookie>;

/** Opcode for xcb_get_geometry. */
pub static XCB_GET_GEOMETRY : u8 = 14;
pub type GetGeometryReply = base::Reply<get_geometry_reply>;
pub type QueryTreeCookie<'self> = base::Cookie<'self, query_tree_cookie>;

/** Opcode for xcb_query_tree. */
pub static XCB_QUERY_TREE : u8 = 15;
pub type InternAtomCookie<'self> = base::Cookie<'self, intern_atom_cookie>;

/** Opcode for xcb_intern_atom. */
pub static XCB_INTERN_ATOM : u8 = 16;
pub type InternAtomReply = base::Reply<intern_atom_reply>;
pub type GetAtomNameCookie<'self> = base::Cookie<'self, get_atom_name_cookie>;

/** Opcode for xcb_get_atom_name. */
pub static XCB_GET_ATOM_NAME : u8 = 17;

pub type prop_mode = c_uint;//{
    
/** Discard the previous property value and store the new data. */
    pub static XCB_PROP_MODE_REPLACE : prop_mode = 0;
    
/** Insert the new data before the beginning of existing data. The `format` must
match existing property value. If the property is undefined, it is treated as
defined with the correct type and format with zero-length data. */
    pub static XCB_PROP_MODE_PREPEND : prop_mode = 1;
    
/** Insert the new data after the beginning of existing data. The `format` must
match existing property value. If the property is undefined, it is treated as
defined with the correct type and format with zero-length data. */
    pub static XCB_PROP_MODE_APPEND : prop_mode = 2;
//}
/** Opcode for xcb_change_property. */
pub static XCB_CHANGE_PROPERTY : u8 = 18;
/** Opcode for xcb_delete_property. */
pub static XCB_DELETE_PROPERTY : u8 = 19;

pub type get_property_type = c_uint;//{
    pub static XCB_GET_PROPERTY_TYPE_ANY : get_property_type = 0;
//}
pub type GetPropertyCookie<'self> = base::Cookie<'self, get_property_cookie>;

/** Opcode for xcb_get_property. */
pub static XCB_GET_PROPERTY : u8 = 20;
pub type ListPropertiesCookie<'self> = base::Cookie<'self, list_properties_cookie>;

/** Opcode for xcb_list_properties. */
pub static XCB_LIST_PROPERTIES : u8 = 21;
/** Opcode for xcb_set_selection_owner. */
pub static XCB_SET_SELECTION_OWNER : u8 = 22;
pub type GetSelectionOwnerCookie<'self> = base::Cookie<'self, get_selection_owner_cookie>;

/** Opcode for xcb_get_selection_owner. */
pub static XCB_GET_SELECTION_OWNER : u8 = 23;
pub type GetSelectionOwnerReply = base::Reply<get_selection_owner_reply>;
/** Opcode for xcb_convert_selection. */
pub static XCB_CONVERT_SELECTION : u8 = 24;

pub type send_event_dest = c_uint;//{
    pub static XCB_SEND_EVENT_DEST_POINTER_WINDOW : send_event_dest = 0;
    pub static XCB_SEND_EVENT_DEST_ITEM_FOCUS : send_event_dest = 1;
//}
/** Opcode for xcb_send_event. */
pub static XCB_SEND_EVENT : u8 = 25;

pub type grab_mode = c_uint;//{
    
/** The state of the keyboard appears to freeze: No further keyboard events are
generated by the server until the grabbing client issues a releasing
`AllowEvents` request or until the keyboard grab is released. */
    pub static XCB_GRAB_MODE_SYNC : grab_mode = 0;
    
/** Keyboard event processing continues normally. */
    pub static XCB_GRAB_MODE_ASYNC : grab_mode = 1;
//}

pub type grab_status = c_uint;//{
    pub static XCB_GRAB_STATUS_SUCCESS : grab_status = 0;
    pub static XCB_GRAB_STATUS_ALREADY_GRABBED : grab_status = 1;
    pub static XCB_GRAB_STATUS_INVALID_TIME : grab_status = 2;
    pub static XCB_GRAB_STATUS_NOT_VIEWABLE : grab_status = 3;
    pub static XCB_GRAB_STATUS_FROZEN : grab_status = 4;
//}

pub type cursor_enum = c_uint;//{
    pub static XCB_CURSOR_NONE : cursor_enum = 0;
//}
pub type GrabPointerCookie<'self> = base::Cookie<'self, grab_pointer_cookie>;

/** Opcode for xcb_grab_pointer. */
pub static XCB_GRAB_POINTER : u8 = 26;
pub type GrabPointerReply = base::Reply<grab_pointer_reply>;
/** Opcode for xcb_ungrab_pointer. */
pub static XCB_UNGRAB_POINTER : u8 = 27;

pub type button_index = c_uint;//{
    
/** Any of the following (or none): */
    pub static XCB_BUTTON_INDEX_ANY : button_index = 0;
    
/** The left mouse button. */
    pub static XCB_BUTTON_INDEX_1 : button_index = 1;
    
/** The right mouse button. */
    pub static XCB_BUTTON_INDEX_2 : button_index = 2;
    
/** The middle mouse button. */
    pub static XCB_BUTTON_INDEX_3 : button_index = 3;
    
/** Scroll wheel. TODO: direction? */
    pub static XCB_BUTTON_INDEX_4 : button_index = 4;
    
/** Scroll wheel. TODO: direction? */
    pub static XCB_BUTTON_INDEX_5 : button_index = 5;
//}
/** Opcode for xcb_grab_button. */
pub static XCB_GRAB_BUTTON : u8 = 28;
/** Opcode for xcb_ungrab_button. */
pub static XCB_UNGRAB_BUTTON : u8 = 29;
/** Opcode for xcb_change_active_pointer_grab. */
pub static XCB_CHANGE_ACTIVE_POINTER_GRAB : u8 = 30;
pub type GrabKeyboardCookie<'self> = base::Cookie<'self, grab_keyboard_cookie>;

/** Opcode for xcb_grab_keyboard. */
pub static XCB_GRAB_KEYBOARD : u8 = 31;
pub type GrabKeyboardReply = base::Reply<grab_keyboard_reply>;
/** Opcode for xcb_ungrab_keyboard. */
pub static XCB_UNGRAB_KEYBOARD : u8 = 32;

pub type grab = c_uint;//{
    pub static XCB_GRAB_ANY : grab = 0;
//}
/** Opcode for xcb_grab_key. */
pub static XCB_GRAB_KEY : u8 = 33;
/** Opcode for xcb_ungrab_key. */
pub static XCB_UNGRAB_KEY : u8 = 34;

pub type allow = c_uint;//{
    
/** For AsyncPointer, if the pointer is frozen by the client, pointer event
processing continues normally. If the pointer is frozen twice by the client on
behalf of two separate grabs, AsyncPointer thaws for both. AsyncPointer has no
effect if the pointer is not frozen by the client, but the pointer need not be
grabbed by the client.

TODO: rewrite this in more understandable terms. */
    pub static XCB_ALLOW_ASYNC_POINTER : allow = 0;
    
/** For SyncPointer, if the pointer is frozen and actively grabbed by the client,
pointer event processing continues normally until the next ButtonPress or
ButtonRelease event is reported to the client, at which time the pointer again
appears to freeze. However, if the reported event causes the pointer grab to be
released, then the pointer does not freeze. SyncPointer has no effect if the
pointer is not frozen by the client or if the pointer is not grabbed by the
client. */
    pub static XCB_ALLOW_SYNC_POINTER : allow = 1;
    
/** For ReplayPointer, if the pointer is actively grabbed by the client and is
frozen as the result of an event having been sent to the client (either from
the activation of a GrabButton or from a previous AllowEvents with mode
SyncPointer but not from a GrabPointer), then the pointer grab is released and
that event is completely reprocessed, this time ignoring any passive grabs at
or above (towards the root) the grab-window of the grab just released. The
request has no effect if the pointer is not grabbed by the client or if the
pointer is not frozen as the result of an event. */
    pub static XCB_ALLOW_REPLAY_POINTER : allow = 2;
    
/** For AsyncKeyboard, if the keyboard is frozen by the client, keyboard event
processing continues normally. If the keyboard is frozen twice by the client on
behalf of two separate grabs, AsyncKeyboard thaws for both. AsyncKeyboard has
no effect if the keyboard is not frozen by the client, but the keyboard need
not be grabbed by the client. */
    pub static XCB_ALLOW_ASYNC_KEYBOARD : allow = 3;
    
/** For SyncKeyboard, if the keyboard is frozen and actively grabbed by the client,
keyboard event processing continues normally until the next KeyPress or
KeyRelease event is reported to the client, at which time the keyboard again
appears to freeze. However, if the reported event causes the keyboard grab to
be released, then the keyboard does not freeze. SyncKeyboard has no effect if
the keyboard is not frozen by the client or if the keyboard is not grabbed by
the client. */
    pub static XCB_ALLOW_SYNC_KEYBOARD : allow = 4;
    
/** For ReplayKeyboard, if the keyboard is actively grabbed by the client and is
frozen as the result of an event having been sent to the client (either from
the activation of a GrabKey or from a previous AllowEvents with mode
SyncKeyboard but not from a GrabKeyboard), then the keyboard grab is released
and that event is completely reprocessed, this time ignoring any passive grabs
at or above (towards the root) the grab-window of the grab just released. The
request has no effect if the keyboard is not grabbed by the client or if the
keyboard is not frozen as the result of an event. */
    pub static XCB_ALLOW_REPLAY_KEYBOARD : allow = 5;
    
/** For AsyncBoth, if the pointer and the keyboard are frozen by the client, event
processing for both devices continues normally. If a device is frozen twice by
the client on behalf of two separate grabs, AsyncBoth thaws for both. AsyncBoth
has no effect unless both pointer and keyboard are frozen by the client. */
    pub static XCB_ALLOW_ASYNC_BOTH : allow = 6;
    
/** For SyncBoth, if both pointer and keyboard are frozen by the client, event
processing (for both devices) continues normally until the next ButtonPress,
ButtonRelease, KeyPress, or KeyRelease event is reported to the client for a
grabbed device (button event for the pointer, key event for the keyboard), at
which time the devices again appear to freeze. However, if the reported event
causes the grab to be released, then the devices do not freeze (but if the
other device is still grabbed, then a subsequent event for it will still cause
both devices to freeze). SyncBoth has no effect unless both pointer and
keyboard are frozen by the client. If the pointer or keyboard is frozen twice
by the client on behalf of two separate grabs, SyncBoth thaws for both (but a
subsequent freeze for SyncBoth will only freeze each device once). */
    pub static XCB_ALLOW_SYNC_BOTH : allow = 7;
//}
/** Opcode for xcb_allow_events. */
pub static XCB_ALLOW_EVENTS : u8 = 35;
/** Opcode for xcb_grab_server. */
pub static XCB_GRAB_SERVER : u8 = 36;
/** Opcode for xcb_ungrab_server. */
pub static XCB_UNGRAB_SERVER : u8 = 37;
pub type QueryPointerCookie<'self> = base::Cookie<'self, query_pointer_cookie>;

/** Opcode for xcb_query_pointer. */
pub static XCB_QUERY_POINTER : u8 = 38;
pub type QueryPointerReply = base::Reply<query_pointer_reply>;
pub type TimecoordIterator = timecoord_iterator;

pub type GetMotionEventsCookie<'self> = base::Cookie<'self, get_motion_events_cookie>;

/** Opcode for xcb_get_motion_events. */
pub static XCB_GET_MOTION_EVENTS : u8 = 39;
pub type TranslateCoordinatesCookie<'self> = base::Cookie<'self, translate_coordinates_cookie>;

/** Opcode for xcb_translate_coordinates. */
pub static XCB_TRANSLATE_COORDINATES : u8 = 40;
pub type TranslateCoordinatesReply = base::Reply<translate_coordinates_reply>;
/** Opcode for xcb_warp_pointer. */
pub static XCB_WARP_POINTER : u8 = 41;

pub type input_focus = c_uint;//{
    
/** The focus reverts to `XCB_NONE`, so no window will have the input focus. */
    pub static XCB_INPUT_FOCUS_NONE : input_focus = 0;
    
/** The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
FocusIn and FocusOut events are generated, but the last-focus-change time is
not changed. */
    pub static XCB_INPUT_FOCUS_POINTER_ROOT : input_focus = 1;
    
/** The focus reverts to the parent (or closest viewable ancestor) and the new
revert_to value is `XCB_INPUT_FOCUS_NONE`. */
    pub static XCB_INPUT_FOCUS_PARENT : input_focus = 2;
    
/** NOT YET DOCUMENTED. Only relevant for the xinput extension. */
    pub static XCB_INPUT_FOCUS_FOLLOW_KEYBOARD : input_focus = 3;
//}
/** Opcode for xcb_set_input_focus. */
pub static XCB_SET_INPUT_FOCUS : u8 = 42;
pub type GetInputFocusCookie<'self> = base::Cookie<'self, get_input_focus_cookie>;

/** Opcode for xcb_get_input_focus. */
pub static XCB_GET_INPUT_FOCUS : u8 = 43;
pub type GetInputFocusReply = base::Reply<get_input_focus_reply>;
pub type QueryKeymapCookie<'self> = base::Cookie<'self, query_keymap_cookie>;

/** Opcode for xcb_query_keymap. */
pub static XCB_QUERY_KEYMAP : u8 = 44;
pub type QueryKeymapReply = base::Reply<query_keymap_reply>;
/** Opcode for xcb_open_font. */
pub static XCB_OPEN_FONT : u8 = 45;
/** Opcode for xcb_close_font. */
pub static XCB_CLOSE_FONT : u8 = 46;

pub type font_draw = c_uint;//{
    pub static XCB_FONT_DRAW_LEFT_TO_RIGHT : font_draw = 0;
    pub static XCB_FONT_DRAW_RIGHT_TO_LEFT : font_draw = 1;
//}
pub type Fontprop = base::Struct<fontprop>;

pub type FontpropIterator = fontprop_iterator;

pub type CharinfoIterator = charinfo_iterator;

pub type QueryFontCookie<'self> = base::Cookie<'self, query_font_cookie>;

/** Opcode for xcb_query_font. */
pub static XCB_QUERY_FONT : u8 = 47;
pub type QueryTextExtentsCookie<'self> = base::Cookie<'self, query_text_extents_cookie>;

/** Opcode for xcb_query_text_extents. */
pub static XCB_QUERY_TEXT_EXTENTS : u8 = 48;
pub type QueryTextExtentsReply = base::Reply<query_text_extents_reply>;
pub type StrIterator = str_iterator;

pub type ListFontsCookie<'self> = base::Cookie<'self, list_fonts_cookie>;

/** Opcode for xcb_list_fonts. */
pub static XCB_LIST_FONTS : u8 = 49;
pub type ListFontsReply = base::Reply<list_fonts_reply>;
pub type ListFontsWithInfoCookie<'self> = base::Cookie<'self, list_fonts_with_info_cookie>;

/** Opcode for xcb_list_fonts_with_info. */
pub static XCB_LIST_FONTS_WITH_INFO : u8 = 50;
pub type ListFontsWithInfoReply = base::Reply<list_fonts_with_info_reply>;
/** Opcode for xcb_set_font_path. */
pub static XCB_SET_FONT_PATH : u8 = 51;
pub type GetFontPathCookie<'self> = base::Cookie<'self, get_font_path_cookie>;

/** Opcode for xcb_get_font_path. */
pub static XCB_GET_FONT_PATH : u8 = 52;
/** Opcode for xcb_create_pixmap. */
pub static XCB_CREATE_PIXMAP : u8 = 53;
/** Opcode for xcb_free_pixmap. */
pub static XCB_FREE_PIXMAP : u8 = 54;

pub type gc = c_uint;//{
    
/** TODO: Refer to GX */
    pub static XCB_GC_FUNCTION : gc = 1;
    
/** In graphics operations, given a source and destination pixel, the result is
computed bitwise on corresponding bits of the pixels; that is, a Boolean
operation is performed in each bit plane. The plane-mask restricts the
operation to a subset of planes, so the result is:

        ((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask)) */
    pub static XCB_GC_PLANE_MASK : gc = 2;
    
/** Foreground colorpixel. */
    pub static XCB_GC_FOREGROUND : gc = 4;
    
/** Background colorpixel. */
    pub static XCB_GC_BACKGROUND : gc = 8;
    
/** The line-width is measured in pixels and can be greater than or equal to one, a wide line, or the
special value zero, a thin line. */
    pub static XCB_GC_LINE_WIDTH : gc = 16;
    
/** The line-style defines which sections of a line are drawn:
Solid                The full path of the line is drawn.
DoubleDash           The full path of the line is drawn, but the even dashes are filled differently
                     than the odd dashes (see fill-style), with Butt cap-style used where even and
                     odd dashes meet.
OnOffDash            Only the even dashes are drawn, and cap-style applies to all internal ends of
                     the individual dashes (except NotLast is treated as Butt). */
    pub static XCB_GC_LINE_STYLE : gc = 32;
    
/** The cap-style defines how the endpoints of a path are drawn:
NotLast    The result is equivalent to Butt, except that for a line-width of zero the final
           endpoint is not drawn.
Butt       The result is square at the endpoint (perpendicular to the slope of the line)
           with no projection beyond.
Round      The result is a circular arc with its diameter equal to the line-width, centered
           on the endpoint; it is equivalent to Butt for line-width zero.
Projecting The result is square at the end, but the path continues beyond the endpoint for
           a distance equal to half the line-width; it is equivalent to Butt for line-width
           zero. */
    pub static XCB_GC_CAP_STYLE : gc = 64;
    
/** The join-style defines how corners are drawn for wide lines:
Miter               The outer edges of the two lines extend to meet at an angle. However, if the
                    angle is less than 11 degrees, a Bevel join-style is used instead.
Round               The result is a circular arc with a diameter equal to the line-width, centered
                    on the joinpoint.
Bevel               The result is Butt endpoint styles, and then the triangular notch is filled. */
    pub static XCB_GC_JOIN_STYLE : gc = 128;
    
/** The fill-style defines the contents of the source for line, text, and fill requests. For all text and fill
requests (for example, PolyText8, PolyText16, PolyFillRectangle, FillPoly, and PolyFillArc)
as well as for line requests with line-style Solid, (for example, PolyLine, PolySegment,
PolyRectangle, PolyArc) and for the even dashes for line requests with line-style OnOffDash
or DoubleDash:
Solid                     Foreground
Tiled                     Tile
OpaqueStippled            A tile with the same width and height as stipple but with background
                          everywhere stipple has a zero and with foreground everywhere stipple
                          has a one
Stippled                  Foreground masked by stipple
For the odd dashes for line requests with line-style DoubleDash:
Solid                     Background
Tiled                     Same as for even dashes
OpaqueStippled            Same as for even dashes
Stippled                  Background masked by stipple */
    pub static XCB_GC_FILL_STYLE : gc = 256;
    
/**  */
    pub static XCB_GC_FILL_RULE : gc = 512;
    
/** The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
origin of whatever destination drawable is specified in a graphics request.
The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
The stipple pixmap must have depth one and must have the same root as the gcontext (or a
Match error results). For fill-style Stippled (but not fill-style
OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
additional clip mask to be ANDed with the clip-mask.
Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
others. */
    pub static XCB_GC_TILE : gc = 1024;
    
/** The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
origin of whatever destination drawable is specified in a graphics request.
The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
The stipple pixmap must have depth one and must have the same root as the gcontext (or a
Match error results). For fill-style Stippled (but not fill-style
OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
additional clip mask to be ANDed with the clip-mask.
Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
others. */
    pub static XCB_GC_STIPPLE : gc = 2048;
    
/** TODO */
    pub static XCB_GC_TILE_STIPPLE_ORIGIN_X : gc = 4096;
    
/** TODO */
    pub static XCB_GC_TILE_STIPPLE_ORIGIN_Y : gc = 8192;
    
/** Which font to use for the `ImageText8` and `ImageText16` requests. */
    pub static XCB_GC_FONT : gc = 16384;
    
/** For ClipByChildren, both source and destination windows are additionally
clipped by all viewable InputOutput children. For IncludeInferiors, neither
source nor destination window is
clipped by inferiors. This will result in including subwindow contents in the source and drawing
through subwindow boundaries of the destination. The use of IncludeInferiors with a source or
destination window of one depth with mapped inferiors of differing depth is not illegal, but the
semantics is undefined by the core protocol. */
    pub static XCB_GC_SUBWINDOW_MODE : gc = 32768;
    
/** Whether ExposureEvents should be generated (1) or not (0).

The default is 1. */
    pub static XCB_GC_GRAPHICS_EXPOSURES : gc = 65536;
    
/** TODO */
    pub static XCB_GC_CLIP_ORIGIN_X : gc = 131072;
    
/** TODO */
    pub static XCB_GC_CLIP_ORIGIN_Y : gc = 262144;
    
/** The clip-mask restricts writes to the destination drawable. Only pixels where the clip-mask has
bits set to 1 are drawn. Pixels are not drawn outside the area covered by the clip-mask or where
the clip-mask has bits set to 0. The clip-mask affects all graphics requests, but it does not clip
sources. The clip-mask origin is interpreted relative to the origin of whatever destination drawable is specified in a graphics request. If a pixmap is specified as the clip-mask, it must have
depth 1 and have the same root as the gcontext (or a Match error results). If clip-mask is None,
then pixels are always drawn, regardless of the clip origin. The clip-mask can also be set with the
SetClipRectangles request. */
    pub static XCB_GC_CLIP_MASK : gc = 524288;
    
/** TODO */
    pub static XCB_GC_DASH_OFFSET : gc = 1048576;
    
/** TODO */
    pub static XCB_GC_DASH_LIST : gc = 2097152;
    
/** TODO */
    pub static XCB_GC_ARC_MODE : gc = 4194304;
//}

pub type gx = c_uint;//{
    pub static XCB_GX_CLEAR : gx = 0;
    pub static XCB_GX_AND : gx = 1;
    pub static XCB_GX_AND_REVERSE : gx = 2;
    pub static XCB_GX_COPY : gx = 3;
    pub static XCB_GX_AND_INVERTED : gx = 4;
    pub static XCB_GX_NOOP : gx = 5;
    pub static XCB_GX_XOR : gx = 6;
    pub static XCB_GX_OR : gx = 7;
    pub static XCB_GX_NOR : gx = 8;
    pub static XCB_GX_EQUIV : gx = 9;
    pub static XCB_GX_INVERT : gx = 10;
    pub static XCB_GX_OR_REVERSE : gx = 11;
    pub static XCB_GX_COPY_INVERTED : gx = 12;
    pub static XCB_GX_OR_INVERTED : gx = 13;
    pub static XCB_GX_NAND : gx = 14;
    pub static XCB_GX_SET : gx = 15;
//}

pub type line_style = c_uint;//{
    pub static XCB_LINE_STYLE_SOLID : line_style = 0;
    pub static XCB_LINE_STYLE_ON_OFF_DASH : line_style = 1;
    pub static XCB_LINE_STYLE_DOUBLE_DASH : line_style = 2;
//}

pub type cap_style = c_uint;//{
    pub static XCB_CAP_STYLE_NOT_LAST : cap_style = 0;
    pub static XCB_CAP_STYLE_BUTT : cap_style = 1;
    pub static XCB_CAP_STYLE_ROUND : cap_style = 2;
    pub static XCB_CAP_STYLE_PROJECTING : cap_style = 3;
//}

pub type join_style = c_uint;//{
    pub static XCB_JOIN_STYLE_MITER : join_style = 0;
    pub static XCB_JOIN_STYLE_ROUND : join_style = 1;
    pub static XCB_JOIN_STYLE_BEVEL : join_style = 2;
//}

pub type fill_style = c_uint;//{
    pub static XCB_FILL_STYLE_SOLID : fill_style = 0;
    pub static XCB_FILL_STYLE_TILED : fill_style = 1;
    pub static XCB_FILL_STYLE_STIPPLED : fill_style = 2;
    pub static XCB_FILL_STYLE_OPAQUE_STIPPLED : fill_style = 3;
//}

pub type fill_rule = c_uint;//{
    pub static XCB_FILL_RULE_EVEN_ODD : fill_rule = 0;
    pub static XCB_FILL_RULE_WINDING : fill_rule = 1;
//}

pub type subwindow_mode = c_uint;//{
    pub static XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN : subwindow_mode = 0;
    pub static XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS : subwindow_mode = 1;
//}

pub type arc_mode = c_uint;//{
    pub static XCB_ARC_MODE_CHORD : arc_mode = 0;
    pub static XCB_ARC_MODE_PIE_SLICE : arc_mode = 1;
//}
/** Opcode for xcb_create_gc. */
pub static XCB_CREATE_GC : u8 = 55;
/** Opcode for xcb_change_gc. */
pub static XCB_CHANGE_GC : u8 = 56;
/** Opcode for xcb_copy_gc. */
pub static XCB_COPY_GC : u8 = 57;
/** Opcode for xcb_set_dashes. */
pub static XCB_SET_DASHES : u8 = 58;

pub type clip_ordering = c_uint;//{
    pub static XCB_CLIP_ORDERING_UNSORTED : clip_ordering = 0;
    pub static XCB_CLIP_ORDERING_Y_SORTED : clip_ordering = 1;
    pub static XCB_CLIP_ORDERING_YX_SORTED : clip_ordering = 2;
    pub static XCB_CLIP_ORDERING_YX_BANDED : clip_ordering = 3;
//}
/** Opcode for xcb_set_clip_rectangles. */
pub static XCB_SET_CLIP_RECTANGLES : u8 = 59;
/** Opcode for xcb_free_gc. */
pub static XCB_FREE_GC : u8 = 60;
/** Opcode for xcb_clear_area. */
pub static XCB_CLEAR_AREA : u8 = 61;
/** Opcode for xcb_copy_area. */
pub static XCB_COPY_AREA : u8 = 62;
/** Opcode for xcb_copy_plane. */
pub static XCB_COPY_PLANE : u8 = 63;

pub type coord_mode = c_uint;//{
    
/** Treats all coordinates as relative to the origin. */
    pub static XCB_COORD_MODE_ORIGIN : coord_mode = 0;
    
/** Treats all coordinates after the first as relative to the previous coordinate. */
    pub static XCB_COORD_MODE_PREVIOUS : coord_mode = 1;
//}
/** Opcode for xcb_poly_point. */
pub static XCB_POLY_POINT : u8 = 64;
/** Opcode for xcb_poly_line. */
pub static XCB_POLY_LINE : u8 = 65;
pub type SegmentIterator = segment_iterator;

/** Opcode for xcb_poly_segment. */
pub static XCB_POLY_SEGMENT : u8 = 66;
/** Opcode for xcb_poly_rectangle. */
pub static XCB_POLY_RECTANGLE : u8 = 67;
/** Opcode for xcb_poly_arc. */
pub static XCB_POLY_ARC : u8 = 68;

pub type poly_shape = c_uint;//{
    pub static XCB_POLY_SHAPE_COMPLEX : poly_shape = 0;
    pub static XCB_POLY_SHAPE_NONCONVEX : poly_shape = 1;
    pub static XCB_POLY_SHAPE_CONVEX : poly_shape = 2;
//}
/** Opcode for xcb_fill_poly. */
pub static XCB_FILL_POLY : u8 = 69;
/** Opcode for xcb_poly_fill_rectangle. */
pub static XCB_POLY_FILL_RECTANGLE : u8 = 70;
/** Opcode for xcb_poly_fill_arc. */
pub static XCB_POLY_FILL_ARC : u8 = 71;

pub type image_format = c_uint;//{
    pub static XCB_IMAGE_FORMAT_XY_BITMAP : image_format = 0;
    pub static XCB_IMAGE_FORMAT_XY_PIXMAP : image_format = 1;
    pub static XCB_IMAGE_FORMAT_Z_PIXMAP : image_format = 2;
//}
/** Opcode for xcb_put_image. */
pub static XCB_PUT_IMAGE : u8 = 72;
pub type GetImageCookie<'self> = base::Cookie<'self, get_image_cookie>;

/** Opcode for xcb_get_image. */
pub static XCB_GET_IMAGE : u8 = 73;
/** Opcode for xcb_poly_text_8. */
pub static XCB_POLY_TEXT_8 : u8 = 74;
/** Opcode for xcb_poly_text_16. */
pub static XCB_POLY_TEXT_16 : u8 = 75;
/** Opcode for xcb_image_text_8. */
pub static XCB_IMAGE_TEXT_8 : u8 = 76;
/** Opcode for xcb_image_text_16. */
pub static XCB_IMAGE_TEXT_16 : u8 = 77;

pub type colormap_alloc = c_uint;//{
    pub static XCB_COLORMAP_ALLOC_NONE : colormap_alloc = 0;
    pub static XCB_COLORMAP_ALLOC_ALL : colormap_alloc = 1;
//}
/** Opcode for xcb_create_colormap. */
pub static XCB_CREATE_COLORMAP : u8 = 78;
/** Opcode for xcb_free_colormap. */
pub static XCB_FREE_COLORMAP : u8 = 79;
/** Opcode for xcb_copy_colormap_and_free. */
pub static XCB_COPY_COLORMAP_AND_FREE : u8 = 80;
/** Opcode for xcb_install_colormap. */
pub static XCB_INSTALL_COLORMAP : u8 = 81;
/** Opcode for xcb_uninstall_colormap. */
pub static XCB_UNINSTALL_COLORMAP : u8 = 82;
pub type ListInstalledColormapsCookie<'self> = base::Cookie<'self, list_installed_colormaps_cookie>;

/** Opcode for xcb_list_installed_colormaps. */
pub static XCB_LIST_INSTALLED_COLORMAPS : u8 = 83;
pub type AllocColorCookie<'self> = base::Cookie<'self, alloc_color_cookie>;

/** Opcode for xcb_alloc_color. */
pub static XCB_ALLOC_COLOR : u8 = 84;
pub type AllocColorReply = base::Reply<alloc_color_reply>;
pub type AllocNamedColorCookie<'self> = base::Cookie<'self, alloc_named_color_cookie>;

/** Opcode for xcb_alloc_named_color. */
pub static XCB_ALLOC_NAMED_COLOR : u8 = 85;
pub type AllocNamedColorReply = base::Reply<alloc_named_color_reply>;
pub type AllocColorCellsCookie<'self> = base::Cookie<'self, alloc_color_cells_cookie>;

/** Opcode for xcb_alloc_color_cells. */
pub static XCB_ALLOC_COLOR_CELLS : u8 = 86;
pub type AllocColorPlanesCookie<'self> = base::Cookie<'self, alloc_color_planes_cookie>;

/** Opcode for xcb_alloc_color_planes. */
pub static XCB_ALLOC_COLOR_PLANES : u8 = 87;
/** Opcode for xcb_free_colors. */
pub static XCB_FREE_COLORS : u8 = 88;

pub type color_flag = c_uint;//{
    pub static XCB_COLOR_FLAG_RED : color_flag = 1;
    pub static XCB_COLOR_FLAG_GREEN : color_flag = 2;
    pub static XCB_COLOR_FLAG_BLUE : color_flag = 4;
//}
pub type Coloritem = base::Struct<coloritem>;

pub type ColoritemIterator = coloritem_iterator;

/** Opcode for xcb_store_colors. */
pub static XCB_STORE_COLORS : u8 = 89;
/** Opcode for xcb_store_named_color. */
pub static XCB_STORE_NAMED_COLOR : u8 = 90;
pub type RgbIterator = rgb_iterator;

pub type QueryColorsCookie<'self> = base::Cookie<'self, query_colors_cookie>;

/** Opcode for xcb_query_colors. */
pub static XCB_QUERY_COLORS : u8 = 91;
pub type QueryColorsReply = base::Reply<query_colors_reply>;
pub type LookupColorCookie<'self> = base::Cookie<'self, lookup_color_cookie>;

/** Opcode for xcb_lookup_color. */
pub static XCB_LOOKUP_COLOR : u8 = 92;
pub type LookupColorReply = base::Reply<lookup_color_reply>;

pub type pixmap_enum = c_uint;//{
    pub static XCB_PIXMAP_NONE : pixmap_enum = 0;
//}
/** Opcode for xcb_create_cursor. */
pub static XCB_CREATE_CURSOR : u8 = 93;

pub type font_enum = c_uint;//{
    pub static XCB_FONT_NONE : font_enum = 0;
//}
/** Opcode for xcb_create_glyph_cursor. */
pub static XCB_CREATE_GLYPH_CURSOR : u8 = 94;
/** Opcode for xcb_free_cursor. */
pub static XCB_FREE_CURSOR : u8 = 95;
/** Opcode for xcb_recolor_cursor. */
pub static XCB_RECOLOR_CURSOR : u8 = 96;

pub type query_shape_of = c_uint;//{
    pub static XCB_QUERY_SHAPE_OF_LARGEST_CURSOR : query_shape_of = 0;
    pub static XCB_QUERY_SHAPE_OF_FASTEST_TILE : query_shape_of = 1;
    pub static XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE : query_shape_of = 2;
//}
pub type QueryBestSizeCookie<'self> = base::Cookie<'self, query_best_size_cookie>;

/** Opcode for xcb_query_best_size. */
pub static XCB_QUERY_BEST_SIZE : u8 = 97;
pub type QueryBestSizeReply = base::Reply<query_best_size_reply>;
pub type QueryExtensionCookie<'self> = base::Cookie<'self, query_extension_cookie>;

/** Opcode for xcb_query_extension. */
pub static XCB_QUERY_EXTENSION : u8 = 98;
pub type QueryExtensionReply = base::Reply<query_extension_reply>;
pub type ListExtensionsCookie<'self> = base::Cookie<'self, list_extensions_cookie>;

/** Opcode for xcb_list_extensions. */
pub static XCB_LIST_EXTENSIONS : u8 = 99;
/** Opcode for xcb_change_keyboard_mapping. */
pub static XCB_CHANGE_KEYBOARD_MAPPING : u8 = 100;
pub type GetKeyboardMappingCookie<'self> = base::Cookie<'self, get_keyboard_mapping_cookie>;

/** Opcode for xcb_get_keyboard_mapping. */
pub static XCB_GET_KEYBOARD_MAPPING : u8 = 101;

pub type kb = c_uint;//{
    pub static XCB_KB_KEY_CLICK_PERCENT : kb = 1;
    pub static XCB_KB_BELL_PERCENT : kb = 2;
    pub static XCB_KB_BELL_PITCH : kb = 4;
    pub static XCB_KB_BELL_DURATION : kb = 8;
    pub static XCB_KB_LED : kb = 16;
    pub static XCB_KB_LED_MODE : kb = 32;
    pub static XCB_KB_KEY : kb = 64;
    pub static XCB_KB_AUTO_REPEAT_MODE : kb = 128;
//}

pub type led_mode = c_uint;//{
    pub static XCB_LED_MODE_OFF : led_mode = 0;
    pub static XCB_LED_MODE_ON : led_mode = 1;
//}

pub type auto_repeat_mode = c_uint;//{
    pub static XCB_AUTO_REPEAT_MODE_OFF : auto_repeat_mode = 0;
    pub static XCB_AUTO_REPEAT_MODE_ON : auto_repeat_mode = 1;
    pub static XCB_AUTO_REPEAT_MODE_DEFAULT : auto_repeat_mode = 2;
//}
/** Opcode for xcb_change_keyboard_control. */
pub static XCB_CHANGE_KEYBOARD_CONTROL : u8 = 102;
pub type GetKeyboardControlCookie<'self> = base::Cookie<'self, get_keyboard_control_cookie>;

/** Opcode for xcb_get_keyboard_control. */
pub static XCB_GET_KEYBOARD_CONTROL : u8 = 103;
pub type GetKeyboardControlReply = base::Reply<get_keyboard_control_reply>;
/** Opcode for xcb_bell. */
pub static XCB_BELL : u8 = 104;
/** Opcode for xcb_change_pointer_control. */
pub static XCB_CHANGE_POINTER_CONTROL : u8 = 105;
pub type GetPointerControlCookie<'self> = base::Cookie<'self, get_pointer_control_cookie>;

/** Opcode for xcb_get_pointer_control. */
pub static XCB_GET_POINTER_CONTROL : u8 = 106;
pub type GetPointerControlReply = base::Reply<get_pointer_control_reply>;

pub type blanking = c_uint;//{
    pub static XCB_BLANKING_NOT_PREFERRED : blanking = 0;
    pub static XCB_BLANKING_PREFERRED : blanking = 1;
    pub static XCB_BLANKING_DEFAULT : blanking = 2;
//}

pub type exposures = c_uint;//{
    pub static XCB_EXPOSURES_NOT_ALLOWED : exposures = 0;
    pub static XCB_EXPOSURES_ALLOWED : exposures = 1;
    pub static XCB_EXPOSURES_DEFAULT : exposures = 2;
//}
/** Opcode for xcb_set_screen_saver. */
pub static XCB_SET_SCREEN_SAVER : u8 = 107;
pub type GetScreenSaverCookie<'self> = base::Cookie<'self, get_screen_saver_cookie>;

/** Opcode for xcb_get_screen_saver. */
pub static XCB_GET_SCREEN_SAVER : u8 = 108;
pub type GetScreenSaverReply = base::Reply<get_screen_saver_reply>;

pub type host_mode = c_uint;//{
    pub static XCB_HOST_MODE_INSERT : host_mode = 0;
    pub static XCB_HOST_MODE_DELETE : host_mode = 1;
//}

pub type family = c_uint;//{
    pub static XCB_FAMILY_INTERNET : family = 0;
    pub static XCB_FAMILY_DECNET : family = 1;
    pub static XCB_FAMILY_CHAOS : family = 2;
    pub static XCB_FAMILY_SERVER_INTERPRETED : family = 5;
    pub static XCB_FAMILY_INTERNET_6 : family = 6;
//}
/** Opcode for xcb_change_hosts. */
pub static XCB_CHANGE_HOSTS : u8 = 109;
pub type HostIterator = host_iterator;

pub type ListHostsCookie<'self> = base::Cookie<'self, list_hosts_cookie>;

/** Opcode for xcb_list_hosts. */
pub static XCB_LIST_HOSTS : u8 = 110;

pub type access_control = c_uint;//{
    pub static XCB_ACCESS_CONTROL_DISABLE : access_control = 0;
    pub static XCB_ACCESS_CONTROL_ENABLE : access_control = 1;
//}
/** Opcode for xcb_set_access_control. */
pub static XCB_SET_ACCESS_CONTROL : u8 = 111;

pub type close_down = c_uint;//{
    pub static XCB_CLOSE_DOWN_DESTROY_ALL : close_down = 0;
    pub static XCB_CLOSE_DOWN_RETAIN_PERMANENT : close_down = 1;
    pub static XCB_CLOSE_DOWN_RETAIN_TEMPORARY : close_down = 2;
//}
/** Opcode for xcb_set_close_down_mode. */
pub static XCB_SET_CLOSE_DOWN_MODE : u8 = 112;

pub type kill = c_uint;//{
    pub static XCB_KILL_ALL_TEMPORARY : kill = 0;
//}
/** Opcode for xcb_kill_client. */
pub static XCB_KILL_CLIENT : u8 = 113;
/** Opcode for xcb_rotate_properties. */
pub static XCB_ROTATE_PROPERTIES : u8 = 114;

pub type screen_saver = c_uint;//{
    pub static XCB_SCREEN_SAVER_RESET : screen_saver = 0;
    pub static XCB_SCREEN_SAVER_ACTIVE : screen_saver = 1;
//}
/** Opcode for xcb_force_screen_saver. */
pub static XCB_FORCE_SCREEN_SAVER : u8 = 115;

pub type mapping_status = c_uint;//{
    pub static XCB_MAPPING_STATUS_SUCCESS : mapping_status = 0;
    pub static XCB_MAPPING_STATUS_BUSY : mapping_status = 1;
    pub static XCB_MAPPING_STATUS_FAILURE : mapping_status = 2;
//}
pub type SetPointerMappingCookie<'self> = base::Cookie<'self, set_pointer_mapping_cookie>;

/** Opcode for xcb_set_pointer_mapping. */
pub static XCB_SET_POINTER_MAPPING : u8 = 116;
pub type SetPointerMappingReply = base::Reply<set_pointer_mapping_reply>;
pub type GetPointerMappingCookie<'self> = base::Cookie<'self, get_pointer_mapping_cookie>;

/** Opcode for xcb_get_pointer_mapping. */
pub static XCB_GET_POINTER_MAPPING : u8 = 117;

pub type map_index = c_uint;//{
    pub static XCB_MAP_INDEX_SHIFT : map_index = 0;
    pub static XCB_MAP_INDEX_LOCK : map_index = 1;
    pub static XCB_MAP_INDEX_CONTROL : map_index = 2;
    pub static XCB_MAP_INDEX_1 : map_index = 3;
    pub static XCB_MAP_INDEX_2 : map_index = 4;
    pub static XCB_MAP_INDEX_3 : map_index = 5;
    pub static XCB_MAP_INDEX_4 : map_index = 6;
    pub static XCB_MAP_INDEX_5 : map_index = 7;
//}
pub type SetModifierMappingCookie<'self> = base::Cookie<'self, set_modifier_mapping_cookie>;

/** Opcode for xcb_set_modifier_mapping. */
pub static XCB_SET_MODIFIER_MAPPING : u8 = 118;
pub type SetModifierMappingReply = base::Reply<set_modifier_mapping_reply>;
pub type GetModifierMappingCookie<'self> = base::Cookie<'self, get_modifier_mapping_cookie>;

/** Opcode for xcb_get_modifier_mapping. */
pub static XCB_GET_MODIFIER_MAPPING : u8 = 119;
/** Opcode for xcb_no_operation. */
pub static XCB_NO_OPERATION : u8 = 127;

pub impl base::Struct<char2b> {
  fn byte1(&self) -> u8 {
    unsafe { accessor!(byte1 -> u8, self.strct) }
  }

  fn byte2(&self) -> u8 {
    unsafe { accessor!(byte2 -> u8, self.strct) }
  }

}

impl<'self, Char2b> Iterator<&'self Char2b> for Char2bIterator {
    fn next(&mut self) -> Option<&'self Char2b> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *char2b_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_char2b_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Window = window;


impl<'self, Window> Iterator<&'self Window> for WindowIterator {
    fn next(&mut self) -> Option<&'self Window> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *window_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_window_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pixmap = pixmap;


impl<'self, Pixmap> Iterator<&'self Pixmap> for PixmapIterator {
    fn next(&mut self) -> Option<&'self Pixmap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *pixmap_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_pixmap_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Cursor = cursor;


impl<'self, Cursor> Iterator<&'self Cursor> for CursorIterator {
    fn next(&mut self) -> Option<&'self Cursor> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *cursor_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_cursor_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Font = font;


impl<'self, Font> Iterator<&'self Font> for FontIterator {
    fn next(&mut self) -> Option<&'self Font> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *font_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_font_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Gcontext = gcontext;


impl<'self, Gcontext> Iterator<&'self Gcontext> for GcontextIterator {
    fn next(&mut self) -> Option<&'self Gcontext> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *gcontext_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_gcontext_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Colormap = colormap;


impl<'self, Colormap> Iterator<&'self Colormap> for ColormapIterator {
    fn next(&mut self) -> Option<&'self Colormap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *colormap_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_colormap_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Atom = atom;


impl<'self, Atom> Iterator<&'self Atom> for AtomIterator {
    fn next(&mut self) -> Option<&'self Atom> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *atom_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_atom_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Drawable = drawable;


impl<'self, Drawable> Iterator<&'self Drawable> for DrawableIterator {
    fn next(&mut self) -> Option<&'self Drawable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *drawable_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_drawable_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Fontable = fontable;


impl<'self, Fontable> Iterator<&'self Fontable> for FontableIterator {
    fn next(&mut self) -> Option<&'self Fontable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *fontable_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_fontable_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Visualid = visualid;


impl<'self, Visualid> Iterator<&'self Visualid> for VisualidIterator {
    fn next(&mut self) -> Option<&'self Visualid> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *visualid_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_visualid_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Timestamp = timestamp;


impl<'self, Timestamp> Iterator<&'self Timestamp> for TimestampIterator {
    fn next(&mut self) -> Option<&'self Timestamp> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *timestamp_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_timestamp_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Keysym = keysym;


impl<'self, Keysym> Iterator<&'self Keysym> for KeysymIterator {
    fn next(&mut self) -> Option<&'self Keysym> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *keysym_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_keysym_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Keycode = keycode;


impl<'self, Keycode> Iterator<&'self Keycode> for KeycodeIterator {
    fn next(&mut self) -> Option<&'self Keycode> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *keycode_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_keycode_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Button = button;


impl<'self, Button> Iterator<&'self Button> for ButtonIterator {
    fn next(&mut self) -> Option<&'self Button> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *button_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_button_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Point = base::Struct<point>;


pub impl base::Struct<point> {
  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

}

impl<'self, Point> Iterator<&'self Point> for PointIterator {
    fn next(&mut self) -> Option<&'self Point> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *point_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_point_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Rectangle = base::Struct<rectangle>;


pub impl base::Struct<rectangle> {
  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, self.strct) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, self.strct) }
  }

}

impl<'self, Rectangle> Iterator<&'self Rectangle> for RectangleIterator {
    fn next(&mut self) -> Option<&'self Rectangle> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *rectangle_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_rectangle_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Arc = base::Struct<arc>;


pub impl base::Struct<arc> {
  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, self.strct) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, self.strct) }
  }

  fn angle1(&self) -> i16 {
    unsafe { accessor!(angle1 -> i16, self.strct) }
  }

  fn angle2(&self) -> i16 {
    unsafe { accessor!(angle2 -> i16, self.strct) }
  }

}

impl<'self, Arc> Iterator<&'self Arc> for ArcIterator {
    fn next(&mut self) -> Option<&'self Arc> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *arc_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_arc_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Format = base::Struct<format>;


pub impl base::Struct<format> {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, self.strct) }
  }

  fn bits_per_pixel(&self) -> u8 {
    unsafe { accessor!(bits_per_pixel -> u8, self.strct) }
  }

  fn scanline_pad(&self) -> u8 {
    unsafe { accessor!(scanline_pad -> u8, self.strct) }
  }

}

impl<'self, Format> Iterator<&'self Format> for FormatIterator {
    fn next(&mut self) -> Option<&'self Format> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *format_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_format_next(iter);
            Some(cast::transmute(data))
        }
    }
}


pub impl base::Struct<visualtype> {
  fn visual_id(&self) -> Visualid {
    unsafe { accessor!(visual_id -> Visualid, self.strct) }
  }

  fn class(&self) -> u8 {
    unsafe { accessor!(class -> u8, self.strct) }
  }

  fn bits_per_rgb_value(&self) -> u8 {
    unsafe { accessor!(bits_per_rgb_value -> u8, self.strct) }
  }

  fn colormap_entries(&self) -> u16 {
    unsafe { accessor!(colormap_entries -> u16, self.strct) }
  }

  fn red_mask(&self) -> u32 {
    unsafe { accessor!(red_mask -> u32, self.strct) }
  }

  fn green_mask(&self) -> u32 {
    unsafe { accessor!(green_mask -> u32, self.strct) }
  }

  fn blue_mask(&self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, self.strct) }
  }

}

impl<'self, Visualtype> Iterator<&'self Visualtype> for VisualtypeIterator {
    fn next(&mut self) -> Option<&'self Visualtype> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *visualtype_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_visualtype_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Depth = base::Struct<depth>;


pub impl base::Struct<depth> {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, self.strct) }
  }

  fn visuals(&self) -> VisualtypeIterator {
    unsafe { accessor!(VisualtypeIterator, xcb_depth_visuals_iterator, self.strct) }
  }

}

impl<'self, Depth> Iterator<&'self Depth> for DepthIterator {
    fn next(&mut self) -> Option<&'self Depth> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *depth_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_depth_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Screen = base::Struct<screen>;


pub impl base::Struct<screen> {
  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, self.strct) }
  }

  fn default_colormap(&self) -> Colormap {
    unsafe { accessor!(default_colormap -> Colormap, self.strct) }
  }

  fn white_pixel(&self) -> u32 {
    unsafe { accessor!(white_pixel -> u32, self.strct) }
  }

  fn black_pixel(&self) -> u32 {
    unsafe { accessor!(black_pixel -> u32, self.strct) }
  }

  fn current_input_masks(&self) -> u32 {
    unsafe { accessor!(current_input_masks -> u32, self.strct) }
  }

  fn width_in_pixels(&self) -> u16 {
    unsafe { accessor!(width_in_pixels -> u16, self.strct) }
  }

  fn height_in_pixels(&self) -> u16 {
    unsafe { accessor!(height_in_pixels -> u16, self.strct) }
  }

  fn width_in_millimeters(&self) -> u16 {
    unsafe { accessor!(width_in_millimeters -> u16, self.strct) }
  }

  fn height_in_millimeters(&self) -> u16 {
    unsafe { accessor!(height_in_millimeters -> u16, self.strct) }
  }

  fn min_installed_maps(&self) -> u16 {
    unsafe { accessor!(min_installed_maps -> u16, self.strct) }
  }

  fn max_installed_maps(&self) -> u16 {
    unsafe { accessor!(max_installed_maps -> u16, self.strct) }
  }

  fn root_visual(&self) -> Visualid {
    unsafe { accessor!(root_visual -> Visualid, self.strct) }
  }

  fn backing_stores(&self) -> u8 {
    unsafe { accessor!(backing_stores -> u8, self.strct) }
  }

  fn save_unders(&self) -> u8 {
    unsafe { accessor!(save_unders -> u8, self.strct) }
  }

  fn root_depth(&self) -> u8 {
    unsafe { accessor!(root_depth -> u8, self.strct) }
  }

  fn allowed_depths(&self) -> DepthIterator {
    unsafe { accessor!(DepthIterator, xcb_screen_allowed_depths_iterator, self.strct) }
  }

}

impl<'self, Screen> Iterator<&'self Screen> for ScreenIterator {
    fn next(&mut self) -> Option<&'self Screen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *screen_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_screen_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type SetupRequest = base::Struct<setup_request>;


pub impl base::Struct<setup_request> {
  fn byte_order(&self) -> u8 {
    unsafe { accessor!(byte_order -> u8, self.strct) }
  }

  fn protocol_major_version(&self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.strct) }
  }

  fn protocol_minor_version(&self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.strct) }
  }

  fn authorization_protocol_name(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_request_authorization_protocol_name_length, xcb_setup_request_authorization_protocol_name, self.strct) }
  }

  fn authorization_protocol_data(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_request_authorization_protocol_data_length, xcb_setup_request_authorization_protocol_data, self.strct) }
  }

}

impl<'self, SetupRequest> Iterator<&'self SetupRequest> for SetupRequestIterator {
    fn next(&mut self) -> Option<&'self SetupRequest> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_request_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_request_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type SetupFailed = base::Struct<setup_failed>;


pub impl base::Struct<setup_failed> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, self.strct) }
  }

  fn protocol_major_version(&self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.strct) }
  }

  fn protocol_minor_version(&self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.strct) }
  }

  fn length(&self) -> u16 {
    unsafe { accessor!(length -> u16, self.strct) }
  }

  fn reason(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_failed_reason_length, xcb_setup_failed_reason, self.strct) }
  }

}

impl<'self, SetupFailed> Iterator<&'self SetupFailed> for SetupFailedIterator {
    fn next(&mut self) -> Option<&'self SetupFailed> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_failed_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_failed_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type SetupAuthenticate = base::Struct<setup_authenticate>;


pub impl base::Struct<setup_authenticate> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, self.strct) }
  }

  fn reason(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_authenticate_reason_length, xcb_setup_authenticate_reason, self.strct) }
  }

}

impl<'self, SetupAuthenticate> Iterator<&'self SetupAuthenticate> for SetupAuthenticateIterator {
    fn next(&mut self) -> Option<&'self SetupAuthenticate> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_authenticate_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_authenticate_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Setup = base::Struct<setup>;


pub impl base::Struct<setup> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, self.strct) }
  }

  fn protocol_major_version(&self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.strct) }
  }

  fn protocol_minor_version(&self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.strct) }
  }

  fn length(&self) -> u16 {
    unsafe { accessor!(length -> u16, self.strct) }
  }

  fn release_number(&self) -> u32 {
    unsafe { accessor!(release_number -> u32, self.strct) }
  }

  fn resource_id_base(&self) -> u32 {
    unsafe { accessor!(resource_id_base -> u32, self.strct) }
  }

  fn resource_id_mask(&self) -> u32 {
    unsafe { accessor!(resource_id_mask -> u32, self.strct) }
  }

  fn motion_buffer_size(&self) -> u32 {
    unsafe { accessor!(motion_buffer_size -> u32, self.strct) }
  }

  fn maximum_request_length(&self) -> u16 {
    unsafe { accessor!(maximum_request_length -> u16, self.strct) }
  }

  fn image_byte_order(&self) -> u8 {
    unsafe { accessor!(image_byte_order -> u8, self.strct) }
  }

  fn bitmap_format_bit_order(&self) -> u8 {
    unsafe { accessor!(bitmap_format_bit_order -> u8, self.strct) }
  }

  fn bitmap_format_scanline_unit(&self) -> u8 {
    unsafe { accessor!(bitmap_format_scanline_unit -> u8, self.strct) }
  }

  fn bitmap_format_scanline_pad(&self) -> u8 {
    unsafe { accessor!(bitmap_format_scanline_pad -> u8, self.strct) }
  }

  fn min_keycode(&self) -> Keycode {
    unsafe { accessor!(min_keycode -> Keycode, self.strct) }
  }

  fn max_keycode(&self) -> Keycode {
    unsafe { accessor!(max_keycode -> Keycode, self.strct) }
  }

  fn vendor(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_vendor_length, xcb_setup_vendor, self.strct) }
  }

  fn pixmap_formats(&self) -> FormatIterator {
    unsafe { accessor!(FormatIterator, xcb_setup_pixmap_formats_iterator, self.strct) }
  }

  fn roots(&self) -> ScreenIterator {
    unsafe { accessor!(ScreenIterator, xcb_setup_roots_iterator, self.strct) }
  }

}

impl<'self, Setup> Iterator<&'self Setup> for SetupIterator {
    fn next(&mut self) -> Option<&'self Setup> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_next(iter);
            Some(cast::transmute(data))
        }
    }
}


pub impl base::Event<key_press_event> {
  fn detail(&self) -> Keycode {
    unsafe { accessor!(detail -> Keycode, (*self.event)) }
  }

  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.event)) }
  }

  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.event)) }
  }

  fn root_x(&self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.event)) }
  }

  fn root_y(&self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.event)) }
  }

  fn event_x(&self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.event)) }
  }

  fn event_y(&self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.event)) }
  }

  fn state(&self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.event)) }
  }

  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.event)) }
  }

  fn new(detail : Keycode,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8) -> KeyPressEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut key_press_event;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      Event { event : raw as *key_press_event }
    }
  }
}

pub impl base::Event<button_press_event> {
  fn detail(&self) -> Button {
    unsafe { accessor!(detail -> Button, (*self.event)) }
  }

  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.event)) }
  }

  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.event)) }
  }

  fn root_x(&self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.event)) }
  }

  fn root_y(&self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.event)) }
  }

  fn event_x(&self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.event)) }
  }

  fn event_y(&self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.event)) }
  }

  fn state(&self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.event)) }
  }

  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.event)) }
  }

  fn new(detail : Button,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8) -> ButtonPressEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut button_press_event;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      Event { event : raw as *button_press_event }
    }
  }
}

pub impl base::Event<motion_notify_event> {
  fn detail(&self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.event)) }
  }

  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.event)) }
  }

  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.event)) }
  }

  fn root_x(&self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.event)) }
  }

  fn root_y(&self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.event)) }
  }

  fn event_x(&self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.event)) }
  }

  fn event_y(&self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.event)) }
  }

  fn state(&self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.event)) }
  }

  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.event)) }
  }

  fn new(detail : u8,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8) -> MotionNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut motion_notify_event;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      Event { event : raw as *motion_notify_event }
    }
  }
}

pub impl base::Event<enter_notify_event> {
  fn detail(&self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.event)) }
  }

  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.event)) }
  }

  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.event)) }
  }

  fn root_x(&self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.event)) }
  }

  fn root_y(&self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.event)) }
  }

  fn event_x(&self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.event)) }
  }

  fn event_y(&self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.event)) }
  }

  fn state(&self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.event)) }
  }

  fn mode(&self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.event)) }
  }

  fn same_screen_focus(&self) -> u8 {
    unsafe { accessor!(same_screen_focus -> u8, (*self.event)) }
  }

  fn new(detail : u8,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         mode : u8,
         same_screen_focus : u8) -> EnterNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut enter_notify_event;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).mode = mode;
      (*raw).same_screen_focus = same_screen_focus;
      Event { event : raw as *enter_notify_event }
    }
  }
}

pub impl base::Event<focus_in_event> {
  fn detail(&self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.event)) }
  }

  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn mode(&self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.event)) }
  }

  fn new(detail : u8,
         event : Window,
         mode : u8) -> FocusInEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut focus_in_event;
      (*raw).detail = detail;
      (*raw).event = event;
      (*raw).mode = mode;
      Event { event : raw as *focus_in_event }
    }
  }
}

pub impl base::Event<keymap_notify_event> {
  fn keys(&self) -> ~[u8,..31] {
    unsafe { ~(copy (*self.event).keys) }
  }

  fn new(keys : [u8,..31]) -> KeymapNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut keymap_notify_event;
      (*raw).keys = keys;
      Event { event : raw as *keymap_notify_event }
    }
  }
}

pub impl base::Event<expose_event> {
  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn x(&self) -> u16 {
    unsafe { accessor!(x -> u16, (*self.event)) }
  }

  fn y(&self) -> u16 {
    unsafe { accessor!(y -> u16, (*self.event)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.event)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.event)) }
  }

  fn count(&self) -> u16 {
    unsafe { accessor!(count -> u16, (*self.event)) }
  }

  fn new(window : Window,
         x : u16,
         y : u16,
         width : u16,
         height : u16,
         count : u16) -> ExposeEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut expose_event;
      (*raw).window = window;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).count = count;
      Event { event : raw as *expose_event }
    }
  }
}

pub impl base::Event<graphics_exposure_event> {
  fn drawable(&self) -> Drawable {
    unsafe { accessor!(drawable -> Drawable, (*self.event)) }
  }

  fn x(&self) -> u16 {
    unsafe { accessor!(x -> u16, (*self.event)) }
  }

  fn y(&self) -> u16 {
    unsafe { accessor!(y -> u16, (*self.event)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.event)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.event)) }
  }

  fn minor_opcode(&self) -> u16 {
    unsafe { accessor!(minor_opcode -> u16, (*self.event)) }
  }

  fn count(&self) -> u16 {
    unsafe { accessor!(count -> u16, (*self.event)) }
  }

  fn major_opcode(&self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.event)) }
  }

  fn new(drawable : Drawable,
         x : u16,
         y : u16,
         width : u16,
         height : u16,
         minor_opcode : u16,
         count : u16,
         major_opcode : u8) -> GraphicsExposureEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut graphics_exposure_event;
      (*raw).drawable = drawable;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).minor_opcode = minor_opcode;
      (*raw).count = count;
      (*raw).major_opcode = major_opcode;
      Event { event : raw as *graphics_exposure_event }
    }
  }
}

pub impl base::Event<no_exposure_event> {
  fn drawable(&self) -> Drawable {
    unsafe { accessor!(drawable -> Drawable, (*self.event)) }
  }

  fn minor_opcode(&self) -> u16 {
    unsafe { accessor!(minor_opcode -> u16, (*self.event)) }
  }

  fn major_opcode(&self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.event)) }
  }

  fn new(drawable : Drawable,
         minor_opcode : u16,
         major_opcode : u8) -> NoExposureEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut no_exposure_event;
      (*raw).drawable = drawable;
      (*raw).minor_opcode = minor_opcode;
      (*raw).major_opcode = major_opcode;
      Event { event : raw as *no_exposure_event }
    }
  }
}

pub impl base::Event<visibility_notify_event> {
  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn state(&self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.event)) }
  }

  fn new(window : Window,
         state : u8) -> VisibilityNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut visibility_notify_event;
      (*raw).window = window;
      (*raw).state = state;
      Event { event : raw as *visibility_notify_event }
    }
  }
}

pub impl base::Event<create_notify_event> {
  fn parent(&self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.event)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.event)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.event)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.event)) }
  }

  fn border_width(&self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.event)) }
  }

  fn override_redirect(&self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.event)) }
  }

  fn new(parent : Window,
         window : Window,
         x : i16,
         y : i16,
         width : u16,
         height : u16,
         border_width : u16,
         override_redirect : u8) -> CreateNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut create_notify_event;
      (*raw).parent = parent;
      (*raw).window = window;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).border_width = border_width;
      (*raw).override_redirect = override_redirect;
      Event { event : raw as *create_notify_event }
    }
  }
}

pub impl base::Event<destroy_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn new(event : Window,
         window : Window) -> DestroyNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut destroy_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      Event { event : raw as *destroy_notify_event }
    }
  }
}

pub impl base::Event<unmap_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn from_configure(&self) -> u8 {
    unsafe { accessor!(from_configure -> u8, (*self.event)) }
  }

  fn new(event : Window,
         window : Window,
         from_configure : u8) -> UnmapNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut unmap_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).from_configure = from_configure;
      Event { event : raw as *unmap_notify_event }
    }
  }
}

pub impl base::Event<map_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn override_redirect(&self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.event)) }
  }

  fn new(event : Window,
         window : Window,
         override_redirect : u8) -> MapNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut map_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).override_redirect = override_redirect;
      Event { event : raw as *map_notify_event }
    }
  }
}

pub impl base::Event<map_request_event> {
  fn parent(&self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn new(parent : Window,
         window : Window) -> MapRequestEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut map_request_event;
      (*raw).parent = parent;
      (*raw).window = window;
      Event { event : raw as *map_request_event }
    }
  }
}

pub impl base::Event<reparent_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn parent(&self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.event)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.event)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.event)) }
  }

  fn override_redirect(&self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.event)) }
  }

  fn new(event : Window,
         window : Window,
         parent : Window,
         x : i16,
         y : i16,
         override_redirect : u8) -> ReparentNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut reparent_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).parent = parent;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).override_redirect = override_redirect;
      Event { event : raw as *reparent_notify_event }
    }
  }
}

pub impl base::Event<configure_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn above_sibling(&self) -> Window {
    unsafe { accessor!(above_sibling -> Window, (*self.event)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.event)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.event)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.event)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.event)) }
  }

  fn border_width(&self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.event)) }
  }

  fn override_redirect(&self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.event)) }
  }

  fn new(event : Window,
         window : Window,
         above_sibling : Window,
         x : i16,
         y : i16,
         width : u16,
         height : u16,
         border_width : u16,
         override_redirect : u8) -> ConfigureNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut configure_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).above_sibling = above_sibling;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).border_width = border_width;
      (*raw).override_redirect = override_redirect;
      Event { event : raw as *configure_notify_event }
    }
  }
}

pub impl base::Event<configure_request_event> {
  fn stack_mode(&self) -> u8 {
    unsafe { accessor!(stack_mode -> u8, (*self.event)) }
  }

  fn parent(&self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn sibling(&self) -> Window {
    unsafe { accessor!(sibling -> Window, (*self.event)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.event)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.event)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.event)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.event)) }
  }

  fn border_width(&self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.event)) }
  }

  fn value_mask(&self) -> u16 {
    unsafe { accessor!(value_mask -> u16, (*self.event)) }
  }

  fn new(stack_mode : u8,
         parent : Window,
         window : Window,
         sibling : Window,
         x : i16,
         y : i16,
         width : u16,
         height : u16,
         border_width : u16,
         value_mask : u16) -> ConfigureRequestEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut configure_request_event;
      (*raw).stack_mode = stack_mode;
      (*raw).parent = parent;
      (*raw).window = window;
      (*raw).sibling = sibling;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).border_width = border_width;
      (*raw).value_mask = value_mask;
      Event { event : raw as *configure_request_event }
    }
  }
}

pub impl base::Event<gravity_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.event)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.event)) }
  }

  fn new(event : Window,
         window : Window,
         x : i16,
         y : i16) -> GravityNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut gravity_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).x = x;
      (*raw).y = y;
      Event { event : raw as *gravity_notify_event }
    }
  }
}

pub impl base::Event<resize_request_event> {
  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.event)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.event)) }
  }

  fn new(window : Window,
         width : u16,
         height : u16) -> ResizeRequestEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut resize_request_event;
      (*raw).window = window;
      (*raw).width = width;
      (*raw).height = height;
      Event { event : raw as *resize_request_event }
    }
  }
}

pub impl base::Event<circulate_notify_event> {
  fn event(&self) -> Window {
    unsafe { accessor!(event -> Window, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn place(&self) -> u8 {
    unsafe { accessor!(place -> u8, (*self.event)) }
  }

  fn new(event : Window,
         window : Window,
         place : u8) -> CirculateNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut circulate_notify_event;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).place = place;
      Event { event : raw as *circulate_notify_event }
    }
  }
}

pub impl base::Event<property_notify_event> {
  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn atom(&self) -> Atom {
    unsafe { accessor!(atom -> Atom, (*self.event)) }
  }

  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn state(&self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.event)) }
  }

  fn new(window : Window,
         atom : Atom,
         time : Timestamp,
         state : u8) -> PropertyNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut property_notify_event;
      (*raw).window = window;
      (*raw).atom = atom;
      (*raw).time = time;
      (*raw).state = state;
      Event { event : raw as *property_notify_event }
    }
  }
}

pub impl base::Event<selection_clear_event> {
  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn owner(&self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.event)) }
  }

  fn selection(&self) -> Atom {
    unsafe { accessor!(selection -> Atom, (*self.event)) }
  }

  fn new(time : Timestamp,
         owner : Window,
         selection : Atom) -> SelectionClearEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut selection_clear_event;
      (*raw).time = time;
      (*raw).owner = owner;
      (*raw).selection = selection;
      Event { event : raw as *selection_clear_event }
    }
  }
}

pub impl base::Event<selection_request_event> {
  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn owner(&self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.event)) }
  }

  fn requestor(&self) -> Window {
    unsafe { accessor!(requestor -> Window, (*self.event)) }
  }

  fn selection(&self) -> Atom {
    unsafe { accessor!(selection -> Atom, (*self.event)) }
  }

  fn target(&self) -> Atom {
    unsafe { accessor!(target -> Atom, (*self.event)) }
  }

  fn property(&self) -> Atom {
    unsafe { accessor!(property -> Atom, (*self.event)) }
  }

  fn new(time : Timestamp,
         owner : Window,
         requestor : Window,
         selection : Atom,
         target : Atom,
         property : Atom) -> SelectionRequestEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut selection_request_event;
      (*raw).time = time;
      (*raw).owner = owner;
      (*raw).requestor = requestor;
      (*raw).selection = selection;
      (*raw).target = target;
      (*raw).property = property;
      Event { event : raw as *selection_request_event }
    }
  }
}

pub impl base::Event<selection_notify_event> {
  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.event)) }
  }

  fn requestor(&self) -> Window {
    unsafe { accessor!(requestor -> Window, (*self.event)) }
  }

  fn selection(&self) -> Atom {
    unsafe { accessor!(selection -> Atom, (*self.event)) }
  }

  fn target(&self) -> Atom {
    unsafe { accessor!(target -> Atom, (*self.event)) }
  }

  fn property(&self) -> Atom {
    unsafe { accessor!(property -> Atom, (*self.event)) }
  }

  fn new(time : Timestamp,
         requestor : Window,
         selection : Atom,
         target : Atom,
         property : Atom) -> SelectionNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut selection_notify_event;
      (*raw).time = time;
      (*raw).requestor = requestor;
      (*raw).selection = selection;
      (*raw).target = target;
      (*raw).property = property;
      Event { event : raw as *selection_notify_event }
    }
  }
}

pub impl base::Event<colormap_notify_event> {
  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn colormap(&self) -> Colormap {
    unsafe { accessor!(colormap -> Colormap, (*self.event)) }
  }

  fn new_(&self) -> u8 {
    unsafe { accessor!(new_ -> u8, (*self.event)) }
  }

  fn state(&self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.event)) }
  }

  fn new(window : Window,
         colormap : Colormap,
         new_ : u8,
         state : u8) -> ColormapNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut colormap_notify_event;
      (*raw).window = window;
      (*raw).colormap = colormap;
      (*raw).new_ = new_;
      (*raw).state = state;
      Event { event : raw as *colormap_notify_event }
    }
  }
}
pub type ClientMessageData = base::Struct<client_message_data>;

impl<'self, ClientMessageData> Iterator<&'self ClientMessageData> for ClientMessageDataIterator {
    fn next(&mut self) -> Option<&'self ClientMessageData> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *client_message_data_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_client_message_data_next(iter);
            Some(cast::transmute(data))
        }
    }
}


pub impl base::Event<client_message_event> {
  fn format(&self) -> u8 {
    unsafe { accessor!(format -> u8, (*self.event)) }
  }

  fn window(&self) -> Window {
    unsafe { accessor!(window -> Window, (*self.event)) }
  }

  fn type_(&self) -> Atom {
    unsafe { accessor!(type_ -> Atom, (*self.event)) }
  }

  fn data(&self) -> ClientMessageData {
    unsafe { cast::transmute((*self.event).data) }
  }
  fn new(format : u8,
         window : Window,
         type_ : Atom,
         data : ClientMessageData) -> ClientMessageEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut client_message_event;
      (*raw).format = format;
      (*raw).window = window;
      (*raw).type_ = type_;
      (*raw).data = data.strct;
      Event { event : raw as *client_message_event }
    }
  }
}

pub impl base::Event<mapping_notify_event> {
  fn request(&self) -> u8 {
    unsafe { accessor!(request -> u8, (*self.event)) }
  }

  fn first_keycode(&self) -> Keycode {
    unsafe { accessor!(first_keycode -> Keycode, (*self.event)) }
  }

  fn count(&self) -> u8 {
    unsafe { accessor!(count -> u8, (*self.event)) }
  }

  fn new(request : u8,
         first_keycode : Keycode,
         count : u8) -> MappingNotifyEvent {
    unsafe {
      let raw = malloc(32u as size_t) as *mut mapping_notify_event;
      (*raw).request = request;
      (*raw).first_keycode = first_keycode;
      (*raw).count = count;
      Event { event : raw as *mapping_notify_event }
    }
  }
}
pub fn CreateWindowChecked<'r> (c : &'r Connection,
                            depth : u8,
                            wid : Window,
                            parent : Window,
                            x : i16,
                            y : i16,
                            width : u16,
                            height : u16,
                            border_width : u16,
                            class : u16,
                            visual : Visualid,
                            value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_create_window_checked(c.get_raw_conn(),
        depth as u8, //1
        wid as window, //2
        parent as window, //3
        x as i16, //4
        y as i16, //5
        width as u16, //6
        height as u16, //7
        border_width as u16, //8
        class as u16, //9
        visual as visualid, //10
        value_list_mask as u32, //11
        value_list_ptr as *u32); //12
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateWindow<'r> (c : &'r Connection,
                     depth : u8,
                     wid : Window,
                     parent : Window,
                     x : i16,
                     y : i16,
                     width : u16,
                     height : u16,
                     border_width : u16,
                     class : u16,
                     visual : Visualid,
                     value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_create_window(c.get_raw_conn(),
        depth as u8, //1
        wid as window, //2
        parent as window, //3
        x as i16, //4
        y as i16, //5
        width as u16, //6
        height as u16, //7
        border_width as u16, //8
        class as u16, //9
        visual as visualid, //10
        value_list_mask as u32, //11
        value_list_ptr as *u32); //12
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ChangeWindowAttributesChecked<'r> (c : &'r Connection,
                                      window : Window,
                                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_change_window_attributes_checked(c.get_raw_conn(),
        window as window, //1
        value_list_mask as u32, //2
        value_list_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeWindowAttributes<'r> (c : &'r Connection,
                               window : Window,
                               value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_change_window_attributes(c.get_raw_conn(),
        window as window, //1
        value_list_mask as u32, //2
        value_list_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetWindowAttributes<'r> (c : &'r Connection,
                            window : Window) -> GetWindowAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_get_window_attributes(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetWindowAttributesUnchecked<'r> (c : &'r Connection,
                                     window : Window) -> GetWindowAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_get_window_attributes_unchecked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_window_attributes_reply> {
  fn backing_store(&self) -> u8 {
    unsafe { accessor!(backing_store -> u8, (*self.reply)) }
  }

  fn visual(&self) -> Visualid {
    unsafe { accessor!(visual -> Visualid, (*self.reply)) }
  }

  fn class(&self) -> u16 {
    unsafe { accessor!(class -> u16, (*self.reply)) }
  }

  fn bit_gravity(&self) -> u8 {
    unsafe { accessor!(bit_gravity -> u8, (*self.reply)) }
  }

  fn win_gravity(&self) -> u8 {
    unsafe { accessor!(win_gravity -> u8, (*self.reply)) }
  }

  fn backing_planes(&self) -> u32 {
    unsafe { accessor!(backing_planes -> u32, (*self.reply)) }
  }

  fn backing_pixel(&self) -> u32 {
    unsafe { accessor!(backing_pixel -> u32, (*self.reply)) }
  }

  fn save_under(&self) -> u8 {
    unsafe { accessor!(save_under -> u8, (*self.reply)) }
  }

  fn map_is_installed(&self) -> u8 {
    unsafe { accessor!(map_is_installed -> u8, (*self.reply)) }
  }

  fn map_state(&self) -> u8 {
    unsafe { accessor!(map_state -> u8, (*self.reply)) }
  }

  fn override_redirect(&self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.reply)) }
  }

  fn colormap(&self) -> Colormap {
    unsafe { accessor!(colormap -> Colormap, (*self.reply)) }
  }

  fn all_event_masks(&self) -> u32 {
    unsafe { accessor!(all_event_masks -> u32, (*self.reply)) }
  }

  fn your_event_mask(&self) -> u32 {
    unsafe { accessor!(your_event_mask -> u32, (*self.reply)) }
  }

  fn do_not_propagate_mask(&self) -> u16 {
    unsafe { accessor!(do_not_propagate_mask -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(GetWindowAttributesCookie<'self>, get_window_attributes_reply, GetWindowAttributesReply, xcb_get_window_attributes_reply)

pub fn DestroyWindowChecked<'r> (c : &'r Connection,
                             window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_window_checked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn DestroyWindow<'r> (c : &'r Connection,
                      window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_window(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn DestroySubwindowsChecked<'r> (c : &'r Connection,
                                 window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_subwindows_checked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn DestroySubwindows<'r> (c : &'r Connection,
                          window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_subwindows(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ChangeSaveSetChecked<'r> (c : &'r Connection,
                             mode : u8,
                             window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_save_set_checked(c.get_raw_conn(),
        mode as u8, //1
        window as window); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeSaveSet<'r> (c : &'r Connection,
                      mode : u8,
                      window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_save_set(c.get_raw_conn(),
        mode as u8, //1
        window as window); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ReparentWindowChecked<'r> (c : &'r Connection,
                              window : Window,
                              parent : Window,
                              x : i16,
                              y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_reparent_window_checked(c.get_raw_conn(),
        window as window, //1
        parent as window, //2
        x as i16, //3
        y as i16); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ReparentWindow<'r> (c : &'r Connection,
                       window : Window,
                       parent : Window,
                       x : i16,
                       y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_reparent_window(c.get_raw_conn(),
        window as window, //1
        parent as window, //2
        x as i16, //3
        y as i16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn MapWindowChecked<'r> (c : &'r Connection,
                         window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_window_checked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn MapWindow<'r> (c : &'r Connection,
                  window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_window(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn MapSubwindowsChecked<'r> (c : &'r Connection,
                             window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_subwindows_checked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn MapSubwindows<'r> (c : &'r Connection,
                      window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_subwindows(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn UnmapWindowChecked<'r> (c : &'r Connection,
                           window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_window_checked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UnmapWindow<'r> (c : &'r Connection,
                    window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_window(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn UnmapSubwindowsChecked<'r> (c : &'r Connection,
                               window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_subwindows_checked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UnmapSubwindows<'r> (c : &'r Connection,
                        window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_subwindows(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ConfigureWindowChecked<'r> (c : &'r Connection,
                               window : Window,
                               value_list : &[(u16,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_configure_window_checked(c.get_raw_conn(),
        window as window, //1
        value_list_mask as u16, //2
        value_list_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ConfigureWindow<'r> (c : &'r Connection,
                        window : Window,
                        value_list : &[(u16,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_configure_window(c.get_raw_conn(),
        window as window, //1
        value_list_mask as u16, //2
        value_list_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CirculateWindowChecked<'r> (c : &'r Connection,
                               direction : u8,
                               window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_circulate_window_checked(c.get_raw_conn(),
        direction as u8, //1
        window as window); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CirculateWindow<'r> (c : &'r Connection,
                        direction : u8,
                        window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_circulate_window(c.get_raw_conn(),
        direction as u8, //1
        window as window); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetGeometry<'r> (c : &'r Connection,
                    drawable : Drawable) -> GetGeometryCookie<'r> {
  unsafe {
    let cookie = xcb_get_geometry(c.get_raw_conn(),
        drawable as drawable); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetGeometryUnchecked<'r> (c : &'r Connection,
                             drawable : Drawable) -> GetGeometryCookie<'r> {
  unsafe {
    let cookie = xcb_get_geometry_unchecked(c.get_raw_conn(),
        drawable as drawable); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_geometry_reply> {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.reply)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.reply)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.reply)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.reply)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.reply)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.reply)) }
  }

  fn border_width(&self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(GetGeometryCookie<'self>, get_geometry_reply, GetGeometryReply, xcb_get_geometry_reply)

pub type QueryTreeReply = base::Reply<query_tree_reply>;
pub fn QueryTree<'r> (c : &'r Connection,
                  window : Window) -> QueryTreeCookie<'r> {
  unsafe {
    let cookie = xcb_query_tree(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryTreeUnchecked<'r> (c : &'r Connection,
                           window : Window) -> QueryTreeCookie<'r> {
  unsafe {
    let cookie = xcb_query_tree_unchecked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_tree_reply> {
  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.reply)) }
  }

  fn parent(&self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.reply)) }
  }

  fn children(&self) -> ~[Window] {
    unsafe { accessor!(Window, xcb_query_tree_children_length, xcb_query_tree_children, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryTreeCookie<'self>, query_tree_reply, QueryTreeReply, xcb_query_tree_reply)

pub fn InternAtom<'r> (c : &'r Connection,
                   only_if_exists : u8,
                   name : &str) -> InternAtomCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_intern_atom(c.get_raw_conn(),
        only_if_exists as u8, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn InternAtomUnchecked<'r> (c : &'r Connection,
                            only_if_exists : u8,
                            name : &str) -> InternAtomCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_intern_atom_unchecked(c.get_raw_conn(),
        only_if_exists as u8, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<intern_atom_reply> {
  fn atom(&self) -> Atom {
    unsafe { accessor!(atom -> Atom, (*self.reply)) }
  }

}
impl_reply_cookie!(InternAtomCookie<'self>, intern_atom_reply, InternAtomReply, xcb_intern_atom_reply)

pub type GetAtomNameReply = base::Reply<get_atom_name_reply>;
pub fn GetAtomName<'r> (c : &'r Connection,
                    atom : Atom) -> GetAtomNameCookie<'r> {
  unsafe {
    let cookie = xcb_get_atom_name(c.get_raw_conn(),
        atom as atom); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetAtomNameUnchecked<'r> (c : &'r Connection,
                             atom : Atom) -> GetAtomNameCookie<'r> {
  unsafe {
    let cookie = xcb_get_atom_name_unchecked(c.get_raw_conn(),
        atom as atom); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_atom_name_reply> {
  fn name(&self) -> ~str {
    unsafe { accessor!(str, xcb_get_atom_name_name_length, xcb_get_atom_name_name, (*self.reply)) }
  }

}
impl_reply_cookie!(GetAtomNameCookie<'self>, get_atom_name_reply, GetAtomNameReply, xcb_get_atom_name_reply)

pub fn ChangePropertyChecked<'r> (c : &'r Connection,
                              mode : u8,
                              window : Window,
                              property : Atom,
                              type_ : Atom,
                              format : u8,
                              data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = core::vec::raw::to_ptr(data);
    let cookie = xcb_change_property_checked(c.get_raw_conn(),
        mode as u8, //1
        window as window, //2
        property as atom, //3
        type_ as atom, //4
        format as u8, //5
        data_len as u32, //6
        data_ptr as *c_void); //7
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeProperty<'r> (c : &'r Connection,
                       mode : u8,
                       window : Window,
                       property : Atom,
                       type_ : Atom,
                       format : u8,
                       data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = core::vec::raw::to_ptr(data);
    let cookie = xcb_change_property(c.get_raw_conn(),
        mode as u8, //1
        window as window, //2
        property as atom, //3
        type_ as atom, //4
        format as u8, //5
        data_len as u32, //6
        data_ptr as *c_void); //7
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn DeletePropertyChecked<'r> (c : &'r Connection,
                              window : Window,
                              property : Atom) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_delete_property_checked(c.get_raw_conn(),
        window as window, //1
        property as atom); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn DeleteProperty<'r> (c : &'r Connection,
                       window : Window,
                       property : Atom) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_delete_property(c.get_raw_conn(),
        window as window, //1
        property as atom); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type GetPropertyReply = base::Reply<get_property_reply>;
pub fn GetProperty<'r> (c : &'r Connection,
                    delete : u8,
                    window : Window,
                    property : Atom,
                    type_ : Atom,
                    long_offset : u32,
                    long_length : u32) -> GetPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_get_property(c.get_raw_conn(),
        delete as u8, //1
        window as window, //2
        property as atom, //3
        type_ as atom, //4
        long_offset as u32, //5
        long_length as u32); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetPropertyUnchecked<'r> (c : &'r Connection,
                             delete : u8,
                             window : Window,
                             property : Atom,
                             type_ : Atom,
                             long_offset : u32,
                             long_length : u32) -> GetPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_get_property_unchecked(c.get_raw_conn(),
        delete as u8, //1
        window as window, //2
        property as atom, //3
        type_ as atom, //4
        long_offset as u32, //5
        long_length as u32); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_property_reply> {
  fn format(&self) -> u8 {
    unsafe { accessor!(format -> u8, (*self.reply)) }
  }

  fn type_(&self) -> Atom {
    unsafe { accessor!(type_ -> Atom, (*self.reply)) }
  }

  fn bytes_after(&self) -> u32 {
    unsafe { accessor!(bytes_after -> u32, (*self.reply)) }
  }

  fn value(&self) -> ~[c_void] {
    unsafe { accessor!(c_void, xcb_get_property_value_length, xcb_get_property_value, (*self.reply)) }
  }

}
impl_reply_cookie!(GetPropertyCookie<'self>, get_property_reply, GetPropertyReply, xcb_get_property_reply)

pub type ListPropertiesReply = base::Reply<list_properties_reply>;
pub fn ListProperties<'r> (c : &'r Connection,
                       window : Window) -> ListPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_list_properties(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ListPropertiesUnchecked<'r> (c : &'r Connection,
                                window : Window) -> ListPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_list_properties_unchecked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<list_properties_reply> {
  fn atoms(&self) -> ~[Atom] {
    unsafe { accessor!(Atom, xcb_list_properties_atoms_length, xcb_list_properties_atoms, (*self.reply)) }
  }

}
impl_reply_cookie!(ListPropertiesCookie<'self>, list_properties_reply, ListPropertiesReply, xcb_list_properties_reply)

pub fn SetSelectionOwnerChecked<'r> (c : &'r Connection,
                                 owner : Window,
                                 selection : Atom,
                                 time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_selection_owner_checked(c.get_raw_conn(),
        owner as window, //1
        selection as atom, //2
        time as timestamp); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetSelectionOwner<'r> (c : &'r Connection,
                          owner : Window,
                          selection : Atom,
                          time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_selection_owner(c.get_raw_conn(),
        owner as window, //1
        selection as atom, //2
        time as timestamp); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetSelectionOwner<'r> (c : &'r Connection,
                          selection : Atom) -> GetSelectionOwnerCookie<'r> {
  unsafe {
    let cookie = xcb_get_selection_owner(c.get_raw_conn(),
        selection as atom); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetSelectionOwnerUnchecked<'r> (c : &'r Connection,
                                   selection : Atom) -> GetSelectionOwnerCookie<'r> {
  unsafe {
    let cookie = xcb_get_selection_owner_unchecked(c.get_raw_conn(),
        selection as atom); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_selection_owner_reply> {
  fn owner(&self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.reply)) }
  }

}
impl_reply_cookie!(GetSelectionOwnerCookie<'self>, get_selection_owner_reply, GetSelectionOwnerReply, xcb_get_selection_owner_reply)

pub fn ConvertSelectionChecked<'r> (c : &'r Connection,
                                requestor : Window,
                                selection : Atom,
                                target : Atom,
                                property : Atom,
                                time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_convert_selection_checked(c.get_raw_conn(),
        requestor as window, //1
        selection as atom, //2
        target as atom, //3
        property as atom, //4
        time as timestamp); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ConvertSelection<'r> (c : &'r Connection,
                         requestor : Window,
                         selection : Atom,
                         target : Atom,
                         property : Atom,
                         time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_convert_selection(c.get_raw_conn(),
        requestor as window, //1
        selection as atom, //2
        target as atom, //3
        property as atom, //4
        time as timestamp); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SendEventChecked<'r> (c : &'r Connection,
                         propagate : u8,
                         destination : Window,
                         event_mask : u32,
                         event : &str) -> base::VoidCookie<'r> {
  unsafe {
    let event = core::str::to_bytes(event);
    let event_ptr = core::vec::raw::to_ptr(event);
    let cookie = xcb_send_event_checked(c.get_raw_conn(),
        propagate as u8, //1
        destination as window, //2
        event_mask as u32, //3
        event_ptr as *c_char); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SendEvent<'r> (c : &'r Connection,
                  propagate : u8,
                  destination : Window,
                  event_mask : u32,
                  event : &str) -> base::VoidCookie<'r> {
  unsafe {
    let event = core::str::to_bytes(event);
    let event_ptr = core::vec::raw::to_ptr(event);
    let cookie = xcb_send_event(c.get_raw_conn(),
        propagate as u8, //1
        destination as window, //2
        event_mask as u32, //3
        event_ptr as *c_char); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabPointer<'r> (c : &'r Connection,
                    owner_events : u8,
                    grab_window : Window,
                    event_mask : u16,
                    pointer_mode : u8,
                    keyboard_mode : u8,
                    confine_to : Window,
                    cursor : Cursor,
                    time : Timestamp) -> GrabPointerCookie<'r> {
  unsafe {
    let cookie = xcb_grab_pointer(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as window, //6
        cursor as cursor, //7
        time as timestamp); //8
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabPointerUnchecked<'r> (c : &'r Connection,
                             owner_events : u8,
                             grab_window : Window,
                             event_mask : u16,
                             pointer_mode : u8,
                             keyboard_mode : u8,
                             confine_to : Window,
                             cursor : Cursor,
                             time : Timestamp) -> GrabPointerCookie<'r> {
  unsafe {
    let cookie = xcb_grab_pointer_unchecked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as window, //6
        cursor as cursor, //7
        time as timestamp); //8
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<grab_pointer_reply> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(GrabPointerCookie<'self>, grab_pointer_reply, GrabPointerReply, xcb_grab_pointer_reply)

pub fn UngrabPointerChecked<'r> (c : &'r Connection,
                             time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_pointer_checked(c.get_raw_conn(),
        time as timestamp); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UngrabPointer<'r> (c : &'r Connection,
                      time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_pointer(c.get_raw_conn(),
        time as timestamp); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabButtonChecked<'r> (c : &'r Connection,
                          owner_events : u8,
                          grab_window : Window,
                          event_mask : u16,
                          pointer_mode : u8,
                          keyboard_mode : u8,
                          confine_to : Window,
                          cursor : Cursor,
                          button : u8,
                          modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_button_checked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as window, //6
        cursor as cursor, //7
        button as u8, //8
        modifiers as u16); //9
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn GrabButton<'r> (c : &'r Connection,
                   owner_events : u8,
                   grab_window : Window,
                   event_mask : u16,
                   pointer_mode : u8,
                   keyboard_mode : u8,
                   confine_to : Window,
                   cursor : Cursor,
                   button : u8,
                   modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_button(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as window, //6
        cursor as cursor, //7
        button as u8, //8
        modifiers as u16); //9
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn UngrabButtonChecked<'r> (c : &'r Connection,
                            button : u8,
                            grab_window : Window,
                            modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_button_checked(c.get_raw_conn(),
        button as u8, //1
        grab_window as window, //2
        modifiers as u16); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UngrabButton<'r> (c : &'r Connection,
                     button : u8,
                     grab_window : Window,
                     modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_button(c.get_raw_conn(),
        button as u8, //1
        grab_window as window, //2
        modifiers as u16); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ChangeActivePointerGrabChecked<'r> (c : &'r Connection,
                                       cursor : Cursor,
                                       time : Timestamp,
                                       event_mask : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_active_pointer_grab_checked(c.get_raw_conn(),
        cursor as cursor, //1
        time as timestamp, //2
        event_mask as u16); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeActivePointerGrab<'r> (c : &'r Connection,
                                cursor : Cursor,
                                time : Timestamp,
                                event_mask : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_active_pointer_grab(c.get_raw_conn(),
        cursor as cursor, //1
        time as timestamp, //2
        event_mask as u16); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabKeyboard<'r> (c : &'r Connection,
                     owner_events : u8,
                     grab_window : Window,
                     time : Timestamp,
                     pointer_mode : u8,
                     keyboard_mode : u8) -> GrabKeyboardCookie<'r> {
  unsafe {
    let cookie = xcb_grab_keyboard(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        time as timestamp, //3
        pointer_mode as u8, //4
        keyboard_mode as u8); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabKeyboardUnchecked<'r> (c : &'r Connection,
                              owner_events : u8,
                              grab_window : Window,
                              time : Timestamp,
                              pointer_mode : u8,
                              keyboard_mode : u8) -> GrabKeyboardCookie<'r> {
  unsafe {
    let cookie = xcb_grab_keyboard_unchecked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        time as timestamp, //3
        pointer_mode as u8, //4
        keyboard_mode as u8); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<grab_keyboard_reply> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(GrabKeyboardCookie<'self>, grab_keyboard_reply, GrabKeyboardReply, xcb_grab_keyboard_reply)

pub fn UngrabKeyboardChecked<'r> (c : &'r Connection,
                              time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_keyboard_checked(c.get_raw_conn(),
        time as timestamp); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UngrabKeyboard<'r> (c : &'r Connection,
                       time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_keyboard(c.get_raw_conn(),
        time as timestamp); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabKeyChecked<'r> (c : &'r Connection,
                       owner_events : u8,
                       grab_window : Window,
                       modifiers : u16,
                       key : Keycode,
                       pointer_mode : u8,
                       keyboard_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_key_checked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        modifiers as u16, //3
        key as keycode, //4
        pointer_mode as u8, //5
        keyboard_mode as u8); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn GrabKey<'r> (c : &'r Connection,
                owner_events : u8,
                grab_window : Window,
                modifiers : u16,
                key : Keycode,
                pointer_mode : u8,
                keyboard_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_key(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as window, //2
        modifiers as u16, //3
        key as keycode, //4
        pointer_mode as u8, //5
        keyboard_mode as u8); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn UngrabKeyChecked<'r> (c : &'r Connection,
                         key : Keycode,
                         grab_window : Window,
                         modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_key_checked(c.get_raw_conn(),
        key as keycode, //1
        grab_window as window, //2
        modifiers as u16); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UngrabKey<'r> (c : &'r Connection,
                  key : Keycode,
                  grab_window : Window,
                  modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_key(c.get_raw_conn(),
        key as keycode, //1
        grab_window as window, //2
        modifiers as u16); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn AllowEventsChecked<'r> (c : &'r Connection,
                           mode : u8,
                           time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_allow_events_checked(c.get_raw_conn(),
        mode as u8, //1
        time as timestamp); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn AllowEvents<'r> (c : &'r Connection,
                    mode : u8,
                    time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_allow_events(c.get_raw_conn(),
        mode as u8, //1
        time as timestamp); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GrabServerChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_server_checked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn GrabServer<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_server(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn UngrabServerChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_server_checked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UngrabServer<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_server(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryPointer<'r> (c : &'r Connection,
                     window : Window) -> QueryPointerCookie<'r> {
  unsafe {
    let cookie = xcb_query_pointer(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryPointerUnchecked<'r> (c : &'r Connection,
                              window : Window) -> QueryPointerCookie<'r> {
  unsafe {
    let cookie = xcb_query_pointer_unchecked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_pointer_reply> {
  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.reply)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.reply)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.reply)) }
  }

  fn root_x(&self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.reply)) }
  }

  fn root_y(&self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.reply)) }
  }

  fn win_x(&self) -> i16 {
    unsafe { accessor!(win_x -> i16, (*self.reply)) }
  }

  fn win_y(&self) -> i16 {
    unsafe { accessor!(win_y -> i16, (*self.reply)) }
  }

  fn mask(&self) -> u16 {
    unsafe { accessor!(mask -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryPointerCookie<'self>, query_pointer_reply, QueryPointerReply, xcb_query_pointer_reply)

pub type Timecoord = base::Struct<timecoord>;


pub impl base::Struct<timecoord> {
  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, self.strct) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

}

impl<'self, Timecoord> Iterator<&'self Timecoord> for TimecoordIterator {
    fn next(&mut self) -> Option<&'self Timecoord> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *timecoord_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_timecoord_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type GetMotionEventsReply = base::Reply<get_motion_events_reply>;
pub fn GetMotionEvents<'r> (c : &'r Connection,
                        window : Window,
                        start : Timestamp,
                        stop : Timestamp) -> GetMotionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_get_motion_events(c.get_raw_conn(),
        window as window, //1
        start as timestamp, //2
        stop as timestamp); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetMotionEventsUnchecked<'r> (c : &'r Connection,
                                 window : Window,
                                 start : Timestamp,
                                 stop : Timestamp) -> GetMotionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_get_motion_events_unchecked(c.get_raw_conn(),
        window as window, //1
        start as timestamp, //2
        stop as timestamp); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_motion_events_reply> {
  fn events(&self) -> TimecoordIterator {
    unsafe { accessor!(TimecoordIterator, xcb_get_motion_events_events_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(GetMotionEventsCookie<'self>, get_motion_events_reply, GetMotionEventsReply, xcb_get_motion_events_reply)

pub fn TranslateCoordinates<'r> (c : &'r Connection,
                             src_window : Window,
                             dst_window : Window,
                             src_x : i16,
                             src_y : i16) -> TranslateCoordinatesCookie<'r> {
  unsafe {
    let cookie = xcb_translate_coordinates(c.get_raw_conn(),
        src_window as window, //1
        dst_window as window, //2
        src_x as i16, //3
        src_y as i16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn TranslateCoordinatesUnchecked<'r> (c : &'r Connection,
                                      src_window : Window,
                                      dst_window : Window,
                                      src_x : i16,
                                      src_y : i16) -> TranslateCoordinatesCookie<'r> {
  unsafe {
    let cookie = xcb_translate_coordinates_unchecked(c.get_raw_conn(),
        src_window as window, //1
        dst_window as window, //2
        src_x as i16, //3
        src_y as i16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<translate_coordinates_reply> {
  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.reply)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.reply)) }
  }

  fn dst_x(&self) -> i16 {
    unsafe { accessor!(dst_x -> i16, (*self.reply)) }
  }

  fn dst_y(&self) -> i16 {
    unsafe { accessor!(dst_y -> i16, (*self.reply)) }
  }

}
impl_reply_cookie!(TranslateCoordinatesCookie<'self>, translate_coordinates_reply, TranslateCoordinatesReply, xcb_translate_coordinates_reply)

pub fn WarpPointerChecked<'r> (c : &'r Connection,
                           src_window : Window,
                           dst_window : Window,
                           src_x : i16,
                           src_y : i16,
                           src_width : u16,
                           src_height : u16,
                           dst_x : i16,
                           dst_y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_warp_pointer_checked(c.get_raw_conn(),
        src_window as window, //1
        dst_window as window, //2
        src_x as i16, //3
        src_y as i16, //4
        src_width as u16, //5
        src_height as u16, //6
        dst_x as i16, //7
        dst_y as i16); //8
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn WarpPointer<'r> (c : &'r Connection,
                    src_window : Window,
                    dst_window : Window,
                    src_x : i16,
                    src_y : i16,
                    src_width : u16,
                    src_height : u16,
                    dst_x : i16,
                    dst_y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_warp_pointer(c.get_raw_conn(),
        src_window as window, //1
        dst_window as window, //2
        src_x as i16, //3
        src_y as i16, //4
        src_width as u16, //5
        src_height as u16, //6
        dst_x as i16, //7
        dst_y as i16); //8
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetInputFocusChecked<'r> (c : &'r Connection,
                             revert_to : u8,
                             focus : Window,
                             time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_input_focus_checked(c.get_raw_conn(),
        revert_to as u8, //1
        focus as window, //2
        time as timestamp); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetInputFocus<'r> (c : &'r Connection,
                      revert_to : u8,
                      focus : Window,
                      time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_input_focus(c.get_raw_conn(),
        revert_to as u8, //1
        focus as window, //2
        time as timestamp); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetInputFocus<'r> (c : &'r Connection) -> GetInputFocusCookie<'r> {
  unsafe {
    let cookie = xcb_get_input_focus(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetInputFocusUnchecked<'r> (c : &'r Connection) -> GetInputFocusCookie<'r> {
  unsafe {
    let cookie = xcb_get_input_focus_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_input_focus_reply> {
  fn revert_to(&self) -> u8 {
    unsafe { accessor!(revert_to -> u8, (*self.reply)) }
  }

  fn focus(&self) -> Window {
    unsafe { accessor!(focus -> Window, (*self.reply)) }
  }

}
impl_reply_cookie!(GetInputFocusCookie<'self>, get_input_focus_reply, GetInputFocusReply, xcb_get_input_focus_reply)

pub fn QueryKeymap<'r> (c : &'r Connection) -> QueryKeymapCookie<'r> {
  unsafe {
    let cookie = xcb_query_keymap(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryKeymapUnchecked<'r> (c : &'r Connection) -> QueryKeymapCookie<'r> {
  unsafe {
    let cookie = xcb_query_keymap_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_keymap_reply> {
  fn keys(&self) -> ~[u8,..32] {
    unsafe { ~(copy (*self.reply).keys) }
  }

}
impl_reply_cookie!(QueryKeymapCookie<'self>, query_keymap_reply, QueryKeymapReply, xcb_query_keymap_reply)

pub fn OpenFontChecked<'r> (c : &'r Connection,
                        fid : Font,
                        name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_open_font_checked(c.get_raw_conn(),
        fid as font, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn OpenFont<'r> (c : &'r Connection,
                 fid : Font,
                 name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_open_font(c.get_raw_conn(),
        fid as font, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CloseFontChecked<'r> (c : &'r Connection,
                         font : Font) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_close_font_checked(c.get_raw_conn(),
        font as font); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CloseFont<'r> (c : &'r Connection,
                  font : Font) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_close_font(c.get_raw_conn(),
        font as font); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Struct<fontprop> {
  fn name(&self) -> Atom {
    unsafe { accessor!(name -> Atom, self.strct) }
  }

  fn value(&self) -> u32 {
    unsafe { accessor!(value -> u32, self.strct) }
  }

}

impl<'self, Fontprop> Iterator<&'self Fontprop> for FontpropIterator {
    fn next(&mut self) -> Option<&'self Fontprop> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *fontprop_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_fontprop_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Charinfo = base::Struct<charinfo>;


pub impl base::Struct<charinfo> {
  fn left_side_bearing(&self) -> i16 {
    unsafe { accessor!(left_side_bearing -> i16, self.strct) }
  }

  fn right_side_bearing(&self) -> i16 {
    unsafe { accessor!(right_side_bearing -> i16, self.strct) }
  }

  fn character_width(&self) -> i16 {
    unsafe { accessor!(character_width -> i16, self.strct) }
  }

  fn ascent(&self) -> i16 {
    unsafe { accessor!(ascent -> i16, self.strct) }
  }

  fn descent(&self) -> i16 {
    unsafe { accessor!(descent -> i16, self.strct) }
  }

  fn attributes(&self) -> u16 {
    unsafe { accessor!(attributes -> u16, self.strct) }
  }

}

impl<'self, Charinfo> Iterator<&'self Charinfo> for CharinfoIterator {
    fn next(&mut self) -> Option<&'self Charinfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *charinfo_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_charinfo_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type QueryFontReply = base::Reply<query_font_reply>;
pub fn QueryFont<'r> (c : &'r Connection,
                  font : Fontable) -> QueryFontCookie<'r> {
  unsafe {
    let cookie = xcb_query_font(c.get_raw_conn(),
        font as fontable); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryFontUnchecked<'r> (c : &'r Connection,
                           font : Fontable) -> QueryFontCookie<'r> {
  unsafe {
    let cookie = xcb_query_font_unchecked(c.get_raw_conn(),
        font as fontable); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_font_reply> {
  fn min_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).min_bounds) }
  }
  fn max_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).max_bounds) }
  }
  fn min_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(min_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn max_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(max_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn default_char(&self) -> u16 {
    unsafe { accessor!(default_char -> u16, (*self.reply)) }
  }

  fn draw_direction(&self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.reply)) }
  }

  fn min_byte1(&self) -> u8 {
    unsafe { accessor!(min_byte1 -> u8, (*self.reply)) }
  }

  fn max_byte1(&self) -> u8 {
    unsafe { accessor!(max_byte1 -> u8, (*self.reply)) }
  }

  fn all_chars_exist(&self) -> u8 {
    unsafe { accessor!(all_chars_exist -> u8, (*self.reply)) }
  }

  fn font_ascent(&self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.reply)) }
  }

  fn font_descent(&self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.reply)) }
  }

  fn properties(&self) -> FontpropIterator {
    unsafe { accessor!(FontpropIterator, xcb_query_font_properties_iterator, (*self.reply)) }
  }

  fn char_infos(&self) -> CharinfoIterator {
    unsafe { accessor!(CharinfoIterator, xcb_query_font_char_infos_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryFontCookie<'self>, query_font_reply, QueryFontReply, xcb_query_font_reply)

pub fn QueryTextExtents<'r> (c : &'r Connection,
                         font : Fontable,
                         string : &[Char2b]) -> QueryTextExtentsCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = core::vec::raw::to_ptr(string);
    let cookie = xcb_query_text_extents(c.get_raw_conn(),
        font as fontable, //1
        string_len as u32, //2
        string_ptr as *char2b); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryTextExtentsUnchecked<'r> (c : &'r Connection,
                                  font : Fontable,
                                  string : &[Char2b]) -> QueryTextExtentsCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = core::vec::raw::to_ptr(string);
    let cookie = xcb_query_text_extents_unchecked(c.get_raw_conn(),
        font as fontable, //1
        string_len as u32, //2
        string_ptr as *char2b); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_text_extents_reply> {
  fn draw_direction(&self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.reply)) }
  }

  fn font_ascent(&self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.reply)) }
  }

  fn font_descent(&self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.reply)) }
  }

  fn overall_ascent(&self) -> i16 {
    unsafe { accessor!(overall_ascent -> i16, (*self.reply)) }
  }

  fn overall_descent(&self) -> i16 {
    unsafe { accessor!(overall_descent -> i16, (*self.reply)) }
  }

  fn overall_width(&self) -> i32 {
    unsafe { accessor!(overall_width -> i32, (*self.reply)) }
  }

  fn overall_left(&self) -> i32 {
    unsafe { accessor!(overall_left -> i32, (*self.reply)) }
  }

  fn overall_right(&self) -> i32 {
    unsafe { accessor!(overall_right -> i32, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryTextExtentsCookie<'self>, query_text_extents_reply, QueryTextExtentsReply, xcb_query_text_extents_reply)

pub type Str = base::Struct<str_>;


pub impl base::Struct<str_> {
  fn name(&self) -> ~str {
    unsafe { accessor!(str, xcb_str_name_length, xcb_str_name, self.strct) }
  }

}

impl<'self, Str> Iterator<&'self Str> for StrIterator {
    fn next(&mut self) -> Option<&'self Str> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *str_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_str_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub fn ListFonts<'r> (c : &'r Connection,
                  max_names : u16,
                  pattern : &str) -> ListFontsCookie<'r> {
  unsafe {
    let pattern = core::str::to_bytes(pattern);
    let pattern_len = pattern.len();
    let pattern_ptr = core::vec::raw::to_ptr(pattern);
    let cookie = xcb_list_fonts(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ListFontsUnchecked<'r> (c : &'r Connection,
                           max_names : u16,
                           pattern : &str) -> ListFontsCookie<'r> {
  unsafe {
    let pattern = core::str::to_bytes(pattern);
    let pattern_len = pattern.len();
    let pattern_ptr = core::vec::raw::to_ptr(pattern);
    let cookie = xcb_list_fonts_unchecked(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<list_fonts_reply> {
  fn names(&self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_list_fonts_names_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(ListFontsCookie<'self>, list_fonts_reply, ListFontsReply, xcb_list_fonts_reply)

pub fn ListFontsWithInfo<'r> (c : &'r Connection,
                          max_names : u16,
                          pattern : &str) -> ListFontsWithInfoCookie<'r> {
  unsafe {
    let pattern = core::str::to_bytes(pattern);
    let pattern_len = pattern.len();
    let pattern_ptr = core::vec::raw::to_ptr(pattern);
    let cookie = xcb_list_fonts_with_info(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ListFontsWithInfoUnchecked<'r> (c : &'r Connection,
                                   max_names : u16,
                                   pattern : &str) -> ListFontsWithInfoCookie<'r> {
  unsafe {
    let pattern = core::str::to_bytes(pattern);
    let pattern_len = pattern.len();
    let pattern_ptr = core::vec::raw::to_ptr(pattern);
    let cookie = xcb_list_fonts_with_info_unchecked(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<list_fonts_with_info_reply> {
  fn min_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).min_bounds) }
  }
  fn max_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).max_bounds) }
  }
  fn min_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(min_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn max_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(max_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn default_char(&self) -> u16 {
    unsafe { accessor!(default_char -> u16, (*self.reply)) }
  }

  fn draw_direction(&self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.reply)) }
  }

  fn min_byte1(&self) -> u8 {
    unsafe { accessor!(min_byte1 -> u8, (*self.reply)) }
  }

  fn max_byte1(&self) -> u8 {
    unsafe { accessor!(max_byte1 -> u8, (*self.reply)) }
  }

  fn all_chars_exist(&self) -> u8 {
    unsafe { accessor!(all_chars_exist -> u8, (*self.reply)) }
  }

  fn font_ascent(&self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.reply)) }
  }

  fn font_descent(&self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.reply)) }
  }

  fn replies_hint(&self) -> u32 {
    unsafe { accessor!(replies_hint -> u32, (*self.reply)) }
  }

  fn properties(&self) -> FontpropIterator {
    unsafe { accessor!(FontpropIterator, xcb_list_fonts_with_info_properties_iterator, (*self.reply)) }
  }

  fn name(&self) -> ~str {
    unsafe { accessor!(str, xcb_list_fonts_with_info_name_length, xcb_list_fonts_with_info_name, (*self.reply)) }
  }

}
impl_reply_cookie!(ListFontsWithInfoCookie<'self>, list_fonts_with_info_reply, ListFontsWithInfoReply, xcb_list_fonts_with_info_reply)

pub fn SetFontPathChecked<'r> (c : &'r Connection,
                           font : &[Str]) -> base::VoidCookie<'r> {
  unsafe {
    let font_len = font.len();
    let font_ptr = core::vec::raw::to_ptr(font);
    let cookie = xcb_set_font_path_checked(c.get_raw_conn(),
        font_len as u16, //1
        font_ptr as *str_); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetFontPath<'r> (c : &'r Connection,
                    font : &[Str]) -> base::VoidCookie<'r> {
  unsafe {
    let font_len = font.len();
    let font_ptr = core::vec::raw::to_ptr(font);
    let cookie = xcb_set_font_path(c.get_raw_conn(),
        font_len as u16, //1
        font_ptr as *str_); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type GetFontPathReply = base::Reply<get_font_path_reply>;
pub fn GetFontPath<'r> (c : &'r Connection) -> GetFontPathCookie<'r> {
  unsafe {
    let cookie = xcb_get_font_path(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetFontPathUnchecked<'r> (c : &'r Connection) -> GetFontPathCookie<'r> {
  unsafe {
    let cookie = xcb_get_font_path_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_font_path_reply> {
  fn path(&self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_get_font_path_path_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(GetFontPathCookie<'self>, get_font_path_reply, GetFontPathReply, xcb_get_font_path_reply)

pub fn CreatePixmapChecked<'r> (c : &'r Connection,
                            depth : u8,
                            pid : Pixmap,
                            drawable : Drawable,
                            width : u16,
                            height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_pixmap_checked(c.get_raw_conn(),
        depth as u8, //1
        pid as pixmap, //2
        drawable as drawable, //3
        width as u16, //4
        height as u16); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreatePixmap<'r> (c : &'r Connection,
                     depth : u8,
                     pid : Pixmap,
                     drawable : Drawable,
                     width : u16,
                     height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_pixmap(c.get_raw_conn(),
        depth as u8, //1
        pid as pixmap, //2
        drawable as drawable, //3
        width as u16, //4
        height as u16); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreePixmapChecked<'r> (c : &'r Connection,
                          pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_pixmap_checked(c.get_raw_conn(),
        pixmap as pixmap); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreePixmap<'r> (c : &'r Connection,
                   pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_pixmap(c.get_raw_conn(),
        pixmap as pixmap); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CreateGcChecked<'r> (c : &'r Connection,
                        cid : Gcontext,
                        drawable : Drawable,
                        value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_create_gc_checked(c.get_raw_conn(),
        cid as gcontext, //1
        drawable as drawable, //2
        value_list_mask as u32, //3
        value_list_ptr as *u32); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateGc<'r> (c : &'r Connection,
                 cid : Gcontext,
                 drawable : Drawable,
                 value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_create_gc(c.get_raw_conn(),
        cid as gcontext, //1
        drawable as drawable, //2
        value_list_mask as u32, //3
        value_list_ptr as *u32); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ChangeGcChecked<'r> (c : &'r Connection,
                        gc : Gcontext,
                        value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_change_gc_checked(c.get_raw_conn(),
        gc as gcontext, //1
        value_list_mask as u32, //2
        value_list_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeGc<'r> (c : &'r Connection,
                 gc : Gcontext,
                 value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_change_gc(c.get_raw_conn(),
        gc as gcontext, //1
        value_list_mask as u32, //2
        value_list_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CopyGcChecked<'r> (c : &'r Connection,
                      src_gc : Gcontext,
                      dst_gc : Gcontext,
                      value_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_gc_checked(c.get_raw_conn(),
        src_gc as gcontext, //1
        dst_gc as gcontext, //2
        value_mask as u32); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CopyGc<'r> (c : &'r Connection,
               src_gc : Gcontext,
               dst_gc : Gcontext,
               value_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_gc(c.get_raw_conn(),
        src_gc as gcontext, //1
        dst_gc as gcontext, //2
        value_mask as u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetDashesChecked<'r> (c : &'r Connection,
                         gc : Gcontext,
                         dash_offset : u16,
                         dashes : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let dashes_len = dashes.len();
    let dashes_ptr = core::vec::raw::to_ptr(dashes);
    let cookie = xcb_set_dashes_checked(c.get_raw_conn(),
        gc as gcontext, //1
        dash_offset as u16, //2
        dashes_len as u16, //3
        dashes_ptr as *u8); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetDashes<'r> (c : &'r Connection,
                  gc : Gcontext,
                  dash_offset : u16,
                  dashes : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let dashes_len = dashes.len();
    let dashes_ptr = core::vec::raw::to_ptr(dashes);
    let cookie = xcb_set_dashes(c.get_raw_conn(),
        gc as gcontext, //1
        dash_offset as u16, //2
        dashes_len as u16, //3
        dashes_ptr as *u8); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetClipRectanglesChecked<'r> (c : &'r Connection,
                                 ordering : u8,
                                 gc : Gcontext,
                                 clip_x_origin : i16,
                                 clip_y_origin : i16,
                                 rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = core::vec::raw::to_ptr(rectangles);
    let cookie = xcb_set_clip_rectangles_checked(c.get_raw_conn(),
        ordering as u8, //1
        gc as gcontext, //2
        clip_x_origin as i16, //3
        clip_y_origin as i16, //4
        rectangles_len as u32, //5
        rectangles_ptr as *rectangle); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetClipRectangles<'r> (c : &'r Connection,
                          ordering : u8,
                          gc : Gcontext,
                          clip_x_origin : i16,
                          clip_y_origin : i16,
                          rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = core::vec::raw::to_ptr(rectangles);
    let cookie = xcb_set_clip_rectangles(c.get_raw_conn(),
        ordering as u8, //1
        gc as gcontext, //2
        clip_x_origin as i16, //3
        clip_y_origin as i16, //4
        rectangles_len as u32, //5
        rectangles_ptr as *rectangle); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreeGcChecked<'r> (c : &'r Connection,
                      gc : Gcontext) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_gc_checked(c.get_raw_conn(),
        gc as gcontext); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreeGc<'r> (c : &'r Connection,
               gc : Gcontext) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_gc(c.get_raw_conn(),
        gc as gcontext); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ClearAreaChecked<'r> (c : &'r Connection,
                         exposures : u8,
                         window : Window,
                         x : i16,
                         y : i16,
                         width : u16,
                         height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_clear_area_checked(c.get_raw_conn(),
        exposures as u8, //1
        window as window, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ClearArea<'r> (c : &'r Connection,
                  exposures : u8,
                  window : Window,
                  x : i16,
                  y : i16,
                  width : u16,
                  height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_clear_area(c.get_raw_conn(),
        exposures as u8, //1
        window as window, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CopyAreaChecked<'r> (c : &'r Connection,
                        src_drawable : Drawable,
                        dst_drawable : Drawable,
                        gc : Gcontext,
                        src_x : i16,
                        src_y : i16,
                        dst_x : i16,
                        dst_y : i16,
                        width : u16,
                        height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_area_checked(c.get_raw_conn(),
        src_drawable as drawable, //1
        dst_drawable as drawable, //2
        gc as gcontext, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16); //9
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CopyArea<'r> (c : &'r Connection,
                 src_drawable : Drawable,
                 dst_drawable : Drawable,
                 gc : Gcontext,
                 src_x : i16,
                 src_y : i16,
                 dst_x : i16,
                 dst_y : i16,
                 width : u16,
                 height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_area(c.get_raw_conn(),
        src_drawable as drawable, //1
        dst_drawable as drawable, //2
        gc as gcontext, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16); //9
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CopyPlaneChecked<'r> (c : &'r Connection,
                         src_drawable : Drawable,
                         dst_drawable : Drawable,
                         gc : Gcontext,
                         src_x : i16,
                         src_y : i16,
                         dst_x : i16,
                         dst_y : i16,
                         width : u16,
                         height : u16,
                         bit_plane : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_plane_checked(c.get_raw_conn(),
        src_drawable as drawable, //1
        dst_drawable as drawable, //2
        gc as gcontext, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16, //9
        bit_plane as u32); //10
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CopyPlane<'r> (c : &'r Connection,
                  src_drawable : Drawable,
                  dst_drawable : Drawable,
                  gc : Gcontext,
                  src_x : i16,
                  src_y : i16,
                  dst_x : i16,
                  dst_y : i16,
                  width : u16,
                  height : u16,
                  bit_plane : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_plane(c.get_raw_conn(),
        src_drawable as drawable, //1
        dst_drawable as drawable, //2
        gc as gcontext, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16, //9
        bit_plane as u32); //10
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyPointChecked<'r> (c : &'r Connection,
                         coordinate_mode : u8,
                         drawable : Drawable,
                         gc : Gcontext,
                         points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = core::vec::raw::to_ptr(points);
    let cookie = xcb_poly_point_checked(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        points_len as u32, //4
        points_ptr as *point); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyPoint<'r> (c : &'r Connection,
                  coordinate_mode : u8,
                  drawable : Drawable,
                  gc : Gcontext,
                  points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = core::vec::raw::to_ptr(points);
    let cookie = xcb_poly_point(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        points_len as u32, //4
        points_ptr as *point); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyLineChecked<'r> (c : &'r Connection,
                        coordinate_mode : u8,
                        drawable : Drawable,
                        gc : Gcontext,
                        points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = core::vec::raw::to_ptr(points);
    let cookie = xcb_poly_line_checked(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        points_len as u32, //4
        points_ptr as *point); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyLine<'r> (c : &'r Connection,
                 coordinate_mode : u8,
                 drawable : Drawable,
                 gc : Gcontext,
                 points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = core::vec::raw::to_ptr(points);
    let cookie = xcb_poly_line(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        points_len as u32, //4
        points_ptr as *point); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type Segment = base::Struct<segment>;


pub impl base::Struct<segment> {
  fn x1(&self) -> i16 {
    unsafe { accessor!(x1 -> i16, self.strct) }
  }

  fn y1(&self) -> i16 {
    unsafe { accessor!(y1 -> i16, self.strct) }
  }

  fn x2(&self) -> i16 {
    unsafe { accessor!(x2 -> i16, self.strct) }
  }

  fn y2(&self) -> i16 {
    unsafe { accessor!(y2 -> i16, self.strct) }
  }

}

impl<'self, Segment> Iterator<&'self Segment> for SegmentIterator {
    fn next(&mut self) -> Option<&'self Segment> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *segment_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_segment_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub fn PolySegmentChecked<'r> (c : &'r Connection,
                           drawable : Drawable,
                           gc : Gcontext,
                           segments : &[Segment]) -> base::VoidCookie<'r> {
  unsafe {
    let segments_len = segments.len();
    let segments_ptr = core::vec::raw::to_ptr(segments);
    let cookie = xcb_poly_segment_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        segments_len as u32, //3
        segments_ptr as *segment); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolySegment<'r> (c : &'r Connection,
                    drawable : Drawable,
                    gc : Gcontext,
                    segments : &[Segment]) -> base::VoidCookie<'r> {
  unsafe {
    let segments_len = segments.len();
    let segments_ptr = core::vec::raw::to_ptr(segments);
    let cookie = xcb_poly_segment(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        segments_len as u32, //3
        segments_ptr as *segment); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyRectangleChecked<'r> (c : &'r Connection,
                             drawable : Drawable,
                             gc : Gcontext,
                             rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = core::vec::raw::to_ptr(rectangles);
    let cookie = xcb_poly_rectangle_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        rectangles_len as u32, //3
        rectangles_ptr as *rectangle); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyRectangle<'r> (c : &'r Connection,
                      drawable : Drawable,
                      gc : Gcontext,
                      rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = core::vec::raw::to_ptr(rectangles);
    let cookie = xcb_poly_rectangle(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        rectangles_len as u32, //3
        rectangles_ptr as *rectangle); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyArcChecked<'r> (c : &'r Connection,
                       drawable : Drawable,
                       gc : Gcontext,
                       arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = core::vec::raw::to_ptr(arcs);
    let cookie = xcb_poly_arc_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        arcs_len as u32, //3
        arcs_ptr as *arc); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyArc<'r> (c : &'r Connection,
                drawable : Drawable,
                gc : Gcontext,
                arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = core::vec::raw::to_ptr(arcs);
    let cookie = xcb_poly_arc(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        arcs_len as u32, //3
        arcs_ptr as *arc); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FillPolyChecked<'r> (c : &'r Connection,
                        drawable : Drawable,
                        gc : Gcontext,
                        shape : u8,
                        coordinate_mode : u8,
                        points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = core::vec::raw::to_ptr(points);
    let cookie = xcb_fill_poly_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        shape as u8, //3
        coordinate_mode as u8, //4
        points_len as u32, //5
        points_ptr as *point); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FillPoly<'r> (c : &'r Connection,
                 drawable : Drawable,
                 gc : Gcontext,
                 shape : u8,
                 coordinate_mode : u8,
                 points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = core::vec::raw::to_ptr(points);
    let cookie = xcb_fill_poly(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        shape as u8, //3
        coordinate_mode as u8, //4
        points_len as u32, //5
        points_ptr as *point); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyFillRectangleChecked<'r> (c : &'r Connection,
                                 drawable : Drawable,
                                 gc : Gcontext,
                                 rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = core::vec::raw::to_ptr(rectangles);
    let cookie = xcb_poly_fill_rectangle_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        rectangles_len as u32, //3
        rectangles_ptr as *rectangle); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyFillRectangle<'r> (c : &'r Connection,
                          drawable : Drawable,
                          gc : Gcontext,
                          rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = core::vec::raw::to_ptr(rectangles);
    let cookie = xcb_poly_fill_rectangle(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        rectangles_len as u32, //3
        rectangles_ptr as *rectangle); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyFillArcChecked<'r> (c : &'r Connection,
                           drawable : Drawable,
                           gc : Gcontext,
                           arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = core::vec::raw::to_ptr(arcs);
    let cookie = xcb_poly_fill_arc_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        arcs_len as u32, //3
        arcs_ptr as *arc); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyFillArc<'r> (c : &'r Connection,
                    drawable : Drawable,
                    gc : Gcontext,
                    arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = core::vec::raw::to_ptr(arcs);
    let cookie = xcb_poly_fill_arc(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        arcs_len as u32, //3
        arcs_ptr as *arc); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PutImageChecked<'r> (c : &'r Connection,
                        format : u8,
                        drawable : Drawable,
                        gc : Gcontext,
                        width : u16,
                        height : u16,
                        dst_x : i16,
                        dst_y : i16,
                        left_pad : u8,
                        depth : u8,
                        data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = core::vec::raw::to_ptr(data);
    let cookie = xcb_put_image_checked(c.get_raw_conn(),
        format as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        width as u16, //4
        height as u16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        left_pad as u8, //8
        depth as u8, //9
        data_len as u32, //10
        data_ptr as *u8); //11
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PutImage<'r> (c : &'r Connection,
                 format : u8,
                 drawable : Drawable,
                 gc : Gcontext,
                 width : u16,
                 height : u16,
                 dst_x : i16,
                 dst_y : i16,
                 left_pad : u8,
                 depth : u8,
                 data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = core::vec::raw::to_ptr(data);
    let cookie = xcb_put_image(c.get_raw_conn(),
        format as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        width as u16, //4
        height as u16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        left_pad as u8, //8
        depth as u8, //9
        data_len as u32, //10
        data_ptr as *u8); //11
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type GetImageReply = base::Reply<get_image_reply>;
pub fn GetImage<'r> (c : &'r Connection,
                 format : u8,
                 drawable : Drawable,
                 x : i16,
                 y : i16,
                 width : u16,
                 height : u16,
                 plane_mask : u32) -> GetImageCookie<'r> {
  unsafe {
    let cookie = xcb_get_image(c.get_raw_conn(),
        format as u8, //1
        drawable as drawable, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16, //6
        plane_mask as u32); //7
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetImageUnchecked<'r> (c : &'r Connection,
                          format : u8,
                          drawable : Drawable,
                          x : i16,
                          y : i16,
                          width : u16,
                          height : u16,
                          plane_mask : u32) -> GetImageCookie<'r> {
  unsafe {
    let cookie = xcb_get_image_unchecked(c.get_raw_conn(),
        format as u8, //1
        drawable as drawable, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16, //6
        plane_mask as u32); //7
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_image_reply> {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.reply)) }
  }

  fn visual(&self) -> Visualid {
    unsafe { accessor!(visual -> Visualid, (*self.reply)) }
  }

  fn data(&self) -> ~[u8] {
    unsafe { accessor!(u8, xcb_get_image_data_length, xcb_get_image_data, (*self.reply)) }
  }

}
impl_reply_cookie!(GetImageCookie<'self>, get_image_reply, GetImageReply, xcb_get_image_reply)

pub fn PolyText8Checked<'r> (c : &'r Connection,
                         drawable : Drawable,
                         gc : Gcontext,
                         x : i16,
                         y : i16,
                         items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = core::vec::raw::to_ptr(items);
    let cookie = xcb_poly_text_8_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *u8); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyText8<'r> (c : &'r Connection,
                  drawable : Drawable,
                  gc : Gcontext,
                  x : i16,
                  y : i16,
                  items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = core::vec::raw::to_ptr(items);
    let cookie = xcb_poly_text_8(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *u8); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn PolyText16Checked<'r> (c : &'r Connection,
                          drawable : Drawable,
                          gc : Gcontext,
                          x : i16,
                          y : i16,
                          items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = core::vec::raw::to_ptr(items);
    let cookie = xcb_poly_text_16_checked(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *u8); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn PolyText16<'r> (c : &'r Connection,
                   drawable : Drawable,
                   gc : Gcontext,
                   x : i16,
                   y : i16,
                   items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = core::vec::raw::to_ptr(items);
    let cookie = xcb_poly_text_16(c.get_raw_conn(),
        drawable as drawable, //1
        gc as gcontext, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *u8); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ImageText8Checked<'r> (c : &'r Connection,
                          drawable : Drawable,
                          gc : Gcontext,
                          x : i16,
                          y : i16,
                          string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let string = core::str::to_bytes(string);
    let string_len = string.len();
    let string_ptr = core::vec::raw::to_ptr(string);
    let cookie = xcb_image_text_8_checked(c.get_raw_conn(),
        string_len as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *c_char); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ImageText8<'r> (c : &'r Connection,
                   drawable : Drawable,
                   gc : Gcontext,
                   x : i16,
                   y : i16,
                   string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let string = core::str::to_bytes(string);
    let string_len = string.len();
    let string_ptr = core::vec::raw::to_ptr(string);
    let cookie = xcb_image_text_8(c.get_raw_conn(),
        string_len as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *c_char); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ImageText16Checked<'r> (c : &'r Connection,
                           drawable : Drawable,
                           gc : Gcontext,
                           x : i16,
                           y : i16,
                           string : &[Char2b]) -> base::VoidCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = core::vec::raw::to_ptr(string);
    let cookie = xcb_image_text_16_checked(c.get_raw_conn(),
        string_len as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *char2b); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ImageText16<'r> (c : &'r Connection,
                    drawable : Drawable,
                    gc : Gcontext,
                    x : i16,
                    y : i16,
                    string : &[Char2b]) -> base::VoidCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = core::vec::raw::to_ptr(string);
    let cookie = xcb_image_text_16(c.get_raw_conn(),
        string_len as u8, //1
        drawable as drawable, //2
        gc as gcontext, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *char2b); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CreateColormapChecked<'r> (c : &'r Connection,
                              alloc : u8,
                              mid : Colormap,
                              window : Window,
                              visual : Visualid) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_colormap_checked(c.get_raw_conn(),
        alloc as u8, //1
        mid as colormap, //2
        window as window, //3
        visual as visualid); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateColormap<'r> (c : &'r Connection,
                       alloc : u8,
                       mid : Colormap,
                       window : Window,
                       visual : Visualid) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_colormap(c.get_raw_conn(),
        alloc as u8, //1
        mid as colormap, //2
        window as window, //3
        visual as visualid); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreeColormapChecked<'r> (c : &'r Connection,
                            cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_colormap_checked(c.get_raw_conn(),
        cmap as colormap); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreeColormap<'r> (c : &'r Connection,
                     cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_colormap(c.get_raw_conn(),
        cmap as colormap); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CopyColormapAndFreeChecked<'r> (c : &'r Connection,
                                   mid : Colormap,
                                   src_cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_colormap_and_free_checked(c.get_raw_conn(),
        mid as colormap, //1
        src_cmap as colormap); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CopyColormapAndFree<'r> (c : &'r Connection,
                            mid : Colormap,
                            src_cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_colormap_and_free(c.get_raw_conn(),
        mid as colormap, //1
        src_cmap as colormap); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn InstallColormapChecked<'r> (c : &'r Connection,
                               cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_install_colormap_checked(c.get_raw_conn(),
        cmap as colormap); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn InstallColormap<'r> (c : &'r Connection,
                        cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_install_colormap(c.get_raw_conn(),
        cmap as colormap); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn UninstallColormapChecked<'r> (c : &'r Connection,
                                 cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_uninstall_colormap_checked(c.get_raw_conn(),
        cmap as colormap); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn UninstallColormap<'r> (c : &'r Connection,
                          cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_uninstall_colormap(c.get_raw_conn(),
        cmap as colormap); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type ListInstalledColormapsReply = base::Reply<list_installed_colormaps_reply>;
pub fn ListInstalledColormaps<'r> (c : &'r Connection,
                               window : Window) -> ListInstalledColormapsCookie<'r> {
  unsafe {
    let cookie = xcb_list_installed_colormaps(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ListInstalledColormapsUnchecked<'r> (c : &'r Connection,
                                        window : Window) -> ListInstalledColormapsCookie<'r> {
  unsafe {
    let cookie = xcb_list_installed_colormaps_unchecked(c.get_raw_conn(),
        window as window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<list_installed_colormaps_reply> {
  fn cmaps(&self) -> ~[Colormap] {
    unsafe { accessor!(Colormap, xcb_list_installed_colormaps_cmaps_length, xcb_list_installed_colormaps_cmaps, (*self.reply)) }
  }

}
impl_reply_cookie!(ListInstalledColormapsCookie<'self>, list_installed_colormaps_reply, ListInstalledColormapsReply, xcb_list_installed_colormaps_reply)

pub fn AllocColor<'r> (c : &'r Connection,
                   cmap : Colormap,
                   red : u16,
                   green : u16,
                   blue : u16) -> AllocColorCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color(c.get_raw_conn(),
        cmap as colormap, //1
        red as u16, //2
        green as u16, //3
        blue as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn AllocColorUnchecked<'r> (c : &'r Connection,
                            cmap : Colormap,
                            red : u16,
                            green : u16,
                            blue : u16) -> AllocColorCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_unchecked(c.get_raw_conn(),
        cmap as colormap, //1
        red as u16, //2
        green as u16, //3
        blue as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<alloc_color_reply> {
  fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, (*self.reply)) }
  }

  fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, (*self.reply)) }
  }

  fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, (*self.reply)) }
  }

  fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocColorCookie<'self>, alloc_color_reply, AllocColorReply, xcb_alloc_color_reply)

pub fn AllocNamedColor<'r> (c : &'r Connection,
                        cmap : Colormap,
                        name : &str) -> AllocNamedColorCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_alloc_named_color(c.get_raw_conn(),
        cmap as colormap, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn AllocNamedColorUnchecked<'r> (c : &'r Connection,
                                 cmap : Colormap,
                                 name : &str) -> AllocNamedColorCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_alloc_named_color_unchecked(c.get_raw_conn(),
        cmap as colormap, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<alloc_named_color_reply> {
  fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, (*self.reply)) }
  }

  fn exact_red(&self) -> u16 {
    unsafe { accessor!(exact_red -> u16, (*self.reply)) }
  }

  fn exact_green(&self) -> u16 {
    unsafe { accessor!(exact_green -> u16, (*self.reply)) }
  }

  fn exact_blue(&self) -> u16 {
    unsafe { accessor!(exact_blue -> u16, (*self.reply)) }
  }

  fn visual_red(&self) -> u16 {
    unsafe { accessor!(visual_red -> u16, (*self.reply)) }
  }

  fn visual_green(&self) -> u16 {
    unsafe { accessor!(visual_green -> u16, (*self.reply)) }
  }

  fn visual_blue(&self) -> u16 {
    unsafe { accessor!(visual_blue -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocNamedColorCookie<'self>, alloc_named_color_reply, AllocNamedColorReply, xcb_alloc_named_color_reply)

pub type AllocColorCellsReply = base::Reply<alloc_color_cells_reply>;
pub fn AllocColorCells<'r> (c : &'r Connection,
                        contiguous : u8,
                        cmap : Colormap,
                        colors : u16,
                        planes : u16) -> AllocColorCellsCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_cells(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as colormap, //2
        colors as u16, //3
        planes as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn AllocColorCellsUnchecked<'r> (c : &'r Connection,
                                 contiguous : u8,
                                 cmap : Colormap,
                                 colors : u16,
                                 planes : u16) -> AllocColorCellsCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_cells_unchecked(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as colormap, //2
        colors as u16, //3
        planes as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<alloc_color_cells_reply> {
  fn pixels(&self) -> ~[u32] {
    unsafe { accessor!(u32, xcb_alloc_color_cells_pixels_length, xcb_alloc_color_cells_pixels, (*self.reply)) }
  }

  fn masks(&self) -> ~[u32] {
    unsafe { accessor!(u32, xcb_alloc_color_cells_masks_length, xcb_alloc_color_cells_masks, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocColorCellsCookie<'self>, alloc_color_cells_reply, AllocColorCellsReply, xcb_alloc_color_cells_reply)

pub type AllocColorPlanesReply = base::Reply<alloc_color_planes_reply>;
pub fn AllocColorPlanes<'r> (c : &'r Connection,
                         contiguous : u8,
                         cmap : Colormap,
                         colors : u16,
                         reds : u16,
                         greens : u16,
                         blues : u16) -> AllocColorPlanesCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_planes(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as colormap, //2
        colors as u16, //3
        reds as u16, //4
        greens as u16, //5
        blues as u16); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn AllocColorPlanesUnchecked<'r> (c : &'r Connection,
                                  contiguous : u8,
                                  cmap : Colormap,
                                  colors : u16,
                                  reds : u16,
                                  greens : u16,
                                  blues : u16) -> AllocColorPlanesCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_planes_unchecked(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as colormap, //2
        colors as u16, //3
        reds as u16, //4
        greens as u16, //5
        blues as u16); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<alloc_color_planes_reply> {
  fn red_mask(&self) -> u32 {
    unsafe { accessor!(red_mask -> u32, (*self.reply)) }
  }

  fn green_mask(&self) -> u32 {
    unsafe { accessor!(green_mask -> u32, (*self.reply)) }
  }

  fn blue_mask(&self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, (*self.reply)) }
  }

  fn pixels(&self) -> ~[u32] {
    unsafe { accessor!(u32, xcb_alloc_color_planes_pixels_length, xcb_alloc_color_planes_pixels, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocColorPlanesCookie<'self>, alloc_color_planes_reply, AllocColorPlanesReply, xcb_alloc_color_planes_reply)

pub fn FreeColorsChecked<'r> (c : &'r Connection,
                          cmap : Colormap,
                          plane_mask : u32,
                          pixels : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = core::vec::raw::to_ptr(pixels);
    let cookie = xcb_free_colors_checked(c.get_raw_conn(),
        cmap as colormap, //1
        plane_mask as u32, //2
        pixels_len as u32, //3
        pixels_ptr as *u32); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreeColors<'r> (c : &'r Connection,
                   cmap : Colormap,
                   plane_mask : u32,
                   pixels : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = core::vec::raw::to_ptr(pixels);
    let cookie = xcb_free_colors(c.get_raw_conn(),
        cmap as colormap, //1
        plane_mask as u32, //2
        pixels_len as u32, //3
        pixels_ptr as *u32); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Struct<coloritem> {
  fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, self.strct) }
  }

  fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, self.strct) }
  }

  fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, self.strct) }
  }

  fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, self.strct) }
  }

  fn flags(&self) -> u8 {
    unsafe { accessor!(flags -> u8, self.strct) }
  }

}

impl<'self, Coloritem> Iterator<&'self Coloritem> for ColoritemIterator {
    fn next(&mut self) -> Option<&'self Coloritem> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *coloritem_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_coloritem_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub fn StoreColorsChecked<'r> (c : &'r Connection,
                           cmap : Colormap,
                           items : &[Coloritem]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = core::vec::raw::to_ptr(items);
    let cookie = xcb_store_colors_checked(c.get_raw_conn(),
        cmap as colormap, //1
        items_len as u32, //2
        items_ptr as *coloritem); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn StoreColors<'r> (c : &'r Connection,
                    cmap : Colormap,
                    items : &[Coloritem]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = core::vec::raw::to_ptr(items);
    let cookie = xcb_store_colors(c.get_raw_conn(),
        cmap as colormap, //1
        items_len as u32, //2
        items_ptr as *coloritem); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn StoreNamedColorChecked<'r> (c : &'r Connection,
                               flags : u8,
                               cmap : Colormap,
                               pixel : u32,
                               name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_store_named_color_checked(c.get_raw_conn(),
        flags as u8, //1
        cmap as colormap, //2
        pixel as u32, //3
        name_len as u16, //4
        name_ptr as *c_char); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn StoreNamedColor<'r> (c : &'r Connection,
                        flags : u8,
                        cmap : Colormap,
                        pixel : u32,
                        name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_store_named_color(c.get_raw_conn(),
        flags as u8, //1
        cmap as colormap, //2
        pixel as u32, //3
        name_len as u16, //4
        name_ptr as *c_char); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type Rgb = base::Struct<rgb>;


pub impl base::Struct<rgb> {
  fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, self.strct) }
  }

  fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, self.strct) }
  }

  fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, self.strct) }
  }

}

impl<'self, Rgb> Iterator<&'self Rgb> for RgbIterator {
    fn next(&mut self) -> Option<&'self Rgb> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *rgb_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_rgb_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub fn QueryColors<'r> (c : &'r Connection,
                    cmap : Colormap,
                    pixels : &[u32]) -> QueryColorsCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = core::vec::raw::to_ptr(pixels);
    let cookie = xcb_query_colors(c.get_raw_conn(),
        cmap as colormap, //1
        pixels_len as u32, //2
        pixels_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryColorsUnchecked<'r> (c : &'r Connection,
                             cmap : Colormap,
                             pixels : &[u32]) -> QueryColorsCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = core::vec::raw::to_ptr(pixels);
    let cookie = xcb_query_colors_unchecked(c.get_raw_conn(),
        cmap as colormap, //1
        pixels_len as u32, //2
        pixels_ptr as *u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_colors_reply> {
  fn colors(&self) -> RgbIterator {
    unsafe { accessor!(RgbIterator, xcb_query_colors_colors_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryColorsCookie<'self>, query_colors_reply, QueryColorsReply, xcb_query_colors_reply)

pub fn LookupColor<'r> (c : &'r Connection,
                    cmap : Colormap,
                    name : &str) -> LookupColorCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_lookup_color(c.get_raw_conn(),
        cmap as colormap, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn LookupColorUnchecked<'r> (c : &'r Connection,
                             cmap : Colormap,
                             name : &str) -> LookupColorCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_lookup_color_unchecked(c.get_raw_conn(),
        cmap as colormap, //1
        name_len as u16, //2
        name_ptr as *c_char); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<lookup_color_reply> {
  fn exact_red(&self) -> u16 {
    unsafe { accessor!(exact_red -> u16, (*self.reply)) }
  }

  fn exact_green(&self) -> u16 {
    unsafe { accessor!(exact_green -> u16, (*self.reply)) }
  }

  fn exact_blue(&self) -> u16 {
    unsafe { accessor!(exact_blue -> u16, (*self.reply)) }
  }

  fn visual_red(&self) -> u16 {
    unsafe { accessor!(visual_red -> u16, (*self.reply)) }
  }

  fn visual_green(&self) -> u16 {
    unsafe { accessor!(visual_green -> u16, (*self.reply)) }
  }

  fn visual_blue(&self) -> u16 {
    unsafe { accessor!(visual_blue -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(LookupColorCookie<'self>, lookup_color_reply, LookupColorReply, xcb_lookup_color_reply)

pub fn CreateCursorChecked<'r> (c : &'r Connection,
                            cid : Cursor,
                            source : Pixmap,
                            mask : Pixmap,
                            fore_red : u16,
                            fore_green : u16,
                            fore_blue : u16,
                            back_red : u16,
                            back_green : u16,
                            back_blue : u16,
                            x : u16,
                            y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_cursor_checked(c.get_raw_conn(),
        cid as cursor, //1
        source as pixmap, //2
        mask as pixmap, //3
        fore_red as u16, //4
        fore_green as u16, //5
        fore_blue as u16, //6
        back_red as u16, //7
        back_green as u16, //8
        back_blue as u16, //9
        x as u16, //10
        y as u16); //11
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateCursor<'r> (c : &'r Connection,
                     cid : Cursor,
                     source : Pixmap,
                     mask : Pixmap,
                     fore_red : u16,
                     fore_green : u16,
                     fore_blue : u16,
                     back_red : u16,
                     back_green : u16,
                     back_blue : u16,
                     x : u16,
                     y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_cursor(c.get_raw_conn(),
        cid as cursor, //1
        source as pixmap, //2
        mask as pixmap, //3
        fore_red as u16, //4
        fore_green as u16, //5
        fore_blue as u16, //6
        back_red as u16, //7
        back_green as u16, //8
        back_blue as u16, //9
        x as u16, //10
        y as u16); //11
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CreateGlyphCursorChecked<'r> (c : &'r Connection,
                                 cid : Cursor,
                                 source_font : Font,
                                 mask_font : Font,
                                 source_char : u16,
                                 mask_char : u16,
                                 fore_red : u16,
                                 fore_green : u16,
                                 fore_blue : u16,
                                 back_red : u16,
                                 back_green : u16,
                                 back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_glyph_cursor_checked(c.get_raw_conn(),
        cid as cursor, //1
        source_font as font, //2
        mask_font as font, //3
        source_char as u16, //4
        mask_char as u16, //5
        fore_red as u16, //6
        fore_green as u16, //7
        fore_blue as u16, //8
        back_red as u16, //9
        back_green as u16, //10
        back_blue as u16); //11
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateGlyphCursor<'r> (c : &'r Connection,
                          cid : Cursor,
                          source_font : Font,
                          mask_font : Font,
                          source_char : u16,
                          mask_char : u16,
                          fore_red : u16,
                          fore_green : u16,
                          fore_blue : u16,
                          back_red : u16,
                          back_green : u16,
                          back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_glyph_cursor(c.get_raw_conn(),
        cid as cursor, //1
        source_font as font, //2
        mask_font as font, //3
        source_char as u16, //4
        mask_char as u16, //5
        fore_red as u16, //6
        fore_green as u16, //7
        fore_blue as u16, //8
        back_red as u16, //9
        back_green as u16, //10
        back_blue as u16); //11
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreeCursorChecked<'r> (c : &'r Connection,
                          cursor : Cursor) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_cursor_checked(c.get_raw_conn(),
        cursor as cursor); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreeCursor<'r> (c : &'r Connection,
                   cursor : Cursor) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_cursor(c.get_raw_conn(),
        cursor as cursor); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn RecolorCursorChecked<'r> (c : &'r Connection,
                             cursor : Cursor,
                             fore_red : u16,
                             fore_green : u16,
                             fore_blue : u16,
                             back_red : u16,
                             back_green : u16,
                             back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_recolor_cursor_checked(c.get_raw_conn(),
        cursor as cursor, //1
        fore_red as u16, //2
        fore_green as u16, //3
        fore_blue as u16, //4
        back_red as u16, //5
        back_green as u16, //6
        back_blue as u16); //7
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn RecolorCursor<'r> (c : &'r Connection,
                      cursor : Cursor,
                      fore_red : u16,
                      fore_green : u16,
                      fore_blue : u16,
                      back_red : u16,
                      back_green : u16,
                      back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_recolor_cursor(c.get_raw_conn(),
        cursor as cursor, //1
        fore_red as u16, //2
        fore_green as u16, //3
        fore_blue as u16, //4
        back_red as u16, //5
        back_green as u16, //6
        back_blue as u16); //7
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryBestSize<'r> (c : &'r Connection,
                      class : u8,
                      drawable : Drawable,
                      width : u16,
                      height : u16) -> QueryBestSizeCookie<'r> {
  unsafe {
    let cookie = xcb_query_best_size(c.get_raw_conn(),
        class as u8, //1
        drawable as drawable, //2
        width as u16, //3
        height as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryBestSizeUnchecked<'r> (c : &'r Connection,
                               class : u8,
                               drawable : Drawable,
                               width : u16,
                               height : u16) -> QueryBestSizeCookie<'r> {
  unsafe {
    let cookie = xcb_query_best_size_unchecked(c.get_raw_conn(),
        class as u8, //1
        drawable as drawable, //2
        width as u16, //3
        height as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_best_size_reply> {
  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.reply)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryBestSizeCookie<'self>, query_best_size_reply, QueryBestSizeReply, xcb_query_best_size_reply)

pub fn QueryExtension<'r> (c : &'r Connection,
                       name : &str) -> QueryExtensionCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_query_extension(c.get_raw_conn(),
        name_len as u16, //1
        name_ptr as *c_char); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryExtensionUnchecked<'r> (c : &'r Connection,
                                name : &str) -> QueryExtensionCookie<'r> {
  unsafe {
    let name = core::str::to_bytes(name);
    let name_len = name.len();
    let name_ptr = core::vec::raw::to_ptr(name);
    let cookie = xcb_query_extension_unchecked(c.get_raw_conn(),
        name_len as u16, //1
        name_ptr as *c_char); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<query_extension_reply> {
  fn present(&self) -> u8 {
    unsafe { accessor!(present -> u8, (*self.reply)) }
  }

  fn major_opcode(&self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.reply)) }
  }

  fn first_event(&self) -> u8 {
    unsafe { accessor!(first_event -> u8, (*self.reply)) }
  }

  fn first_error(&self) -> u8 {
    unsafe { accessor!(first_error -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryExtensionCookie<'self>, query_extension_reply, QueryExtensionReply, xcb_query_extension_reply)

pub type ListExtensionsReply = base::Reply<list_extensions_reply>;
pub fn ListExtensions<'r> (c : &'r Connection) -> ListExtensionsCookie<'r> {
  unsafe {
    let cookie = xcb_list_extensions(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ListExtensionsUnchecked<'r> (c : &'r Connection) -> ListExtensionsCookie<'r> {
  unsafe {
    let cookie = xcb_list_extensions_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<list_extensions_reply> {
  fn names(&self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_list_extensions_names_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(ListExtensionsCookie<'self>, list_extensions_reply, ListExtensionsReply, xcb_list_extensions_reply)

pub fn ChangeKeyboardMappingChecked<'r> (c : &'r Connection,
                                     first_keycode : Keycode,
                                     keysyms_per_keycode : u8,
                                     keysyms : &[Keysym]) -> base::VoidCookie<'r> {
  unsafe {
    let keysyms_len = keysyms.len();
    let keysyms_ptr = core::vec::raw::to_ptr(keysyms);
    let cookie = xcb_change_keyboard_mapping_checked(c.get_raw_conn(),
        keysyms_len as u8, //1
        first_keycode as keycode, //2
        keysyms_per_keycode as u8, //3
        keysyms_ptr as *keysym); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeKeyboardMapping<'r> (c : &'r Connection,
                              first_keycode : Keycode,
                              keysyms_per_keycode : u8,
                              keysyms : &[Keysym]) -> base::VoidCookie<'r> {
  unsafe {
    let keysyms_len = keysyms.len();
    let keysyms_ptr = core::vec::raw::to_ptr(keysyms);
    let cookie = xcb_change_keyboard_mapping(c.get_raw_conn(),
        keysyms_len as u8, //1
        first_keycode as keycode, //2
        keysyms_per_keycode as u8, //3
        keysyms_ptr as *keysym); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type GetKeyboardMappingReply = base::Reply<get_keyboard_mapping_reply>;
pub fn GetKeyboardMapping<'r> (c : &'r Connection,
                           first_keycode : Keycode,
                           count : u8) -> GetKeyboardMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_mapping(c.get_raw_conn(),
        first_keycode as keycode, //1
        count as u8); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetKeyboardMappingUnchecked<'r> (c : &'r Connection,
                                    first_keycode : Keycode,
                                    count : u8) -> GetKeyboardMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_mapping_unchecked(c.get_raw_conn(),
        first_keycode as keycode, //1
        count as u8); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_keyboard_mapping_reply> {
  fn keysyms_per_keycode(&self) -> u8 {
    unsafe { accessor!(keysyms_per_keycode -> u8, (*self.reply)) }
  }

  fn keysyms(&self) -> ~[Keysym] {
    unsafe { accessor!(Keysym, xcb_get_keyboard_mapping_keysyms_length, xcb_get_keyboard_mapping_keysyms, (*self.reply)) }
  }

}
impl_reply_cookie!(GetKeyboardMappingCookie<'self>, get_keyboard_mapping_reply, GetKeyboardMappingReply, xcb_get_keyboard_mapping_reply)

pub fn ChangeKeyboardControlChecked<'r> (c : &'r Connection,
                                     value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_change_keyboard_control_checked(c.get_raw_conn(),
        value_list_mask as u32, //1
        value_list_ptr as *u32); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeKeyboardControl<'r> (c : &'r Connection,
                              value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = core::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_change_keyboard_control(c.get_raw_conn(),
        value_list_mask as u32, //1
        value_list_ptr as *u32); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetKeyboardControl<'r> (c : &'r Connection) -> GetKeyboardControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_control(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetKeyboardControlUnchecked<'r> (c : &'r Connection) -> GetKeyboardControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_control_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_keyboard_control_reply> {
  fn global_auto_repeat(&self) -> u8 {
    unsafe { accessor!(global_auto_repeat -> u8, (*self.reply)) }
  }

  fn led_mask(&self) -> u32 {
    unsafe { accessor!(led_mask -> u32, (*self.reply)) }
  }

  fn key_click_percent(&self) -> u8 {
    unsafe { accessor!(key_click_percent -> u8, (*self.reply)) }
  }

  fn bell_percent(&self) -> u8 {
    unsafe { accessor!(bell_percent -> u8, (*self.reply)) }
  }

  fn bell_pitch(&self) -> u16 {
    unsafe { accessor!(bell_pitch -> u16, (*self.reply)) }
  }

  fn bell_duration(&self) -> u16 {
    unsafe { accessor!(bell_duration -> u16, (*self.reply)) }
  }

  fn auto_repeats(&self) -> ~[u8,..32] {
    unsafe { ~(copy (*self.reply).auto_repeats) }
  }

}
impl_reply_cookie!(GetKeyboardControlCookie<'self>, get_keyboard_control_reply, GetKeyboardControlReply, xcb_get_keyboard_control_reply)

pub fn BellChecked<'r> (c : &'r Connection,
                    percent : i8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_bell_checked(c.get_raw_conn(),
        percent as i8); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn Bell<'r> (c : &'r Connection,
             percent : i8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_bell(c.get_raw_conn(),
        percent as i8); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ChangePointerControlChecked<'r> (c : &'r Connection,
                                    acceleration_numerator : i16,
                                    acceleration_denominator : i16,
                                    threshold : i16,
                                    do_acceleration : u8,
                                    do_threshold : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_pointer_control_checked(c.get_raw_conn(),
        acceleration_numerator as i16, //1
        acceleration_denominator as i16, //2
        threshold as i16, //3
        do_acceleration as u8, //4
        do_threshold as u8); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangePointerControl<'r> (c : &'r Connection,
                             acceleration_numerator : i16,
                             acceleration_denominator : i16,
                             threshold : i16,
                             do_acceleration : u8,
                             do_threshold : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_pointer_control(c.get_raw_conn(),
        acceleration_numerator as i16, //1
        acceleration_denominator as i16, //2
        threshold as i16, //3
        do_acceleration as u8, //4
        do_threshold as u8); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetPointerControl<'r> (c : &'r Connection) -> GetPointerControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_control(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetPointerControlUnchecked<'r> (c : &'r Connection) -> GetPointerControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_control_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_pointer_control_reply> {
  fn acceleration_numerator(&self) -> u16 {
    unsafe { accessor!(acceleration_numerator -> u16, (*self.reply)) }
  }

  fn acceleration_denominator(&self) -> u16 {
    unsafe { accessor!(acceleration_denominator -> u16, (*self.reply)) }
  }

  fn threshold(&self) -> u16 {
    unsafe { accessor!(threshold -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(GetPointerControlCookie<'self>, get_pointer_control_reply, GetPointerControlReply, xcb_get_pointer_control_reply)

pub fn SetScreenSaverChecked<'r> (c : &'r Connection,
                              timeout : i16,
                              interval : i16,
                              prefer_blanking : u8,
                              allow_exposures : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_screen_saver_checked(c.get_raw_conn(),
        timeout as i16, //1
        interval as i16, //2
        prefer_blanking as u8, //3
        allow_exposures as u8); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetScreenSaver<'r> (c : &'r Connection,
                       timeout : i16,
                       interval : i16,
                       prefer_blanking : u8,
                       allow_exposures : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_screen_saver(c.get_raw_conn(),
        timeout as i16, //1
        interval as i16, //2
        prefer_blanking as u8, //3
        allow_exposures as u8); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetScreenSaver<'r> (c : &'r Connection) -> GetScreenSaverCookie<'r> {
  unsafe {
    let cookie = xcb_get_screen_saver(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetScreenSaverUnchecked<'r> (c : &'r Connection) -> GetScreenSaverCookie<'r> {
  unsafe {
    let cookie = xcb_get_screen_saver_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_screen_saver_reply> {
  fn timeout(&self) -> u16 {
    unsafe { accessor!(timeout -> u16, (*self.reply)) }
  }

  fn interval(&self) -> u16 {
    unsafe { accessor!(interval -> u16, (*self.reply)) }
  }

  fn prefer_blanking(&self) -> u8 {
    unsafe { accessor!(prefer_blanking -> u8, (*self.reply)) }
  }

  fn allow_exposures(&self) -> u8 {
    unsafe { accessor!(allow_exposures -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(GetScreenSaverCookie<'self>, get_screen_saver_reply, GetScreenSaverReply, xcb_get_screen_saver_reply)

pub fn ChangeHostsChecked<'r> (c : &'r Connection,
                           mode : u8,
                           family : u8,
                           address : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let address_len = address.len();
    let address_ptr = core::vec::raw::to_ptr(address);
    let cookie = xcb_change_hosts_checked(c.get_raw_conn(),
        mode as u8, //1
        family as u8, //2
        address_len as u16, //3
        address_ptr as *u8); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangeHosts<'r> (c : &'r Connection,
                    mode : u8,
                    family : u8,
                    address : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let address_len = address.len();
    let address_ptr = core::vec::raw::to_ptr(address);
    let cookie = xcb_change_hosts(c.get_raw_conn(),
        mode as u8, //1
        family as u8, //2
        address_len as u16, //3
        address_ptr as *u8); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type Host = base::Struct<host>;


pub impl base::Struct<host> {
  fn family(&self) -> u8 {
    unsafe { accessor!(family -> u8, self.strct) }
  }

  fn address(&self) -> ~[u8] {
    unsafe { accessor!(u8, xcb_host_address_length, xcb_host_address, self.strct) }
  }

}

impl<'self, Host> Iterator<&'self Host> for HostIterator {
    fn next(&mut self) -> Option<&'self Host> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *host_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_host_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type ListHostsReply = base::Reply<list_hosts_reply>;
pub fn ListHosts<'r> (c : &'r Connection) -> ListHostsCookie<'r> {
  unsafe {
    let cookie = xcb_list_hosts(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ListHostsUnchecked<'r> (c : &'r Connection) -> ListHostsCookie<'r> {
  unsafe {
    let cookie = xcb_list_hosts_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<list_hosts_reply> {
  fn mode(&self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.reply)) }
  }

  fn hosts(&self) -> HostIterator {
    unsafe { accessor!(HostIterator, xcb_list_hosts_hosts_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(ListHostsCookie<'self>, list_hosts_reply, ListHostsReply, xcb_list_hosts_reply)

pub fn SetAccessControlChecked<'r> (c : &'r Connection,
                                mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_access_control_checked(c.get_raw_conn(),
        mode as u8); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetAccessControl<'r> (c : &'r Connection,
                         mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_access_control(c.get_raw_conn(),
        mode as u8); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetCloseDownModeChecked<'r> (c : &'r Connection,
                                mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_close_down_mode_checked(c.get_raw_conn(),
        mode as u8); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetCloseDownMode<'r> (c : &'r Connection,
                         mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_close_down_mode(c.get_raw_conn(),
        mode as u8); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn KillClientChecked<'r> (c : &'r Connection,
                          resource : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_kill_client_checked(c.get_raw_conn(),
        resource as u32); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn KillClient<'r> (c : &'r Connection,
                   resource : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_kill_client(c.get_raw_conn(),
        resource as u32); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn RotatePropertiesChecked<'r> (c : &'r Connection,
                                window : Window,
                                delta : i16,
                                atoms : &[Atom]) -> base::VoidCookie<'r> {
  unsafe {
    let atoms_len = atoms.len();
    let atoms_ptr = core::vec::raw::to_ptr(atoms);
    let cookie = xcb_rotate_properties_checked(c.get_raw_conn(),
        window as window, //1
        atoms_len as u16, //2
        delta as i16, //3
        atoms_ptr as *atom); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn RotateProperties<'r> (c : &'r Connection,
                         window : Window,
                         delta : i16,
                         atoms : &[Atom]) -> base::VoidCookie<'r> {
  unsafe {
    let atoms_len = atoms.len();
    let atoms_ptr = core::vec::raw::to_ptr(atoms);
    let cookie = xcb_rotate_properties(c.get_raw_conn(),
        window as window, //1
        atoms_len as u16, //2
        delta as i16, //3
        atoms_ptr as *atom); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ForceScreenSaverChecked<'r> (c : &'r Connection,
                                mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_force_screen_saver_checked(c.get_raw_conn(),
        mode as u8); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ForceScreenSaver<'r> (c : &'r Connection,
                         mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_force_screen_saver(c.get_raw_conn(),
        mode as u8); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetPointerMapping<'r> (c : &'r Connection,
                          map : &[u8]) -> SetPointerMappingCookie<'r> {
  unsafe {
    let map_len = map.len();
    let map_ptr = core::vec::raw::to_ptr(map);
    let cookie = xcb_set_pointer_mapping(c.get_raw_conn(),
        map_len as u8, //1
        map_ptr as *u8); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetPointerMappingUnchecked<'r> (c : &'r Connection,
                                   map : &[u8]) -> SetPointerMappingCookie<'r> {
  unsafe {
    let map_len = map.len();
    let map_ptr = core::vec::raw::to_ptr(map);
    let cookie = xcb_set_pointer_mapping_unchecked(c.get_raw_conn(),
        map_len as u8, //1
        map_ptr as *u8); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<set_pointer_mapping_reply> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(SetPointerMappingCookie<'self>, set_pointer_mapping_reply, SetPointerMappingReply, xcb_set_pointer_mapping_reply)

pub type GetPointerMappingReply = base::Reply<get_pointer_mapping_reply>;
pub fn GetPointerMapping<'r> (c : &'r Connection) -> GetPointerMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_mapping(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetPointerMappingUnchecked<'r> (c : &'r Connection) -> GetPointerMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_mapping_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_pointer_mapping_reply> {
  fn map(&self) -> ~[u8] {
    unsafe { accessor!(u8, xcb_get_pointer_mapping_map_length, xcb_get_pointer_mapping_map, (*self.reply)) }
  }

}
impl_reply_cookie!(GetPointerMappingCookie<'self>, get_pointer_mapping_reply, GetPointerMappingReply, xcb_get_pointer_mapping_reply)

pub fn SetModifierMapping<'r> (c : &'r Connection,
                           keycodes : &[Keycode]) -> SetModifierMappingCookie<'r> {
  unsafe {
    let keycodes_len = keycodes.len();
    let keycodes_ptr = core::vec::raw::to_ptr(keycodes);
    let cookie = xcb_set_modifier_mapping(c.get_raw_conn(),
        keycodes_len as u8, //1
        keycodes_ptr as *keycode); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetModifierMappingUnchecked<'r> (c : &'r Connection,
                                    keycodes : &[Keycode]) -> SetModifierMappingCookie<'r> {
  unsafe {
    let keycodes_len = keycodes.len();
    let keycodes_ptr = core::vec::raw::to_ptr(keycodes);
    let cookie = xcb_set_modifier_mapping_unchecked(c.get_raw_conn(),
        keycodes_len as u8, //1
        keycodes_ptr as *keycode); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<set_modifier_mapping_reply> {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(SetModifierMappingCookie<'self>, set_modifier_mapping_reply, SetModifierMappingReply, xcb_set_modifier_mapping_reply)

pub type GetModifierMappingReply = base::Reply<get_modifier_mapping_reply>;
pub fn GetModifierMapping<'r> (c : &'r Connection) -> GetModifierMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_modifier_mapping(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetModifierMappingUnchecked<'r> (c : &'r Connection) -> GetModifierMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_modifier_mapping_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

pub impl base::Reply<get_modifier_mapping_reply> {
  fn keycodes(&self) -> ~[Keycode] {
    unsafe { accessor!(Keycode, xcb_get_modifier_mapping_keycodes_length, xcb_get_modifier_mapping_keycodes, (*self.reply)) }
  }

}
impl_reply_cookie!(GetModifierMappingCookie<'self>, get_modifier_mapping_reply, GetModifierMappingReply, xcb_get_modifier_mapping_reply)

pub fn NoOperationChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_no_operation_checked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn NoOperation<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_no_operation(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

