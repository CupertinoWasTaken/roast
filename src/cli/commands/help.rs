use crate::colors::parse_colors;

const HELP_MESSAGE: &str = const_str::join!(
    &[
        "",
        "§lRoast§r CLI Guide",
        "",
        "┃  rem §8§o…beans §r§8->§r Remove packages from your config file",
        "┃  add §8§o…beans §r§8->§r Add packages to your config file",
        "┃  pour       §8->§r Build your config file",
        "┃  help       §8->§r Show this message",
        "",
    ],
    "\n"
);

pub fn help(_: &[String]) {
    println!("{}", parse_colors(HELP_MESSAGE))
}
