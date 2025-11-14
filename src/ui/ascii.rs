pub const SPLASH: &[&str] = &[
    r"███╗   ██╗ ██████╗ ████████╗██╗   ██╗██╗   ██╗██╗███╗   ███╗",
    r"████╗  ██║██╔═══██╗╚══██╔══╝██║   ██║██║   ██║██║████╗ ████║",
    r"██╔██╗ ██║██║   ██║   ██║   ██║   ██║██║   ██║██║██╔████╔██║",
    r"██║╚██╗██║██║   ██║   ██║   ╚██╗ ██╔╝██║   ██║██║██║╚██╔╝██║",
    r"██║ ╚████║╚██████╔╝   ██║    ╚████╔╝ ╚██████╔╝██║██║ ╚═╝ ██║",
    r"╚═╝  ╚═══╝ ╚═════╝    ╚═╝     ╚═══╝   ╚═════╝ ╚═╝╚═╝     ╚═╝",
    r"",
    r"        A   T I N Y   R U S T   E D I T O R",
];

const COLORS: [&str; 6] = [
    "\x1b[31m", // Red
    "\x1b[33m", // Yellow
    "\x1b[32m", // Green
    "\x1b[36m", // Cyan
    "\x1b[34m", // Blue
    "\x1b[35m", // Magenta
];

pub fn print_rainbow_splash() {
    for (i, line) in SPLASH.iter().enumerate() {
        let color = COLORS[i % COLORS.len()];
        println!("{}{}{}", color, line, "\x1b[0m");
    }
}
