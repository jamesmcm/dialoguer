//! Customizes the rendering of the elements.
use std::fmt;
use std::io;

use console::{style, Term};

/// Rendering style for a selected item
#[derive(Debug, Clone, Copy)]
pub enum SelectionStyle {
    /// Renders an unchecked but selected checkbox
    CheckboxUncheckedSelected,
    /// Renders an unchecked and unselected checkbox
    CheckboxUncheckedUnselected,
    /// Renders a checked but selected checkbox
    CheckboxCheckedSelected,
    /// Renders a checked and unselected checkbox
    CheckboxCheckedUnselected,
    /// Renders a selected menu item
    MenuSelected,
    /// Renders un unselected menu item
    MenuUnselected,
}

/// Implements a theme for dialoguer.
pub trait Theme {
    /// Given a prompt this formats out what the prompt should look like (multiline).
    fn format_prompt(&self, f: &mut fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    /// Given a prompt this formats out what the prompt should look like (singleline).
    fn format_singleline_prompt(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(f, "{} [{}]: ", prompt, default),
            None => write!(f, "{}: ", prompt),
        }
    }

    /// Formats out an error.
    fn format_error(&self, f: &mut fmt::Write, err: &str) -> fmt::Result {
        write!(f, "error: {}", err)
    }

    /// Formats a confirmation prompt.
    fn format_confirmation_prompt(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        write!(f, "{}", &prompt)?;
        match default {
            None => {}
            Some(true) => write!(f, " [Y/n] ")?,
            Some(false) => write!(f, " [y/N] ")?,
        }
        Ok(())
    }

    /// Formats a confirmation prompt.
    fn format_confirmation_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(f, "{} {}", &prompt, if selection { "yes" } else { "no" })
    }

    /// Renders a prompt and a single selection made.
    fn format_single_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, sel)
    }

    /// Renders a prompt and multiple selections,
    fn format_multi_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }

    /// Renders a prompt and multiple selections,
    fn format_password_prompt_selection(&self, f: &mut fmt::Write, prompt: &str) -> fmt::Result {
        self.format_single_prompt_selection(f, prompt, "[hidden]")
    }

    /// Formats a selection.
    fn format_selection(
        &self,
        f: &mut fmt::Write,
        text: &str,
        style: SelectionStyle,
    ) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match style {
                SelectionStyle::CheckboxUncheckedSelected => "> [ ] ",
                SelectionStyle::CheckboxUncheckedUnselected => "  [ ] ",
                SelectionStyle::CheckboxCheckedSelected => "> [x] ",
                SelectionStyle::CheckboxCheckedUnselected => "  [x] ",
                SelectionStyle::MenuSelected => "> ",
                SelectionStyle::MenuUnselected => "  ",
            },
            text
        )
    }
}

/// The default theme.
pub struct SimpleTheme;

impl Theme for SimpleTheme {}

/// A colorful theme
pub struct ColorfulTheme;

impl Theme for ColorfulTheme {
    fn format_prompt(&self, f: &mut fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    fn format_singleline_prompt(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(f, "{} [{}]: ", prompt, style(default).dim()),
            None => write!(f, "{}: ", prompt),
        }
    }

    fn format_error(&self, f: &mut fmt::Write, err: &str) -> fmt::Result {
        write!(f, "{}: {}", style("error").red(), err)
    }

    fn format_confirmation_prompt(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        write!(f, "{}", &prompt)?;
        match default {
            None => {}
            Some(true) => write!(f, " {} ", style("[Y/n]").dim())?,
            Some(false) => write!(f, " {} ", style("[y/N]").dim())?,
        }
        Ok(())
    }

    fn format_confirmation_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &prompt,
            if selection {
                style("yes").green()
            } else {
                style("no").red()
            }
        )
    }

    fn format_single_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, style(sel).cyan())
    }

    fn format_multi_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if idx == 0 { "" } else { ", " },
                style(sel).cyan()
            )?;
        }
        Ok(())
    }

    fn format_selection(&self, f: &mut fmt::Write, text: &str, st: SelectionStyle) -> fmt::Result {
        match st {
            SelectionStyle::CheckboxUncheckedSelected => {
                write!(f, "{} [ ] {}", style(">").cyan().bold(), text)
            }
            SelectionStyle::CheckboxUncheckedUnselected => write!(f, "  [ ] {}", style(text).dim()),
            SelectionStyle::CheckboxCheckedSelected => write!(
                f,
                "{} [{}] {}",
                style(">").cyan().bold(),
                style("x").green().bold(),
                text
            ),
            SelectionStyle::CheckboxCheckedUnselected => {
                write!(f, "  [{}] {}", style("x").green().bold(), style(text).dim())
            }
            SelectionStyle::MenuSelected => write!(f, "{} {}", style(">").cyan().bold(), text),
            SelectionStyle::MenuUnselected => write!(f, "  {}", style(text).dim()),
        }
    }
}

/// Helper struct to conveniently render a theme ot a term.
pub(crate) struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term: term,
            theme: theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    pub fn set_prompts_reset_height(&mut self, val: bool) {
        self.prompts_reset_height = val;
    }

    pub fn term(&self) -> &Term {
        self.term
    }

    pub fn add_line(&mut self) {
        self.height += 1;
    }

    fn write_formatted_str<F: FnOnce(&mut TermThemeRenderer, &mut fmt::Write) -> fmt::Result>(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count();
        self.term.write_str(&buf)
    }

    fn write_formatted_line<F: FnOnce(&mut TermThemeRenderer, &mut fmt::Write) -> fmt::Result>(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        self.term.write_line(&buf)
    }

    fn write_formatted_prompt<F: FnOnce(&mut TermThemeRenderer, &mut fmt::Write) -> fmt::Result>(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    pub fn error(&mut self, err: &str) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_error(buf, err))
    }

    pub fn prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_prompt(buf, prompt))
    }

    pub fn input_prompt(&mut self, prompt: &str, default: Option<&str>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            this.theme.format_singleline_prompt(buf, prompt, default)
        })
    }

    pub fn password_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            write!(buf, "\r")?;
            this.theme.format_singleline_prompt(buf, prompt, None)
        })
    }

    pub fn confirmation_prompt(&mut self, prompt: &str, default: Option<bool>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            this.theme.format_confirmation_prompt(buf, prompt, default)
        })
    }

    pub fn confirmation_prompt_selection(&mut self, prompt: &str, sel: bool) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_confirmation_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn single_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_single_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn multi_prompt_selection(&mut self, prompt: &str, selections: &[&str]) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_multi_prompt_selection(buf, prompt, selections)
        })
    }

    pub fn password_prompt_selection(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_password_prompt_selection(buf, prompt)
        })
    }

    pub fn selection(&mut self, text: &str, style: SelectionStyle) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_selection(buf, text, style))
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self) -> io::Result<()> {
        self.term.clear_last_lines(self.height)?;
        self.height = 0;
        Ok(())
    }
}

/// Returns the default theme.
///
/// (This returns the simple theme)
pub fn get_default_theme() -> &'static Theme {
    &SimpleTheme
}
