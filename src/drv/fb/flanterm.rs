use core::ffi::c_void;

pub const FLANTERM_MAX_ESC_VALUES: usize = 16;

#[repr(C)]
pub struct FlantermContext {
    pub tab_size: usize,
    pub autoflush: bool,
    pub cursor_enabled: bool,
    pub scroll_enabled: bool,
    pub control_sequence: bool,
    pub escape: bool,
    pub osc: bool,
    pub osc_escape: bool,
    pub rrr: bool,
    pub discard_next: bool,
    pub bold: bool,
    pub bg_bold: bool,
    pub reverse_video: bool,
    pub dec_private: bool,
    pub insert_mode: bool,
    pub code_point: u64,
    pub unicode_remaining: usize,
    pub g_select: u8,
    pub charsets: [u8; 2],
    pub current_charset: usize,
    pub escape_offset: usize,
    pub esc_values_i: usize,
    pub saved_cursor_x: usize,
    pub saved_cursor_y: usize,
    pub current_primary: usize,
    pub current_bg: usize,
    pub scroll_top_margin: usize,
    pub scroll_bottom_margin: usize,
    pub esc_values: [u32; FLANTERM_MAX_ESC_VALUES],
    pub oob_output: u64,
    pub saved_state_bold: bool,
    pub saved_state_bg_bold: bool,
    pub saved_state_reverse_video: bool,
    pub saved_state_current_charset: usize,
    pub saved_state_current_primary: usize,
    pub saved_state_current_bg: usize,

    /* to be set by backend */
    pub rows: usize,
    pub cols: usize,

    pub raw_putchar: Option<unsafe extern "C" fn(*mut FlantermContext, u8)>,
    pub clear: Option<unsafe extern "C" fn(*mut FlantermContext, bool)>,
    pub set_cursor_pos: Option<unsafe extern "C" fn(*mut FlantermContext, usize, usize)>,
    pub get_cursor_pos: Option<unsafe extern "C" fn(*mut FlantermContext, *mut usize, *mut usize)>,
    pub set_text_fg: Option<unsafe extern "C" fn(*mut FlantermContext, usize)>,
    pub set_text_bg: Option<unsafe extern "C" fn(*mut FlantermContext, usize)>,
    pub set_text_fg_bright: Option<unsafe extern "C" fn(*mut FlantermContext, usize)>,
    pub set_text_bg_bright: Option<unsafe extern "C" fn(*mut FlantermContext, usize)>,
    pub set_text_fg_rgb: Option<unsafe extern "C" fn(*mut FlantermContext, u32)>,
    pub set_text_bg_rgb: Option<unsafe extern "C" fn(*mut FlantermContext, u32)>,
    pub set_text_fg_default: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub set_text_bg_default: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub set_text_fg_default_bright: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub set_text_bg_default_bright: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub move_character:
        Option<unsafe extern "C" fn(*mut FlantermContext, usize, usize, usize, usize)>,
    pub scroll: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub revscroll: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub swap_palette: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub save_state: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub restore_state: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub double_buffer_flush: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub full_refresh: Option<unsafe extern "C" fn(*mut FlantermContext)>,
    pub deinit: Option<
        unsafe extern "C" fn(*mut FlantermContext, unsafe extern "C" fn(*mut c_void, usize)),
    >,

    /* to be set by client */
    pub callback: Option<unsafe extern "C" fn(*mut FlantermContext, u64, u64, u64, u64)>,
}

unsafe extern "C" {
    pub fn flanterm_fb_init(
        /* If _malloc and _free are nulled, use the bump allocated instance (1 use only). */
        _malloc: Option<unsafe extern "C" fn(size: usize) -> *mut c_void>,
        _free: Option<unsafe extern "C" fn(ptr: *mut c_void, size: usize)>,
        framebuffer: *mut u32,
        width: usize,
        height: usize,
        pitch: usize,
        red_mask_size: u8,
        red_mask_shift: u8,
        green_mask_size: u8,
        green_mask_shift: u8,
        blue_mask_size: u8,
        blue_mask_shift: u8,
        canvas: *mut u32,              /* If nulled, no canvas. */
        ansi_colours: *mut u32,        /* If nulled, default. */
        ansi_bright_colours: *mut u32, /* If nulled, default. */
        default_bg: *mut u32,          /* If nulled, default. */
        default_fg: *mut u32,          /* If nulled, default. */
        default_bg_bright: *mut u32,   /* If nulled, default. */
        default_fg_bright: *mut u32,   /* If nulled, default. */
        /* If font is null, use default font and font_width and font_height ignored. */
        font: *mut c_void,
        font_width: usize,
        font_height: usize,
        font_spacing: usize,
        /* If scale_x and scale_y are 0, automatically scale font based on resolution. */
        font_scale_x: usize,
        font_scale_y: usize,
        margin: usize,
    ) -> *mut FlantermContext;

    pub fn flanterm_write(ctx: *mut FlantermContext, buf: *const u8, count: usize);
    pub fn flanterm_flush(ctx: *mut FlantermContext);
    pub fn flanterm_deinit(
        ctx: *mut FlantermContext,
        _free: Option<unsafe extern "C" fn(ptr: *mut c_void, size: usize)>,
    );
    pub fn flanterm_get_dimensions(ctx: *mut FlantermContext, cols: *mut usize, rows: *mut usize);
}
