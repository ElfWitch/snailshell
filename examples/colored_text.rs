use crossterm::style::Stylize;
use snailshell::{snailprint, snailprint_s};


fn main() {
    snailprint("Works with color!!!".magenta());
    snailprint("Or really anything you would normally be able to print. Doesn't have to be a String".blue());
    snailprint_s(20294, 0.6);
    snailprint_s(-0.0005, 0.6);
}